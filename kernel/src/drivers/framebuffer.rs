use limine::request::FramebufferRequest;

use crate::font;

#[used]
#[unsafe(link_section = ".requests")]
static FB_REQUEST: FramebufferRequest = FramebufferRequest::new();

pub fn init() {
    if let Some(framebuffer_response) = FB_REQUEST.get_response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
            let fb_ptr = framebuffer.addr() as *mut u32;
            let pitch = framebuffer.pitch() as usize;

            unsafe {
                font::psf::draw_str(
                    fb_ptr,
                    pitch,
                    16,
                    16,
                    b"Hello, World!",
                    0x00FFFFFF,
                    0x00000000,
                );
            }
        }
    }
}
