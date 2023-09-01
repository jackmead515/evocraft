use macroquad::prelude::*;

use crate::consts;
use crate::models::*;
use crate::animation::*;

pub fn input(game_state: &mut GameState) {
    let player = &mut game_state.player;
    let elapsed = game_state.stats.elapsed;

    if player.animation.is_none() && player.energy.current > 5.0 {
        if is_key_down(KeyCode::D) {
            player.animation = Some(
                AnimationTransition::new(
                    player.position.get(),
                    (player.position.x + consts::GRID_SIZE as f32, player.position.y),
                    elapsed, 0.3, CurveType::EaseQuadInOut
                )
            );
            player.energy.expend(5.0);

        } else if is_key_down(KeyCode::A) {
            player.animation = Some(
                    AnimationTransition::new(
                    player.position.get(),
                    (player.position.x - consts::GRID_SIZE as f32, player.position.y),
                    elapsed, 0.3, CurveType::EaseQuadInOut
                )
            );
            player.energy.expend(5.0);

        } else if is_key_down(KeyCode::W) {
            player.animation = Some(
                AnimationTransition::new(
                    player.position.get(),
                    (player.position.x, player.position.y - consts::GRID_SIZE as f32),
                    elapsed, 0.3, CurveType::EaseQuadInOut
                )
            );
            player.energy.expend(5.0);

        } else if is_key_down(KeyCode::S) {
            player.animation = Some(
                AnimationTransition::new(
                    player.position.get(),
                    (player.position.x, player.position.y + consts::GRID_SIZE as f32),
                    elapsed, 0.3, CurveType::EaseQuadInOut
                )
            );
            player.energy.expend(5.0);

        }
    }
}