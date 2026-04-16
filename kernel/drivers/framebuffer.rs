use limine::request::FramebufferRequest;

use crate::font;

#[used]
#[unsafe(link_section = ".requests")]
static FB_REQUEST: FramebufferRequest = FramebufferRequest::new();

static mut FB: (*mut u32, usize) = (core::ptr::null_mut(), 0);

// Initialize framebuffer (loaded first on booting)
pub fn init() {
    if let Some(framebuffer_response) = FB_REQUEST.get_response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
            let fb_ptr = framebuffer.addr() as *mut u32;
            let pitch = framebuffer.pitch() as usize;
            let width = framebuffer.width() as usize;
            unsafe {
                FB = (fb_ptr, pitch);
                font::psf::set_screen_width(width);
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
