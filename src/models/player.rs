use macroquad::prelude::*;

use crate::animation::AnimationMovement;
use crate::models::ZeroMaxStat;

pub struct Player {
    pub text: &'static str,
    pub color: Color,
    pub position: Vec2,
    pub animation: Option<AnimationMovement>,
    pub health: ZeroMaxStat,
    pub energy: ZeroMaxStat,
}