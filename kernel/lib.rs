#![no_std]

use core::panic::PanicInfo;

mod drivers;
mod font;
mod gdt;
mod io;
mod memory;

use drivers::framebuffer;

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    framebuffer::init();
    gdt::gdt::init();
    framebuffer::print("GDT Initialized!\n");

    unsafe {
        io::io_wait();
    }
    framebuffer::print("io_wait done\n");

    unsafe {
        io::outb(0x80, 0xAB);
    }
    framebuffer::print("outb done\n");

    let _val = unsafe { io::inb(0x80) };
    framebuffer::print("inb done\n");

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
