mod entities;

use entities::hole::Hole;
use macroquad::prelude::*;

const MARGING: f32 = 100f32;

#[macroquad::main("WHACK-A-GOPHER")]
async fn main() {
    let gopher_texture: Texture2D = load_texture("assets/gopher.png").await.unwrap();
    let land_texture: Texture2D = load_texture("assets/grass.png").await.unwrap();

    // TODO: this could go to a game control struct
    let mut holes: Vec<Hole> = Vec::new();
    let mut gophers_count = 0;
    let mut last_gopher_spawned = 0.0;
    let mut points = 0;

    for i in 0..6 {
        let hole = Hole::new(i as f32, gopher_texture.clone());
        holes.push(hole);
    }

    loop {
        let score_text = format!("Score: {}", points);

        clear_background(WHITE);
        draw_texture_ex(&land_texture, 1.5*MARGING*0.5f32, 1.5*MARGING*0.5f32, GREEN, DrawTextureParams {
            dest_size: Some(vec2(screen_width()-1.5*MARGING, screen_height()-1.5*MARGING)),
            ..Default::default()
        });
        draw_text("WHACK-A-GOPHER", 20.0, 20.0, 30.0, DARKGRAY);
        draw_text(&score_text, 20.0, 55.0, 30.0, DARKGRAY);

        for hole in &mut holes {
            hole.update(get_frame_time(), &mut gophers_count, &mut last_gopher_spawned, &mut points);
            hole.draw();
        }

        next_frame().await
    }
}
