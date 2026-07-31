use basic_raylib_core::system::input_handler::InputState;
use raylib::{
    RaylibHandle, RaylibThread,
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
    ffi::KeyboardKey,
};

use crate::{
    entities::entity_manager::CharacterEntry,
    map::tile_map::MapObjectGrid,
};

#[derive(Debug)]
pub enum SelectingMode {
    Objects,
    Characters,
}

pub struct EntitySelectingManager {
    /// unfortunately maintaining single source of truth for something like this is just very difficult
    /// i will have to keep in mind that anything added to this list will need to have is_selected = to true
    /// thankfully i can manage this invariant with the methods on this struct. so i should never have to actually
    /// do it myself
    selected_objects: Vec<usize>,
    selected_characters: Vec<usize>,
    pub selecting_mode: SelectingMode,
}

impl EntitySelectingManager {
    pub fn new() -> Self {
        return Self {
            selected_objects: Vec::new(),
            selected_characters: Vec::new(),
            selecting_mode: SelectingMode::Objects,
        };
    }

    pub fn update(&mut self, rl: &mut RaylibHandle) {
        if rl.is_key_pressed(KeyboardKey::KEY_ONE) {
            self.selecting_mode = SelectingMode::Objects;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_TWO) {
            self.selecting_mode = SelectingMode::Characters;
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_text(
            &format!("{:?}", self.selecting_mode),
            2,
            160,
            30,
            Color::BLACK,
        );
    }

    pub fn select_single_obj(&mut self, object_grid: &mut MapObjectGrid, idx: usize) {
        self.deselect_objs(object_grid);

        let obj = &mut object_grid[idx];
        
        obj.get_mut_data().is_selected = true;
        self.selected_objects.push(idx);
    }

    /// deselects all characters and then selects the character with the same id
    pub fn select_single_char(&mut self, character_entries: &mut Vec<CharacterEntry>, id: usize) {
        self.deselect_chars(character_entries);

        let character = &mut character_entries.iter_mut().find(|c| c.unique_id == id).unwrap().character;
        
        character.get_mut_data().is_selected = true;
        self.selected_characters.push(id);
    }

    pub fn deselect_objs(&mut self, object_grid: &mut MapObjectGrid) {
        for idx in self.selected_objects.drain(..) {
            object_grid[idx].get_mut_data().is_selected = false;
        }
    }

    pub fn deselect_chars(&mut self, character_entries: &mut Vec<CharacterEntry>) {
        // im so sorry
        for id in self.selected_characters.drain(..) {
            character_entries
                .iter_mut()
                .find(|c| c.unique_id == id)
                .unwrap()
                .character
                .get_mut_data()
                .is_selected = false;
        }
    }
}
