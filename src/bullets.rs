pub mod bullets {
    use macroquad::{
        color::WHITE,
        math::vec2,
        texture::{
            draw_texture_ex, load_texture, DrawTextureParams, FilterMode, Texture2D,
        },
    };

    #[derive(Debug)]
    pub struct Bullet {
        velocity: f32,
        size: (f32, f32),

        texture: String,

        pos_y: f32,
        pub pos_x: f32,
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

        pub async fn travel(&mut self) {
            let bullet_texture: Texture2D = load_texture("fireball.png").await.unwrap();
            bullet_texture.set_filter(FilterMode::Nearest);
            self.pos_x += 10.0;

            draw_texture_ex(
                &bullet_texture,
                self.pos_x,
                self.pos_y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(self.size.0, self.size.1)),
                    ..Default::default()
                },
            );

            //draw_texture(&bullet_texture, self.pos_x, self.pos_y, WHITE);
        }
    }
}
