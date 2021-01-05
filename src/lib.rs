#![no_std]

use core::sync::atomic::{AtomicU32, Ordering};

#[panic_handler]
fn handle_panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

const BG: u32 = 0xff342c28;
const GREEN: u32 = 0xff65be98;

const WIDTH: usize = 512;
const HEIGHT: usize = 512;

static FRAME: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
static mut BUFFER: [u32; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];

#[no_mangle]
pub unsafe extern fn go() {
    render_frame_safe(&mut BUFFER)
}

fn render_frame_safe(buffer: &mut [u32; WIDTH * HEIGHT]) {
    let f = (FRAME.fetch_add(1, Ordering::Relaxed) / 10) % (1000) as u32;

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            buffer[y * WIDTH + x] = if f.wrapping_add((x&y) as u32) % ((f*f/200+1) % WIDTH as u32) as u32 == 0 {GREEN} else {BG};
        }
    }
}
