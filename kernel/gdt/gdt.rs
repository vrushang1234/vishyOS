////////// GDT Entry //////////
// Limit Low: Lower 16 bits of the segment size (ignored here for 64 bit)
// Limit High: Upper 4 bits of the segment size
// Base Low: Lower 16 bits of the base address (where the memory starts)
// Base Mid: Middle 8 bits of the base address
// Base High: Upper 8 bits of the base address

// Access: 8 bits that define the type of segment
//      Bit 7:      (Present) 1 the memory segment exists and 0 if it doesn't
//      Bits 6-5:   (Mode) 00 is Kernel Mode and 11 is User mode
//      Bit 4:      (Descriptor) 1 is Code/Data Segment and 0 is System Segment
//      Bit 3:      (Executable) 1 is Code segment and 0 is Data Segment
//      Bit 2:      (Direction) 0 grows upward and 1 grows downward like a stack (0 for Data and 1
//                  for Code)
//      Bit 1:      (Read/Write) 1 = Readable and 0 = Execute only (For code) 1 = Writable and 0 =
//                  Read only (For Data)
//      Bit 0:      (Accessed) 0 = Has not been used yet and 1 = Has alrady been used

// Flags: Upper 4 bits of Flags and Limit High
//      Bit 7:      (Granularity) 1 = Limit in 4KB blocks and 0 = Limit in Bytes
//      Bit 6:      (Default Size) of segments 1 = 32 bit segment and 0 = 16 bit segment (Ignored
//                  for 64 bits)
//      Bit 5:      (Long Mode) 1 = 64 bit mode
//      Bit 4:      (Available) Not used anymore, free bit

#[derive(Clone, Copy)]
#[repr(C, packed)]
struct GdtEntry {
    limit_low: u16,
    base_low: u16,
    base_mid: u8,
    access: u8,
    flags_and_limit_high: u8,
    base_high: u8,
}

impl GdtEntry {
    // Null GDT with all values 0
    // An invalid entry required to protect the system
    const fn null() -> Self {
        Self {
            limit_low: 0,
            base_low: 0,
            base_mid: 0,
            access: 0,
            flags_and_limit_high: 0,
            base_high: 0,
        }
    }

    // GDT for Kernel code that defines how kernel code is executed
    const fn kernel_code() -> Self {
        Self {
            limit_low: 0xFFFF,
            base_low: 0,
            base_mid: 0,
            access: 0x9A,
            flags_and_limit_high: 0xAF,
            base_high: 0,
        }
    }

    // Kernel Data, defines how kernel data is read and cannot be read by user processes
    const fn kernel_data() -> Self {
        Self {
            limit_low: 0xFFFF,
            base_low: 0,
            base_mid: 0,
            access: 0x92,
            flags_and_limit_high: 0xCF,
            base_high: 0,
        }
    }
}

// Descriptor that defines where the GDT entry is
#[repr(C, packed)]
struct GdtDescriptor {
    size: u16,
    offset: u64,
}

// GDT: A table/list of all GDT entries
static mut GDT: [GdtEntry; 3] = [
    GdtEntry::null(),
    GdtEntry::kernel_code(),
    GdtEntry::kernel_data(),
];

pub fn init() {
    let descriptor = GdtDescriptor {
        size: (core::mem::size_of::<[GdtEntry; 3]>() - 1) as u16,
        offset: core::ptr::addr_of!(GDT) as u64,
    };
}
