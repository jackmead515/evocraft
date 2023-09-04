use macroquad::prelude::*;

use crate::models::Position;
use crate::animation::AnimationTransition;
use crate::models::ZeroMaxStat;

pub struct Player {
    pub text: &'static str,
    pub color: Color,
    pub position: Position,
    pub animation: Option<AnimationTransition>,
    pub health: ZeroMaxStat,
    pub energy: ZeroMaxStat,
}