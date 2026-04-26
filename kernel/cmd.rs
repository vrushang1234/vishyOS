use crate::framebuffer;
use libm::{cosf, sinf};

pub fn handle_command(buf: &[u8]) {
    if buf == b"help" {
        cmd_help();
    } else if buf == b"clear" {
        cmd_clear();
    } else if buf == b"donut" {
        donut_cmd();
    } else {
        framebuffer::print("\nUnknown command\n", 0xFFFFFFFF);
    }
}

fn cmd_help() {
    framebuffer::print(
        "\nCommands:\n help  - show this message\n clear - clear screen\n donut - spinning donut\n",
        0xFFFFFFFF,
    );
}

fn cmd_clear() {
    framebuffer::clear();
}

fn donut_cmd() {
    let mut a: f32 = 0.0;
    let mut b: f32 = 0.0;

    let shades: [u8; 12] = *b".,-~:;=!*#$@";

    loop {
        let mut z = [0.0f32; 1760];
        let mut out = [b' '; 1760];

        let sin_a = sinf(a);
        let cos_a = cosf(a);
        let sin_b = sinf(b);
        let cos_b = cosf(b);

        for j in (0..628).step_by(7) {
            let fj = j as f32 / 100.0;
            let sin_j = sinf(fj);
            let cos_j = cosf(fj);

            for i in (0..628).step_by(3) {
                let fi = i as f32 / 100.0;
                let sin_i = sinf(fi);
                let cos_i = cosf(fi);

                let circle = cos_j + 2.0;
                let depth = 1.0 / (sin_i * circle * sin_a + sin_j * cos_a + 5.0);
                let t = sin_i * circle * cos_a - sin_j * sin_a;

                let x = (40.0 + 30.0 * depth * (cos_i * circle * cos_b - t * sin_b)) as isize;
                let y = (12.0 + 15.0 * depth * (cos_i * circle * sin_b + t * cos_b)) as isize;

                let luminance = 8.0
                    * ((sin_j * sin_a - sin_i * cos_j * cos_a) * cos_b
                        - sin_i * cos_j * sin_a
                        - sin_j * cos_a
                        - cos_i * cos_j * sin_b);

                if x > 0 && x < 80 && y > 0 && y < 22 {
                    let idx = x as usize + 80 * y as usize;

                    if depth > z[idx] {
                        z[idx] = depth;

                        let shade_idx = if luminance > 0.0 {
                            luminance as usize
                        } else {
                            0
                        };

                        out[idx] = shades[shade_idx.min(shades.len() - 1)];
                    }
                }
            }
        }

        let mut frame = [0u8; 22 * 81];
        let mut k = 0;

        for row in 0..22 {
            for col in 0..80 {
                frame[k] = out[row * 80 + col];
                k += 1;
            }

            frame[k] = b'\n';
            k += 1;
        }

        framebuffer::reset_cursor();

        let s = core::str::from_utf8(&frame[..k]).unwrap();
        framebuffer::print(s, 0xFFFFFFFF);

        a += 0.07;
        b += 0.03;
    }
}
