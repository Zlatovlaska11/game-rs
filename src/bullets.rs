pub mod bullets {
    use macroquad::{
        color::WHITE, shapes::draw_circle, time::get_frame_time, window::screen_width,
    };

    use crate::GameObjects;

    #[derive(Debug, Clone)]
    pub struct Bullet {
        velocity: f32,
        size: (f32, f32),

        texture: String,

        pos_y: f32,
        pos_x: f32,
    }

    impl Bullet {
        pub fn new(x: f32, y: f32) -> Self {
            return Self {
                velocity: 400.0,
                size: (20.0, 20.0),
                texture: "todo".to_string(),

                pos_x: x,
                pos_y: y,
            };
        }

        pub fn travel(&mut self) {
            self.pos_x += self.velocity * get_frame_time();
            draw_circle(self.pos_x, self.pos_y, 10.0, WHITE);
        }
    }

}
