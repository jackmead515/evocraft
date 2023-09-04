use crate::consts::*;
use crate::models::*;

pub fn update(game_state: &mut GameState) { 
    // update the entity map with the new creature positions
    let entity_map = &mut game_state.entity_map;
    let creatures = &mut game_state.creatures;

    let mut some_dead = false;

    // update the entity map with the new creatures grid positions
    for grid_cell in entity_map.iter_mut() {
        for entity_ref in grid_cell.iter_mut() {
            let creature = &creatures[entity_ref.index];
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

    // minor optimization: if there are no dead creatures, don't bother
    if some_dead {
        // empty the entity map
        for grid_cell in entity_map.iter_mut() {
            grid_cell.clear();
        }

        // remove dead creatures from the creatures vector
        creatures.retain(|creature| creature.alive);

        // update the entity map indicies from the new creatures vector
        for (index, creature) in creatures.iter().enumerate() {
            let (gx, gy) = grid_pos(creature.position.x, creature.position.y);
            let entity_ref = EntityRef::new(EntityType::Creature, index, gx, gy);
            entity_map[gx as usize][gy as usize].push(entity_ref);
        }
    }
}