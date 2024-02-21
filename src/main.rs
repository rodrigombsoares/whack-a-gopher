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

struct Control {
    gophers_count: i32,
    gophers_spawned: i8,
    last_gopher_spawned: f64,
    points: i32,
    game_state: GameState,
}

impl Default for Control {
    fn default() -> Control {
        Control {
            gophers_count: 0,
            gophers_spawned: 0,
            last_gopher_spawned: 0.0,
            points: 0,
            game_state: GameState::Menu,
        }
    }
}

#[macroquad::main("WHACK-A-GOPHER")]
async fn main() {
    let gopher_texture: Texture2D = load_texture("assets/gopher.png").await.unwrap();
    let land_texture: Texture2D = load_texture("assets/grass.png").await.unwrap();
    
    let mut holes: Vec<Hole> = Vec::new();
    let mut control: Control = Control {
        ..Default::default()
    };

    for i in 0..6 {
        let hole = Hole::new(i as f32, gopher_texture.clone());
        holes.push(hole);
    }

    loop {
        let score_text: String = format!("Score: {}", control.points);

        clear_background(WHITE);
        draw_texture_ex(
            &land_texture,
            1.5 * MARGING * 0.5f32,
            1.5 * MARGING * 0.5f32,
            GREEN,
            DrawTextureParams {
                dest_size: Some(vec2(
                    screen_width() - 1.5 * MARGING,
                    screen_height() - 1.5 * MARGING,
                )),
                ..Default::default()
            },
        );
        draw_text("WHACK-A-GOPHER", 20.0, 20.0, 30.0, DARKGRAY);
        draw_text(&score_text, 20.0, 55.0, 30.0, DARKGRAY);

        match control.game_state {
            GameState::Menu => {
                utils::draw_title_text("Press SPACE to start");
                if is_key_pressed(KeyCode::Space) {
                    control.game_state = GameState::Game;
                }
            }
            GameState::Game => {
                for hole in &mut holes {
                    hole.update(get_frame_time(), &mut control);
                    hole.draw();
                }
            }
            GameState::End => {
                utils::draw_title_text("Press SPACE to restart");
                if is_key_pressed(KeyCode::Space) {
                    control = Control {
                        game_state: GameState::Game,
                        ..Default::default()
                    };
                }
            }
        }

        next_frame().await
    }
}
