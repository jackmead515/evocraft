use rayon::prelude::*;
use macroquad::prelude::*;
use perlin2d::PerlinNoise2D;
use::rand::random;

use crate::world::*;

/// Generates a circular gradient grid. The center of the grid will be closer to 0 and the
/// outside edges will be closer to 1.
fn generate_circular_gradient(range: usize, padding: f32, power: f32, offset: f32) -> Vec<f32> {
    let mut gradient = Vec::new();

    let range_perc = range as f32 * padding;

    let c = (range / 2) as f32;

    for x in 0..range {
        for y in 0..range {
            let x1 = x as f32;
            let y1 = y as f32;
            
            // d is distance from c,c to x1, x2
            let mut d = ((x1 - c).powf(2.0) + (y1 - c).powf(2.0)).sqrt();

            d += offset;

            // v is d / range at an exponential rate
            let v = (d / range_perc).powf(power);

            gradient.push(v);
        }
    }

    let min = gradient.iter().cloned().fold(f32::INFINITY, f32::min);
    let max = gradient.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let range = max - min;

    return gradient.iter().map(|v| (v - min) / range).collect();
}


/// Generate a random island using perlin noise. A circular gradient is applied to the
/// noise to create a more natural looking island. The result is a grid where each cell
/// is a float value between 0 and 1. The outside edges will be closer to 0 and the center
pub fn generate_noise(range: usize, seed: i16, octaves: i32, amplitude: f64) -> Vec<(Vec2, f64)> {
    let mut grid = (0..range)
        .into_par_iter()
        .map(|x| {
            let perlin = PerlinNoise2D::new(
                octaves, amplitude, 1.0, 1.00, 2.0, (100.0, 100.0), 1.0, seed.abs() as i32
            );

            let mut grid = Vec::with_capacity((range*2) as usize);

            for y in 0..range {
                let noise = perlin.get_noise(x as f64, y as f64);

                //let index = y * range + x;
    
                grid.push((Vec2::new(x as f32, y as f32), noise));
            }

            return grid;
        })
        .flat_map(|x| x)
        .collect::<Vec<(Vec2, f64)>>();

    // normalize the noise values
    let max = grid.iter().map(|(_, noise)| *noise).fold(f64::NEG_INFINITY, f64::max);
    let min = grid.iter().map(|(_, noise)| *noise).fold(f64::INFINITY, f64::min);
    let range = max - min;
    for (pos, noise) in grid.iter_mut() {
        *noise = (*noise - min) / range;
    }

    return grid;
}


pub fn generate_random_world() -> World {
    let water_threshold = 0.15;
    let grid_size = 500;

    let mut world = World::new(grid_size, grid_size);

    let island_seed = random::<i16>();
    let mountain_seed = random::<i16>();


    // generate the island grid;
    let mut island = generate_noise(grid_size, island_seed, 5, 1.0);
    let island_gradient = generate_circular_gradient(grid_size, 0.3, 1.5, 0.0);
    island.par_iter_mut().enumerate().for_each(|(index, (pos, noise))| {
        let g = island_gradient[index];
        *noise -= g as f64;
        *noise = noise.max(0.0);
    });
    std::mem::drop(island_gradient);


    // generate the mountain grid;
    // let mountain_size = 250;
    // let mut mountain = generate_noise(mountain_size, mountain_seed, 5, 1.0);
    // let mountain_gradient = generate_circular_gradient(mountain_size, 0.3, 1.5, 20.0);
    // mountain.par_iter_mut().enumerate().for_each(|(index, (pos, noise))| {
    //     let g = mountain_gradient[index];
    //     *noise -= g as f64;
    //     *noise = noise.max(0.0);
    //     pos.x += grid_size as f32 / 2.0 - (mountain_size / 2) as f32;
    //     pos.y += grid_size as f32 / 2.0 - (mountain_size / 2) as f32;
    // });
    // std::mem::drop(mountain_gradient);

    // initialize the island as grass and water tiles
    for (pos, noise) in island {
        let mut tile = "dungeon/floor/grass/grass0-dirt-mix_1";

        if noise < water_threshold {
            tile = "dungeon/water/deep_water";
            world.tile_grid[pos.x as usize][pos.y as usize][WORLD_WALL_LAYER] = Some(TileSet {
                position: pos,
                texture: tile.to_string(),
                elevation: 0.0,
            });
        }

        world.tile_grid[pos.x as usize][pos.y as usize][WORLD_FLOOR_LAYER] = Some(TileSet {
            position: pos,
            texture: tile.to_string(),
            elevation: 0.0,
        });
    }

    // initialize the mountain as dirt tiles
    // let mountain_tiles = [
    //     ("dungeon/floor/mud_0", 0.6),
    //     ("dungeon/floor/mud_1", 0.65),
    //     ("dungeon/floor/pebble_brown_0_new", 0.7),
    //     ("dungeon/floor/pebble_brown_1_new", 0.75),
    //     ("dungeon/floor/grey_dirt_0_new", 0.8),
    //     ("dungeon/floor/grey_dirt_1_new", 0.85),
    // ];

    // for (pos, noise) in mountain {
    //     for (tile, threshold) in mountain_tiles.iter() {
    //         if noise > *threshold {
    //             world.tile_grid[pos.x as usize][pos.y as usize][WORLD_FLOOR_LAYER] = Some(TileSet {
    //                 position: pos,
    //                 texture: tile.to_string(),
    //                 elevation: 0.0,
    //             });
    //             break;
    //         }
    //     }
    // }


    // loop through the grid and set beach tiles. If a tile is surrounded by water tiles set it to a beach tile
    for x in 0..grid_size {
        for y in 0..grid_size {
            if let Some(tile) = world.tile_grid[x][y][WORLD_FLOOR_LAYER].as_ref() {
                if tile.texture == "dungeon/floor/grass/grass0-dirt-mix_1" {
                    // check the surrounding tiles
                    let mut set_to_beach = false;
                    for dx in -1..2 {
                        for dy in -1..2 {
                            let nx = x as i32 + dx;
                            let ny = y as i32 + dy;

                            if nx < 0 || ny < 0 || nx >= grid_size as i32 || ny >= grid_size as i32 {
                                continue;
                            }

                            if let Some(adj_tile) = world.tile_grid[nx as usize][ny as usize][WORLD_FLOOR_LAYER].as_ref() {
                                if adj_tile.texture == "dungeon/water/deep_water" {

                                    // random number 1-8
                                    let r = random::<u8>() % 8 + 1;

                                    let tile_val = format!("dungeon/floor/sand_{}", r);

                                    world.tile_grid[x][y][WORLD_FLOOR_LAYER] = Some(TileSet {
                                        position: Vec2::new(x as f32, y as f32),
                                        texture: tile_val,
                                        elevation: 0.0,
                                    });
                                    set_to_beach = true;
                                    break;
                                }
                            }
                        }

                        if set_to_beach {
                            break;
                        }
                    }
                }
            }
        }
    }


    // randomly generate forest tiles
    // for x in 0..grid_size {
    //     for y in 0..grid_size {
    //         if let Some(tile) = world.tile_grid[x][y][WORLD_FLOOR_LAYER].as_ref() {
    //             if tile.texture == "dungeon/floor/grass/grass0-dirt-mix_1" {
    //                 // random number 0-2
    //                 let r = random::<u8>() % 3;

    //                 let tile_val = format!("dungeon/trees/tree_1_lightred");

    //                 if r == 1 {
    //                     world.tile_grid[x][y][WORLD_WALL_LAYER] = Some(TileSet {
    //                         position: Vec2::new(x as f32, y as f32),
    //                         texture: tile_val,
    //                     });
    //                 }
    //             }
    //         }
    //     }
    // }


    // randomize the grass tiles
    for x in 0..grid_size {
        for y in 0..grid_size {
            if let Some(tile) = world.tile_grid[x][y][WORLD_FLOOR_LAYER].as_ref() {
                if tile.texture == "dungeon/floor/grass/grass0-dirt-mix_1" {
                    // random number 0-2
                    let r = random::<u8>() % 3 + 1;

                    let tile_val = format!("dungeon/floor/grass/grass0-dirt-mix_{}", r);

                    world.tile_grid[x][y][WORLD_FLOOR_LAYER] = Some(TileSet {
                        position: Vec2::new(x as f32, y as f32),
                        texture: tile_val,
                        elevation: 0.0,
                    });
                }
            }
        }
    }


    return world;
}