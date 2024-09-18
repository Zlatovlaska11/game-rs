pub mod player {
    use std::time::{Duration, Instant};

    static GRAVITY: f32 = 9.8;

    use macroquad::{
        color::GREEN,
        input::is_key_down,
        math::clamp,
        shapes::draw_rectangle,
        time::get_frame_time,
        window::{screen_height, screen_width},
    };

    use crate::bullets;

    pub struct Player {
        pos_x: f32,
        pub pos_y: f32,
        speed: f32,
        Vy: f32,
        shooting_cooldown: Duration,
        last_shot: Instant,
    }

    impl Player {
        pub fn input_handle(&mut self, projectiles: &mut Vec<bullets::bullets::Bullet>) {
            if self.pos_y != (screen_height() - 60.0).round() {
                self.pos_y += self.Vy * get_frame_time();
            }

            if is_key_down(macroquad::input::KeyCode::D) {
                self.pos_x += self.speed * get_frame_time();
            }

            if is_key_down(macroquad::input::KeyCode::A) {
                self.pos_x -= self.speed * get_frame_time();
            }

            if is_key_down(macroquad::input::KeyCode::Space) {
                if self.pos_y < (screen_height() - 60.0).round() {
                    return;
                }
                // substracting because the top is at 0, 0 and the bottom is at screeen_height
                // delta time = get_frame_time
                self.Vy = 200.0 + GRAVITY * get_frame_time();
                self.pos_y -= self.Vy;
            }
            if is_key_down(macroquad::input::KeyCode::S) {
                let now = Instant::now();

                if now.duration_since(self.last_shot) >= self.shooting_cooldown {

                    let bullet =
                        bullets::bullets::Bullet::new(self.pos_x.clone(), self.pos_y.clone());
                    projectiles.push(bullet);
                    self.last_shot = now;
                }
            }

            self.pos_x = clamp(self.pos_x, 0.0, screen_width() - 120.0);
            self.pos_y = clamp(self.pos_y, 0.0, screen_height() - 60.0);
        }

        pub fn draw(&mut self) {
            draw_rectangle(self.pos_x, self.pos_y, 120.0, 60.0, GREEN);
        }

        pub fn new() -> Self {
            return Player {
                last_shot: Instant::now(),
                shooting_cooldown: Duration::from_millis(500),
                pos_x: 0.0,
                pos_y: (screen_height() - 60.0).round(),
                speed: 200.0,
                Vy: 0.0,
            };
        }
    }
}
