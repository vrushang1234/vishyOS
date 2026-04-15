use crate::io::{inb, io_wait, outb};

// PIC port addresses
const PIC1_CMD: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_CMD: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;

// Initialization command words
const ICW1_ICW4: u8 = 0x01; // ICW4 
const ICW1_INIT: u8 = 0x10; // Initialization
const ICW4_8086: u8 = 0x01; // 8086/88 mode

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
