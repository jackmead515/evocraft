use macroquad::rand::gen_range;
use macroquad::prelude::Vec2;

use crate::brain::{Brain, BrainInputTypes, OutputTypes};
use crate::models::ZeroMaxStat;
use crate::animation::AnimationTransition;

#[derive(Debug, Clone)]
pub struct Creature {
    pub position: Vec2,
    pub behavior_brain: Brain,
    pub current_behavior: Option<OutputTypes>,
    pub movement: Option<AnimationTransition>,
    pub health: ZeroMaxStat,
    pub energy: ZeroMaxStat,
    pub alive: bool,
    pub birth_time: f64,
    pub total_travel_distance: u32,
}

impl Creature {
    pub fn new_random(position: Vec2, health: f32, energy: f32, birth_time: f64) -> Self {
        return Creature {
            position: position,
            behavior_brain: Brain::random(BrainInputTypes::Behavior, 2.0),
            current_behavior: None,
            movement: None,
            health: ZeroMaxStat::new(health, health),
            energy: ZeroMaxStat::new(energy, energy),
            alive: true,
            birth_time: birth_time,
            total_travel_distance: 0,
        };
    }
}


