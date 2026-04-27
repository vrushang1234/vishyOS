use crate::drivers::framebuffer;
use core::fmt::{self, Write};
use limine::memory_map::EntryType;
use limine::request::MemoryMapRequest;

#[used]
#[unsafe(link_section = ".requests")]
static MEMMAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();

struct FbWriter;

fn entry_type_name(entry_type: EntryType) -> &'static str {
    match entry_type {
        EntryType::USABLE => "Usable",
        EntryType::RESERVED => "Reserved",
        EntryType::ACPI_RECLAIMABLE => "ACPI Reclaimable",
        EntryType::ACPI_NVS => "ACPI NVS",
        EntryType::BAD_MEMORY => "Bad Memory",
        EntryType::BOOTLOADER_RECLAIMABLE => "Bootloader Reclaimable",
        EntryType::EXECUTABLE_AND_MODULES => "Executable and Modules",
        EntryType::FRAMEBUFFER => "Framebuffer",
        _ => "Unknown",
    }
}

impl Write for FbWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        framebuffer::print(s, 0xFFFFFFFF);
        Ok(())
    }
}
pub fn init_memory() {
    if MEMMAP_REQUEST.get_response().is_none() {
        framebuffer::print("No memory map\n", 0xFFFFFFFF);
    }
}

pub fn print_memory_map() {
    let Some(response) = MEMMAP_REQUEST.get_response() else {
        framebuffer::print("\nNo memory map\n", 0xFFFFFFFF);
        return;
    };

    let mut writer = FbWriter;
    let _ = write!(writer, "\n");
    for entry in response.entries().iter() {
        let _ = write!(
            writer,
            "base={:#x}, len={:#x}, kind={}\n",
            entry.base,
            entry.length,
            entry_type_name(entry.entry_type)
        );
    }
}
