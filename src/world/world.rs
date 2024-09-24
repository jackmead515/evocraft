use grid::Grid;
use macroquad::prelude::{Rect, Vec2};
use crate::consts::*;


pub fn iter_layer<'a, T>(grid: &'a Grid<Vec<Option<T>>>, layer: usize, search: Rect) -> impl Iterator<Item = &Option<T>> {
    let mut x = search.left() as usize;
    let mut y = search.top() as usize;

    return std::iter::from_fn(move || {
        if x >= search.right() as usize || x >= grid.rows() {
            x = search.left() as usize;
            y += 1;
        }

        if y >= search.bottom() as usize || y >= grid.cols() {
            return None;
        }

        let tile = &grid[x][y][layer];

        x += 1;

        return Some(tile);
    });
}

pub fn collide_with<'a, T>(grid: &'a Grid<Vec<Option<T>>>, layer: usize, grid_position: &Vec2) -> Option<&'a T> {
    let x = grid_position.x as usize;
    let y = grid_position.y as usize;

    if let Some(tile) = &grid[x][y][layer] {
        return Some(tile);
    }

    return None;
}