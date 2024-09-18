pub mod player {
    use std::time::{Duration, Instant};

    static GRAVITY: f32 = 9.8;

    use macroquad::{
        color::GREEN,
        input::is_key_down,
        math::{clamp, Rect, Vec2},
        shapes::draw_rectangle,
        texture::{draw_texture_ex, load_texture, DrawTextureParams, Texture2D},
        time::get_frame_time,
        window::{screen_height, screen_width},
    };

    use crate::bullets;

    pub struct Player {
        pos_x: f32,
        pos_y: f32,
        speed: f32,
        vy: f32,
        shooting_cooldown: Duration,
        last_shot: Instant,
        texture: Option<Texture2D>,
        current_frame: i32,
        animation_timer: f32,
        animation_speed: f32,
    }

    impl Player {
        pub async fn load_texture(&mut self) {
            self.texture = Some(
                load_texture("Wizardgooseassets/duck_run.png")
                    .await
                    .unwrap(),
            );
        }

        pub fn update_animation(&mut self) {
            if let Some(_) = self.texture {
                self.animation_timer += get_frame_time();
                if self.animation_timer > self.animation_speed {
                    self.animate();
                    self.animation_timer = 0.0;
                }
            }
        }

        fn animate(&mut self) {
            self.current_frame = (self.current_frame + 1) % 12; // Assuming 12 frames as in your original code
        }

        pub fn draw(&self) {
            if let Some(texture) = &self.texture {
                let frame_width = 64.0;
                let frame_height = 64.0;

                draw_texture_ex(
                    texture,
                    self.pos_x,
                    self.pos_y,
                    macroquad::color::WHITE,
                    DrawTextureParams {
                        source: Some(Rect::new(
                            self.current_frame as f32 * frame_width,
                            0.0,
                            frame_width,
                            frame_height,
                        )),
                        dest_size: Some(Vec2::new(128.0, 128.0)), // Match the size of your original rectangle
                        ..Default::default()
                    },
                );
            } else {
                // Fallback to drawing a rectangle if texture is not loaded
                draw_rectangle(self.pos_x, self.pos_y, 120.0, 60.0, GREEN);
            }
        }

        pub fn input_handle(&mut self, projectiles: &mut Vec<bullets::bullets::Bullet>) {
            if self.pos_y != (screen_height() - 128.0).round() {
                self.pos_y += self.vy * get_frame_time();
            }

            if is_key_down(macroquad::input::KeyCode::D) {
                self.pos_x += self.speed * get_frame_time();

                self.update_animation();
            }

            if is_key_down(macroquad::input::KeyCode::A) {
                self.pos_x -= self.speed * get_frame_time();

                self.update_animation();
            }

            if is_key_down(macroquad::input::KeyCode::Space) {
                if self.pos_y < (screen_height() - 128.0).round() {
                    return;
                }
                // substracting because the top is at 0, 0 and the bottom is at screeen_height
                // delta time = get_frame_time
                self.vy = 100.0 + GRAVITY * get_frame_time();
                self.pos_y -= self.vy;
            }
            if is_key_down(macroquad::input::KeyCode::S) {
                let now = Instant::now();

                if now.duration_since(self.last_shot) >= self.shooting_cooldown {
                    let bullet = bullets::bullets::Bullet::new(self.pos_x + 64.0, self.pos_y + 64.0);
                    projectiles.push(bullet);
                    self.last_shot = now;
                }
            }


            self.pos_x = clamp(self.pos_x, 0.0, screen_width() - 128.0);
            self.pos_y = clamp(self.pos_y, 0.0, screen_height() - 128.0);
        }

        pub fn new() -> Self {
            return Player {
                current_frame: 0,
                animation_timer: 0.0,
                animation_speed: 0.1,
                texture: None,
                last_shot: Instant::now(),
                shooting_cooldown: Duration::from_millis(500),
                pos_x: 0.0,
                pos_y: (screen_height() - 128.0).round(),
                speed: 200.0,
                vy: 0.0,
            };
        }

        pub fn pos_y(&self) -> f32 {
            self.pos_y
        }
    }
}
