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

impl Deref for Creature {
    type Target = Creature;

    fn deref(&self) -> &Creature {
        return &self;
    }
}

impl DerefMut for Creature {
    fn deref_mut(&mut self) -> &mut Creature {
        return self;
    }
}