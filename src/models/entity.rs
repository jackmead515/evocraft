use crate::models::game_state::GameState;

pub trait Entity {
    
    fn update(&mut self, game_state: &mut GameState);

    fn draw(&self, game_state: &GameState);

}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EntityType {
    Player,
    Creature,
    Item,
    Terrain,
    Structure,
    Projectile,
    Effect,
    Other,
    None,
}

#[derive(Debug, Clone, Copy)]
pub struct EntityRef {
    pub entity_type: EntityType,
    pub index: usize,
    pub gx: usize,
    pub gy: usize,
}