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

    let perlin = PerlinNoise2D::new(
        6, 1.0, 0.5, 1.0, 2.0, (100.0, 100.0), 1.0, 42
    );

    let (creatures, entity_map) = generate_random_creatures();
    //let mut clones = clone_creatures(&creatures);

    let texture_map = textures::load().await;

    let wall_options = vec![
        "dungeon/wall/brick_gray_0",
        "dungeon/wall/brick_gray_1",
        "dungeon/wall/brick_gray_2",
        "dungeon/wall/brick_gray_3",
    ];

    let floor_options = vec![
        "dungeon/floor/grass/grass0-dirt-mix_1",
        "dungeon/floor/grass/grass0-dirt-mix_2",
        "dungeon/floor/grass/grass0-dirt-mix_3",
    ];

    let get_random_int = |min: i32, max: i32| -> i32 {
        return rand::gen_range(min, max);
    };

    let mut floors = Vec::new();

    for x in -50..50 {
        for y in -50..50 {
            let noise = perlin.get_noise(x as f64, y as f64);
            let mut texture = floor_options[get_random_int(0, floor_options.len() as i32 - 1) as usize].to_string();

            if noise < -0.5 {
                texture = "dungeon/floor/dirt_0_new".to_string();
            } else if noise < 0.0 {
                texture = "dungeon/floor/floor_sand_stone_0".to_string();
            } else if noise < 0.5 {
                texture = "dungeon/floor/limestone_0".to_string();
            } else if noise < 1.0 {
                texture = "dungeon/floor/mud_0".to_string();
            }

            floors.push(Floor { 
                position: Vec2::new(x as f32, y as f32),
                texture: texture
            });
        }
    }

    let get_random_int = |min: i32, max: i32| -> i32 {
        return rand::gen_range(min, max);
    };

    let world = World {
        floor: floors,
        boundaries: vec![
            Boundary { position: Vec2::new(10.0, 0.0), texture: wall_options[get_random_int(0, wall_options.len() as i32 - 1) as usize].to_string() },
            Boundary { position: Vec2::new(10.0, 1.0), texture: wall_options[get_random_int(0, wall_options.len() as i32 - 1) as usize].to_string() },
            Boundary { position: Vec2::new(10.0, 2.0), texture: wall_options[get_random_int(0, wall_options.len() as i32 - 1) as usize].to_string() },
            Boundary { position: Vec2::new(10.0, 3.0), texture: wall_options[get_random_int(0, wall_options.len() as i32 - 1) as usize].to_string() },
            Boundary { position: Vec2::new(10.0, 4.0), texture: wall_options[get_random_int(0, wall_options.len() as i32 - 1) as usize].to_string() },
            Boundary { position: Vec2::new(10.0, 5.0), texture: wall_options[get_random_int(0, wall_options.len() as i32 - 1) as usize].to_string() },
            Boundary { position: Vec2::new(11.0, 5.0), texture: wall_options[get_random_int(0, wall_options.len() as i32 - 1) as usize].to_string() },
            Boundary { position: Vec2::new(12.0, 5.0), texture: wall_options[get_random_int(0, wall_options.len() as i32 - 1) as usize].to_string() },
            Boundary { position: Vec2::new(13.0, 5.0), texture: wall_options[get_random_int(0, wall_options.len() as i32 - 1) as usize].to_string() },
            Boundary { position: Vec2::new(14.0, 5.0), texture: wall_options[get_random_int(0, wall_options.len() as i32 - 1) as usize].to_string() },
            Boundary { position: Vec2::new(15.0, 5.0), texture: wall_options[get_random_int(0, wall_options.len() as i32 - 1) as usize].to_string() },
        ]
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
        player: Player {
            text: "8",
            color: Color::new(0.0, 1.0, 0.0, 1.0),
            position: Vec2::new(0.0, 0.0),
            health: ZeroMaxStat::new(100.0, 100.0),
            energy: ZeroMaxStat::new(100.0, 100.0),
            animation: None,
        },
        creatures: creatures,
        entity_map: entity_map,
    };
}

fn generate_random_creatures() -> (Vec<Creature>, Grid<Vec<EntityRef>>) {
    let mut entity_map: Grid<Vec<EntityRef>> = grid::Grid::new((GRID_WIDTH + 1) as usize, (GRID_HEIGHT + 1) as usize);

    let mut creatures = Vec::with_capacity(1000);

    let mut gx = 30;
    let mut gy = 10;

    for _ in 0..500 {
        let p = Vec2::new(gx as f32, gy as f32);
        let c = Creature::new_random(p, 100.0, 100.0, get_time());
        let entity_ref = EntityRef::new(EntityType::Creature, creatures.len(), gx, gy);
        entity_map[gx as usize][gy as usize].push(entity_ref);
        creatures.push(c);
        gx += 1;
        if gx >= 60 {
            gx = 30;
            gy += 1;
        }
    }

    return (creatures, entity_map);
}


// fn gather_longest_survivers(clones: &mut Vec<Creature>) -> (Vec<Creature>, Grid<Vec<EntityRef>>) {
//     let mut longest_survivers = Vec::new();
//     let mut entity_map: Grid<Vec<EntityRef>> = grid::Grid::new((GRID_WIDTH + 1) as usize, (GRID_HEIGHT + 1) as usize);

//     clones.sort_by(|a, b| b.birth_time.partial_cmp(&a.birth_time).unwrap());

//     // select the first half of the creatures
//     // and clone them. But clone them twice
//     // to get original population size
//     for i in 0..(clones.len() / 2) {
//         let mut c1 = clones[i].clone();
//         let mut c2 = clones[i].clone();
//         c1.mutate();
//         c2.mutate();
//         longest_survivers.push(c1);
//         longest_survivers.push(c2);
//     }

//     for i in 0..longest_survivers.len() {
//         let survivor = &mut longest_survivers[i];
//         let (gx, gy) = grid_pos(survivor.position.x, survivor.position.y);
//         let entity_ref = EntityRef::new(EntityType::Creature, i, gx, gy);
//         entity_map[gx as usize][gy as usize].push(entity_ref);
//     }


//     return (longest_survivers, entity_map);
// }


// fn clone_creatures(creatures: &Vec<Creature>) -> Vec<Creature> {
//     let mut clones = Vec::new();
//     for c in creatures {
//         clones.push(c.clone());
//     }
//     return clones;
// }