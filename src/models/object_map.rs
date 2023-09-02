// impl ObjectMap {

//     pub fn new(width: usize, height: usize) -> ObjectMap {
//         ObjectMap {
//             map: vec![vec![vec![]; height]; width],
//         }
//     }

//     pub fn get(&self, x: usize, y: usize, z: usize) -> Option<(usize, usize, usize, usize)> {
//         unimplemented!("ObjectMap::get");
//         // match self.map[x][y][z] {
//         //     Some(object) => Some((x, y, z, object)),
//         //     None => None,
//         // }
//     }

//     pub fn get_all(&self, x: usize, y: usize) -> Vec<(usize, usize, usize, usize)> {
//         let mut objects = Vec::new();
//         for z in 0..self.map[x][y].len() {
//             match self.map[x][y][z] {
//                 Some(object) => objects.push((x, y, z, object)),
//                 None => {},
//             }
//         }
//         objects
//     }

//     pub fn insert(&mut self, x: usize, y: usize, object: usize) {
//         self.map[x][y].push(Some(object));
//     }

//     pub fn get_rect(&self, x: usize, y: usize, width: usize, height: usize) -> Vec<(usize, usize, usize, usize)> {
//         let mut objects = Vec::new();
//         for x in x..x+width {
//             for y in y..y+height {
//                 objects.extend_from_slice(&self.get_all(x, y));
//             }
//         }
//         objects
//     }

// }