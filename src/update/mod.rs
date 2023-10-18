use crate::models::GameState;

pub mod player;
pub mod creatures;
//pub mod world;

pub fn update(game_state: &mut GameState) {
    player::update(game_state);
    creatures::update(game_state);
    //world::update(game_state);
}