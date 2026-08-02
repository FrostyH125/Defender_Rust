use raylib::{
    RaylibHandle,
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
    ffi::KeyboardKey,
};

use crate::{
    entities::{entity_manager::CharacterEntry, object::Object},
    map::tile_map::MapObjectGrid,
};

#[derive(Debug)]
pub enum SelectingMode {
    Objects,
    Characters,
}

pub struct EntitySelectingManager {
    // these store map id's and character unique ids respectively
    // if it weren't for needing the character id's and object id's for the action buttons being able
    // to know which characters should be checked, i probably wouldnt need these lists at all.
    pub selected_objects: Vec<usize>,
    pub selected_characters: Vec<usize>,
    pub selecting_mode: SelectingMode,
    pub is_deselecting_chars: bool,
    pub is_deselecting_objs: bool,
}

impl EntitySelectingManager {
    pub fn new() -> Self {
        return Self {
            selected_objects: Vec::new(),
            selected_characters: Vec::new(),
            selecting_mode: SelectingMode::Objects,
            is_deselecting_chars: false,
            is_deselecting_objs: false,
        };
    }

    pub fn update(&mut self, rl: &mut RaylibHandle) {
        self.is_deselecting_chars = false;
        self.is_deselecting_objs = false;

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

    pub fn select_single_obj(&mut self, object: &mut Object, idx: usize) {
        self.deselect_objs();
        let data = object.get_mut_data();
        data.is_selected = true;
        self.selected_objects.push(idx);
    }

    pub fn select_single_char(&mut self, character_entry: &mut CharacterEntry) {
        self.deselect_chars();

        character_entry.character.get_mut_data().is_selected = true;
        self.selected_characters.push(character_entry.unique_id);
    }

    pub fn select_multiple_chars(&mut self, hover_characters: Vec<&mut CharacterEntry>) {
        self.deselect_chars();

        for ch in hover_characters {
            ch.character.get_mut_data().is_selected = true;
            self.selected_characters.push(ch.unique_id);
        }
    }

    pub fn select_multiple_objs(&mut self, object_grid: &mut MapObjectGrid, indexes: Vec<usize>) {
        self.deselect_objs();
        for idx in indexes {
            let obj = &mut object_grid[idx];
            obj.get_mut_data().is_selected = true;
            self.selected_objects.push(idx);
        }
    }

    pub fn deselect_objs(&mut self) {
        self.is_deselecting_objs = true;
        self.selected_objects.clear();
    }

    pub fn deselect_chars(&mut self) {
        self.is_deselecting_chars = true;
        self.selected_characters.clear()
    }
}
