mod pattern;

use std::ops::DerefMut;

use macroquad::hash;
use macroquad::prelude::*;
use macroquad::ui;
use macroquad::ui::widgets;
pub use pattern::*;

/// `length` is length in beats.
struct Options {
    length: f32,
}

impl Default for Options {
    fn default() -> Self {
        Self { length: 32.0 }
    }
}

fn ui() -> impl DerefMut<Target = ui::Ui> {
    ui::root_ui()
}

fn button(text: &str) -> bool {
    widgets::Button::new(text).size(vec2(screen_width(), 24.0)).ui(&mut ui())
}

#[macroquad::main("breakgen")]
async fn main() {
    let mut opts = Options::default();
    let mut pattern = Pattern::new();
    let mut file_name = String::from("break.mid");
    let mut error_text = String::from("");
    loop {
        clear_background(DARKGRAY);
        ui().slider(
            hash!("length_slider"),
            "length",
            0.0..64.0,
            &mut opts.length,
        );
        ui().separator();
        if button("Generate") {
            match Pattern::generate(opts.length.round() as _) {
                Ok(p) => pattern = p,
                Err(e) => error_text = e,
            }
        }
        ui().editbox(
            hash!("filename").into(),
            vec2(screen_width(), 24.0),
            &mut file_name,
        );
        
        if button("Save") {
            let mid = pattern.to_midi();
            mid.save(&file_name);
        }
        ui().label(None, &error_text);
        pattern.draw(Rect::new(0.0, screen_height() - 96.0, screen_width(), 96.0));
        next_frame().await
    }
}
