use macroquad::prelude::*;
use macroquad::prelude::scene::camera_pos;

use crate::consts;
use crate::models::*;
use crate::brain::*;
use crate::world::WORLD_FLOOR_LAYER;
use crate::world::WORLD_WALL_LAYER;

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

    let camera = &Camera2D {
        zoom: vec2(zoom_factor * aspect_ratio, zoom_factor),
        target: player.position,
        ..Default::default()
    };

    // get mouse position
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

    for f in world.iter_layer(WORLD_FLOOR_LAYER, viewport) {
        if let Some(tile) = f {
            draw_tile(&texture_map.get(&tile.texture), tile.position, Vec2::new(1.0, 1.0));
        }
    }
    for f in world.iter_layer(WORLD_WALL_LAYER, viewport) {
        if let Some(tile) = f {
            draw_tile(&texture_map.get(&tile.texture), tile.position, Vec2::new(1.0, 1.0));
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

            let gpos = consts::grid_pos(&c.position);

            draw_rectangle_lines(gpos.x, gpos.y, 1.0, 1.0, 0.05, Color::new(1.0, 1.0, 1.0, 0.5));
            draw_rectangle_lines(c.position.x, c.position.y, 1.0, 1.0, 0.05, GREEN);

            // draw creature health and energy if mouse is over it
            if gpos == mouse_gpos {
                let health_ratio = c.health.value / c.health.max;
                let energy_ratio = c.energy.value / c.energy.max;
                let (cx, cy) = (c.position.x, c.position.y);

                draw_rectangle(cx, cy - 0.2, 1.0, 0.1, RED);
                draw_rectangle(cx, cy - 0.2, 1.0 * health_ratio, 0.1, GREEN);
                draw_rectangle(cx, cy-0.1, 1.0, 0.1, RED);
                draw_rectangle(cx, cy-0.1, 1.0 * energy_ratio, 0.1, YELLOW);
            }
        }
    }


    // draw player
    draw_tile(&texture_map.get("player/base/human_male"), player.position, Vec2::new(1.0, 1.0));
    let player_grid_pos = consts::grid_pos(&player.position);
    draw_rectangle_lines(player_grid_pos.x, player_grid_pos.y, 1.0, 1.0, 0.05, Color::new(1.0, 1.0, 1.0, 0.5));
    draw_rectangle_lines(player.position.x, player.position.y, 1.0, 1.0, 0.05, GREEN);
    let health_ratio = player.health.value / player.health.max;
    let energy_ratio = player.energy.value / player.energy.max;
    let (px, py) = (player.position.x, player.position.y);
    draw_rectangle(px, py - 0.2, 1.0, 0.1, RED);
    draw_rectangle(px, py - 0.2, 1.0 * health_ratio, 0.1, GREEN);
    draw_rectangle(px, py-0.1, 1.0, 0.1, RED);
    draw_rectangle(px, py-0.1, 1.0 * energy_ratio, 0.1, YELLOW);


    // draw mouse position
    draw_rectangle_lines(mouse_gpos.x, mouse_gpos.y, 1.0, 1.0, 0.05, Color::new(1.0, 1.0, 1.0, 0.5));
    //draw_circle_lines(mouse_gpos.x, mouse_gpos.y, 0.5, 0.05, Color::new(1.0, 1.0, 1.0, 0.5));
    
    // draw circle at mouse position
    draw_circle_lines(mouse_wpos.x, mouse_wpos.y, 0.5, 0.05, GREEN);


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
