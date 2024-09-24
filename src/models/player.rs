use macroquad::prelude::*;

use crate::util::animation::AnimationMovement;
use crate::models::ZeroMaxStat;
use crate::util::delay::Delay;

pub struct PlayerDelays {
    pub energy_restore: Option<Delay>,
}

pub struct Player {
    pub color: Color,
    pub position: Vec2,
    pub movement: Option<AnimationMovement>,
    pub delays: PlayerDelays,
    pub health: ZeroMaxStat,
    pub energy: ZeroMaxStat,
    pub vigor: ZeroMaxStat,
}