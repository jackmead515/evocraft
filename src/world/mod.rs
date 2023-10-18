use macroquad::prelude::*;

pub struct Boundary {
    pub position: Vec2,
    pub texture: String,
}

pub struct Floor {
    pub position: Vec2,
    pub texture: String,
}

pub struct World {
    pub boundaries: Vec<Boundary>,
    pub floor: Vec<Floor>,
}

impl World {

    // collide with the world boundaries
    pub fn collide(&self, position: &Vec2) -> Option<&Boundary> {
        let pos_box = Rect::new(position.x, position.y, 1.0, 1.0);

        for boundary in &self.boundaries {
            let bod_box = Rect::new(boundary.position.x, boundary.position.y, 1.0, 1.0);

            if pos_box.left() < bod_box.right()
            && pos_box.right() > bod_box.left()
            && pos_box.top() < bod_box.bottom()
            && pos_box.bottom() > bod_box.top() {
                return Some(boundary);
            }
        }
        return None;
    }

}