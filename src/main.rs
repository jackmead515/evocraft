extern crate rand;

use macroquad::prelude::*;

pub mod consts;
pub mod draw;
pub mod input;
pub mod update;
pub mod animation;
pub mod creature;
pub mod genes;

use animation::{AnimationTransition, CurveType};
use creature::{Creature, OutputTypes};

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

struct Player {
    text: &'static str,
    color: Color,
    x: f32,
    y: f32,
    animation: Option<AnimationTransition>,
}

#[macroquad::main(window_conf)]
async fn main() {

    let font = load_ttf_font("assets/unifont-15.0.06.ttf").await.expect("Failed to load font");
    let ref_font = Some(&font);

    let ppos = consts::world_pos(20, 20);

    let mut player = Player {
        text: "8",
        color: Color::new(0.0, 1.0, 0.0, 1.0),
        x: ppos.0,
        y: ppos.1,
        animation: None,
    };

    let mut creatures = Vec::new();

    let mut cpos = consts::world_pos(30, 30);
    for i in 0..1000 {
        cpos.0 + (i as f32 * consts::GRID_SIZE as f32);
        creatures.push(Creature {
            text: "@",
            x: cpos.0,
            y: cpos.1,
            brain: creature::Brain::random(),
            animation: None
        });

        if i > 0 && i % 10 == 0 {
            cpos.0 = consts::world_pos(30, 30).0;
            cpos.1 += consts::GRID_SIZE as f32;
        }
    }

    loop {
        let fps = get_fps();
        let elapsed = get_time();
        let frame_time = get_frame_time();

        //input::input();
        //update::update();
        draw::draw();

        if is_key_down(KeyCode::D) {
            if player.animation.is_none() {
                player.animation = Some(
                    AnimationTransition::new(
                        (player.x, player.y),
                        (player.x + consts::GRID_SIZE as f32, player.y),
                        elapsed, 0.3, CurveType::EaseQuadInOut
                    )
                )
            }
        } else if is_key_down(KeyCode::A) {
            if player.animation.is_none() {
                player.animation = Some(
                    AnimationTransition::new(
                        (player.x, player.y),
                        (player.x - consts::GRID_SIZE as f32, player.y),
                        elapsed, 0.3, CurveType::EaseQuadInOut
                    )
                )
            }
        } else if is_key_down(KeyCode::W) {
            if player.animation.is_none() {
                player.animation = Some(
                    AnimationTransition::new(
                        (player.x, player.y),
                        (player.x, player.y - consts::GRID_SIZE as f32),
                        elapsed, 0.3, CurveType::EaseQuadInOut
                    )
                )
            }
        } else if is_key_down(KeyCode::S) {
            if player.animation.is_none() {
                player.animation = Some(
                    AnimationTransition::new(
                        (player.x, player.y),
                        (player.x, player.y + consts::GRID_SIZE as f32),
                        elapsed, 0.3, CurveType::EaseQuadInOut
                    )
                )
            }
        }

        match player.animation {
            Some(ref mut animation) => {
                let (x, y) = animation.interpolate(elapsed);
                player.x = x;
                player.y = y;
                if elapsed - animation.start_time > animation.duration as f64 {
                    player.animation = None;
                }
            },
            None => {}
        }
        
        for creature in creatures.iter_mut() {

            // gather all the inputs regardless of 
            // whether or not the brain needs them
            let inputs = vec![
                creature.x,
                creature.y,
                player.x,
                player.y,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
            ];

            match creature.animation {
                Some(ref mut animation) => {
                    let (x, y) = animation.interpolate(elapsed);
                    creature.x = x;
                    creature.y = y;
                    if elapsed - animation.start_time > animation.duration as f64 {
                        creature.animation = None;
                    }
                },
                None => {}
            }
            
            if creature.animation.is_none() {
                let (_, output_type) = creature.brain.compute(inputs);

                match output_type {
                    creature::OutputTypes::MoveUp => {
                        creature.animation = Some(
                            AnimationTransition::new(
                                (creature.x, creature.y),
                                (creature.x, creature.y - consts::GRID_SIZE as f32),
                                elapsed, 0.3, CurveType::EaseQuadInOut
                            )
                        );
                    },
                    creature::OutputTypes::MoveDown => {
                        creature.animation = Some(
                            AnimationTransition::new(
                                (creature.x, creature.y),
                                (creature.x, creature.y + consts::GRID_SIZE as f32),
                                elapsed, 0.3, CurveType::EaseQuadInOut
                            )
                        );
                    },
                    creature::OutputTypes::MoveLeft => {
                        creature.animation = Some(
                            AnimationTransition::new(
                                (creature.x, creature.y),
                                (creature.x - consts::GRID_SIZE as f32, creature.y),
                                elapsed, 0.3, CurveType::EaseQuadInOut
                            )
                        );
                    },
                    creature::OutputTypes::MoveRight => {
                        creature.animation = Some(
                            AnimationTransition::new(
                                (creature.x, creature.y),
                                (creature.x + consts::GRID_SIZE as f32, creature.y),
                                elapsed, 0.3, CurveType::EaseQuadInOut
                            )
                        );
                    },
                    _ => {}
                }
            }

            // collide with wall
            if creature.x <= 0.0 {
                creature.x = 0.0;
            }
            if creature.x >= consts::SCREEN_WIDTH as f32 {
                creature.x = consts::SCREEN_WIDTH as f32;
            }
            if creature.y <= 0.0 {
                creature.y = 0.0;
            }
            if creature.y >= consts::SCREEN_HEIGHT as f32 {
                creature.y = consts::SCREEN_HEIGHT as f32;
            }

            draw_text_ex(creature.text, creature.x, creature.y, TextParams {
                font: ref_font,
                font_size: 20,
                ..Default::default()
            });
        }

        draw_text_ex(player.text, player.x, player.y, TextParams {
            font: ref_font,
            font_size: 20,
            color: player.color,
            ..Default::default()
        });

        // draw fps
        draw_text(&format!("FPS: {}", fps), 10.0, 20.0, 20.0, WHITE);

        next_frame().await
    }
}