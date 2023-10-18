use crate::models::game_state::GameState;

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
    pub grid_x: u32,
    pub grid_y: u32,
    remove: bool,
}

impl EntityRef {

    pub fn new(entity_type: EntityType, index: usize, gx: u32, gy: u32) -> Self {
        EntityRef {
            entity_type: entity_type,
            index: index,
            grid_x: gx,
            grid_y: gy,
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