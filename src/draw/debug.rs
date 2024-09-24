use macroquad::prelude::*;

use crate::GameState;
use crate::consts;

pub fn draw_debug_stats(game_state: &GameState, mouse_wpos: Vec2, mouse_gpos: Vec2) {

    let player = &game_state.world.player;
    let creatures = &game_state.world.creatures;
    let player_grid_pos = consts::grid_pos(&player.position);

    let zoom_factor = game_state.stats.zoom_factor;
    let aspect_ratio = screen_height() / screen_width();

    let camera = &Camera2D {
        zoom: vec2(zoom_factor * aspect_ratio, zoom_factor),
        target: player.position,
        ..Default::default()
    };

     // draw fps
    draw_text(
        &format!("FPS: {}", game_state.stats.fps),
        10.0,
        20.0,
        20.0,
        WHITE,
    );

    
    draw_text(&format!("Mouse WP: {:?}", (mouse_wpos.x, mouse_wpos.y)), 10.0, 40.0, 20.0, WHITE);
    draw_text(&format!("Mouse GP: {:?}", (mouse_gpos.x, mouse_gpos.y)), 10.0, 60.0, 20.0, WHITE);

    draw_text(
        &format!("Player WP: {:?}", (player.position.x, player.position.y)),
        10.0,
        80.0,
        20.0,
        WHITE,
    );

    draw_text(
        &format!("Player GP: {:?}", (player_grid_pos.x, player_grid_pos.y)),
        10.0,
        100.0,
        20.0,
        WHITE,
    );
    draw_text(
        &format!("Player Health: {:?}", player.health.value),
        10.0,
        120.0,
        20.0,
        WHITE,
    );
    draw_text(
        &format!("Player Energy: {:?}", player.energy.value),
        10.0,
        140.0,
        20.0,
        WHITE,
    );

    draw_text(
        &format!("Time: {}", game_state.stats.elapsed as u64),
        10.0,
        160.0,
        20.0,
        WHITE,
    );
    draw_text(
        &format!("Creatures: {}", creatures.len()),
        10.0,
        180.0,
        20.0,
        WHITE,
    );
}