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

    let creatures = generate_random_creatures();
    //let mut clones = clone_creatures(&creatures);

    let texture_map = textures::load().await;

    let world = generate_random_world();

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
            position: Vec2::new(250.0, 250.0),
            health: ZeroMaxStat::new(100.0, 100.0),
            energy: ZeroMaxStat::new(100.0, 100.0),
            animation: None,
        },
        creatures: creatures,
    };
}

fn generate_random_creatures() -> Vec<Creature> {
    let mut creatures = Vec::with_capacity(1000);

    let mut gx = 250;
    let mut gy = 250;

    for _ in 0..500 {
        let p = Vec2::new(gx as f32, gy as f32);
        let c = Creature::new_random(p, 100.0, 100.0, get_time());
        creatures.push(c);
        gx += 1;
        if gx >= 300 {
            gx = 250;
            gy += 1;
        }
    }

    return creatures;
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