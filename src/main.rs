use macroquad::prelude::*;
use miniquad::log;

pub mod player;

#[macroquad::main("2d Game")]
async fn main() {
    let mut player = player::player::Player::new();
    loop {
        clear_background(BLUE);
        player.input_handle();
    log!(log::Level::Debug, "{}, {}", screen_height(), player.pos_y.round());
        player.draw();
        next_frame().await
    }
}
