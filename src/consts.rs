pub const SCREEN_WIDTH: i32 = 1920;
pub const SCREEN_HEIGHT: i32 = 1080;
pub const GRID_SIZE: i32 = 20;

/// Returns the grid position of a given x and y coordinate.
/// x and y are assumed to be in screen coordinates.
pub fn grid_pos(x: f32, y: f32) -> (i32, i32) {
    return (x as i32 / GRID_SIZE, y as i32 / GRID_SIZE);
}

/// Returns the world position of a given x and y coordinate.
/// x and y are assumed to be in grid coordinates.
pub fn world_pos(x: i32, y: i32) -> (f32, f32) {
    return (x as f32 * GRID_SIZE as f32, y as f32 * GRID_SIZE as f32);
}