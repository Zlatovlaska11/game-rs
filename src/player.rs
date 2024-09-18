pub mod player {

    static GRAVITY: f32 = 9.8;

    use macroquad::{
        color::GREEN,
        input::is_key_down,
        math::{self, clamp},
        miniquad::log,
        shapes::draw_rectangle,
        time::get_frame_time,
        window::{screen_height, screen_width},
    };

    use crate::{bullets, GameObjects, GAME_OBJ};

    pub struct Player {
        pos_x: f32,
        pub pos_y: f32,
        speed: f32,
        Vy: f32,
    }

    impl Player {
        pub fn input_handle(&mut self, projectiles: &mut Vec<bullets::bullets::Bullet>) {
            log!(log::Level::Warn, "inside input handle");
            if self.pos_y != (screen_height() - 60.0).round() {
                self.pos_y += self.Vy * get_frame_time();
            }

            log!(log::Level::Warn, "inside input handle");
            if is_key_down(macroquad::input::KeyCode::D) {
                self.pos_x += self.speed * get_frame_time();
            }

            log!(log::Level::Warn, "inside input handle");

            if is_key_down(macroquad::input::KeyCode::A) {
                self.pos_x -= self.speed * get_frame_time();
            }

            log!(log::Level::Warn, "inside input handle");

            if is_key_down(macroquad::input::KeyCode::Space) {
                if self.pos_y < (screen_height() - 60.0).round() {
                    return;
                }
                // substracting because the top is at 0, 0 and the bottom is at screeen_height
                // delta time = get_frame_time
                self.Vy = 200.0 + GRAVITY * get_frame_time();
                self.pos_y -= self.Vy;
            }

            log!(log::Level::Warn, "inside input handle");
            if is_key_down(macroquad::input::KeyCode::S) {
                let bullet = bullets::bullets::Bullet::new(self.pos_x, self.pos_y);
                projectiles.push(bullet);
            }

            self.pos_x = clamp(self.pos_x, 0.0, screen_width() - 120.0);
            self.pos_y = clamp(self.pos_y, 0.0, screen_height() - 60.0);
            log!(log::Level::Warn, "last line")
        }

        pub fn draw(&mut self) {
            draw_rectangle(self.pos_x, self.pos_y, 120.0, 60.0, GREEN);
        }

        pub fn new() -> Self {
            return Player {
                pos_x: 0.0,
                pos_y: (screen_height() - 60.0).round(),
                speed: 200.0,
                Vy: 0.0,
            };
        }
    }
}
