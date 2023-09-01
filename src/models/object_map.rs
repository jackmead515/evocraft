pub struct ObjectMap<T> {
    pub map: Vec<Vec<Vec<Option<T>>>>,
}

impl<T> ObjectMap<T> {

    pub fn new(width: usize, height: usize, depth: usize) -> ObjectMap<T> {
        let mut map = Vec::with_capacity(width);
        for _ in 0..width {
            let mut row = Vec::with_capacity(height);
            for _ in 0..height {
                let mut column = Vec::with_capacity(depth);
                for _ in 0..depth {
                    column.push(None);
                }
                row.push(column);
            }
            map.push(row);
        }
        ObjectMap {
            map: map,
        }
    }

    pub fn reset(&mut self) {
        for x in 0..self.map.len() {
            for y in 0..self.map[x].len() {
                for z in 0..self.map[x][y].len() {
                    self.map[x][y][z] = None;
                }
            }
        }
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> Option<&T> {
        match self.map[x][y][z] {
            Some(ref object) => Some(object),
            None => None,
        }
    }

    pub fn get_all(&self, x: usize, y: usize) -> Vec<&T> {
        return self.map[x][y].iter().filter_map(|object| match object {
            Some(object) => Some(object),
            None => None,
        }).collect();
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, object: T) {
        self.map[x][y][z] = Some(object);
    }

    pub fn insert(&mut self, x: usize, y: usize, object: T) {
        self.map[x][y].push(Some(object));
    }

    pub fn remove(&mut self, x: usize, y: usize, z: usize) {
        self.map[x][y][z] = None;
    }

    pub fn get_rect(&self, x: usize, y: usize, width: usize, height: usize, z: usize) -> Vec<&T> {
        let mut objects = Vec::new();
        for x in x..x+width {
            for y in y..y+height {
                match self.map[x][y][z] {
                    Some(ref object) => objects.push(object),
                    None => {}
                }
            }
        }
        objects
    }

    pub fn remove_rect(&mut self, x: usize, y: usize, width: usize, height: usize, z: usize) {
        for x in x..x+width {
            for y in y..y+height {
                self.map[x][y][z] = None;
            }
        }
    }

}