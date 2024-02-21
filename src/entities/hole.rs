use macroquad::prelude::*;
use super::gopher::Gopher;
use std::collections::HashMap;

use ::rand::Rng;
use ::rand::thread_rng;

const MARGING: f32 = 100f32;
const RADIUS: f32 = 50f32;
const HIT_SCORE: i32 = 100;

pub struct Hole {
    circle: Circle,
    gopher: Option<Gopher>,
    is_pressed: bool,
    index: i32,
    gopher_texture: Texture2D,
}

impl Hole {
    pub fn new(i: f32, gopher_texture: Texture2D) -> Self {
        let x = screen_width()*0.5 - 4.0*RADIUS - 20.0 + MARGING+((i-i%2.0)*(RADIUS+10.0));
        let y = screen_height()*0.5 - RADIUS - 10.0 +(RADIUS+10.0)*2.0*(i%2.0);

        Self {
            circle: Circle::new(x, y, RADIUS),
            gopher: None,
            is_pressed: false,
            index: i as i32,
            gopher_texture: gopher_texture,
        }
    }

    pub fn update(&mut self, dt: f32, gophers_count: &mut i32, last_gopher_spawned: &mut f64, points: &mut i32) {
        let key_map = HashMap::from([
            (0, KeyCode::Q),
            (1, KeyCode::A),
            (2, KeyCode::W),
            (3, KeyCode::S),
            (4, KeyCode::E),
            (5, KeyCode::D),
        ]);
    
        // Spawn gophers
        let time_since_last_gopher = get_time() - *last_gopher_spawned;
        if self.gopher.is_none() && *gophers_count < 3 && time_since_last_gopher > 0.5{
            // If the hole is empty and last gopher spawned within more than 0.5s 
            // add a gopher with 50% probability
            let random: f32 = thread_rng().gen();
            if random > 0.5 {
                *gophers_count += 1;
                *last_gopher_spawned = get_time();
                self.gopher = Some(Gopher::new());
            }
        }
        
        // Count gopher time in the hole, delete if too long
        if let Some(gopher) = self.gopher.as_mut()  {
            gopher.time_elapsed += dt;
            if gopher.time_elapsed > gopher.max_time {
                self.gopher=None;
                *gophers_count -= 1;
            }
        };

        // Kill gopher if key is pressed
        self.is_pressed = is_key_down(key_map[&self.index]);
        if self.is_pressed && self.gopher.is_some() {
            *gophers_count -= 1;
            *points += HIT_SCORE - (100.0*self.gopher.as_ref().unwrap().time_elapsed/3.0) as i32;
            self.gopher=None;
        }
    }

    pub fn draw(&self) {
        draw_circle(self.circle.x, self.circle.y, RADIUS, BROWN);
        if self.gopher.is_some() {
            draw_texture_ex(&self.gopher_texture, self.circle.x-1.5*32.3/2.0, self.circle.y-1.5*17.9/2.0, WHITE, DrawTextureParams {
                dest_size: Some(vec2(1.5*32.3, 1.5*17.9)),
                ..Default::default()
            });
        }
        if self.is_pressed {
            draw_circle(self.circle.x, self.circle.y, 20.0, RED);
        }
    }
}
