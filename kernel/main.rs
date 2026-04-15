// Main file reached from boot.s
// Current implementation initializes GDT and IDT, enabline interrupts
// It also enables keyboard interrupts and can be tested
// After intialization goes into hlt
#![no_std]

use core::panic::PanicInfo;

mod drivers;
mod font;
mod gdt;
mod interrupts;
mod io;
mod keyboard;
mod memory;

use drivers::framebuffer;

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    framebuffer::init();
    gdt::gdt::init();
    framebuffer::print("GDT Initialized!\n");

    interrupts::idt::init();
    framebuffer::print("Interrupts Initialized");

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
