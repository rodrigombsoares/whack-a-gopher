mod entities;
mod utils;

use entities::hole::Hole;
use macroquad::prelude::*;

const MARGING: f32 = 100f32;

enum GameState {
    Menu,
    Game,
    End,
}

#[macroquad::main("WHACK-A-GOPHER")]
async fn main() {
    let gopher_texture: Texture2D = load_texture("assets/gopher.png").await.unwrap();
    let land_texture: Texture2D = load_texture("assets/grass.png").await.unwrap();

    // TODO: this could go to a game control struct
    let mut holes: Vec<Hole> = Vec::new();
    let mut gophers_count = 0;
    let mut gophers_spawned = 0;
    let mut last_gopher_spawned = 0.0;
    let mut points = 0;
    let mut game_state = GameState::Menu;

    for i in 0..6 {
        let hole = Hole::new(i as f32, gopher_texture.clone());
        holes.push(hole);
    }

    loop {
        let score_text: String = format!("Score: {}", points);

        clear_background(WHITE);
        draw_texture_ex(&land_texture, 1.5*MARGING*0.5f32, 1.5*MARGING*0.5f32, GREEN, DrawTextureParams {
            dest_size: Some(vec2(screen_width()-1.5*MARGING, screen_height()-1.5*MARGING)),
            ..Default::default()
        });
        draw_text("WHACK-A-GOPHER", 20.0, 20.0, 30.0, DARKGRAY);
        draw_text(&score_text, 20.0, 55.0, 30.0, DARKGRAY);

        match game_state {
            GameState::Menu => {
                utils::draw_title_text("Press SPACE to start");
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Game;
                }
            }
            GameState::Game => {
                for hole in &mut holes {
                    hole.update(get_frame_time(), &mut gophers_count, &mut last_gopher_spawned, &mut points, &mut gophers_spawned, &mut game_state);
                    hole.draw();
                }        
            }
            GameState::End => {
                utils::draw_title_text("Press SPACE to restart");
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Game;
                }
            }
        }

        next_frame().await
    }
}
