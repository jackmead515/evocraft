
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(pos: (f32, f32)) -> Position {
        Position {
            x: pos.0,
            y: pos.1,
        }
    }

    pub fn set(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn get(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}