use std::collections::{HashSet};

use crate::{map::tile_map::MapDimensions, utils::{map_cord::MapCord, map_utils::cords_to_index}};

pub const CELL_SIZE: u16 = 50;

pub struct MapCell {
    // these are to be primarily used by the characters
    // as such, these ID's are going to be indices into the character array
    // rather than the unique ID's presented in CharacterEntry
    // not only does this provide faster lookup, but it also lets characters
    // look up character data without needing access to the character array, just
    // a cut down version of it with the pure info they need
    pub chars_in_cell: Vec<usize>
}

impl MapCell {
    pub fn new() -> Self {
        return MapCell {
            chars_in_cell: Vec::new()
        };
    }

    #[inline]
    pub fn add_char(&mut self, char_index: usize) {
        self.chars_in_cell.push(char_index);
    }

    #[inline]
    pub fn clear_chars(&mut self) {
        self.chars_in_cell.clear();
    }
}
