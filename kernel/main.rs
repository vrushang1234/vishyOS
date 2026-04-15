// Main file reached from boot.s
// Current implementation initializes GDT and IDT, enabline interrupts
// Tests a divide by 0 error and if the interrupt/trap runs
// After intialization goes into hlt
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

    interrupts::idt::init();
    framebuffer::print("Interrupts Initialized");

    unsafe {
        core::arch::asm!("xor eax, eax", "div eax");
    }

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
