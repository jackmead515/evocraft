use std::sync::{Arc, Mutex};
use macroquad::prelude::Font;
use grid::Grid;

use crate::models::*;

pub struct GameStats {
    pub fps: i32,
    pub frame_time: f32,
    pub elapsed: f64,
}

pub struct GameState {
    pub stats: GameStats,
    pub font: Font,
    pub player: Player,
    pub creatures: Vec<Creature>,
    pub entity_map: Grid<Vec<EntityRef>>,
}