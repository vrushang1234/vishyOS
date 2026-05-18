// Main file reached from boot.s
// Current implementation initializes GDT and IDT, enabline interrupts
// It also enables keyboard interrupts and can be tested
// After intialization goes into hlt
#![no_std]

use core::panic::PanicInfo;

mod arch;
mod drivers;
mod font;
mod gdt;
mod interrupts;
mod memory;
mod shell;

use core::fmt::Write;
use drivers::colors::{Color, get_color};
use drivers::framebuffer;

struct FbWriter;
impl Write for FbWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        framebuffer::print(s, 0xFFFFFFFF);
        Ok(())
    }
}

unsafe fn enable_sse() {
    let mut cr0: u64;

    core::arch::asm!("mov {}, cr0", out(reg) cr0);
    cr0 &= !(1u64 << 2);
    cr0 |= 1u64 << 1;
    core::arch::asm!("mov cr0, {}", in(reg) cr0);

    let mut cr4: u64;

    core::arch::asm!("mov {}, cr4", out(reg) cr4);
    cr4 |= (1u64 << 9) | (1u64 << 10);
    core::arch::asm!("mov cr4, {}", in(reg) cr4);
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    unsafe {
        enable_sse();
    };
    let white_font: u32 = get_color(Color::White);
    let orange_font: u32 = get_color(Color::Orange);

    framebuffer::init();
    gdt::gdt::init();
    framebuffer::print("GDT Initialized!\n", white_font);

    interrupts::idt::init();
    framebuffer::print("Interrupts Initialized\n", white_font);
    memory::mmu::init_memory();

    framebuffer::print(
        "##    ## ######  ######  ##  ## ##    ##  ######  ######\n\
        ##    ##   ##   ##       ##  ##  ##  ##  ##    ## ##     \n\
        ##    ##   ##    #####   ######   ####   ##    ##  #####\n\
        ##  ##    ##        ##  ##  ##    ##    ##    ##      ##\n\
        ####   ######  ######   ##  ##    ##    ######  ######\n",
        orange_font,
    );

    let mut w = FbWriter;
    for i in 0..4 {
        match memory::mmu::alloc_frame() {
            Some(addr) => {
                let _ = write!(w, "frame {} = {:#x}\n", i, addr);
            }
            None => {
                let _ = write!(w, "frame {} = OOM\n", i);
                break;
            }
        }
    }

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
