extern crate rand;

use macroquad::prelude::*;

pub mod consts;
pub mod draw;
pub mod input;
pub mod update;
pub mod animation;
pub mod creature;
pub mod genes;
pub mod models;

use models::*;
use creature::{Creature, Brain};

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

    let mut creatures = Vec::with_capacity(1000);
    let mut cpos = Position::new(consts::world_pos(30, 30));
    let mut i = 30;

    // create  creatures with random brains across
    // the grid starting from 30, 30 to 50, 50
    while i < 50 {
        let mut j = 30;
        while j < 50 {
            creatures.push(Creature { 
                text: "@",
                x: cpos.x,
                y: cpos.y,
                brain: Brain::random(),
                animation: None,
            });
            cpos.x += consts::GRID_SIZE as f32;
            j += 1;
        }
        cpos.x = consts::world_x(30);
        cpos.y += consts::world_y(1);
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
            position: models::Position::new(consts::world_pos(20, 20)),
            animation: None,
        },
        creatures: creatures
    };

    loop {
        game_state.stats.fps = get_fps();
        game_state.stats.frame_time = get_frame_time();
        game_state.stats.elapsed = get_time();

        let player = &game_state.player;
        let mut camera = Camera2D::from_display_rect(
            Rect::new(
                player.position.x - consts::SCREEN_WIDTH as f32 / 2.0,
                player.position.y - consts::SCREEN_HEIGHT as f32 / 2.0,
                consts::SCREEN_WIDTH as f32,
                consts::SCREEN_HEIGHT as f32
            )
        );
        set_camera(&camera);


        // set_camera(&Camera2D {
        //     zoom: vec2(0.003, 0.003),
        //     target: vec2(player.position.x, player.position.y),
        //     ..Default::default()
        // });

        input::input(&mut game_state);
        update::update(&mut game_state);
        draw::draw(&game_state);

        set_default_camera();

        next_frame().await
    }
}