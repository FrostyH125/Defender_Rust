use basic_raylib_core::system::input_handler::InputState;
use raylib::{RaylibHandle, RaylibThread, color::Color, drawing::{RaylibDraw, RaylibDrawHandle}, ffi::KeyboardKey};

use crate::entities::{character::Character, object::Object};

#[derive(Debug)]
pub enum SelectingMode {
    Objects,
    Characters
}

pub struct EntitySelectingManager<'a> {
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
        self.selected_objects.push(obj);
    }

    pub fn select_single_char(&mut self, char: &'a mut Character) {
        self.selected_characters.push(char);
    }
}