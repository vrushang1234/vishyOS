use super::pic;
use core::arch::global_asm;

// Register save frame pushed by our stubs + the CPU-pushed exception frame.
// [rsp + n * 8]
// Where n:
//  1  rax
//  2  rcx
//  3  rdx
//  4  rbx
//  5  _rsp_unused    (stale value during push; not restored)
//  6  rbp
//  7  rsi
//  8  rdi
//  9  r8
// 10  r9
// 11  r10
// 12  r11
// 13  r12
// 14  r13
// 15  r14
// 16  r15
// 17  interrupt_number
// 18  error_code
// 19  rip
// 20  cs
// 21  rflags
// 22  rsp      (original RSP)
// 23  ss

#[repr(C)]
pub struct InterruptFrame {
    pub rax: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rbx: u64,
    pub _rsp_unused: u64,
    pub rbp: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub interrupt_number: u64,
    pub error_code: u64,

    // CPU-pushed exception frame
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
}

// Interrupt stubs
global_asm!(include_str!("isr_stubs.s"));

// Extern declarations for each stub
unsafe extern "C" {
    fn isr_stub_0();
    fn isr_stub_1();
    fn isr_stub_2();
    fn isr_stub_3();
    fn isr_stub_4();
    fn isr_stub_5();
    fn isr_stub_6();
    fn isr_stub_7();
    fn isr_stub_8();
    fn isr_stub_9();
    fn isr_stub_10();
    fn isr_stub_11();
    fn isr_stub_12();
    fn isr_stub_13();
    fn isr_stub_14();
    fn isr_stub_15();
    fn isr_stub_16();
    fn isr_stub_17();
    fn isr_stub_18();
    fn isr_stub_19();
    fn isr_stub_20();
    fn isr_stub_21();
    fn isr_stub_22();
    fn isr_stub_23();
    fn isr_stub_24();
    fn isr_stub_25();
    fn isr_stub_26();
    fn isr_stub_27();
    fn isr_stub_28();
    fn isr_stub_29();
    fn isr_stub_30();
    fn isr_stub_31();
    fn isr_stub_32();
    fn isr_stub_33();
    fn isr_stub_34();
    fn isr_stub_35();
    fn isr_stub_36();
    fn isr_stub_37();
    fn isr_stub_38();
    fn isr_stub_39();
    fn isr_stub_40();
    fn isr_stub_41();
    fn isr_stub_42();
    fn isr_stub_43();
    fn isr_stub_44();
    fn isr_stub_45();
    fn isr_stub_46();
    fn isr_stub_47();
}

// Convenience array for loading all stubs into the IDT at once
static ISR_STUBS: [unsafe extern "C" fn(); 48] = [
    isr_stub_0,
    isr_stub_1,
    isr_stub_2,
    isr_stub_3,
    isr_stub_4,
    isr_stub_5,
    isr_stub_6,
    isr_stub_7,
    isr_stub_8,
    isr_stub_9,
    isr_stub_10,
    isr_stub_11,
    isr_stub_12,
    isr_stub_13,
    isr_stub_14,
    isr_stub_15,
    isr_stub_16,
    isr_stub_17,
    isr_stub_18,
    isr_stub_19,
    isr_stub_20,
    isr_stub_21,
    isr_stub_22,
    isr_stub_23,
    isr_stub_24,
    isr_stub_25,
    isr_stub_26,
    isr_stub_27,
    isr_stub_28,
    isr_stub_29,
    isr_stub_30,
    isr_stub_31,
    isr_stub_32,
    isr_stub_33,
    isr_stub_34,
    isr_stub_35,
    isr_stub_36,
    isr_stub_37,
    isr_stub_38,
    isr_stub_39,
    isr_stub_40,
    isr_stub_41,
    isr_stub_42,
    isr_stub_43,
    isr_stub_44,
    isr_stub_45,
    isr_stub_46,
    isr_stub_47,
];

#[derive(Clone, Copy)]
#[repr(C, packed)]
struct IdtEntry {
    offset_low: u16,  // handler[15:0]
    selector: u16,    // kernel code segment selector
    ist: u8,          // bits[2:0] = IST index
    type_attr: u8,    // P | DPL | 0 | gate type
    offset_mid: u16,  // handler[31:16]
    offset_high: u32, // handler[63:32]
    reserved: u32,
}

impl IdtEntry {
    const fn missing() -> Self {
        Self {
            offset_low: 0,
            selector: 0,
            ist: 0,
            type_attr: 0,
            offset_mid: 0,
            offset_high: 0,
            reserved: 0,
        }
    }

    fn new(handler: u64) -> Self {
        Self {
            offset_low: handler as u16,
            selector: 0x08,
            ist: 0,
            type_attr: 0x8E,
            offset_mid: (handler >> 16) as u16,
            offset_high: (handler >> 32) as u32,
            reserved: 0,
        }
    }
}

static mut IDT: [IdtEntry; 256] = [IdtEntry::missing(); 256];

#[repr(C, packed)]
struct IdtDescriptor {
    size: u16,
    offset: u64,
}

#[unsafe(no_mangle)]
extern "C" fn interrupt_dispatch(frame: &InterruptFrame) {
    let num = frame.interrupt_number;

    match num {
        // CPU exceptions (0-31)
        0..=31 => handle_exception(frame),
        34..=47 => {
            unsafe { pic::send_eoi((num - 32) as u8) };
        }
        _ => {}
    }
}

/// CPU exception handler - Print a message and halt
fn handle_exception(frame: &InterruptFrame) {
    crate::drivers::framebuffer::print("\nEXCEPTION: ");
    crate::drivers::framebuffer::print(exception_name(frame.interrupt_number as u8));
    loop {
        unsafe { core::arch::asm!("hlt") };
    }
}

fn exception_name(num: u8) -> &'static str {
    match num {
        0 => "DE Divide Error",
        1 => "DB Debug",
        2 => "NMI",
        3 => "BP Breakpoint",
        4 => "OF Overflow",
        5 => "BR Bound Range",
        6 => "UD Invalid Opcode",
        7 => "NM Device Not Available",
        8 => "DF Double Fault",
        9 => "Coprocessor Overrun",
        10 => "TS Invalid TSS",
        11 => "NP Segment Not Present",
        12 => "SS Stack Fault",
        13 => "GP General Protection Fault",
        14 => "PF Page Fault",
        16 => "MF x87 FPU Error",
        17 => "AC Alignment Check",
        18 => "MC Machine Check",
        19 => "XM SIMD Exception",
        20 => "VE Virtualization Exception",
        21 => "CP Control Protection",
        _ => "Unknown Exception",
    }
}

// Initialize IDT and load it, then initalize PIC and enable interrupts
pub fn init() {
    // Install stubs for vectors 0-47
    unsafe {
        for (i, &stub) in ISR_STUBS.iter().enumerate() {
            IDT[i] = IdtEntry::new(stub as u64);
        }
    }

    // Load the IDT
    let descriptor = IdtDescriptor {
        size: (core::mem::size_of::<[IdtEntry; 256]>() - 1) as u16,
        offset: core::ptr::addr_of!(IDT) as u64,
    };
    unsafe {
        core::arch::asm!(
            "lidt [{0}]",
            in(reg) &descriptor,
            options(readonly, nostack, preserves_flags)
        );
    }

    // Remap the PIC and unmask timer (IRQ0) and keyboard (IRQ1)
    unsafe {
        pic::init();
        pic::unmask_irq(0); // timer
        pic::unmask_irq(1); // keyboard
    }

    // Enable interrupts
    unsafe { core::arch::asm!("sti", options(nomem, nostack)) };
}
