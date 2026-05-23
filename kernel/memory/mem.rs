// AI assisted in this file
// Verified by author - Vrushang

use core::ptr;

/// Copy `n` bytes from `src` to `dest` via `rep movsb`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    unsafe {
        core::arch::asm!(
            "rep movsb",
            inout("rdi") dest => _,
            inout("rsi") src => _,
            inout("rcx") n => _,
            options(nostack, preserves_flags),
        );
    }
    dest
}

/// Set `n` bytes at `dest` to `val` via `rep stosb`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn memset(dest: *mut u8, val: i32, n: usize) -> *mut u8 {
    unsafe {
        core::arch::asm!(
            "rep stosb",
            inout("rdi") dest => _,
            inout("rcx") n => _,
            in("al") val as u8,
            options(nostack, preserves_flags),
        );
    }
    dest
}

/// Copy `n` bytes from `src` to `dest`, overlap-safe
#[unsafe(no_mangle)]
pub unsafe extern "C" fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    unsafe {
        if (dest as usize) < (src as usize) {
            core::arch::asm!(
                "rep movsb",
                inout("rdi") dest => _,
                inout("rsi") src => _,
                inout("rcx") n => _,
                options(nostack, preserves_flags),
            );
        } else if n > 0 {
            let d = dest.add(n - 1);
            let s = src.add(n - 1);
            core::arch::asm!(
                "std",
                "rep movsb",
                "cld",
                inout("rdi") d => _,
                inout("rsi") s => _,
                inout("rcx") n => _,
                options(nostack),
            );
        }
    }
    dest
}

/// Compute length of C string (null-terminated)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn strlen(s: *const u8) -> usize {
    let mut len = 0;
    while unsafe { ptr::read(s.add(len)) } != 0 {
        len += 1;
    }
    len
}

/// Compare n bytes of memory
#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcmp(a: *const u8, b: *const u8, n: usize) -> i32 {
    let mut i = 0;
    while i < n {
        let av = unsafe { ptr::read(a.add(i)) };
        let bv = unsafe { ptr::read(b.add(i)) };
        if av != bv {
            return av as i32 - bv as i32;
        }
        i += 1;
    }
    0
}
