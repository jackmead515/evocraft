use macroquad::prelude::*;

use crate::consts;
use crate::models::*;
use crate::animation::*;
use crate::grid_map;

pub fn update_player(game_state: &mut GameState) {
    let player = &mut game_state.player;
    let elapsed = game_state.stats.elapsed;

    // update the player position if there is an animation
    match player.animation {
        Some(ref mut animation) => {
            player.position.sett(animation.interpolate(elapsed));
            if elapsed - animation.start_time > animation.duration as f64 {
                player.animation = None;
            }
        },
        None => {}
    }

    // every 1 second, restore 1 energy to player
    if elapsed % 1.0 < 0.01 {
        player.energy.restore(1.0);
    }
}

pub fn update_creatures(game_state: &mut GameState) {
    let elapsed = game_state.stats.elapsed;
    let creatures = &mut game_state.creatures;
    let entity_map = &game_state.entity_map;
    let player = &game_state.player;

    for index in 0..creatures.len() {
        let creature = &mut creatures[index];

        if let Ok(mut creature) = creature.lock() {

            // update creature position if there is an animation
            if creature.animation.is_some() {
                let a = creature.animation.as_ref().unwrap();
                let p = a.interpolate(elapsed);
                let animation_expired = elapsed - a.start_time > a.duration as f64;
                if animation_expired {
                    creature.animation = None;
                }

                creature.position.sett(p);
            }
            
            if creature.animation.is_none() {

                let mut brain_inputs = Vec::new();

                // gather only the indicies that the brain needs
                for input_type in &creature.brain.input_types {

                    match input_type {
                        InputTypes::CurrentPosition => {
                            brain_inputs.push(creature.position.x);
                            brain_inputs.push(creature.position.y);
                        },
                        InputTypes::PlayerPosition => {
                            brain_inputs.push(player.position.x);
                            brain_inputs.push(player.position.y);
                        },
                        InputTypes::CurrentHealth => {
                            brain_inputs.push(creature.health.current);
                        },
                        InputTypes::CurrentEnergy => {
                            brain_inputs.push(creature.energy.current);
                        },
                        InputTypes::PlayerEnergy => {
                            brain_inputs.push(player.energy.current);
                        },
                        InputTypes::PlayerHealth => {
                            brain_inputs.push(player.health.current);
                        },
                        InputTypes::NearCreatures => {
                            let (gx, gy) = consts::grid_pos(creature.position.x, creature.position.y);

                            let (x, y, w, h) = (gx-1, gy-1, 3, 3);

                            // reduce the size of rect if it is on the edge of the map
                            let (x, y, w, h) = if x < 0 {
                                (0, y, w+1, h)
                            } else if x+w >= consts::GRID_WIDTH as i32 {
                                (x, y, w-1, h)
                            } else {
                                (x, y, w, h)
                            };

                            println!("x: {}, y: {}, w: {}, h: {}", x, y, w, h);

                            let nearby = grid_map::get_rect_type(
                                entity_map,
                                x as usize,
                                y as usize,
                                w as usize,
                                h as usize,
                                EntityType::Creature
                            );
                            let mut nearby_coords: Vec<f32> = Vec::with_capacity(16);

                            for near in nearby {
                                if near.index != index {
                                    let g = consts::world_pos(near.gx as i32, near.gy as i32);
                                    nearby_coords.push(g.0 as f32);
                                    nearby_coords.push(g.1 as f32);
                                    if nearby_coords.len() >= 16 {
                                        break;
                                    }
                                }
                            }
                            
                            // while length is less than 16, fill with 0.0
                            while nearby_coords.len() < 16 {
                                nearby_coords.push(0.0);
                            }

                            brain_inputs.extend_from_slice(&nearby_coords);
                        },
                        _ => {}
                    }
                }

                let (_, output_type) = creature.brain.compute(brain_inputs);

                let mut xmove = 0.0;
                let mut ymove = 0.0;

                match output_type {
                    OutputTypes::MoveUp => {
                        ymove -= consts::GRID_SIZE as f32;
                    },
                    OutputTypes::MoveDown => {
                        ymove += consts::GRID_SIZE as f32;
                    },
                    OutputTypes::MoveLeft => {
                        xmove -= consts::GRID_SIZE as f32;
                    },
                    OutputTypes::MoveRight => {
                        xmove += consts::GRID_SIZE as f32;
                    },
                    OutputTypes::Nothing => {
                        creature.energy.restore(2.0);
                    },
                    _ => {}
                }

                // if creature has enough energy, move
                if creature.energy.current >= 5.0 {
                    creature.animation = Some(AnimationTransition::new(
                        creature.position.get(),
                        (creature.position.x + xmove, creature.position.y + ymove),
                        elapsed, 0.4, CurveType::EaseQuadInOut
                    ));
                    creature.energy.expend(5.0)
                }

                if elapsed % 1.0 < 0.01 {
                    creature.energy.restore(1.0);
                }

            }

            // collide creature with wall
            if creature.position.x <= 0.0 {
                creature.position.x = 0.0;
            }
            if creature.position.x >= consts::SCREEN_WIDTH as f32 {
                creature.position.x = consts::SCREEN_WIDTH as f32;
            }
            if creature.position.y <= 0.0 {
                creature.position.y = 0.0;
            }
            if creature.position.y >= consts::SCREEN_HEIGHT as f32 {
                creature.position.y = consts::SCREEN_HEIGHT as f32;
            }

        }
    }

}

fn update_creature_map(game_state: &mut GameState) { 
    // update the entity map with the new creature positions
    let entity_map = &mut game_state.entity_map;
    let creatures = &game_state.creatures;
}


pub fn update(game_state: &mut GameState) {
    
    update_player(game_state);

    update_creatures(game_state);

    update_creature_map(game_state);
}