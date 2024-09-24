use macroquad::prelude::*;

use crate::models::GameState;

pub mod player;
pub mod creatures;
pub mod world;

pub fn update(game_state: &mut GameState) {
    game_state.stats.fps = get_fps();
    game_state.stats.frame_time = get_frame_time();
    game_state.stats.elapsed = get_time();

    let mut zoom_factor = game_state.stats.zoom_factor;
    let scroll = mouse_wheel().1;
    zoom_factor += scroll * 0.01;
    zoom_factor = clamp(zoom_factor, 0.05, 0.1);
    game_state.stats.zoom_factor = zoom_factor;

    player::update(game_state);
    creatures::update(game_state);
    world::update(game_state);
}