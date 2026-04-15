// PSF2 font renderer using UniCyr_8x16.psf
// Header: 32 bytes, 256 glyphs, 8x16 pixels each, 16 bytes per glyph

static FONT_DATA: &[u8] = include_bytes!("font.psf");

// PSF2 header (little-endian u32 fields):
//   [0..4]   magic: 0x864ab572
//   [4..8]   version
//   [8..12]  header_size = 32
//   [12..16] flags
//   [16..20] num_glyph  = 256
//   [20..24] bytes_per_glyph = 16
//   [24..28] height = 16
//   [28..32] width  = 8

const HEADER_SIZE: usize = 32;
const BYTES_PER_GLYPH: usize = 16;
const GLYPH_HEIGHT: usize = 16;
const GLYPH_WIDTH: usize = 8;

static mut CURSOR_X: usize = 0;
static mut CURSOR_Y: usize = 0;
static mut SCREEN_COLS: usize = 80; // updated from pixel width at init

pub unsafe fn set_screen_width(pixel_width: usize) {
    unsafe { SCREEN_COLS = pixel_width / GLYPH_WIDTH };
}

/// Draw a single ASCII character onto the framebuffer.
///
/// * `fb`    - raw framebuffer pointer (u32 pixels, packed as 0x00RRGGBB)
/// * `pitch` - framebuffer pitch in *bytes* (use `framebuffer.pitch()`)
/// * `x`,`y` - top-left pixel position of the character
/// * `c`     - ASCII character to draw (values >= 128 fall back to '?')
/// * `fg`    - foreground colour (e.g. 0x00FFFFFF for white)
/// * `bg`    - background colour (e.g. 0x00000000 for black)
pub unsafe fn draw_char(fb: *mut u32, pitch: usize, x: usize, y: usize, c: u8, fg: u32, bg: u32) {
    let index = if (c as usize) < 256 {
        c as usize
    } else {
        b'?' as usize
    };
    let glyph_offset = HEADER_SIZE + index * BYTES_PER_GLYPH;
    let glyph = &FONT_DATA[glyph_offset..glyph_offset + BYTES_PER_GLYPH];

    // pitch is in bytes; each pixel is 4 bytes (u32)
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

/// Draw a null-terminated ASCII string starting at the cursor position.
pub unsafe fn draw_str(fb: *mut u32, pitch: usize, s: &str, fg: u32, bg: u32) {
    for c in s.bytes() {
        unsafe {
            match c {
                b'\n' => {
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
