use grid::Grid;
use macroquad::prelude::*;
use macroquad::rand::rand;
use perlin2d::PerlinNoise2D;

use crate::models::*;
use crate::world::*;
use crate::creature::Creature;
use crate::textures;
use crate::consts::*;


pub fn update(game_state: &mut GameState) {

    // if game_state.creatures.len() <= 0 {
    //     let (new_creatures, new_entity_map) = gather_longest_survivers(&mut clones);
    //     let clones = clone_creatures(&new_creatures);
    //     game_state.creatures = new_creatures;
    //     game_state.entity_map = new_entity_map;
    // }

}

pub async fn generate()  -> GameState {
    let font = load_ttf_font("assets/unifont-15.0.06.ttf").await.expect("Failed to load font");

    let texture_map = textures::load().await;

    let tile_grid = generate_random_world();
    let creatures = generate_random_creatures();

    let world = World {
        tile_grid: tile_grid,
        creatures: creatures,
        player: Player {
            color: Color::new(0.0, 1.0, 0.0, 1.0),
            position: Vec2::new((GRID_SIZE / 2) as f32, (GRID_SIZE / 2) as f32),
            health: ZeroMaxStat::new(100.0, 100.0),
            energy: ZeroMaxStat::new(100.0, 100.0),
            vigor: ZeroMaxStat::new(95.0, 100.0),
            movement: None,
            delays: PlayerDelays {
                energy_restore: None
            }
        },
    };

    return GameState {
        demo: DemoType::RandomCreatures1,
        texture_map: texture_map,
        world: world,
        stats: GameStats {
            fps: 0,
            frame_time: 0.0,
            elapsed: 0.0,
            zoom_factor: 0.05,
        },
        font: font,
    };
}

fn generate_random_creatures() -> Vec<Creature> {
    let mut creatures = Vec::with_capacity(100);

    let mut gx = GRID_SIZE / 2;
    let mut gy = GRID_SIZE / 2;

    for _ in 0..100 {
        let p = Vec2::new(gx as f32, gy as f32);
        let c = Creature::new_random(p, 100.0, 100.0, get_time());
        creatures.push(c);
        gx += 1;
        if gx >= GRID_SIZE {
            gx = GRID_SIZE / 2;
            gy += 1;
        }
    }

    return creatures;
}