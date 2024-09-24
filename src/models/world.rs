use macroquad::math::Vec2;
use grid::Grid;

use crate::models::Player;
use crate::creature::Creature;


#[derive(Debug, Clone)]
pub struct TileSet {
    pub position: Vec2,
    pub texture: String,
    pub elevation: f32,
}

pub struct World {
    pub tile_grid: Grid<Vec<Option<TileSet>>>,
    pub player: Player,
    pub creatures: Vec<Creature>,
    //pub item_grid: Grid<Option<Item>>,
}