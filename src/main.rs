extern crate rand;
extern crate grid;

use std::sync::{Arc, Mutex};

use macroquad::prelude::*;

pub mod consts;
pub mod draw;
pub mod input;
pub mod update;
pub mod animation;
pub mod genes;
pub mod models;
pub mod creature;
pub mod grid_map;

use grid::Grid;
use models::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Window Conf".to_owned(),
        fullscreen: false,
        window_height: consts::SCREEN_HEIGHT,
        window_width: consts::SCREEN_WIDTH,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let font = load_ttf_font("assets/unifont-15.0.06.ttf").await.expect("Failed to load font");

    let mut entity_map: Grid<Vec<EntityRef>> = grid::Grid::new(100, 100);
 
    // create a 3d array on the stack memory of the grid
    // this represents a complete map of all items in the grid
    // like creatures, items, plants, things the player can
    // interact with. Boundaries are not included in the grid
    // Only 100 entities can be in a single grid cell stacked 
    // on top of each other. Array indicies represent grid
    // coordinates.
    //let mut entity_map: Vec<Vec<Vec<(EntityType, u32)>>> = vec![vec![vec![]; consts::SCREEN_HEIGHT as usize]; consts::SCREEN_WIDTH as usize];

    let mut creatures = Vec::with_capacity(1000);


    // create as many creatures in the center of the screen
    // in a box of 50x50 in grid coordinates
    let mut i = 0;
    while i < 50 {
        let mut j = 0;
        while j < 50 {
            let (gx, gy) = (20 + i, 20 + j);
            let p = Position::new(consts::world_pos(gx, gy));
            let c = Mutex::new(Creature::new_random(p, 100.0, 100.0));

            if entity_map[gx as usize][gy as usize].len() >= 100 {
                println!("grid map is full");
                break;
            }

            let entity_ref = EntityRef {
                entity_type: EntityType::Creature,
                index: creatures.len(),
                gx: gx as usize,
                gy: gy as usize,
            };
        
            entity_map[gx as usize][gy as usize].push(entity_ref);
            creatures.push(c);

            j += 1;
        }
        i += 1;
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

        // let player = &game_state.player;
        // let mut camera = Camera2D::from_display_rect(
        //     Rect::new(
        //         player.position.x - consts::SCREEN_WIDTH as f32 / 2.0,
        //         player.position.y - consts::SCREEN_HEIGHT as f32 / 2.0,
        //         consts::SCREEN_WIDTH as f32,
        //         consts::SCREEN_HEIGHT as f32
        //     )
        // );
        // set_camera(&camera);


        // set_camera(&Camera2D {
        //     zoom: vec2(0.003, 0.003),
        //     target: vec2(player.position.x, player.position.y),
        //     ..Default::default()
        // });

        input::input(&mut game_state);
        update::update(&mut game_state);
        draw::draw(&game_state);

        // set_default_camera();

        next_frame().await
    }
}