// This example program may be used for any purpose proprietary or not.

#[macro_use]
extern crate adi;
extern crate barg;

use adi::{hid, screen, App};
use barg::*;

main!(
    Ctx,
    struct Ctx {
        // Time
        time: f32,
        // Frames
        frames: u32,
        // Barg Font
        font: Font<'static>,
        // FPS / Info String
        info: String,
        // The mode
        mode: fn(app: &mut Ctx),
    }
);

impl App for Ctx {
    fn new() -> Ctx {
        Ctx {
            time: 0.0,
            frames: 0,
            font: Font::new(FONT).expect("Failed to load font!"),
            info: "".to_string(),
            mode: mode,
        }
    }

    fn run(&mut self) {
        (self.mode)(self)
    }
}

// Code that runs every frame.
fn mode(app: &mut Ctx) {
    // Check for exit request
    if hid::Key::Back.pressed(0) {
        adi::old();
    }

    // Begin timing
    app.time += screen::dt();
    app.frames += 1;

    if app.time >= 1.0 {
        app.info = format!(
            "野ウサギ Triangle Example\n\n{} FPS",
            app.frames
        );
        app.time -= 1.0;
        app.frames = 0;
    }

    // Render
    screen::draw(&mut |pixel_buffer| {
        let (w, h) = screen::wh();
        let mut image = Image::new(Size((screen::pitch() / 4) as u16, h));
        let _fw = w as f32;
        let _fh = h as f32;

        image.clear_ptr(pixel_buffer);
        image.text_ptr(
            [255, 255, 255, 255], // White
            (5.0, 5.0, 36.0),     // Pos. & Size
            &app.font,            // Font
            &app.info,            // String
            pixel_buffer,
        );
    });
}
