use std::rc::Rc;
use macroquad::prelude::Font;

use crate::models::Player;
use crate::models::creature::Creature;
use super::ObjectMap;

pub struct GameStats {
    pub fps: i32,
    pub frame_time: f32,
    pub elapsed: f64,
}

pub struct GameState {
    pub stats: GameStats,
    pub font: Font,
    pub player: Player,
    pub creatures: Vec<Rc<Creature>>,
    pub creature_map: ObjectMap<Rc<Creature>>,
}