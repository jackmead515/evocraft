use crate::consts::*;
use crate::models::*;

pub fn update(game_state: &mut GameState) { 
    // update the entity map with the new creature positions
    let creatures = &mut game_state.creatures;

    let some_dead = creatures.iter().any(|creature| !creature.alive);


    // minor optimization: if there are no dead creatures, don't bother
    if some_dead {
        // remove dead creatures from the creatures vector
        creatures.retain(|creature| creature.alive);
    }
}