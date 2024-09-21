use macroquad::prelude::*;
use macroquad::rand::gen_range;

use crate::consts;
use crate::creature::*;
use crate::models::*;
use crate::util::animation::{AnimationMovement, CurveType};
use crate::util::delay::Delay;

pub fn update_movement_animation(creature: &mut Creature, elapsed: f64) {
    match creature.movement {
        Some(ref mut animation) => {
            creature.position = animation.interpolate(elapsed);
            if animation.is_complete(elapsed) {
                creature.position = consts::grid_pos(&animation.final_pos);
                creature.movement = None;
            }
        },
        None => {}
    }
}


pub fn update_creature_behavior(creature: &mut Creature, player: &Player, elapsed: f64) {
    creature.brain.last_decision_time = elapsed;

    let mut brain_inputs = Vec::new();

    // gather only the indicies that the brain needs
    for input_type in &creature.brain.input_types {

        match input_type {
            InputTypes::CurrentAge => {
                let mut age = elapsed - creature.birth_time;
                age = (age / 120.0) * 2.0 - 1.0;
                brain_inputs.push(age as f32);
            },
            InputTypes::LastDecisions => {
                let total_decisions = creature.brain.last_decisions.len();
                let max_decisions = creature.brain.output_types.len();
                for i in 0..InputTypes::total_inputs(&InputTypes::LastDecisions) {
                    if i < total_decisions {
                        let mut d = creature.brain.last_decisions[i] as u8 as f32;
                        d = (d / max_decisions as f32) * 2.0 - 1.0;
                        brain_inputs.push(d);
                    } else {
                        brain_inputs.push(0.0);
                    }
                }
            }
            InputTypes::RandomInput => {
                brain_inputs.push(gen_range(-1.0, 1.0));
            },
            InputTypes::TimeSinoidInput => {
                brain_inputs.push((elapsed / 10.0).sin() as f32);
            },
            InputTypes::CurrentPosition => {
                let gpos = consts::grid_pos(&creature.position);
                let v = consts::normalize_grid_pos(&gpos);
                brain_inputs.push(v.x);
                brain_inputs.push(v.y);
            },
            InputTypes::PlayerPosition => {
                let gpos = consts::grid_pos(&player.position);
                let v = consts::normalize_grid_pos(&gpos);
                brain_inputs.push(v.x);
                brain_inputs.push(v.y);
            },
            InputTypes::CurrentHealth => {
                brain_inputs.push(creature.health.percent());
            },
            InputTypes::CurrentEnergy => {
                brain_inputs.push(creature.energy.percent());
            },
            InputTypes::PlayerEnergy => {
                brain_inputs.push(player.energy.percent());
            },
            InputTypes::PlayerHealth => {
                brain_inputs.push(player.health.percent());
            },
            // InputTypes::NearCreatures => {
            //     // TODO: implement this
            // },
            _ => {}
        }

    }

    let (_, output_type) = creature.brain.compute(brain_inputs);

    match output_type {
        OutputTypes::BehaviorContinue => {
            // continue current behavior!
            // meaning, don't change anything
        },
        _ => {
            creature.current_behavior = Some(output_type);
        }
    }
}


pub fn update_creature_current_behavior(creature: &mut Creature, player: &Player, elapsed: f64) {
    if let Some(behavior) = creature.current_behavior {
        match behavior {
            OutputTypes::BehaviorWander => {
                if creature.movement.is_some() {
                    return;
                }

                let mut xmove = 0.0;
                let mut ymove = 0.0;
    
                let rand = gen_range(0.0, 1.0);
                if rand < 0.25 {
                    xmove = 1.0;
                } else if rand < 0.5 {
                    xmove = -1.0;
                } else if rand < 0.75 {
                    ymove = 1.0;
                } else {
                    ymove = -1.0;
                }
    
                if xmove != 0.0 || ymove != 0.0 {
                    if creature.energy.value >= 3.0 {
                        creature.movement = Some(AnimationMovement::new(
                            creature.position,
                            creature.position + Vec2::new(xmove, ymove),
                            elapsed, 0.4, CurveType::Linear
                        ));
                        creature.total_travel_distance += 1;
                        creature.energy.consume(3.0);
                    }
                }
    
            },
            OutputTypes::BehaviorRest => {
                if let Some(ref mut delay) = creature.delays.behavior_rest {
                    if delay.is_complete(elapsed) {
                        creature.delays.behavior_rest = None;
                        creature.energy.restore(5.0);
                    }
                } else {
                    creature.delays.behavior_rest = Some(Delay::new(elapsed, 3.0));
                }
            },
            _ => {}
        }
    }
}


pub fn update(game_state: &mut GameState) {
    let elapsed = game_state.stats.elapsed;
    let creatures = &mut game_state.creatures;
    let player = &game_state.player;


    for index in 0..creatures.len() {
        let creature = &mut creatures[index];


        // update the creature animation if it exists
        if creature.movement.is_some() {
            update_movement_animation(creature, elapsed);

        // update to creatures behavior
        } else if creature.movement.is_none() && creature.brain.can_decide(elapsed) {
            update_creature_behavior(creature, player, elapsed);
        }

        // update the creatures current behavior
        update_creature_current_behavior(creature, player, elapsed);       

        // restore creature energy every 1 second
        // if creature energy is less than 10, damage creature
        if elapsed % 1.0 < 0.01 {
            creature.health.consume(0.5);

            if creature.energy.value < 10.0 {
                creature.health.consume(5.0);
            }

            // if creature is near the border of the grid, damage creature
            // don't want creatures that just push themselves against the border
            // to live and reproduce
            // TODO: implement this

            // let (gx, gy) = grid_pos(creature.position.x, creature.position.y);
            // let tolerance = 2;
            // if gx < tolerance || gx > GRID_WIDTH - tolerance
            //     || gy < tolerance || gy > GRID_HEIGHT - tolerance {
            //     creature.health.consume(5.0);
            // }
        }

        // if creature health is less than 0, creature should die
        if creature.health.value <= 0.0 {
            creature.alive = false;
        }
        
        // if creature is alive for more than 5 minutes, it should die
        // if elapsed - creature.birth_time >= 10.0 {
        //     creature.alive = false;
        // }

        // collide with world boundaries
        // let world = &game_state.world;
        // if let Some(_) = world.collide(&creature.position) {
        //     if let Some(ref mut animation) = creature.movement {
        //         creature.position = animation.initial_pos;
        //         creature.movement = None;
        //     }
        // }

        // creature should reproduce if:
        // energy is greater than 50%
        // health is greater than 50%
        // there are less than 5 creatures in the grid cell
        // creature has survived for at least 1 minutes
        // creature has traveled at least 10 grid cells
        // creature has no reproduced for at least 1 minute

        // TODO: implement this

        // if creature.energy.percent() > 0.5
        //     && creature.health.percent() > 0.5
        //     && creature.total_travel_distance > 10
        //     && elapsed - creature.birth_time > 60.0
        //     && elapsed - creature.reproduce_time > 60.0
        //     && creature.alive {
            
        //     let (gx, gy) = grid_pos(creature.position.x, creature.position.y);
        //     let entity_refs = grid_map::get_all_type(
        //         entity_map, gx, gy, EntityType::Creature
        //     );

        //     if entity_refs.len() < 3 {
        //         creature.reproduce_time = elapsed;

        //         let mut child = creature.clone();
        //         child.energy.restore(child.energy.max);
        //         child.health.restore(child.health.max);
        //         child.total_travel_distance = 0;
        //         child.birth_time = elapsed;
        //         child.reproduce_time = 0.0;
        //         child.random_offset.0 = gen_range(0.0, 5.0);
        //         child.random_offset.1 = gen_range(0.0, 5.0);
        //         child.generation += 1;
        //         child.mutate();
        //         childen.push(child);
        //     }

        // }
    }


    // for child in childen {
    //     let (gx, gy) = grid_pos(child.position.x, child.position.y);
    //     let entity_ref = EntityRef::new(EntityType::Creature, creatures.len(), gx, gy);
    //     game_state.entity_map[gx as usize][gy as usize].push(entity_ref);
    //     creatures.push(child);
    // }

}