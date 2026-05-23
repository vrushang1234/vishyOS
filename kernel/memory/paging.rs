use crate::memory::mmu;

pub const PAGE_SIZE: u64 = 4096;
pub const ENTRY_COUNT: usize = 512;

pub mod flags {
    pub const PRESENT: u64 = 1 << 0;
    pub const WRITABLE: u64 = 1 << 1;
    pub const USER: u64 = 1 << 2;
    pub const WRITE_THROUGH: u64 = 1 << 3;
    pub const NO_CACHE: u64 = 1 << 4;
    pub const ACCESSED: u64 = 1 << 5;
    pub const DIRTY: u64 = 1 << 6;
    pub const HUGE: u64 = 1 << 7;
    pub const GLOBAL: u64 = 1 << 8;
    pub const NO_EXECUTE: u64 = 1 << 63;
}

const ADDR_MASK: u64 = 0x000F_FFFF_FFFF_F000;

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Entry(pub u64);

impl Entry {
    pub const fn empty() -> Self {
        Self(0)
    }

    pub fn is_present(self) -> bool {
        self.0 & flags::PRESENT != 0
    }

    pub fn addr(self) -> u64 {
        self.0 & ADDR_MASK
    }

    pub fn flags(self) -> u64 {
        self.0 & !ADDR_MASK
    }

    pub fn set(&mut self, addr: u64, flags: u64) {
        self.0 = (addr & ADDR_MASK) | flags;
    }

    pub fn clear(&mut self) {
        self.0 = 0;
    }
}

#[repr(C, align(4096))]
pub struct PageTable {
    pub entries: [Entry; ENTRY_COUNT],
}

impl PageTable {
    pub const fn empty() -> Self {
        Self {
            entries: [Entry(0); ENTRY_COUNT],
        }
    }

    pub fn zero(&mut self) {
        for e in self.entries.iter_mut() {
            e.clear();
        }
    }
}

#[derive(Copy, Clone)]
pub struct VirtAddr(pub u64);

impl VirtAddr {
    pub fn pml4_index(self) -> usize {
        ((self.0 >> 39) & 0x1FF) as usize
    }
    pub fn pdpt_index(self) -> usize {
        ((self.0 >> 30) & 0x1FF) as usize
    }
    pub fn pd_index(self) -> usize {
        ((self.0 >> 21) & 0x1FF) as usize
    }
    pub fn pt_index(self) -> usize {
        ((self.0 >> 12) & 0x1FF) as usize
    }
    pub fn page_offset(self) -> u64 {
        self.0 & 0xFFF
    }
}

pub fn alloc_table() -> Option<u64> {
    let frame = mmu::alloc_frame()?;
    let virt = mmu::phys_to_virt(frame);
    unsafe {
        core::ptr::write_bytes(virt as *mut u8, 0, PAGE_SIZE as usize);
    }
    Some(frame)
}

fn table_ptr(phys: u64) -> *mut PageTable {
    mmu::phys_to_virt(phys) as *mut PageTable
}

pub fn read_cr3() -> u64 {
    let cr3: u64;
    unsafe {
        core::arch::asm!("mov {}, cr3", out(reg) cr3, options(nomem, nostack));
    }
    cr3
}

pub unsafe fn write_cr3(addr: u64) {
    unsafe {
        core::arch::asm!("mov cr3, {}", in(reg) addr, options(nostack));
    }
}

pub unsafe fn invlpg(addr: u64) {
    unsafe {
        core::arch::asm!("invlpg [{}]", in(reg) addr, options(nostack, preserves_flags));
    }
}

#[derive(Debug, Copy, Clone)]
pub enum MapError {
    OutOfFrames,
    AlreadyMapped,
}

unsafe fn next_table_create(e: &mut Entry) -> Result<*mut PageTable, MapError> {
    if e.is_present() {
        Ok(table_ptr(e.addr()))
    } else {
        let phys = alloc_table().ok_or(MapError::OutOfFrames)?;
        e.set(phys, flags::PRESENT | flags::WRITABLE);
        Ok(table_ptr(phys))
    }
}

unsafe fn next_table(e: Entry) -> Option<*mut PageTable> {
    if e.is_present() && (e.flags() & flags::HUGE) == 0 {
        Some(table_ptr(e.addr()))
    } else {
        None
    }
}

pub unsafe fn map_page(
    pml4: *mut PageTable,
    virt: VirtAddr,
    phys: u64,
    leaf_flags: u64,
) -> Result<(), MapError> {
    unsafe {
        let pml4_e = &mut (*pml4).entries[virt.pml4_index()];
        let pdpt = next_table_create(pml4_e)?;

        let pdpt_e = &mut (*pdpt).entries[virt.pdpt_index()];
        let pd = next_table_create(pdpt_e)?;

        let pd_e = &mut (*pd).entries[virt.pd_index()];
        let pt = next_table_create(pd_e)?;

        let pt_e = &mut (*pt).entries[virt.pt_index()];
        if pt_e.is_present() {
            return Err(MapError::AlreadyMapped);
        }
        pt_e.set(phys, leaf_flags | flags::PRESENT);

        invlpg(virt.0);
        Ok(())
    }
}

pub unsafe fn unmap_page(pml4: *mut PageTable, virt: VirtAddr) -> Option<u64> {
    unsafe {
        let pml4_e = (*pml4).entries[virt.pml4_index()];
        let pdpt = next_table(pml4_e)?;
        let pdpt_e = (*pdpt).entries[virt.pdpt_index()];
        let pd = next_table(pdpt_e)?;
        let pd_e = (*pd).entries[virt.pd_index()];
        let pt = next_table(pd_e)?;
        let pt_e = &mut (*pt).entries[virt.pt_index()];
        if !pt_e.is_present() {
            return None;
        }
        let phys = pt_e.addr();
        pt_e.clear();
        invlpg(virt.0);
        Some(phys)
    }
}

pub unsafe fn translate(pml4: *mut PageTable, virt: VirtAddr) -> Option<u64> {
    unsafe {
        let pml4_e = (*pml4).entries[virt.pml4_index()];
        let pdpt = next_table(pml4_e)?;
        let pdpt_e = (*pdpt).entries[virt.pdpt_index()];

        if pdpt_e.flags() & flags::HUGE != 0 {
            return Some(pdpt_e.addr() + (virt.0 & 0x3FFF_FFFF));
        }
        let pd = next_table(pdpt_e)?;
        let pd_e = (*pd).entries[virt.pd_index()];

        if pd_e.flags() & flags::HUGE != 0 {
            return Some(pd_e.addr() + (virt.0 & 0x1F_FFFF));
        }
        let pt = next_table(pd_e)?;
        let pt_e = (*pt).entries[virt.pt_index()];
        if !pt_e.is_present() {
            return None;
        }
        Some(pt_e.addr() + virt.page_offset())
    }
}
