#![no_std]

use core::panic::PanicInfo;
use limine::request::FramebufferRequest;

mod mem;
mod font;

#[used]
#[unsafe(link_section = ".requests")]
static FB_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    if let Some(framebuffer_response) = FB_REQUEST.get_response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
            let fb_ptr = framebuffer.addr() as *mut u32;
            let pitch = framebuffer.pitch() as usize;

            unsafe {
                font::draw_str(
                    fb_ptr,
                    pitch,
                    16,   // x: 16 pixels from left
                    16,   // y: 16 pixels from top
                    b"Hello, World!",
                    0x00FFFFFF, // foreground: white
                    0x00000000, // background: black
                );
            }
        }
    }

    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
