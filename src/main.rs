mod entities;

use entities::hole::Hole;
use macroquad::prelude::*;

const MARGING: f32 = 100f32;

#[macroquad::main("BasicShapes")]
async fn main() {
    let gopher_texture: Texture2D = load_texture("assets/gopher.png").await.unwrap();

    let mut holes: Vec<Hole> = Vec::new();
    let mut gophers_count = 0;
    let mut last_gopher_spawned = 0.0;
    let mut points = 0;

    for i in 0..6 {
        let hole = Hole::new(i as f32, gopher_texture.clone());
        holes.push(hole);
    }

    loop {
        clear_background(WHITE);
        draw_rectangle(1.5*MARGING*0.5f32, 1.5*MARGING*0.5f32, screen_width()-1.5*MARGING, screen_height()-1.5*MARGING, GREEN);
        draw_text("WHACK-A_GOPHER", 20.0, 20.0, 30.0, DARKGRAY);
        let score_text = format!("Score: {}!", points);
        draw_text(&score_text, 20.0, 55.0, 30.0, DARKGRAY);
        for hole in &mut holes {
            hole.update(get_frame_time(), &mut gophers_count, &mut last_gopher_spawned, &mut points);
            hole.draw();
        }
        next_frame().await
    }
}
