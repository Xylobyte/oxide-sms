#![no_std]

pub const WIDTH: usize = 256;
pub const HEIGHT: usize = 192;

pub struct SmsConsole {
    framebuffer: [u32; WIDTH * HEIGHT],
    counter: u32,
}

impl SmsConsole {
    pub fn new() -> Self {
        Self {
            framebuffer: [0; WIDTH * HEIGHT],
            counter: 0,
        }
    }

    pub fn update(&mut self) {
        self.counter = self.counter.wrapping_add(1);

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let color = if ((x + y + self.counter as usize) / 16) % 2 == 0 {
                    0x00_33_AA_55
                } else {
                    0x00_11_11_22
                };
                self.framebuffer[y * WIDTH + x] = color;
            }
        }
    }

    pub fn framebuffer(&self) -> &[u32; WIDTH * HEIGHT] {
        &self.framebuffer
    }
}