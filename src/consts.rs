pub const SCREEN_WIDTH: u32 = 1920;
pub const SCREEN_HEIGHT: u32 = 1080;
pub const GRID_WIDTH: u32 = 96;
pub const GRID_HEIGHT: u32 = 54;
pub const GRID_SIZE: u32 = 20;

pub fn assert_valid_wx(wx: f32) {
    assert!(wx >= 0.0 && wx < SCREEN_WIDTH as f32, "wx = {}", wx);
}

pub fn assert_valid_wy(wy: f32) {
    assert!(wy >= 0.0 && wy < SCREEN_HEIGHT as f32, "wy = {}", wy);
}

pub fn assert_valid_gx(gx: u32) {
    assert!(gx <= GRID_WIDTH, "gx = {}", gx);
}

pub fn assert_valid_gy(gy: u32) {
    assert!(gy <= GRID_HEIGHT, "gy = {}", gy);
}

/// Tests if the given grid position is valid.
pub fn valid_grid_pos(gx: i32, gy: i32) -> bool {
    return gx <= GRID_WIDTH as i32 && gy <= GRID_HEIGHT as i32 && gx >= 0 && gy >= 0;
}

/// Tests if the given world position is valid.
pub fn valid_world_pos(wx: f32, wy: f32) -> bool {
    return wx >= 0.0 && wy >= 0.0 && wx < SCREEN_WIDTH as f32 && wy < SCREEN_HEIGHT as f32;
}


/// Returns the grid position of a given x and y coordinate.
/// x and y are assumed to be in screen coordinates.
pub fn grid_pos(wx: f32, wy: f32) -> (u32, u32) {
    return (grid_x(wx), grid_y(wy));
}

pub fn grid_x(wx: f32) -> u32 {
    assert_valid_wx(wx);

    return wx.round() as u32 / GRID_SIZE;
}

pub fn grid_y(wy: f32) -> u32 {
    assert_valid_wy(wy);

    return wy.round() as u32 / GRID_SIZE;
}

/// Returns the world position of a given x and y coordinate.
/// x and y are assumed to be in grid coordinates.
pub fn world_pos(gx: u32, gy: u32) -> (f32, f32) {
    return (world_x(gx), world_y(gy));
}

pub fn world_x(gx: u32) -> f32 {
    assert_valid_gx(gx);

    return gx as f32 * GRID_SIZE as f32;
}

pub fn world_y(gy: u32) -> f32 {
    assert_valid_gy(gy);

    return gy as f32 * GRID_SIZE as f32;
}

/// Returns a world position from a world position after
/// trimming to the screen.
pub fn trim_to_world(wx: f32, wy: f32) -> (f32, f32) {
    let mut x = wx; let mut y = wy;
    if x <= 0.0 { x = 0.0; }
    if y <= 0.0 { y = 0.0; }
    if x >= SCREEN_WIDTH as f32 { x = (SCREEN_WIDTH-1) as f32; }
    if y >= SCREEN_HEIGHT as f32 { y = (SCREEN_HEIGHT-1) as f32; }
    return (x, y);
}

/// Returns a grid position from a world position after
/// trimming the world position to the screen.
pub fn trim_to_grid(wx: f32, wy: f32) -> (u32, u32) {
    let (x, y) = trim_to_world(wx, wy);
    return grid_pos(x, y);
}

