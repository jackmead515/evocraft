use grid::Grid;

use crate::models::{EntityType, EntityRef};

pub fn get_all(map: &Grid<Vec<EntityRef>>, x: usize, y: usize) -> &Vec<EntityRef> {
    return &map[x as usize][y as usize];
}

pub fn get_all_type(
    map: &Grid<Vec<EntityRef>>,
    x: usize, 
    y: usize,
    entity_type: EntityType
) -> Vec<EntityRef>
{
    let mut objects = Vec::new();
    for eref in get_all(map, x, y) {
        if eref.entity_type == entity_type {
            objects.push(*eref);
        }
    }
    objects
}

pub fn get_rect_type(
    map: &Grid<Vec<EntityRef>>,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    entity_type: EntityType
) -> Vec<EntityRef>
{
    let mut objects = Vec::new();

    for x in x..x+w {

        if x >= map.rows() {
            continue;
        }

        for y in y..y+h {

            if y >= map.cols() {
                continue;
            }

            objects.extend_from_slice(&get_all_type(map, x, y, entity_type));
            
        }
    }

    return objects;
}