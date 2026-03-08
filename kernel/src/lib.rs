#![no_std]

use core::panic::PanicInfo;

mod drivers;
mod font;
mod memory;

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    drivers::framebuffer::init();

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
