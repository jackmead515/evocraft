use grid::Grid;
use macroquad::prelude::*;

use crate::models::{World, TileSet};
use crate::consts::*;

impl World {

    pub fn new(rows: usize, cols: usize) -> World {
        let stack: Vec<Option<TileSet>> = vec![None; WORLD_LAYERS];

        let mut grid: Grid<Vec<Option<TileSet>>> = Grid::new(rows, cols);
        grid.fill(stack);

        return World {
            tile_grid: grid,
        };
    }

    pub fn iter_layer(&self, layer: usize, search: Rect) -> impl Iterator<Item = &Option<TileSet>> {
        let mut x = search.left() as usize;
        let mut y = search.top() as usize;

        return std::iter::from_fn(move || {
            if x >= search.right() as usize || x >= self.tile_grid.rows() {
                x = search.left() as usize;
                y += 1;
            }

            if y >= search.bottom() as usize || y >= self.tile_grid.cols() {
                return None;
            }

            let tile = &self.tile_grid[x][y][layer];

            x += 1;

            return Some(tile);
        });
    }

    pub fn collide_with(&self, grid_position: &Vec2, layer: usize) -> Option<&TileSet> {
        let x = grid_position.x as usize;
        let y = grid_position.y as usize;

        if let Some(tile) = &self.tile_grid[x][y][layer] {
            return Some(&tile);
        }

        return None;
    }
}