use minifb::{Key, Scale, Window, WindowOptions};
use sms_core::{SmsConsole, HEIGHT, WIDTH};

fn main() {
    let mut console = SmsConsole::new();

    let mut window = Window::new(
        "Oxide SMS - PC Test Harness",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X2,
            ..WindowOptions::default()
        },
    )
        .unwrap_or_else(|e| {
            panic!("Window opening error : {}", e);
        });

    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        console.update();

        window
            .update_with_buffer(console.framebuffer(), WIDTH, HEIGHT)
            .unwrap();
    }
}