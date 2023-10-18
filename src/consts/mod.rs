use macroquad::prelude::Vec2;

pub const SCREEN_WIDTH: u32 = 1920;
pub const SCREEN_HEIGHT: u32 = 1080;
pub const GRID_WIDTH: u32 = 100;
pub const GRID_HEIGHT: u32 = 100;
pub const TILE_SIZE: f32 = 32.0;

/// Returns the grid position of a given x and y coordinate.
/// x and y are assumed to be in screen coordinates.
pub fn grid_pos(v: &Vec2) -> Vec2 {
    let gx = v.x.round();
    let gy = v.y.round();
    return Vec2::new(gx, gy);
}

pub fn normalize(v: &Vec2) -> Vec2 {
    let gx = v.x / GRID_WIDTH as f32;
    let gy = v.y / GRID_HEIGHT as f32;
    return Vec2::new(gx, gy);
}