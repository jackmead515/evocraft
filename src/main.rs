extern crate rand;
extern crate grid;

use std::{sync::Mutex, f32::consts::E};

use macroquad::prelude::*;
use grid::Grid;

pub mod consts;
pub mod draw;
pub mod update;
pub mod animation;
pub mod genes;
pub mod models;
pub mod creature;
pub mod grid_map;

use consts::*;
use models::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Window Conf".to_owned(),
        fullscreen: false,
        window_height: SCREEN_HEIGHT as i32,
        window_width: SCREEN_WIDTH as i32,
        window_resizable: false,
        ..Default::default()
    }
}

fn generate_random_creatures() -> (Vec<Creature>, Grid<Vec<EntityRef>>) {
    let mut entity_map: Grid<Vec<EntityRef>> = grid::Grid::new((GRID_WIDTH + 1) as usize, (GRID_HEIGHT + 1) as usize);

    let mut creatures = Vec::with_capacity(1000);

    let mut gx = 30;
    let mut gy = 10;

    for _ in 0..10 {
        let p = Position::new(world_pos(gx, gy));
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


fn gather_longest_survivers(clones: &mut Vec<Creature>) -> (Vec<Creature>, Grid<Vec<EntityRef>>) {
    let mut longest_survivers = Vec::new();
    let mut entity_map: Grid<Vec<EntityRef>> = grid::Grid::new((GRID_WIDTH + 1) as usize, (GRID_HEIGHT + 1) as usize);

    clones.sort_by(|a, b| b.birth_time.partial_cmp(&a.birth_time).unwrap());

    // select the first half of the creatures
    // and clone them. But clone them twice
    // to get original population size
    for i in 0..(clones.len() / 2) {
        let mut c1 = clones[i].clone();
        let mut c2 = clones[i].clone();
        c1.mutate();
        c2.mutate();
        longest_survivers.push(c1);
        longest_survivers.push(c2);
    }

    for i in 0..longest_survivers.len() {
        let survivor = &mut longest_survivers[i];
        let (gx, gy) = grid_pos(survivor.position.x, survivor.position.y);
        let entity_ref = EntityRef::new(EntityType::Creature, i, gx, gy);
        entity_map[gx as usize][gy as usize].push(entity_ref);
    }


    return (longest_survivers, entity_map);
}

fn clone_creatures(creatures: &Vec<Creature>) -> Vec<Creature> {
    let mut clones = Vec::new();
    for c in creatures {
        clones.push(c.clone());
    }
    return clones;
}


#[macroquad::main(window_conf)]
async fn main() {

    let font = load_ttf_font("assets/unifont-15.0.06.ttf").await.expect("Failed to load font");

    let (creatures, entity_map) = generate_random_creatures();
    let mut clones = clone_creatures(&creatures);

    let mut game_state = GameState {
        stats: GameStats {
            fps: 0,
            frame_time: 0.0,
            elapsed: 0.0,
        },
        font: font,
        player: Player {
            text: "8",
            color: Color::new(0.0, 1.0, 0.0, 1.0),
            position: models::Position::new(consts::world_pos(10, 10)),
            health: ZeroMaxStat::new(100.0, 100.0),
            energy: ZeroMaxStat::new(100.0, 100.0),
            animation: None,
        },
        creatures: creatures,
        entity_map: entity_map,
    };

    loop {
        game_state.stats.fps = get_fps();
        game_state.stats.frame_time = get_frame_time();
        game_state.stats.elapsed = get_time();

        if game_state.creatures.len() <= 0 {
            let (new_creatures, new_entity_map) = gather_longest_survivers(&mut clones);
            clones = clone_creatures(&new_creatures);
            game_state.creatures = new_creatures;
            game_state.entity_map = new_entity_map;
        }

        update::update(&mut game_state);
        draw::draw(&game_state);

        next_frame().await;
    }
}