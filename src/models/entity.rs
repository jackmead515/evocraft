use crate::models::game_state::GameState;

pub trait Entity {
    
    fn update(&mut self, game_state: &mut GameState);

    fn draw(&self, game_state: &GameState);

}