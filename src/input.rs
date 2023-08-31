use macroquad::prelude::*;

use crate::consts;
use crate::models::*;
use crate::animation::*;

pub fn input(game_state: &mut GameState) {
    let player = &mut game_state.player;
    let elapsed = game_state.stats.elapsed;

    if is_key_down(KeyCode::D) {
        if player.animation.is_none() {
            player.animation = Some(
                AnimationTransition::new(
                    player.position.get(),
                    (player.position.x + consts::GRID_SIZE as f32, player.position.y),
                    elapsed, 0.3, CurveType::EaseQuadInOut
                )
            )
        }
    } else if is_key_down(KeyCode::A) {
        if player.animation.is_none() {
            player.animation = Some(
                AnimationTransition::new(
                    player.position.get(),
                    (player.position.x - consts::GRID_SIZE as f32, player.position.y),
                    elapsed, 0.3, CurveType::EaseQuadInOut
                )
            )
        }
    } else if is_key_down(KeyCode::W) {
        if player.animation.is_none() {
            player.animation = Some(
                AnimationTransition::new(
                    player.position.get(),
                    (player.position.x, player.position.y - consts::GRID_SIZE as f32),
                    elapsed, 0.3, CurveType::EaseQuadInOut
                )
            )
        }
    } else if is_key_down(KeyCode::S) {
        if player.animation.is_none() {
            player.animation = Some(
                AnimationTransition::new(
                    player.position.get(),
                    (player.position.x, player.position.y + consts::GRID_SIZE as f32),
                    elapsed, 0.3, CurveType::EaseQuadInOut
                )
            )
        }
    }
}