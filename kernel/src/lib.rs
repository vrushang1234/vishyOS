#![no_std]

use core::panic::PanicInfo;
use limine::request::FramebufferRequest;

mod mem;

#[used]
#[unsafe(link_section = ".requests")]
static FB_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    if let Some(framebuffer_response) = FB_REQUEST.get_response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
            let fb_ptr = framebuffer.addr() as *mut u32;
            let width = framebuffer.pitch() as usize / 4;

            for i in 0..100usize {
                unsafe {
                    *fb_ptr.add(i * width + i) = 0xFFFFFFFF;
                }
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
