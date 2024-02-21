use ::rand::Rng;
use ::rand::thread_rng;

pub struct Gopher {
    pub time_elapsed: f32,
    pub max_time: f32,
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
