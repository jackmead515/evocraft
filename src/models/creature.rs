use std::ops::{Deref, DerefMut};

use crate::models::{Position, Brain, Health, Energy};
use crate::animation::AnimationTransition;

#[derive(Debug, Clone)]
pub struct Creature {
    pub text: &'static str,
    pub position: Position,
    pub brain: Brain,
    pub animation: Option<AnimationTransition>,
    pub health: Health,
    pub energy: Energy,
}

impl Creature {
    pub fn new_random(position: Position, health: f32, energy: f32) -> Self {
        return Creature {
            text: "@",
            position: position,
            brain: Brain::random(),
            animation: None,
            health: Health::new(health),
            energy: Energy::new(energy),
        };
    }
}