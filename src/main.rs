extern crate rand;
extern crate grid;

use macroquad::prelude::*;

pub mod consts;
pub mod draw;
pub mod update;
pub mod animation;
pub mod genes;
pub mod models;
pub mod creature;
pub mod grid_map;
pub mod demo;

use consts::*;
use models::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "EvoCraft".to_owned(),
        fullscreen: false,
        window_height: SCREEN_HEIGHT as i32,
        window_width: SCREEN_WIDTH as i32,
        window_resizable: false,
        ..Default::default()
    }
}


#[macroquad::main(window_conf)]
async fn main() {
    
    let mut game_state = demo::generate(DemoType::RandomCreatures1).await;

    loop {
        game_state.stats.fps = get_fps();
        game_state.stats.frame_time = get_frame_time();
        game_state.stats.elapsed = get_time();

        demo::update(&mut game_state);
        update::update(&mut game_state);
        draw::draw(&game_state);

        next_frame().await;
    }
}