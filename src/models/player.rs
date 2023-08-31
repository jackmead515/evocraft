use macroquad::prelude::*;

use crate::models::Position;
use crate::animation::AnimationTransition;

pub struct Player {
    pub text: &'static str,
    pub color: Color,
    pub position: Position,
    pub animation: Option<AnimationTransition>,
}