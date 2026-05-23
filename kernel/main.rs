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

    use memory::paging::{self, MapError, VirtAddr, flags};

    let pml4 = paging::alloc_table().expect("PML4 alloc");
    let _ = write!(w, "pml4 = {:#x}\n", pml4 as u64);

    let old_cr3 = paging::read_cr3();
    let _ = write!(w, "old cr3 = {:#x}\n", old_cr3);

    unsafe {
        let cur_pml4 = (old_cr3 & 0x000F_FFFF_FFFF_F000) as *const paging::PageTable;
        let mut empty_slot: Option<usize> = None;
        for i in 0..paging::ENTRY_COUNT {
            let e = (*cur_pml4).entries[i];
            (*pml4).entries[i] = e;
            if !e.is_present() && empty_slot.is_none() && i < 256 {
                empty_slot = Some(i);
            }
        }

        let slot = empty_slot.expect("no empty PML4 slot");
        let virt = VirtAddr(((slot as u64) << 39) | 0x12345);
        let _ = write!(w, "test virt = {:#x} (pml4 slot {})\n", virt.0, slot);

        let phys = memory::mmu::alloc_frame().expect("frame");
        let off: u64 = 0x123;

        match paging::map_page(pml4, virt, phys, flags::WRITABLE) {
            Ok(()) => {
                let _ = write!(w, "mapped {:#x} -> {:#x}\n", virt.0, phys);
            }
            Err(MapError::OutOfFrames) => {
                let _ = write!(w, "map OOM\n");
            }
            Err(MapError::AlreadyMapped) => {
                let _ = write!(w, "already mapped\n");
            }
        }

        paging::write_cr3(pml4 as u64);
        let _ = write!(w, "switched cr3 -> {:#x}\n", pml4 as u64);

        let aligned_virt = virt.0 & !0xFFF;
        let test_ptr = (aligned_virt + off) as *mut u64;
        *test_ptr = 0xCAFEBABE_DEADBEEF;
        let readback = *test_ptr;
        let _ = write!(w, "wrote {:#x}, read {:#x}\n", 0xCAFEBABE_DEADBEEFu64, readback);

        let phys_ptr = (phys + off) as *const u64;
        let _ = write!(w, "phys {:#x} contains {:#x}\n", phys + off, *phys_ptr);

        paging::write_cr3(old_cr3);
        let _ = write!(w, "restored cr3\n");
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
