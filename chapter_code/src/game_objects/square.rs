use rand::Rng;

pub struct Square {
    pub color: [f32; 3],
    pub position: [f32; 2],
    pub speed: f32,
}

impl Square {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            color: [1.0, 0.0, 0.0],
            position: [0.0, 0.0],
            speed: 1.3,
        }
    }

    pub fn change_to_random_color(&mut self) {
        let get_random_float = || rand::thread_rng().gen_range(0..100) as f32 / 100.0;
        self.color = [get_random_float(), get_random_float(), get_random_float()];
    }

    pub fn move_right(&mut self, seconds_passed: f32) {
        self.position[0] += seconds_passed * self.speed
    }

    pub fn move_left(&mut self, seconds_passed: f32) {
        self.position[0] -= seconds_passed * self.speed
    }

    pub fn move_up(&mut self, seconds_passed: f32) {
        self.position[1] -= seconds_passed * self.speed
    }

    pub fn move_down(&mut self, seconds_passed: f32) {
        self.position[1] += seconds_passed * self.speed
    }
}
