use std::ops::DerefMut;
use std::rc::Rc;

use macroquad::prelude::*;

use crate::consts;
use crate::models::*;
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

    // every 1 second, restore 1 energy to player
    if elapsed % 1.0 < 0.1 {
        player.energy.restore(1.0);
    }

    for c in game_state.creatures.iter_mut() {

        // update animation
        match &c.animation {
            Some(animation) => {
                c.position.sett(animation.interpolate(elapsed));
                if elapsed - animation.start_time > animation.duration as f64 {
                    c.animation = None;
                }
            },
            None => {}
        }
        
        // update the brain if no animation
        if c.animation.is_none() {

            let mut brain_inputs = Vec::new();

            // gather only the indicies that the brain needs
            for input_type in &c.brain.input_types {

                match input_type {
                    InputTypes::CurrentPosition => {
                        brain_inputs.push(c.position.x);
                        brain_inputs.push(c.position.y);
                    },
                    InputTypes::PlayerPosition => {
                        brain_inputs.push(player.position.x);
                        brain_inputs.push(player.position.y);
                    },
                    InputTypes::CurrentHealth => {
                        brain_inputs.push(c.health.current);
                    },
                    InputTypes::CurrentEnergy => {
                        brain_inputs.push(c.energy.current);
                    },
                    InputTypes::PlayerEnergy => {
                        brain_inputs.push(player.energy.current);
                    },
                    InputTypes::PlayerHealth => {
                        brain_inputs.push(player.health.current);
                    },
                    InputTypes::NearCreatures => {

                        let (x, y) = consts::grid_pos(c.position.x, c.position.y);

                        let mut nearby = Vec::with_capacity(9);
                        
                        nearby.extend_from_slice(&game_state.creature_map.get_all((x-1) as usize, (y-1) as usize));
                        nearby.extend_from_slice(&game_state.creature_map.get_all((x-1) as usize, y as usize));
                        nearby.extend_from_slice(&game_state.creature_map.get_all((x-1) as usize, (y+1) as usize));
                        nearby.extend_from_slice(&game_state.creature_map.get_all(x as usize, (y-1) as usize));
                        nearby.extend_from_slice(&game_state.creature_map.get_all(x as usize, y as usize));
                        nearby.extend_from_slice(&game_state.creature_map.get_all(x as usize, (y+1) as usize));
                        nearby.extend_from_slice(&game_state.creature_map.get_all((x+1) as usize, (y-1) as usize));
                        nearby.extend_from_slice(&game_state.creature_map.get_all((x+1) as usize, y as usize));
                        nearby.extend_from_slice(&game_state.creature_map.get_all((x+1) as usize, (y+1) as usize));

                        let mut nearby_coords = Vec::with_capacity(16);
                        for creature in nearby {
                            nearby_coords.push(creature.position.x);
                            nearby_coords.push(creature.position.y);
                            if nearby_coords.len() >= 16 {
                                break;
                            }
                        }

                        brain_inputs.extend_from_slice(&nearby_coords);
                    },
                    _ => {}
                }
            }

            let (_, output_type) = c.brain.compute(brain_inputs);

            match output_type {
                OutputTypes::MoveUp => {
                    if c.energy.current > 5.0 {
                        c.animation = Some(
                            AnimationTransition::new(
                                c.position.get(),
                                (c.position.x, c.position.y - consts::GRID_SIZE as f32),
                                elapsed, 0.3, CurveType::EaseQuadInOut
                            )
                        );
                        c.energy.expend(5.0);
                    }
                },
                OutputTypes::MoveDown => {
                    if c.energy.current > 5.0 {
                        c.animation = Some(
                            AnimationTransition::new(
                                c.position.get(),
                                (c.position.x, c.position.y + consts::GRID_SIZE as f32),
                                elapsed, 0.3, CurveType::EaseQuadInOut
                            )
                        );
                        c.energy.expend(5.0);
                    }
                },
                OutputTypes::MoveLeft => {
                    if c.energy.current > 5.0 {
                        c.animation = Some(
                            AnimationTransition::new(
                                c.position.get(),
                                (c.position.x - consts::GRID_SIZE as f32, c.position.y),
                                elapsed, 0.3, CurveType::EaseQuadInOut
                            )
                        );
                        c.energy.expend(5.0);
                    }
                },
                OutputTypes::MoveRight => {
                    if c.energy.current > 5.0 {
                        c.animation = Some(
                            AnimationTransition::new(
                                c.position.get(),
                                (c.position.x + consts::GRID_SIZE as f32, c.position.y),
                                elapsed, 0.3, CurveType::EaseQuadInOut
                            )
                        );
                        c.energy.expend(5.0);
                    }
                },
                OutputTypes::Nothing => {
                    c.energy.restore(2.0);
                },
                _ => {}
            }
        }

        // collide with wall
        if c.position.x <= 0.0 {
            c.position.x = 0.0;
        }
        if c.position.x >= consts::SCREEN_WIDTH as f32 - consts::GRID_SIZE as f32 {
            c.position.x = consts::SCREEN_WIDTH as f32;
        }
        if c.position.y <= 0.0 {
            c.position.y = 0.0;
        }
        if c.position.y >= consts::SCREEN_HEIGHT as f32 - consts::GRID_SIZE as f32 {
            c.position.y = consts::SCREEN_HEIGHT as f32;
        }
    }

}