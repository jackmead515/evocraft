extern crate rand;
extern crate grid;

use macroquad::prelude::*;

pub mod consts;
pub mod util;
pub mod update;
pub mod models;
pub mod creature;
pub mod textures;
pub mod world;
pub mod demo;
pub mod draw;

use consts::*;
use models::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "EvoCraft".to_owned(),
        fullscreen: false,
        window_height: SCREEN_HEIGHT as i32,
        window_width: SCREEN_WIDTH as i32,
        window_resizable: false,
        high_dpi: true,
        ..Default::default()
    }
}


#[macroquad::main(window_conf)]
async fn main() {

    let mut game_state = demo::generate(DemoType::RandomCreatures1).await;

    loop {
        //demo::update(&mut game_state);
        update::update(&mut game_state);
        draw::draw(&game_state);

        next_frame().await;
    }
}