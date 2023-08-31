use macroquad::prelude::*;

use crate::consts;
use crate::models::GameState;
use crate::creature::*;
use crate::animation::*;


pub fn update(game_state: &mut GameState) {
    let player = &mut game_state.player;
    let elapsed = game_state.stats.elapsed;

    match player.animation {
        Some(ref mut animation) => {
            let (x, y) = animation.interpolate(elapsed);
            player.position.set(x, y);
            if elapsed - animation.start_time > animation.duration as f64 {
                player.animation = None;
            }
        },
        None => {}
    }

    let creatures = &mut game_state.creatures;
    for creature in creatures.iter_mut() {

        // gather all the inputs regardless of 
        // whether or not the brain needs them
        let inputs = vec![
            creature.x,
            creature.y,
            player.position.x,
            player.position.y,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
        ];

        match creature.animation {
            Some(ref mut animation) => {
                let (x, y) = animation.interpolate(elapsed);
                creature.x = x;
                creature.y = y;
                if elapsed - animation.start_time > animation.duration as f64 {
                    creature.animation = None;
                }
            },
            None => {}
        }
        
        if creature.animation.is_none() {
            let (_, output_type) = creature.brain.compute(inputs);

            match output_type {
                OutputTypes::MoveUp => {
                    creature.animation = Some(
                        AnimationTransition::new(
                            (creature.x, creature.y),
                            (creature.x, creature.y - consts::GRID_SIZE as f32),
                            elapsed, 0.3, CurveType::EaseQuadInOut
                        )
                    );
                },
                OutputTypes::MoveDown => {
                    creature.animation = Some(
                        AnimationTransition::new(
                            (creature.x, creature.y),
                            (creature.x, creature.y + consts::GRID_SIZE as f32),
                            elapsed, 0.3, CurveType::EaseQuadInOut
                        )
                    );
                },
                OutputTypes::MoveLeft => {
                    creature.animation = Some(
                        AnimationTransition::new(
                            (creature.x, creature.y),
                            (creature.x - consts::GRID_SIZE as f32, creature.y),
                            elapsed, 0.3, CurveType::EaseQuadInOut
                        )
                    );
                },
                OutputTypes::MoveRight => {
                    creature.animation = Some(
                        AnimationTransition::new(
                            (creature.x, creature.y),
                            (creature.x + consts::GRID_SIZE as f32, creature.y),
                            elapsed, 0.3, CurveType::EaseQuadInOut
                        )
                    );
                },
                _ => {}
            }
        }

        // collide with wall
        if creature.x <= 0.0 {
            creature.x = 0.0;
        }
        if creature.x >= consts::SCREEN_WIDTH as f32 {
            creature.x = consts::SCREEN_WIDTH as f32;
        }
        if creature.y <= 0.0 {
            creature.y = 0.0;
        }
        if creature.y >= consts::SCREEN_HEIGHT as f32 {
            creature.y = consts::SCREEN_HEIGHT as f32;
        }
    }

}