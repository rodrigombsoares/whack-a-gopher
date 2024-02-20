use std::collections::HashMap;

use macroquad::prelude::*;

const MARGING: f32 = 100f32;
const RADIUS: f32 = 50f32;

struct Hole {
    circle: Circle,
    has_gopher: bool,
    is_pressed: bool,
    index: i32,
}

impl Hole {
    pub fn new(i: f32) -> Self {
        let x = screen_width()*0.5 - 4.0*RADIUS - 20.0 + MARGING+((i-i%2.0)*(RADIUS+10.0));
        let y = screen_height()*0.5 - RADIUS - 10.0 +(RADIUS+10.0)*2.0*(i%2.0);

        Self {
            circle: Circle::new(x, y, RADIUS),
            has_gopher: true,
            is_pressed: false,
            index: i as i32,
        }
    }

    pub fn update(&mut self) {
        let key_map = HashMap::from([
            (0, KeyCode::Q),
            (1, KeyCode::A),
            (2, KeyCode::W),
            (3, KeyCode::S),
            (4, KeyCode::E),
            (5, KeyCode::D),
        ]);
    
        self.has_gopher = !self.has_gopher;
        self.is_pressed = is_key_down(key_map[&self.index])
    }

    pub fn draw(&self) {
        draw_circle(self.circle.x, self.circle.y, RADIUS, BROWN);
        if self.has_gopher {
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
    for i in 0..6 {
        let hole = Hole::new(i as f32);
        holes.push(hole);
    }

    loop {
        clear_background(WHITE);
        draw_rectangle(MARGING*0.5f32, MARGING*0.5f32, screen_width()-MARGING, screen_height()-MARGING, GREEN);
        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);
        for hole in &mut holes {
            hole.update();
            hole.draw();
        }
        next_frame().await
    }
}
