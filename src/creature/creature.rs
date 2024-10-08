use macroquad::rand::gen_range;
use macroquad::prelude::Vec2;

use crate::creature::*;
use crate::models::ZeroMaxStat;
use crate::util::animation::AnimationMovement;
use crate::util::delay::Delay;

#[derive(Debug, Clone)]
pub struct CreatureDelays {
    pub behavior_rest: Option<Delay>,
    pub energy_restore: Option<Delay>,
}

#[derive(Debug, Clone)]
pub struct Creature {
    pub position: Vec2,
    pub brain: Brain,
    pub current_behavior: Option<OutputTypes>,
    pub movement: Option<AnimationMovement>,
    pub delays: CreatureDelays,
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
            brain: Brain::random(2.0),
            current_behavior: None,
            movement: None,
            delays: CreatureDelays {
                energy_restore: None,
                behavior_rest: None,
            },
            health: ZeroMaxStat::new(health, health),
            energy: ZeroMaxStat::new(energy, energy),
            alive: true,
            birth_time: birth_time,
            total_travel_distance: 0,
        };
    }
}


