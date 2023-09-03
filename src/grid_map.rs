use grid::Grid;

use crate::models::{EntityType, EntityRef};

pub fn get_all(map: &Grid<Vec<EntityRef>>, gx: u32, gy: u32) -> &Vec<EntityRef> {
    return &map[gx as usize][gy as usize];
}

pub fn get_all_type(
    map: &Grid<Vec<EntityRef>>,
    gx: u32, 
    gy: u32,
    entity_type: EntityType
) -> Vec<EntityRef>
{
    let mut objects = Vec::new();
    for eref in get_all(map, gx, gy) {
        if eref.entity_type == entity_type {
            objects.push(*eref);
        }
    }
    objects
}