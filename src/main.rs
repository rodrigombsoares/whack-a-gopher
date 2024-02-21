use std::collections::HashMap;

use ::rand::Rng;
use ::rand::thread_rng;
use macroquad::prelude::*;

const MARGING: f32 = 100f32;
const RADIUS: f32 = 50f32;
const HIT_SCORE: i32 = 100;

struct Gopher {
    time_elapsed: f32,
    max_time: f32,
}

impl Gopher {
    pub fn new() -> Self {
        let random: f32 = thread_rng().gen();
        Self {
            time_elapsed: 0.0,
            max_time: 2.0*random+1.0,
        }
    }
}

struct Hole {
    circle: Circle,
    gopher: Option<Gopher>,
    is_pressed: bool,
    index: i32,
}

impl Hole {
    pub fn new(i: f32) -> Self {
        let x = screen_width()*0.5 - 4.0*RADIUS - 20.0 + MARGING+((i-i%2.0)*(RADIUS+10.0));
        let y = screen_height()*0.5 - RADIUS - 10.0 +(RADIUS+10.0)*2.0*(i%2.0);

        Self {
            circle: Circle::new(x, y, RADIUS),
            gopher: None,
            is_pressed: false,
            index: i as i32,
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
            self.gopher=None;
            *gophers_count -= 1;
            *points += HIT_SCORE;
        }
    }

    pub fn draw(&self) {
        draw_circle(self.circle.x, self.circle.y, RADIUS, BROWN);
        if self.gopher.is_some() {
            draw_circle(self.circle.x, self.circle.y, 20.0, BLUE);
        }
        if self.is_pressed {
            draw_circle(self.circle.x, self.circle.y, 20.0, RED);
        }
    }
}


#[macroquad::main("BasicShapes")]
async fn main() {    
    let mut holes: Vec<Hole> = Vec::new();
    let mut gophers_count = 0;
    let mut last_gopher_spawned = 0.0;
    let mut points = 0;

    for i in 0..6 {
        let hole = Hole::new(i as f32);
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
