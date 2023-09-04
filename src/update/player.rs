use macroquad::prelude::*;

use crate::models::*;
use crate::consts::*;
use crate::animation::*;

pub fn update(game_state: &mut GameState) {
    let player = &mut game_state.player;
    let elapsed = game_state.stats.elapsed;

    // update the player movement animation
    if player.animation.is_none() && player.energy.value > 3.0 {
        if is_key_down(KeyCode::D) {
            player.animation = Some(
                AnimationTransition::new(
                    player.position.get(),
                    (player.position.x + GRID_SIZE as f32, player.position.y),
                    elapsed, 0.3, CurveType::EaseQuadInOut
                )
            );
            player.energy.consume(3.0);

        } else if is_key_down(KeyCode::A) {
            player.animation = Some(
                    AnimationTransition::new(
                    player.position.get(),
                    (player.position.x - GRID_SIZE as f32, player.position.y),
                    elapsed, 0.3, CurveType::EaseQuadInOut
                )
            );
            player.energy.consume(3.0);

        } else if is_key_down(KeyCode::W) {
            player.animation = Some(
                AnimationTransition::new(
                    player.position.get(),
                    (player.position.x, player.position.y - GRID_SIZE as f32),
                    elapsed, 0.3, CurveType::EaseQuadInOut
                )
            );
            player.energy.consume(3.0);

        } else if is_key_down(KeyCode::S) {
            player.animation = Some(
                AnimationTransition::new(
                    player.position.get(),
                    (player.position.x, player.position.y + GRID_SIZE as f32),
                    elapsed, 0.3, CurveType::EaseQuadInOut
                )
            );
            player.energy.consume(3.0);

        }
    }

    // update the player position if there is an animation
    match player.animation {
        Some(ref mut animation) => {
            player.position.sett(animation.interpolate(elapsed));
            if animation.is_complete(elapsed) {
                player.animation = None;
            }
        },
        None => {}
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