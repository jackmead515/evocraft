use rayon::prelude::*;
use macroquad::prelude::*;
use perlin2d::PerlinNoise2D;
use::rand::random;

use crate::world::*;

fn generate_circular_gradient(range: usize, padding: f32, power: f32) -> Vec<f32> {
    let mut gradient = Vec::new();

    let range_perc = range as f32 * padding;

    let c = (range / 2) as f32;

    for x in 0..range {
        for y in 0..range {
            let x1 = x as f32;
            let y1 = y as f32;
            
            // d is distance from c,c to x1, x2
            let d = ((x1 - c).powf(2.0) + (y1 - c).powf(2.0)).sqrt();

            // v is d / range at an exponential rate
            let v = (d / range_perc).powf(power);

            gradient.push(v);
        }
    }

    return gradient;
}


pub fn generate_island(range: usize, seed: i16) -> Vec<(Vec2, f64)> {
    let padding = 0.3;
    let power = 1.5;

    let gradient = generate_circular_gradient(range, padding, power);

    let grid = (0..range)
        .into_par_iter()
        .map(|x| {
            let perlin = PerlinNoise2D::new(
                5, 1.0, 1.0, 1.00, 2.0, (100.0, 100.0), 1.0, seed.abs() as i32
            );

            let mut grid = Vec::with_capacity((range*2) as usize);

            for y in 0..range {
                let mut noise = perlin.get_noise(x as f64, y as f64);
    
                // normalize to 0 to 1
                noise = (noise + 1.0) / 2.0;

                let index = y * range + x;
    
                // get gradient at index using short hand index
                let g = gradient[index];
                noise -= g as f64;
    
                grid.push((Vec2::new(x as f32, y as f32), noise));
                //grid.push((Vec2::new(x as f32, y as f32), Color::new(noise as f32, noise as f32, noise as f32, 1.0)));
                //grid.push((Vec2::new(x as f32, y as f32), noise, Color::new(g as f32, g as f32, g as f32, 1.0)));
            }

            return grid;
        })
        .flat_map(|x| x)
        .collect::<Vec<(Vec2, f64)>>();

    return grid;
}


pub fn generate_random_world() -> World {
    let grid = 500;
    let depth = 3;
    let mut world = World::new(grid, grid, depth);

    let seed = random::<i16>();

    let island = generate_island(grid, seed);

    for (pos, noise) in island {
        let mut tile = "dungeon/floor/grass/grass0-dirt-mix_1";
    
        if noise < 0.1 {
            tile = "dungeon/water/deep_water";
        }

        world.tile_grid[pos.x as usize][pos.y as usize][WORLD_FLOOR_LAYER] = Some(TileSet {
            position: pos,
            texture: tile.to_string(),
        });
    }

    return world;
}