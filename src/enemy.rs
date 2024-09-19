pub mod enemy {
    use core::f32;

    use ::rand::{rngs, Rng};
    use macroquad::{prelude::rand, rand::rand, window::screen_width};

    pub struct enemy {
        pos_y: f32,
        pos_x: f32,

        speed: f32,

        dist_x: f32,

        health: f32,
    }

    impl enemy {
        pub fn move_towards_player(&mut self) {}

        pub fn new() -> enemy {
            let rand: f32 = rand::gen_range(0.0, screen_width() - 20.0);

            enemy {
                dist_x: f32::MAX,
                pos_y: 128.0,
                pos_x: rand,

                speed: 200.0,
                health: 200.0,
            }
        }

        pub fn update_dist(&mut self, p_pos_x: f32) {
            self.dist_x = p_pos_x - self.pos_x;
        }

        pub fn draw() {


        }
    }
}
