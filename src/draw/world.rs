use macroquad::prelude::*;

use crate::consts;
use crate::models::*;
use crate::world;
use crate::creature::OutputTypes;

fn draw_tile(texture: &Texture2D, pos: Vec2, size: Vec2) {
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

fn draw_tiles(game_state: &GameState, viewport: Rect) {
    let texture_map = &game_state.texture_map;
    let tile_grid = &game_state.world.tile_grid;

    for f in world::iter_layer(tile_grid, consts::WORLD_FLOOR_LAYER, viewport) {
        if let Some(tile) = f {
            draw_tile(&texture_map.get(&tile.texture), tile.position, Vec2::new(1.0, 1.0));
        }
    }
    for f in world::iter_layer(tile_grid, consts::WORLD_WALL_LAYER, viewport) {
        if let Some(tile) = f {
            draw_tile(&texture_map.get(&tile.texture), tile.position, Vec2::new(1.0, 1.0));
        }
    }
}

fn draw_creatures(game_state: &GameState, viewport: Rect, mouse_gpos: Vec2) {
    let texture_map = &game_state.texture_map;
    let creatures = &game_state.world.creatures;

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
}

fn draw_player(game_state: &GameState) {
    let texture_map = &game_state.texture_map;
    let player = &game_state.world.player;

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
}


pub fn draw(game_state: &GameState, viewport: Rect, mouse_gpos: Vec2) {
    draw_tiles(game_state, viewport);
    draw_creatures(game_state, viewport, mouse_gpos);
    draw_player(game_state);
}