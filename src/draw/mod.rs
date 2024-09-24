use macroquad::prelude::*;

use crate::GameState;
use crate::consts;

pub mod debug;
pub mod world;
pub mod ui;

pub fn draw(game_state: &GameState) {
    clear_background(BLACK);
    
    let player = &game_state.world.player;
    let zoom_factor = game_state.stats.zoom_factor;
    let aspect_ratio = screen_height() / screen_width();

    let camera = &Camera2D {
        zoom: vec2(zoom_factor * aspect_ratio, zoom_factor),
        target: player.position,
        ..Default::default()
    };

    let (mouse_x, mouse_y) = mouse_position();
    let mouse_wpos = camera.screen_to_world(Vec2::new(mouse_x, mouse_y));
    let mouse_gpos = consts::grid_pos(&Vec2::new(mouse_wpos.x - 0.5, mouse_wpos.y - 0.5));

    // calculate the viewport
    let vp1 = camera.screen_to_world(Vec2::new(0.0, 0.0));
    let vp2 = camera.screen_to_world(Vec2::new(screen_width(), screen_height()));
    let viewport = Rect::new(
        vp1.x.floor(),
        vp1.y.floor(),
        (vp2.x - vp1.x).ceil(),
        (vp2.y - vp1.y).ceil()
    );

    set_camera(camera);

    world::draw(game_state, viewport, mouse_gpos);

    set_default_camera();

    // draw mouse position
    draw_rectangle_lines(mouse_gpos.x, mouse_gpos.y, 1.0, 1.0, 0.05, Color::new(1.0, 1.0, 1.0, 0.5));

    ui::draw(game_state);

    debug::draw_debug_stats(game_state, mouse_wpos, mouse_gpos);
}