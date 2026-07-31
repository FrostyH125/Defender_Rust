use basic_raylib_core::system::input_handler::InputState;
use raylib::{RaylibHandle, RaylibThread, color::Color, drawing::{RaylibDraw, RaylibDrawHandle}, ffi::KeyboardKey};

use crate::entities::{character::Character, object::Object};

#[derive(Debug)]
pub enum SelectingMode {
    Objects,
    Characters
}

pub struct EntitySelectingManager<'a> {
    /// unfortunately maintaining single source of truth for something like this is just very difficult
    /// i will have to keep in mind that anything added to this list will need to have is_selected = to true
    /// thankfully i can manage this invariant with the methods on this struct. so i should never have to actually
    /// do it myself
    selected_objects: Vec<&'a mut Object>,
    selected_characters: Vec<&'a mut Character>,
    pub selecting_mode: SelectingMode
}

impl<'a> EntitySelectingManager<'a> {
    pub fn new() -> Self {
        return Self {
            selected_objects: Vec::new(),
            selected_characters: Vec::new(),
            selecting_mode: SelectingMode::Objects,
        }
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
        d.draw_text(&format!("{:?}", self.selecting_mode), 2, 160, 30, Color::BLACK);
    }

    pub fn select_single_obj(&mut self, obj: &'a mut Object) {
        obj.get_mut_data().is_selected = true;
        self.selected_objects.push(obj);
    }

    pub fn select_single_char(&mut self, char: &'a mut Character) {
        char.get_mut_data().is_selected = true;
        self.selected_characters.push(char);
    }

    pub fn deselect_objs(&mut self) {
        for i in (0..self.selected_objects.len()).rev() {
            self.selected_objects[i].get_mut_data().is_selected = false;
            self.selected_objects.remove(i);
        }
    }

    pub fn deselect_chars(&mut self) {
        for i in (0..self.selected_characters.len()).rev() {
            self.selected_characters[i].get_mut_data().is_selected = false;
            self.selected_characters.remove(i);
        }
    }
}