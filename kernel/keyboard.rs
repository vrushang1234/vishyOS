use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use crate::cmd;

static SHIFT: AtomicBool = AtomicBool::new(false);
static LINE_LEN: AtomicUsize = AtomicUsize::new(0);

const INPUT_BUFFER_SIZE: usize = 256;
static mut INPUT_BUFFER: [u8; INPUT_BUFFER_SIZE] = [0; INPUT_BUFFER_SIZE];
static mut INPUT_LEN: usize = 0;

#[rustfmt::skip]
static UNSHIFTED: [u8; 58] = [
    0,       // 0x00  unused
    0,       // 0x01  ESC
    b'1',    // 0x02
    b'2',    // 0x03
    b'3',    // 0x04
    b'4',    // 0x05
    b'5',    // 0x06
    b'6',    // 0x07
    b'7',    // 0x08
    b'8',    // 0x09
    b'9',    // 0x0A
    b'0',    // 0x0B
    b'-',    // 0x0C
    b'=',    // 0x0D
    0x08,    // 0x0E  Backspace
    b'\t',   // 0x0F  Tab
    b'q',    // 0x10
    b'w',    // 0x11
    b'e',    // 0x12
    b'r',    // 0x13
    b't',    // 0x14
    b'y',    // 0x15
    b'u',    // 0x16
    b'i',    // 0x17
    b'o',    // 0x18
    b'p',    // 0x19
    b'[',    // 0x1A
    b']',    // 0x1B
    b'\n',   // 0x1C  Enter
    0,       // 0x1D  Left Ctrl
    b'a',    // 0x1E
    b's',    // 0x1F
    b'd',    // 0x20
    b'f',    // 0x21
    b'g',    // 0x22
    b'h',    // 0x23
    b'j',    // 0x24
    b'k',    // 0x25
    b'l',    // 0x26
    b';',    // 0x27
    b'\'',   // 0x28
    b'`',    // 0x29
    0,       // 0x2A  Left Shift
    b'\\',   // 0x2B
    b'z',    // 0x2C
    b'x',    // 0x2D
    b'c',    // 0x2E
    b'v',    // 0x2F
    b'b',    // 0x30
    b'n',    // 0x31
    b'm',    // 0x32
    b',',    // 0x33
    b'.',    // 0x34
    b'/',    // 0x35
    0,       // 0x36  Right Shift
    b'*',    // 0x37  Keypad *
    0,       // 0x38  Left Alt
    b' ',    // 0x39  Space
];

#[rustfmt::skip]
static SHIFTED: [u8; 58] = [
    0,       // 0x00  unused
    0,       // 0x01  ESC
    b'!',    // 0x02
    b'@',    // 0x03
    b'#',    // 0x04
    b'$',    // 0x05
    b'%',    // 0x06
    b'^',    // 0x07
    b'&',    // 0x08
    b'*',    // 0x09
    b'(',    // 0x0A
    b')',    // 0x0B
    b'_',    // 0x0C
    b'+',    // 0x0D
    0x08,    // 0x0E  Backspace
    b'\t',   // 0x0F  Tab
    b'Q',    // 0x10
    b'W',    // 0x11
    b'E',    // 0x12
    b'R',    // 0x13
    b'T',    // 0x14
    b'Y',    // 0x15
    b'U',    // 0x16
    b'I',    // 0x17
    b'O',    // 0x18
    b'P',    // 0x19
    b'{',    // 0x1A
    b'}',    // 0x1B
    b'\n',   // 0x1C  Enter
    0,       // 0x1D  Left Ctrl
    b'A',    // 0x1E
    b'S',    // 0x1F
    b'D',    // 0x20
    b'F',    // 0x21
    b'G',    // 0x22
    b'H',    // 0x23
    b'J',    // 0x24
    b'K',    // 0x25
    b'L',    // 0x26
    b':',    // 0x27
    b'"',    // 0x28
    b'~',    // 0x29
    0,       // 0x2A  Left Shift
    b'|',    // 0x2B
    b'Z',    // 0x2C
    b'X',    // 0x2D
    b'C',    // 0x2E
    b'V',    // 0x2F
    b'B',    // 0x30
    b'N',    // 0x31
    b'M',    // 0x32
    b'<',    // 0x33
    b'>',    // 0x34
    b'?',    // 0x35
    0,       // 0x36  Right Shift
    b'*',    // 0x37  Keypad *
    0,       // 0x38  Left Alt
    b' ',    // 0x39  Space
];

pub fn process_scancode(scancode: u8) {
    use crate::drivers::framebuffer;
    if scancode & 0x80 != 0 {
        let key = scancode & 0x7F;
        if key == 0x2A || key == 0x36 {
            SHIFT.store(false, Ordering::Relaxed);
        }
        return;
    }

    if scancode == 0x2A || scancode == 0x36 {
        SHIFT.store(true, Ordering::Relaxed);
        return;
    }

    let idx = scancode as usize;
    let table = if SHIFT.load(Ordering::Relaxed) {
        &SHIFTED
    } else {
        &UNSHIFTED
    };

    if idx >= table.len() {
        return;
    }

    let byte = table[idx];
    if byte == 0 {
        return;
    }

    framebuffer::cursor_erase();

    if byte == 0x08 {
        if LINE_LEN.load(Ordering::Relaxed) > 0 {
            unsafe {
                if INPUT_LEN >= 1 {
                    INPUT_LEN -= 1;
                    INPUT_BUFFER[INPUT_LEN] = 0;
                }
            }

            framebuffer::backspace();
            LINE_LEN.fetch_sub(1, Ordering::Relaxed);
        }
    } else if byte == b'\n' {
        unsafe {
            let buf = &INPUT_BUFFER[..INPUT_LEN];
            cmd::handle_command(buf);
            INPUT_BUFFER = [0; 256];
            INPUT_LEN = 0;
        }
        framebuffer::print("\n> ", 0xFFFFFFFF);
        LINE_LEN.store(0, Ordering::Relaxed);
    } else {
        let s = unsafe { core::str::from_utf8_unchecked(core::slice::from_ref(&byte)) };
        unsafe {
            if INPUT_LEN < INPUT_BUFFER_SIZE {
                INPUT_BUFFER[INPUT_LEN] = byte;
                INPUT_LEN += 1;
            }
        }
        framebuffer::print(s, 0xFFFFFFFF);
        LINE_LEN.fetch_add(1, Ordering::Relaxed);
    }
    framebuffer::cursor_draw();
}
