use std::{
    borrow::BorrowMut,
    cell::RefCell,
    sync::{Arc, Mutex},
};

use bullets::bullets::Bullet;
use lazy_static::lazy_static;
use macroquad::{prelude::*, ui::root_ui};
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

pub fn update_game_obj() {
    // Handle player input
    {
        let game_obj = GAME_OBJ.lock().unwrap();

        // Borrow player mutably
        let mut player = game_obj.player.borrow_mut();

        // Borrow projectiles mutably
        let mut projectiles = game_obj.projectiles.borrow_mut();

        // Call input_handle with a mutable reference to projectiles
        player.input_handle(&mut projectiles);
    }

    // Draw player
    {
        let mut game_oj = GAME_OBJ.lock().unwrap();
        game_oj.player.borrow_mut().draw();
    }

    // Update projectiles
    {
        let mut game_oj = GAME_OBJ.lock().unwrap();

        let mut projectiles = game_oj.projectiles.borrow_mut();
        for mut bullet in <Vec<Bullet> as Clone>::clone(&projectiles).into_iter() {
            bullet.travel();
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
        log!(log::Level::Error, "about to lock");
        update_game_obj();

        log!(log::Level::Error, "locked");

        next_frame().await
    }
}
