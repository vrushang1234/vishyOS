#![no_std]

use core::panic::PanicInfo;

mod drivers;
mod font;
mod gdt;
mod interrupts;
mod io;
mod memory;

use drivers::framebuffer;

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    framebuffer::init();
    gdt::gdt::init();
    framebuffer::print("GDT Initialized!\n");

    unsafe {
        interrupts::pic::init();
    }
    framebuffer::print("PICs initialized");

    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
