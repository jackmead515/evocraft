extern crate rand;
extern crate grid;

use macroquad::prelude::*;

pub mod consts;
pub mod util;
pub mod draw;
pub mod update;
pub mod models;
pub mod creature;
pub mod grid_map;
pub mod textures;
pub mod world;
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
        high_dpi: true,
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

        let mut zoom_factor = game_state.stats.zoom_factor;
        let scroll = mouse_wheel().1;
        zoom_factor += scroll * 0.01;
        zoom_factor = clamp(zoom_factor, 0.05, 0.1);
        game_state.stats.zoom_factor = zoom_factor;

        //demo::update(&mut game_state);
        update::update(&mut game_state);
        draw::draw(&game_state);

        next_frame().await;
    }
}