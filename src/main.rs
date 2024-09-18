use std::time::{Duration, Instant};
use std::{
    borrow::BorrowMut,
    cell::RefCell,
    sync::{Arc, Mutex},
};

use bullets::bullets::Bullet;
use lazy_static::lazy_static;
use macroquad::prelude::*;
use miniquad::log;

mod bullets;
pub mod player;

pub struct GameObjects {
    pub player: RefCell<player::player::Player>,
    pub projectiles: RefCell<Vec<bullets::bullets::Bullet>>,
}

lazy_static! {
    static ref GAME_OBJ: Arc<Mutex<GameObjects>> = Arc::new(Mutex::new(GameObjects {
        player: RefCell::new(player::player::Player::new()),
        projectiles: RefCell::new(Vec::new()),
    }));
}

pub async fn update_game_obj() {
    {
        let game_obj = GAME_OBJ.lock().unwrap();

        let mut player = game_obj.player.borrow_mut();

        let mut projectiles = game_obj.projectiles.borrow_mut();

        player.input_handle(&mut projectiles);
    }

    {
        let game_oj = GAME_OBJ.lock().unwrap();
        game_oj.player.borrow_mut().draw();
    }

    {
        let game_obj = GAME_OBJ.lock().unwrap();
        let mut projectiles = game_obj.projectiles.borrow_mut();
        projectiles.retain(|x| x.pos_x <= screen_width());
        for bullet in projectiles.iter_mut() {
            if bullet.pos_x >= screen_width() {


            }
            bullet.travel().await;
        }
    }
}

#[macroquad::main("2d Game")]
async fn main() {
    set_pc_assets_folder("assets");
    let background = load_texture("background.png").await.unwrap();
    loop {
        draw_texture_ex(
            &background,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        update_game_obj().await;

        next_frame().await
    }
}
