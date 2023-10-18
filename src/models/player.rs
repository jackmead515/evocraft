use macroquad::prelude::*;

use crate::animation::AnimationTransition;
use crate::models::ZeroMaxStat;

pub struct Player {
    pub text: &'static str,
    pub color: Color,
    pub position: Vec2,
    pub animation: Option<AnimationTransition>,
    pub health: ZeroMaxStat,
    pub energy: ZeroMaxStat,
}