// AI assisted in this file
// Verified by author - Vrushang

use core::ptr;

/// Copy `n` bytes from `src` to `dest`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        unsafe { ptr::write(dest.add(i), ptr::read(src.add(i))) };
        i += 1;
    }
    dest
}

/// Set `n` bytes at `dest` to `val`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn memset(dest: *mut u8, val: i32, n: usize) -> *mut u8 {
    let byte = val as u8;
    let mut i = 0;
    while i < n {
        unsafe { ptr::write(dest.add(i), byte) };
        i += 1;
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
