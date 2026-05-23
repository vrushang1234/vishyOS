use limine::request::FramebufferRequest;

use crate::font;

#[used]
#[unsafe(link_section = ".requests")]
static FB_REQUEST: FramebufferRequest = FramebufferRequest::new();

static mut FB: (*mut u32, usize, usize) = (core::ptr::null_mut(), 0, 0);

// Initialize framebuffer (loaded first on booting)
pub fn init() {
    if let Some(framebuffer_response) = FB_REQUEST.get_response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
            let fb_ptr = framebuffer.addr() as *mut u32;
            let pitch = framebuffer.pitch() as usize;
            let width = framebuffer.width() as usize;
            let height = framebuffer.height() as usize;
            unsafe {
                FB = (fb_ptr, pitch, height);
                font::psf::set_screen_size(width, height);
            }
        }
    }
}

pub fn print(s: &str, fg: u32) {
    let color: u32 = fg;
    unsafe {
        if !FB.0.is_null() {
            font::psf::draw_str(FB.0, FB.1, s, color, 0x00000000);
        }
    }
}
pub fn backspace() {
    unsafe {
        if !FB.0.is_null() {
            font::psf::backspace(FB.0, FB.1);
        }
    }
}

pub fn cursor_draw() {
    unsafe {
        if !FB.0.is_null() {
            font::psf::cursor_draw(FB.0, FB.1);
        }
    }
}

pub fn cursor_erase() {
    unsafe {
        if !FB.0.is_null() {
            font::psf::cursor_erase(FB.0, FB.1);
        }
    }
}

pub fn cursor_toggle() {
    unsafe {
        if !FB.0.is_null() {
            font::psf::cursor_toggle(FB.0, FB.1);
        }
    }
}

pub fn clear() {
    unsafe {
        if FB.0.is_null() {
            return;
        }
        let (ptr, pitch, height) = FB;
        let total_bytes = pitch * height;
        core::ptr::write_bytes(ptr as *mut u8, 0, total_bytes);
        font::psf::reset_cursor();
    }
}

pub fn reset_cursor() {
    unsafe {
        font::psf::reset_cursor();
    }
}
