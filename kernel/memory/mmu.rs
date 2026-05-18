use crate::drivers::framebuffer;
use core::fmt::{self, Write};
use limine::memory_map::EntryType;
use limine::request::MemoryMapRequest;

#[used]
#[unsafe(link_section = ".requests")]
static MEMMAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();

pub const FRAME_SIZE: u64 = 4096;

#[derive(Copy, Clone)]
pub struct MemRegion {
    pub base: u64,
    pub length: u64,
    pub frames: u64,
}

const MAX_REGIONS: usize = 64;
static mut USABLE_REGIONS: [MemRegion; MAX_REGIONS] = [MemRegion {
    base: 0,
    length: 0,
    frames: 0,
}; MAX_REGIONS];
static mut USABLE_COUNT: usize = 0;
static mut TOTAL_FRAMES: u64 = 0;

static mut NEXT_REGION: usize = 0;
static mut NEXT_FRAME_IN_REGION: u64 = 0;

struct FbWriter;

impl Write for FbWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        framebuffer::print(s, 0xFFFFFFFF);
        Ok(())
    }
}

pub fn init_memory() {
    let Some(response) = MEMMAP_REQUEST.get_response() else {
        framebuffer::print("No memory map\n", 0xFFFFFFFF);
        return;
    };

    let mut count = 0;
    let mut total_frames: u64 = 0;

    for entry in response
        .entries()
        .iter()
        .filter(|e| e.entry_type == EntryType::USABLE)
    {
        if count >= MAX_REGIONS {
            break;
        }

        let aligned_base = (entry.base + FRAME_SIZE - 1) & !(FRAME_SIZE - 1);
        let end = entry.base + entry.length;
        let aligned_end = end & !(FRAME_SIZE - 1);
        if aligned_end <= aligned_base {
            continue;
        }
        let length = aligned_end - aligned_base;
        let frames = length / FRAME_SIZE;

        unsafe {
            USABLE_REGIONS[count] = MemRegion {
                base: aligned_base,
                length,
                frames,
            };
        }
        total_frames += frames;
        count += 1;
    }

    unsafe {
        USABLE_COUNT = count;
        TOTAL_FRAMES = total_frames;
        NEXT_REGION = 0;
        NEXT_FRAME_IN_REGION = 0;
    }
}

pub fn usable_regions() -> &'static [MemRegion] {
    unsafe { &USABLE_REGIONS[..USABLE_COUNT] }
}

pub fn total_frames() -> u64 {
    unsafe { TOTAL_FRAMES }
}

pub fn total_usable_bytes() -> u64 {
    total_frames() * FRAME_SIZE
}

pub fn alloc_frame() -> Option<u64> {
    unsafe {
        while NEXT_REGION < USABLE_COUNT {
            let r = USABLE_REGIONS[NEXT_REGION];
            if NEXT_FRAME_IN_REGION < r.frames {
                let addr = r.base + NEXT_FRAME_IN_REGION * FRAME_SIZE;
                NEXT_FRAME_IN_REGION += 1;
                return Some(addr);
            }
            NEXT_REGION += 1;
            NEXT_FRAME_IN_REGION = 0;
        }
        None
    }
}

pub fn print_memory_map() {
    let mut writer = FbWriter;
    let _ = write!(writer, "\n");
    for r in usable_regions() {
        let _ = write!(
            writer,
            "base={:#x} len={:#x} frames={}\n",
            r.base, r.length, r.frames
        );
    }
    let _ = write!(
        writer,
        "total frames={} ({:#x} bytes)\n",
        total_frames(),
        total_usable_bytes()
    );
}
