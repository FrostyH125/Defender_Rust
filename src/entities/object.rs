use raylib::{color::Color, drawing::RaylibDrawHandle, math::Vector2, texture::{self, Texture2D}};

use crate::entities::{object::Object::*, objects::tree::Tree};

/// This houses data that all objects share, as to not repeat fields between objects
pub struct ObjectData {
    pub pos: Vector2,
    pub randomized_offset: Vector2,
    pub draw_offset: Vector2
}

impl ObjectData {
    pub fn new(pos: Vector2, draw_offset: Vector2, randomized_offset: Vector2) -> Self {
        return ObjectData { pos, draw_offset, randomized_offset };
    }

    pub fn draw_pos(&self) -> Vector2 {
        return self.pos + self.draw_offset + self.randomized_offset;
    }
}

pub enum Object {
    NoObject,
    TreeObj(Tree),
}

impl Object {

    pub fn get_data(&self) -> &ObjectData {
        match self {
            TreeObj(tree) => &tree.data,
            NoObject => panic!("why would you try to get data from a None Object?")
        }
    }

    pub fn get_mut_data(&mut self) -> &mut ObjectData {
        match self {
            TreeObj(tree) => &mut tree.data,
            NoObject => panic!("why would you try to get data from a None Object?")
        }
    }
    
    pub fn update(&mut self, dt: f32) {
        match self {
            TreeObj(tree) => tree.update(dt),
            NoObject => ()
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        match self {
            TreeObj(tree) => {
                tree.draw(d, texture);
            },
            NoObject => ()
        }
    }

    pub fn draw_hover(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        match self {
            TreeObj(tree) => {
                tree.draw_hover(d, texture);
            },
            NoObject => ()
        }
    }

    pub fn draw_shadow(&self, d: &mut RaylibDrawHandle, texture: &Texture2D, shear_x: f32, shear_y: f32) {
        match self {
            TreeObj(tree) => tree.draw_shadow(d, texture, shear_x, shear_y),
            NoObject => (),
        }
    }
}