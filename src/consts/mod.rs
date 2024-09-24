use macroquad::prelude::Vec2;

pub const SCREEN_WIDTH: u32 = 1920;
pub const SCREEN_HEIGHT: u32 = 1080;
pub const GRID_SIZE: usize = 500;
pub const TILE_SIZE: f32 = 32.0;

pub const WORLD_LAYERS: usize = 3;
pub const WORLD_FLOOR_LAYER: usize = 0;
pub const WORLD_WALL_LAYER: usize = 1;
pub const WORLD_WATER_LAYER: usize = 2;

/// Returns the grid position of a given x and y coordinate.
/// x and y are assumed to be in screen coordinates.
pub fn grid_pos(v: &Vec2) -> Vec2 {
    let gx = v.x.round();
    let gy = v.y.round();
    return Vec2::new(gx, gy);
}

/// Normalizes a grid position to a value between 0 and 1.
pub fn normalize_grid_pos(v: &Vec2) -> Vec2 {
    let gx = v.x / GRID_SIZE as f32;
    let gy = v.y / GRID_SIZE as f32;
    return Vec2::new(gx, gy);
}