use macroquad::rand::gen_range;

use crate::models::{Position, Brain, Health, Energy};
use crate::animation::AnimationTransition;

#[derive(Debug, Clone)]
pub struct Creature {
    pub text: &'static str,
    pub position: Position,
    pub random_offset: (f32, f32),
    pub brain: Brain,
    pub animation: Option<AnimationTransition>,
    pub health: Health,
    pub energy: Energy,
    pub alive: bool,
    pub reproduce_time: f64,
    pub birth_time: f64,
    pub total_travel_distance: u32,
}

impl Creature {
    pub fn new_random(position: Position, health: f32, energy: f32, birth_time: f64) -> Self {
        // random offset
        let rx = gen_range(0.0, 5.0);
        let ry = gen_range(0.0, -5.0);

        return Creature {
            text: "@",
            position: position,
            random_offset: (rx, ry),
            brain: Brain::random(),
            animation: None,
            health: Health::new(health),
            energy: Energy::new(energy),
            alive: true,
            reproduce_time: 0.0,
            birth_time: birth_time,
            total_travel_distance: 0,
        };
    }
}