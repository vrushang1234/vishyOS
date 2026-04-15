use crate::io::{inb, io_wait, outb};

////////// PIC port addresses //////////
// There are total of 2 PICs, one master and one slave
// Master PIC handles IRQs 0-7 and slae 8-15
// Each PIC has 2 ports CMD and DATA
// Master PIC Command Port:     0x20
// Master PIC Data Port:        0x21
// Slave PIC Command Port:      0xA0
// Slave PIC Data Port:         0xA1

const PIC1_CMD: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_CMD: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;

////////// Initialization command words //////////
// PICs are initalized in multiple steps with the follwing command words
// ICW1_ICW4: Tell PIC that ICW4 will follow
// ICW1_INIT: Start initialization
// ICW4_8086: Set PIC to 8086/88 mode
// PIC_EOI:   Notify end of handling an interrupt

const ICW1_ICW4: u8 = 0x01;
const ICW1_INIT: u8 = 0x10;
const ICW4_8086: u8 = 0x01;
const PIC_EOI: u8 = 0x20;

pub unsafe fn init() {
    unsafe {
        // Save existing masks
        let mask1 = inb(PIC1_DATA);
        let mask2 = inb(PIC2_DATA);

        // Start initialization sequence
        outb(PIC1_CMD, ICW1_INIT | ICW1_ICW4);
        io_wait();
        outb(PIC2_CMD, ICW1_INIT | ICW1_ICW4);
        io_wait();

        // ICW2: vector offsets
        outb(PIC1_DATA, 0x20);
        io_wait();
        outb(PIC2_DATA, 0x28);
        io_wait();

        // ICW3: cascade wiring
        outb(PIC1_DATA, 4);
        io_wait();
        outb(PIC2_DATA, 2);
        io_wait();

        // ICW4: 8086 mode
        outb(PIC1_DATA, ICW4_8086);
        io_wait();
        outb(PIC2_DATA, ICW4_8086);
        io_wait();

        // Restore saved masks
        outb(PIC1_DATA, mask1);
        outb(PIC2_DATA, mask2);
    }
}

// Send End-Of-Interrupt for the given IRQ number (0-based).
pub unsafe fn send_eoi(irq: u8) {
    unsafe {
        if irq >= 8 {
            outb(PIC2_CMD, PIC_EOI);
        }
        outb(PIC1_CMD, PIC_EOI);
    }
}

// Mask (disable) a specific IRQ line (0-based).
#[allow(dead_code)]
pub unsafe fn mask_irq(irq: u8) {
    unsafe {
        let (port, bit) = if irq < 8 {
            (PIC1_DATA, irq)
        } else {
            (PIC2_DATA, irq - 8)
        };
        let mask = inb(port) | (1 << bit);
        outb(port, mask);
    }
}

// Unmask (enable) a specific IRQ line (0-based).
pub unsafe fn unmask_irq(irq: u8) {
    unsafe {
        let (port, bit) = if irq < 8 {
            (PIC1_DATA, irq)
        } else {
            (PIC2_DATA, irq - 8)
        };
        let mask = inb(port) & !(1 << bit);
        outb(port, mask);
    }
}
