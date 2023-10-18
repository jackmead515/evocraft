use macroquad::prelude::Vec2;

#[derive(Debug, Clone)]
pub struct AnimationTransition {

    /// global start position for the animation
    pub initial_pos: Vec2,

    /// global final position for the animation
    pub final_pos: Vec2,

    /// the time at which the animation should start
    pub start_time: f64,

    /// the duration of the animation in seconds
    pub duration: f32,

    /// the type of curve to use for the animation
    pub curve_type: CurveType,
}

#[derive(Debug, Clone)]
pub enum CurveType {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    EaseQuadInOut,
    SinInOut,
    EaseBounceIn,
    EaseBounceOut
}

impl AnimationTransition {

    pub fn new(
        initial_pos: Vec2,
        final_pos: Vec2,
        start_time: f64,
        duration: f32,
        curve_type: CurveType
    ) -> AnimationTransition {
        AnimationTransition {
            initial_pos,
            final_pos,
            start_time,
            duration,
            curve_type
        }
    }

    pub fn is_complete(&self, now: f64) -> bool {
        return now - self.start_time > self.duration as f64;
    }

    pub fn interpolate(&self, now: f64) -> Vec2 {
        let progress: f32 = (now - self.start_time) as f32 / self.duration;
        match self.curve_type {
            CurveType::Linear => self.linear_interpolation(progress),
            CurveType::EaseIn => self.ease_in_interpolation(progress),
            CurveType::EaseOut => self.ease_out_interpolation(progress),
            CurveType::EaseInOut => self.ease_in_out_interpolation(progress),
            CurveType::EaseQuadInOut => self.ease_quad_in_out_interpolation(progress),
            CurveType::SinInOut => self.sin_in_out_interpolation(progress),
            CurveType::EaseBounceIn => self.ease_bounce_in_interpolation(progress),
            CurveType::EaseBounceOut => self.ease_bounce_out_interpolation(progress),
        }
    }

    pub fn linear_interpolation(&self, progress: f32) -> Vec2 {
        let initial_x = self.initial_pos.x;
        let initial_y = self.initial_pos.y;
        let final_x = self.final_pos.x;
        let final_y = self.final_pos.y;
        let x = initial_x + progress * (final_x - initial_x);
        let y = initial_y + progress * (final_y - initial_y);
        return Vec2::new(x, y);
    }

    pub fn ease_in_interpolation(&self, progress: f32) -> Vec2 {
        // Use an easing function to calculate the interpolated position
        // Example: quadratic easing in
        let initial_x = self.initial_pos.x;
        let initial_y = self.initial_pos.y;
        let final_x = self.final_pos.x;
        let final_y = self.final_pos.y;
        let t = progress;
        let x = initial_x + (t * t) * (final_x - initial_x);
        let y = initial_y + (t * t) * (final_y - initial_y);
        return Vec2::new(x, y);
    }

    pub fn ease_out_interpolation(&self, progress: f32) -> Vec2 {
        // Use an easing function to calculate the interpolated position
        // Example: quadratic easing out
        let initial_x = self.initial_pos.x;
        let initial_y = self.initial_pos.y;
        let final_x = self.final_pos.x;
        let final_y = self.final_pos.y;
        let t = progress;
        let x = initial_x + (-t * (t - 2.0)) * (final_x - initial_x);
        let y = initial_y + (-t * (t - 2.0)) * (final_y - initial_y);
        return Vec2::new(x, y);
    }

    pub fn ease_in_out_interpolation(&self, progress: f32) -> Vec2 {
        // Use an ease-in-out function to calculate the interpolated position
        let initial_x = self.initial_pos.x;
        let initial_y = self.initial_pos.y;
        let final_x = self.final_pos.x;
        let final_y = self.final_pos.y;
        let t = progress;
        let t_squared = t * t;
        let t_cubed = t * t * t;
        let x = initial_x + (-2.0 * t_cubed + 3.0 * t_squared) * (final_x - initial_x);
        let y = initial_y + (-2.0 * t_cubed + 3.0 * t_squared) * (final_y - initial_y);
        return Vec2::new(x, y);
    }

    pub fn ease_quad_in_out_interpolation(&self, progress: f32) -> Vec2 {
        // Use the ease-in-out quadratic function to calculate the interpolated position
        let initial_x = self.initial_pos.x;
        let initial_y = self.initial_pos.y;
        let final_x = self.final_pos.x;
        let final_y = self.final_pos.y;
        let t = if progress < 0.5 {
            2.0 * progress * progress
        } else {
            -1.0 + (4.0 - 2.0 * progress) * progress
        };
        let x = initial_x + t * (final_x - initial_x);
        let y = initial_y + t * (final_y - initial_y);
        return Vec2::new(x, y);
    }

    pub fn sin_in_out_interpolation(&self, progress: f32) -> Vec2 {
        // Use the sine in-out function to calculate the interpolated position
        let initial_x = self.initial_pos.x;
        let initial_y = self.initial_pos.y;
        let final_x = self.final_pos.x;
        let final_y = self.final_pos.y;
        let t = progress;
        let half = 0.5;
        let pi = std::f32::consts::PI;
        let x = initial_x + (half - (half * (pi * t).cos())) * (final_x - initial_x);
        let y = initial_y + (half - (half * (pi * t).cos())) * (final_y - initial_y);
        return Vec2::new(x, y);
    }

    pub fn ease_bounce_in_interpolation(&self, progress: f32) -> Vec2 {
        // Use the bounce-in function to calculate the interpolated position
        let initial_x = self.initial_pos.x;
        let initial_y = self.initial_pos.y;
        let final_x = self.final_pos.x;
        let final_y = self.final_pos.y;
        let t = 1.0 - progress;
        let b = 1.0 - (1.0 - t).powi(5);
        let x = initial_x + b * (final_x - initial_x);
        let y = initial_y + b * (final_y - initial_y);
        return Vec2::new(x, y);
    }

    fn ease_bounce_out_interpolation(&self, progress: f32) -> Vec2 {
        // Use the bounce-out function to calculate the interpolated position
        let initial_x = self.initial_pos.x;
        let initial_y = self.initial_pos.y;
        let final_x = self.final_pos.x;
        let final_y = self.final_pos.y;

        let t = progress;
        let b = 1.0 - (1.0 - t).powi(5);
        let x = initial_x + b * (final_x - initial_x);
        let y = initial_y + b * (final_y - initial_y);
        return Vec2::new(x, y);
    }

    

}