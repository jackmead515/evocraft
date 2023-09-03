pub const SCREEN_WIDTH: u32 = 1920;
pub const SCREEN_HEIGHT: u32 = 1080;
pub const GRID_WIDTH: u32 = 96;
pub const GRID_HEIGHT: u32 = 54;
pub const GRID_SIZE: u32 = 20;

/// Returns the grid position of a given x and y coordinate.
/// x and y are assumed to be in screen coordinates.
pub fn grid_pos(wx: f32, wy: f32) -> (u32, u32) {
    assert!(wx >= 0.0 && wx <= SCREEN_WIDTH as f32 && wy >= 0.0 && wy <= SCREEN_HEIGHT as f32, "wx = {}, wy = {}", wx, wy);
    return (wx as u32 / GRID_SIZE, wy as u32 / GRID_SIZE);
}

pub fn grid_x(wx: f32) -> u32 {
    assert!(wx >= 0.0 && wx <= SCREEN_WIDTH as f32);
    return wx as u32 / GRID_SIZE;
}

pub fn grid_y(wy: f32) -> u32 {
    assert!(wy >= 0.0 && wy <= SCREEN_HEIGHT as f32);
    return wy as u32 / GRID_SIZE;
}

pub fn is_valid_grid_pos(gx: i32, gy: i32) -> bool {
    return gx < GRID_WIDTH as i32 && gy < GRID_HEIGHT as i32 && gx >= 0 && gy >= 0;
}

pub fn trim_to_grid(wx: f32, wy: f32) -> (u32, u32) {
    let mut x = wx; let mut y = wy;
    if x <= 0.0 { x = 0.0; }
    if y <= 0.0 { y = 0.0; }
    if x >= SCREEN_WIDTH as f32 { x = SCREEN_WIDTH as f32; }
    if y >= SCREEN_HEIGHT as f32 { y = SCREEN_HEIGHT as f32; }
    return grid_pos(x, y);
}

/// Returns the world position of a given x and y coordinate.
/// x and y are assumed to be in grid coordinates.
pub fn world_pos(gx: u32, gy: u32) -> (f32, f32) {
    assert!(gx <= GRID_WIDTH && gy <= GRID_HEIGHT, "gx = {}, gy = {}", gx, gy);
    let gz = GRID_SIZE as f32;
    return (gx as f32 * gz, gy as f32 * gz);
}

pub fn world_x(gx: u32) -> f32 {
    assert!(gx < GRID_WIDTH, "gx = {}", gx);
    return gx as f32 * GRID_SIZE as f32;
}

pub fn world_y(gy: u32) -> f32 {
    assert!(gy <= GRID_HEIGHT, "gy = {}", gy);
    return gy as f32 * GRID_SIZE as f32;
}

pub fn trim_to_world(wx: f32, wy: f32) -> (f32, f32) {
    let mut x = wx; let mut y = wy;
    if x <= 0.0 { x = 0.0; }
    if y <= 0.0 { y = 0.0; }
    if x >= SCREEN_WIDTH as f32 { x = SCREEN_WIDTH as f32; }
    if y >= SCREEN_HEIGHT as f32 { y = SCREEN_HEIGHT as f32; }
    return (x, y);
}