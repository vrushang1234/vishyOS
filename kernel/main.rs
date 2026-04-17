// Main file reached from boot.s
// Current implementation initializes GDT and IDT, enabline interrupts
// It also enables keyboard interrupts and can be tested
// After intialization goes into hlt
#![no_std]

use core::panic::PanicInfo;

mod colors;
mod drivers;
mod font;
mod gdt;
mod interrupts;
mod io;
mod keyboard;
mod memory;

use colors::{Color, get_color};
use drivers::framebuffer;

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    let white_font: u32 = get_color(Color::White);
    let orange_font: u32 = get_color(Color::Orange);

    framebuffer::init();
    gdt::gdt::init();
    framebuffer::print("GDT Initialized!\n", white_font);

    interrupts::idt::init();
    framebuffer::print("Interrupts Initialized\n", white_font);
    framebuffer::print(
        "##    ## ######  ######  ##  ## ##    ##  ######  ######\n\
        ##    ##   ##   ##       ##  ##  ##  ##  ##    ## ##     \n\
        ##    ##   ##    #####   ######   ####   ##    ##  #####\n\
        ##  ##    ##        ##  ##  ##    ##    ##    ##      ##\n\
        ####   ######  ######   ##  ##    ##    ######  ######\n",
        orange_font,
    );

    framebuffer::print("\n> ", white_font);

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
