use rand::random;
use raylib::{
    drawing::RaylibDrawHandle,
    math::{Rectangle, Vector2},
    texture::Texture2D,
};

use crate::entities::{object::Object::*, objects::tree::Tree};

/// This houses data that all objects share, as to not repeat fields between objects
pub struct ObjectData {
    pub pos: Vector2,
    pub draw_pos: Vector2,
    pub hover_rect: Rectangle,
    pub is_hovering: bool,
}

impl ObjectData {
    pub fn new(
        pos: Vector2,
        draw_offset: Vector2,
        randomized_offset: Vector2,
        width: f32,
        height: f32,
    ) -> Self {

        let true_pos = pos + randomized_offset;
        let true_draw_pos = true_pos + draw_offset;

        let hover_rect = Rectangle::new(true_draw_pos.x, true_draw_pos.y, width, height);
        
        return ObjectData {
            pos: true_pos,
            draw_pos: true_draw_pos,
            is_hovering: false,
            hover_rect
        };
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
            NoObject => panic!("why would you try to get data from a None Object?"),
        }
    }

    pub fn get_mut_data(&mut self) -> &mut ObjectData {
        match self {
            TreeObj(tree) => &mut tree.data,
            NoObject => panic!("why would you try to get data from a None Object?"),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.get_mut_data().is_hovering = false;

        match self {
            TreeObj(tree) => tree.update(dt),
            NoObject => (),
        }
    }

    pub fn is_point_intersecting(&self, p: Vector2) -> bool {
        return self.get_data().hover_rect.check_collision_point_rec(p);
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        match self {
            TreeObj(tree) => {
                tree.draw(d, texture);
            }
            NoObject => (),
        }
    }

    pub fn draw_hover(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        match self {
            TreeObj(tree) => {
                tree.draw_hover(d, texture);
            }
            NoObject => (),
        }
    }

    pub fn draw_shadow(
        &self,
        d: &mut RaylibDrawHandle,
        texture: &Texture2D,
        shear_x: f32,
        shear_y: f32,
    ) {
        match self {
            TreeObj(tree) => tree.draw_shadow(d, texture, shear_x, shear_y),
            NoObject => (),
        }
    }
}
