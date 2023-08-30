use macroquad::prelude::*;

use crate::consts;

fn draw_debug_grid(width: i32, height: i32, grid_size: i32) {
    let grid_color = Color::new(1.0, 1.0, 1.0, 0.1);

    for i in 0..(width / grid_size) {
        let t = (i * grid_size) as f32;
        draw_line(t, 0.0, t, height as f32, 1.0, grid_color);
    }

    for i in 0..(height / grid_size) {
        let t = (i * grid_size) as f32;
        draw_line(0.0, t, width as f32, t, 1.0, grid_color);
    }
}

pub fn draw() {
    clear_background(BLACK);
    draw_debug_grid(consts::SCREEN_WIDTH, consts::SCREEN_HEIGHT, consts::GRID_SIZE);
}

