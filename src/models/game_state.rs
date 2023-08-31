use macroquad::prelude::Font;

use crate::models::Player;
use crate::creature::Creature;

pub struct GameStats {
    pub fps: i32,
    pub frame_time: f32,
    pub elapsed: f64,
}

pub struct GameState {
    pub stats: GameStats,
    pub font: Font,
    pub player: Player,
    pub creatures: Vec<Creature>
}