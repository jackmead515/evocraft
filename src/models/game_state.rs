use macroquad::prelude::Font;

use crate::creature::Creature;
use crate::textures::TextureMap;
use crate::models::*;

pub struct GameStats {
    pub fps: i32,
    pub frame_time: f32,
    pub elapsed: f64,
    pub zoom_factor: f32,
}

pub struct GameState {
    pub demo: DemoType,
    pub texture_map: TextureMap,
    pub world: World,
    pub stats: GameStats,
    pub font: Font,
}