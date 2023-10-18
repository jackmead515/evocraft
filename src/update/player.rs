use macroquad::prelude::*;

use crate::models::*;
use crate::animation::*;
use crate::consts;

pub fn update(game_state: &mut GameState) {
    let player = &mut game_state.player;
    let elapsed = game_state.stats.elapsed;

    let speed = 0.1;

    // update the player movement animation
    if player.animation.is_none() {
        if is_key_down(KeyCode::D) {
            player.animation = Some(
                AnimationTransition::new(
                    player.position,
                    Vec2::new(player.position.x + 1.0, player.position.y),
                    elapsed, speed, CurveType::Linear
                )
            );
            player.energy.consume(3.0);

        } else if is_key_down(KeyCode::A) {
            player.animation = Some(
                    AnimationTransition::new(
                    player.position,
                    Vec2::new(player.position.x - 1.0, player.position.y),
                    elapsed, speed, CurveType::Linear
                )
            );
            player.energy.consume(3.0);

        } else if is_key_down(KeyCode::W) {
            player.animation = Some(
                AnimationTransition::new(
                    player.position,
                    Vec2::new(player.position.x, player.position.y - 1.0),
                    elapsed, speed, CurveType::Linear
                )
            );
            player.energy.consume(3.0);

        } else if is_key_down(KeyCode::S) {
            player.animation = Some(
                AnimationTransition::new(
                    player.position,
                    Vec2::new(player.position.x, player.position.y + 1.0),
                    elapsed, speed, CurveType::Linear
                )
            );
            player.energy.consume(3.0);

        }
    }

    // update the player position if there is an animation
    match player.animation {
        Some(ref mut animation) => {
            player.position = animation.interpolate(elapsed);
            if animation.is_complete(elapsed) {
                player.animation = None;
                player.position = consts::grid_pos(&player.position);
            }
        },
        None => {}
    }

    // collide with world boundaries
    let world = &game_state.world;
    if let Some(boundary) = world.collide(&player.position) {
        if let Some(ref mut animation) = player.animation {
            player.position = animation.initial_pos;
            player.animation = None;
        }
    }

    // every 1 second, restore 1 energy to player
    if elapsed % 1.0 < 0.01 {
        player.energy.restore(1.0);
        player.health.restore(1.0);

        if player.energy.value < 10.0 {
            player.health.consume(5.0);
        }
    }
}