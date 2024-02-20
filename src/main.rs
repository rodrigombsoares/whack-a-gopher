use macroquad::prelude::*;

const MARGING: f32  = 100f32;
const RADIUS: f32  = 50f32;

#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        clear_background(WHITE);

        draw_rectangle(MARGING*0.5f32, MARGING*0.5f32, screen_width()-MARGING, screen_height()-MARGING, GREEN);
        for i in 0..6 {
            let n = i as f32;
            let x = screen_width()*0.5 - 4.0*RADIUS - 20.0 + MARGING+((n-n%2.0)*(RADIUS+10.0));
            let y = screen_height()*0.5 - RADIUS - 10.0 +(RADIUS+10.0)*2.0*(n%2.0);

            draw_circle(x, y, RADIUS, BROWN);
        }
        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
