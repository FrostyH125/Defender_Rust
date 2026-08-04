use std::collections::{HashSet};

use crate::{map::tile_map::MapDimensions, utils::{map_cord::MapCord, map_utils::cords_to_index}};

pub const CELL_SIZE: u16 = 50;

pub struct MapCell {
    // map id's of valid objects, in other words, the tile index
    pub objects_in_cell: HashSet<usize>,
}

impl MapCell {
    pub fn new() -> Self {
        return MapCell {
            objects_in_cell: HashSet::new(),
        };
    }

    pub fn add_obj(&mut self, index: usize) {
        self.objects_in_cell.insert(index);
    }

    pub fn add_obj_from_cord(&mut self, map_dimensions: MapDimensions, cord: MapCord) {
        let idx = cords_to_index(map_dimensions, cord);
        self.add_obj(idx);
    }

    pub fn remove_obj(&mut self, index: usize) {
        self.objects_in_cell.remove(&index);
    }
}
