use macroquad::prelude::*;
use macroquad::prelude::scene::camera_pos;

use crate::consts;
use crate::models::*;
use crate::brain::*;

pub fn draw_debug_grid(game_state: &GameState) {
    let player = &game_state.player;
    let gv = consts::grid_pos(&player.position);

    let grid_color = Color::new(0.1, 0.1, 0.1, 0.5);

    let min = -10.0;
    let max = 10.0;

    for x in ((gv.x + min) as i32)..((gv.x + max + 1.0) as i32) {
        draw_line(x as f32, min + gv.y, x as f32, max + gv.y, 0.05, grid_color);
    }

    for y in ((gv.y + min) as i32)..((gv.y + max + 1.0) as i32) {
        draw_line(min + gv.x, y as f32, max + gv.x, y as f32, 0.05, grid_color);
    }
}

/// Draws a tile from a tileset
pub fn draw_tile(texture: &Texture2D, pos: Vec2, size: Vec2) {
    draw_texture_ex(
        texture,
        pos.x,
        pos.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(size),
            ..Default::default()
        },
    );
}

pub fn draw(game_state: &GameState) {
    clear_background(BLACK);

    let player = &game_state.player;
    let world = &game_state.world;
    let texture_map = &game_state.texture_map;
    let aspect_ratio = screen_height() / screen_width();
    let zoom_factor = game_state.stats.zoom_factor;
    let pv = player.position;

    let camera = &Camera2D {
        zoom: vec2(zoom_factor * aspect_ratio, zoom_factor),
        target: pv,
        ..Default::default()
    };

    let vp1 = camera.screen_to_world(Vec2::new(0.0, 0.0));
    let vp2 = camera.screen_to_world(Vec2::new(screen_width(), screen_height()));
    let viewport = Rect::new(
        vp1.x.floor(),
        vp1.y.floor(),
        (vp2.x - vp1.x).ceil(),
        (vp2.y - vp1.y).ceil()
    );

    set_camera(camera);

    for f in &world.floor {
        if viewport.contains(f.position) {
            draw_tile(&texture_map.get(&f.texture), f.position, Vec2::new(1.0, 1.0));
        }
    }

    for b in &world.boundaries {
        if viewport.contains(b.position) {
            draw_tile(&texture_map.get(&b.texture), b.position, Vec2::new(1.0, 1.0));
        }
    }

    //draw_debug_grid(game_state);

    let creatures = &game_state.creatures;

    for c in creatures {
        if viewport.contains(c.position) {
            draw_tile(&texture_map.get("monster/eyes/giant_eyeball"), c.position, Vec2::new(1.0, 1.0));

            if let Some(behavior) = c.current_behavior {
                match behavior {
                    OutputTypes::BehaviorRest => {
                        draw_tile(&texture_map.get("misc/brands/top_right/sleeping"), c.position, Vec2::new(1.0, 1.0));
                    },
                    _ => {}
                }
            }
        }
    }

    draw_tile(&texture_map.get("player/base/human_male"), player.position, Vec2::new(1.0, 1.0));

    let (mx, my) = mouse_position();
    let mouse_wpos = camera.screen_to_world(Vec2::new(mx, my));
    let mouse_gpos = consts::grid_pos(&mouse_wpos);

    set_default_camera();

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
        &format!("Player WP: {:?}", (pv.x, pv.y)),
        10.0,
        80.0,
        20.0,
        WHITE,
    );

    let gv = consts::grid_pos(&player.position);

    draw_text(
        &format!("Player GP: {:?}", (gv.x, gv.y)),
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
