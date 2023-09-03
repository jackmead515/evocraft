use std::sync::Mutex;

use macroquad::prelude::*;
use grid::Grid;

use crate::consts::*;
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
        player.health.heal(1.0);

        if player.energy.current < 10.0 {
            player.health.damage(5.0);
        }
    }
}

pub fn update_creatures(game_state: &mut GameState) {
    let elapsed = game_state.stats.elapsed;
    let creatures = &mut game_state.creatures;
    let entity_map = &game_state.entity_map;
    let player = &game_state.player;

    let mut childen: Vec<Creature> = Vec::new();

    for index in 0..creatures.len() {
        let creature = &mut creatures[index];

        if let Ok(mut creature) = creature.lock() {

            // update creature position if there is an animation
            if creature.animation.is_some() {
                let a = creature.animation.as_ref().unwrap();
                let p = a.interpolate(elapsed);
                if elapsed - a.start_time > a.duration as f64 {
                    creature.animation = None;
                }

                creature.position.sett(p);

            // if animation is none, that means the creature is not moving
            // so we can update the creature's brain
            } else if creature.animation.is_none() {

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

                            // get the search area
                            let (gx, gy) = grid_pos(creature.position.x, creature.position.y);
                            // cast to i32 to avoid underflow
                            let gx = gx as i32;
                            let gy = gy as i32;

                            // search area is a 3x3 grid around the creature
                            let search_coords: Vec<(i32, i32)> = vec![
                                (gx-1, gy-1),
                                (gx, gy-1),
                                (gx+1, gy-1),
                                (gx-1, gy),
                                (gx+1, gy),
                                (gx-1, gy+1),
                                (gx, gy+1),
                                (gx+1, gy+1),
                            ];

                            // filter out invalid grid positions due to edge cases
                            // if on the edge of the grid.
                            let search_coords: Vec<(i32, i32)> = search_coords.iter().filter_map(|g| {
                                return if is_valid_grid_pos(g.0, g.1) { Some(*g) } else  { None }
                            }).collect();
                            
                            // gather the nearby creatures
                            let mut nearby = Vec::new();
                            for coords in search_coords {
                                let entity_refs = grid_map::get_all_type(
                                    entity_map, coords.0 as u32, coords.1 as u32, EntityType::Creature
                                );
                                nearby.extend_from_slice(&entity_refs);
                            };
                            
                            // get the grid coordinates of the nearby creatures
                            // TODO: randomly select 16 nearby creatures. If there
                            // are a lot of nearby creatures, there will be a bias
                            // to select the ones that are first in the list
                            let mut nearby_coords: Vec<f32> = Vec::with_capacity(16);
                            for near in nearby {
                                if near.index != index {
                                    let g = world_pos(near.gx, near.gy);
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
                    OutputTypes::MoveUp => ymove -= GRID_SIZE as f32,
                    OutputTypes::MoveDown => ymove += GRID_SIZE as f32,
                    OutputTypes::MoveLeft => xmove -= GRID_SIZE as f32,
                    OutputTypes::MoveRight => xmove += GRID_SIZE as f32,
                    OutputTypes::Nothing => creature.energy.restore(5.0),
                    _ => {}
                }
                
                // if creature has enough energy and there are less than 5 creatures 
                // in the grid cell move the creature to the new position
                // TODO: There may be a chance since the entity_map isn't updated
                // that more than 5 creatures will be in the grid cell at a time.
                if xmove != 0.0 || ymove != 0.0 {

                    let (nx, ny) = (creature.position.x + xmove, creature.position.y + ymove);
                    let (ogx, ogy) = grid_pos(creature.position.x, creature.position.y);
                    let (ngx, ngy) = trim_to_grid(nx, ny);

                    // check if the creature is moving to a new grid cell
                    if ngy != ogy || ngx != ogx {

                        // check if there are already 5 creatures in the grid cell
                        let next_entity_refs = grid_map::get_all_type(
                            entity_map, ngx, ngy, EntityType::Creature
                        );

                        if next_entity_refs.len() < 3 && creature.energy.current >= 3.0 {
                            creature.animation = Some(AnimationTransition::new(
                                creature.position.get(), world_pos(ngx, ngy),
                                elapsed, 0.4, CurveType::EaseQuadInOut
                            ));
                            creature.total_travel_distance += 1;
                            creature.energy.expend(3.0);
                        }
                    }
                }
            }

            // restore creature energy every 1 second
            // if creature energy is less than 10, damage creature
            if elapsed % 1.0 < 0.01 {
                creature.energy.restore(1.0);
                creature.health.heal(1.0);

                if creature.energy.current < 10.0 {
                    creature.health.damage(5.0);
                }

                // if creature is near the border of the grid, damage creature
                let (gx, gy) = grid_pos(creature.position.x, creature.position.y);
                let tolerance = 2;
                if gx < tolerance || gx > GRID_WIDTH - tolerance
                    || gy < tolerance || gy > GRID_HEIGHT - tolerance {
                    creature.health.damage(5.0);
                }
            }

            let (wx, wy) = creature.position.get();
            let (wx, wy) = trim_to_world(wx, wy);
            creature.position.set(wx, wy);

            if creature.health.current <= 0.0 {
                creature.alive = false;
            }
            
            // if creature is alive for more than 5 minutes, it should die
            if elapsed - creature.birth_time > 300.0 {
                creature.alive = false;
            }

            // creature should reproduce if:
            // energy is greater than 50%
            // health is greater than 50%
            // there are less than 5 creatures in the grid cell
            // creature has survived for at least 1 minutes
            // creature has traveled at least 10 grid cells
            // creature has no reproduced for at least 1 minute

            if creature.energy.percent() > 0.5 
                && creature.health.percent() > 0.5
                && creature.total_travel_distance > 10
                && elapsed - creature.birth_time > 60.0
                && elapsed - creature.reproduce_time > 60.0
                && creature.alive {
                
                let (gx, gy) = grid_pos(creature.position.x, creature.position.y);
                let entity_refs = grid_map::get_all_type(
                    entity_map, gx, gy, EntityType::Creature
                );

                if entity_refs.len() < 3 {
                    creature.reproduce_time = elapsed;

                    let mut child = creature.clone();
                    child.energy.restore(child.energy.max);
                    child.health.heal(child.health.max);
                    child.total_travel_distance = 0;
                    child.birth_time = elapsed;
                    child.reproduce_time = 0.0;
                    childen.push(child);
                }

            }
        }
    }


    for child in childen {
        let (gx, gy) = grid_pos(child.position.x, child.position.y);
        let entity_ref = EntityRef::new(EntityType::Creature, creatures.len(), gx, gy);
        game_state.entity_map[gx as usize][gy as usize].push(entity_ref);
        creatures.push(Mutex::new(child));
    }

}

fn update_creature_map(game_state: &mut GameState) { 
    // update the entity map with the new creature positions
    let entity_map = &mut game_state.entity_map;
    let creatures = &mut game_state.creatures;

    let mut some_dead = false;

    // update the entity map with the new creatures grid positions
    for grid_cell in entity_map.iter_mut() {
        for entity_ref in grid_cell.iter_mut() {
            let creature = &creatures[entity_ref.index];
            if let Ok(creature) = creature.lock() {
                if creature.alive == false {
                    some_dead = true;
                    entity_ref.remove();
                } else {
                    let (gx, gy) = grid_pos(creature.position.x, creature.position.y);
                    entity_ref.gx = gx;
                    entity_ref.gy = gy;
                }
            }
        }
    }

    // minor optimization: if there are no dead creatures, don't bother
    if some_dead {
        // empty the entity map
        for grid_cell in entity_map.iter_mut() {
            grid_cell.clear();
        }

        // remove dead creatures from the creatures vector
        creatures.retain(|creature| {
            if let Ok(creature) = creature.lock() {
                creature.alive
            } else {
                false
            }
        });

        // update the entity map indicies from the new creatures vector
        for (index, creature) in creatures.iter().enumerate() {
            if let Ok(creature) = creature.lock() {
                let (gx, gy) = grid_pos(creature.position.x, creature.position.y);
                let entity_ref = EntityRef::new(EntityType::Creature, index, gx, gy);
                entity_map[gx as usize][gy as usize].push(entity_ref);
            }
        }
    }
}


pub fn update(game_state: &mut GameState) {
    update_player(game_state);
    update_creatures(game_state);
    update_creature_map(game_state);
}