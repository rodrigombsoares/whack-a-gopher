use macroquad::prelude::*;

// Copied from tantandev https://github.com/TanTanDev/breakout_tutorial/blob/main/src/main.rs
pub fn draw_title_text(text: &str) {
    let dims = measure_text(text, None, 50u16, 1.0f32);
    draw_text_ex(
        text,
        screen_width() * 0.5f32 - dims.width * 0.5f32,
        screen_height() * 0.5f32 - dims.height * 0.5f32,
        TextParams {
            font_size: 50u16,
            color: BLACK,
            ..Default::default()
        },
    );
}
