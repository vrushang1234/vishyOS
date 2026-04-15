// AI assisted for this file
// Verified by author - Vrushang
use core::sync::atomic::{AtomicBool, Ordering};
static FONT_DATA: &[u8] = include_bytes!("font.psf");

const HEADER_SIZE: usize = 32;
const BYTES_PER_GLYPH: usize = 16;
const GLYPH_HEIGHT: usize = 16;
const GLYPH_WIDTH: usize = 8;

static mut CURSOR_X: usize = 0;
static mut CURSOR_Y: usize = 0;
static mut SCREEN_COLS: usize = 80;
const CURSOR_TOP: usize = GLYPH_HEIGHT - 2;

static mut PREV_LINE_END_X: usize = 0;

pub unsafe fn set_screen_width(pixel_width: usize) {
    unsafe { SCREEN_COLS = pixel_width / GLYPH_WIDTH };
}

static CURSOR_ON: AtomicBool = AtomicBool::new(false);

pub unsafe fn draw_char(fb: *mut u32, pitch: usize, x: usize, y: usize, c: u8, fg: u32, bg: u32) {
    let index = if (c as usize) < 256 {
        c as usize
    } else {
        b'?' as usize
    };
    let glyph_offset = HEADER_SIZE + index * BYTES_PER_GLYPH;
    let glyph = &FONT_DATA[glyph_offset..glyph_offset + BYTES_PER_GLYPH];

    let stride = pitch / 4;

    for row in 0..GLYPH_HEIGHT {
        let byte = glyph[row];
        for col in 0..GLYPH_WIDTH {
            let pixel = if byte & (0x80 >> col) != 0 { fg } else { bg };
            let px_offset = (y + row) * stride + (x + col);
            unsafe {
                *fb.add(px_offset) = pixel;
            }
        }
    }
}

pub unsafe fn draw_str(fb: *mut u32, pitch: usize, s: &str, fg: u32, bg: u32) {
    for c in s.bytes() {
        unsafe {
            match c {
                b'\n' => {
                    PREV_LINE_END_X = CURSOR_X;
                    CURSOR_Y += GLYPH_HEIGHT;
                    CURSOR_X = 0;
                }
                _ => {
                    draw_char(fb, pitch, CURSOR_X, CURSOR_Y, c, fg, bg);
                    CURSOR_X += GLYPH_WIDTH;
                }
            }
        }
    }
}

pub unsafe fn backspace(fb: *mut u32, pitch: usize) {
    let (x, y, cols) = unsafe { (CURSOR_X, CURSOR_Y, SCREEN_COLS) };

    let (new_x, new_y) = if x >= GLYPH_WIDTH {
        (x - GLYPH_WIDTH, y)
    } else if y >= GLYPH_HEIGHT {
        unsafe { (PREV_LINE_END_X, y - GLYPH_HEIGHT) }
    } else {
        return; // already at (0, 0)
    };

    unsafe {
        CURSOR_X = new_x;
        CURSOR_Y = new_y;
        draw_char(fb, pitch, new_x, new_y, b' ', 0x00FFFFFF, 0x00000000);
    }
}

pub unsafe fn cursor_draw(fb: *mut u32, pitch: usize) {
    let (x, y) = unsafe { (CURSOR_X, CURSOR_Y) };
    let stride = pitch / 4;
    for row in CURSOR_TOP..GLYPH_HEIGHT {
        for col in 0..GLYPH_WIDTH {
            unsafe { *fb.add((y + row) * stride + x + col) = 0x00FFFFFF };
        }
    }
    CURSOR_ON.store(true, Ordering::Relaxed);
}

pub unsafe fn cursor_erase(fb: *mut u32, pitch: usize) {
    let (x, y) = unsafe { (CURSOR_X, CURSOR_Y) };
    let stride = pitch / 4;
    for row in CURSOR_TOP..GLYPH_HEIGHT {
        for col in 0..GLYPH_WIDTH {
            unsafe { *fb.add((y + row) * stride + x + col) = 0x00000000 };
        }
    }
    CURSOR_ON.store(false, Ordering::Relaxed);
}

pub unsafe fn cursor_toggle(fb: *mut u32, pitch: usize) {
    if CURSOR_ON.load(Ordering::Relaxed) {
        unsafe { cursor_erase(fb, pitch) };
    } else {
        unsafe { cursor_draw(fb, pitch) };
    }
}
