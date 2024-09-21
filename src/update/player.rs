use macroquad::prelude::*;

use crate::models::*;
use crate::util::animation::{AnimationMovement, CurveType};
use crate::consts;
use crate::util::delay::Delay;

pub fn update(game_state: &mut GameState) {
    let player = &mut game_state.player;
    let world = &game_state.world;
    let elapsed = game_state.stats.elapsed;

    let speed = 0.1;

    // update the player movement animation
    if player.movement.is_none() {
        if is_key_down(KeyCode::D) {
            let final_pos = Vec2::new(player.position.x + 1.0, player.position.y);
            let fgrid_pos = consts::grid_pos(&final_pos);

            if player.energy.value > 3.0 && world.collide_with(&fgrid_pos, consts::WORLD_WALL_LAYER).is_none() {
                player.movement = Some(
                    AnimationMovement::new(
                        player.position,
                        final_pos,
                        elapsed, speed, CurveType::Linear
                    )
                );
                player.energy.consume(3.0);
            }
        } else if is_key_down(KeyCode::A) {
            let final_pos = Vec2::new(player.position.x - 1.0, player.position.y);
            let fgrid_pos = consts::grid_pos(&final_pos);

            if player.energy.value > 3.0 && world.collide_with(&fgrid_pos, consts::WORLD_WALL_LAYER).is_none() {
                player.movement = Some(
                    AnimationMovement::new(
                        player.position,
                        final_pos,
                        elapsed, speed, CurveType::Linear
                    )
                );
                player.energy.consume(3.0);
            }

        } else if is_key_down(KeyCode::W) {
            let final_pos = Vec2::new(player.position.x, player.position.y - 1.0);
            let fgrid_pos = consts::grid_pos(&final_pos);

            if player.energy.value > 3.0 && world.collide_with(&fgrid_pos, consts::WORLD_WALL_LAYER).is_none() {
                player.movement = Some(
                    AnimationMovement::new(
                        player.position,
                        final_pos,
                        elapsed, speed, CurveType::Linear
                    )
                );
                player.energy.consume(3.0);
            }

        } else if is_key_down(KeyCode::S) {
            let final_pos = Vec2::new(player.position.x, player.position.y + 1.0);
            let fgrid_pos = consts::grid_pos(&final_pos);

            if player.energy.value > 3.0 && world.collide_with(&fgrid_pos, consts::WORLD_WALL_LAYER).is_none() {
                player.movement = Some(
                    AnimationMovement::new(
                        player.position,
                        final_pos,
                        elapsed, speed, CurveType::Linear
                    )
                );
                player.energy.consume(3.0);
            }

        }
    }

    // update the player position if there is an animation
    match player.movement {
        Some(ref mut animation) => {
            player.position = animation.interpolate(elapsed);
            if animation.is_complete(elapsed) {
                player.position = consts::grid_pos(&animation.final_pos);
                player.movement = None;
            }
        },
        None => {}
    }

    match player.delays.energy_restore {
        Some(ref mut delay) => {
            if delay.is_complete(elapsed) {
                player.energy.restore(1.0);
                player.delays.energy_restore = None;
            }
        },
        None => {
            let vigor = player.vigor.invert_percent().into();
            player.delays.energy_restore = Some(Delay::new(vigor, elapsed));
        }
    }

    // every 1 second, restore 1 energy to player
    if elapsed % 1.0 < 0.01 {
        player.health.consume(0.5);

        if player.energy.value < 10.0 {
            player.health.consume(5.0);
        }
    }
}