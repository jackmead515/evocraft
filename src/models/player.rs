use macroquad::prelude::*;

use crate::models::Position;
use crate::animation::AnimationTransition;
use crate::models::{Health, Energy};

pub struct Player {
    pub text: &'static str,
    pub color: Color,
    pub position: Position,
    pub animation: Option<AnimationTransition>,
    pub health: Health,
    pub energy: Energy,
}