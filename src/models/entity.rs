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
    pub gx: u32,
    pub gy: u32,
    remove: bool,
}

impl EntityRef {

    pub fn new(entity_type: EntityType, index: usize, gx: u32, gy: u32) -> Self {
        EntityRef {
            entity_type: entity_type,
            index: index,
            gx: gx,
            gy: gy,
            remove: false,
        }
    }

    pub fn remove(&mut self) {
        self.remove = true;
    }

    pub fn is_removed(&self) -> bool {
        self.remove
    }
}