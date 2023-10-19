use grid::Grid;

use macroquad::prelude::*;

pub const WORLD_FLOOR_LAYER: usize = 0;
pub const WORLD_WALL_LAYER: usize = 1;
pub const WORLD_WATER_LAYER: usize = 2;

#[derive(Debug, Clone)]
pub struct TileSet {
    pub position: Vec2,
    pub texture: String,
}

pub struct World {
    pub tile_grid: Grid<Vec<Option<TileSet>>>,
}

impl World {

    pub fn new(rows: usize, cols: usize, depth: usize) -> World {
        let stack: Vec<Option<TileSet>> = vec![None; depth];

        let mut grid: Grid<Vec<Option<TileSet>>> = Grid::new(rows, cols);
        grid.fill(stack);

        return World {
            tile_grid: grid,
        };
    }

    pub fn iter_floor(&self, search: Rect) -> impl Iterator<Item = &Option<TileSet>> {
        let mut x = search.left() as usize;
        let mut y = search.top() as usize;

        return std::iter::from_fn(move || {
            if x >= search.right() as usize {
                x = search.left() as usize;
                y += 1;
            }

            if y >= search.bottom() as usize {
                return None;
            }

            let tile = &self.tile_grid[x][y][WORLD_FLOOR_LAYER];

            x += 1;

            return Some(tile);
        });
    }

    pub fn collide_with(&self, position: &Vec2, search: &Rect, layer: usize) -> Option<&TileSet> {
        let pos_box = Rect::new(position.x, position.y, 1.0, 1.0);

        for x in search.left() as usize..search.right() as usize {
            for y in search.top() as usize..search.bottom() as usize {
                if let Some(tile) = &self.tile_grid[x][y][layer] {
                    let bod_box = Rect::new(tile.position.x, tile.position.y, 1.0, 1.0);

                    if pos_box.left() < bod_box.right()
                    && pos_box.right() > bod_box.left()
                    && pos_box.top() < bod_box.bottom()
                    && pos_box.bottom() > bod_box.top() {
                        return Some(&tile);
                    }
                }
            }
        }

        return None;
    }

}