extern crate rand;
extern crate grid;

use std::sync::Mutex;

use macroquad::prelude::*;
use grid::Grid;

pub mod consts;
pub mod draw;
pub mod input;
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

#[macroquad::main(window_conf)]
async fn main() {

    let font = load_ttf_font("assets/unifont-15.0.06.ttf").await.expect("Failed to load font");

    let mut entity_map: Grid<Vec<EntityRef>> = grid::Grid::new(100, 100);

    let mut creatures = Vec::with_capacity(600);

    let mut gx = 30;
    let mut gy = 20;

    for _ in 0..600 {
        let p = Position::new(world_pos(gx, gy));
        let c = Mutex::new(Creature::new_random(p, 100.0, 100.0, get_time()));

        if entity_map[gx as usize][gy as usize].len() < 100 {
            let entity_ref = EntityRef::new(EntityType::Creature, creatures.len(), gx, gy);
            entity_map[gx as usize][gy as usize].push(entity_ref);
            creatures.push(c);
        }

        gx += 1;
        if gx >= 60 {
            gx = 30;
            gy += 1;
        }
    }

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
            health: Health::new(100.0),
            energy: Energy::new(100.0),
            animation: None,
        },
        creatures: creatures,
        entity_map: entity_map,
    };

    loop {
        game_state.stats.fps = get_fps();
        game_state.stats.frame_time = get_frame_time();
        game_state.stats.elapsed = get_time();
        
        input::input(&mut game_state);
        update::update(&mut game_state);
        draw::draw(&game_state);

        next_frame().await;
    }
}