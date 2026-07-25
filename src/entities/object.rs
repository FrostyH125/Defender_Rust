use basic_raylib_core::graphics::sprite::Sprite;
use rand::rngs::ThreadRng;
use raylib::{
    drawing::RaylibDrawHandle,
    math::{Rectangle, Vector2},
    texture::Texture2D,
};

use crate::{
    entities::{
        object::Object::*,
        objects::{grass::Grass, tree::Tree},
    }, systems::day_night_cycle::DayNightCycle, utils::draw_utils,
};

/// This houses data that all objects share, as to not repeat fields between objects
pub struct ObjectData {
    pub pos: Vector2,
    pub draw_pos: Vector2,
    pub hover_rect: Rectangle,
    pub is_hovering: bool,
    pub shear_x: f32,
    pub scale_y: f32,
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
            hover_rect,
            shear_x: 0.0,
            scale_y: 0.0,
        };
    }
}

pub enum Object {
    NoObject,
    TreeObj(Tree),
    GrassObj(Grass),
}

impl Object {
    pub fn get_data(&self) -> &ObjectData {
        match self {
            TreeObj(tree) => &tree.data,
            GrassObj(grass) => &grass.data,
            NoObject => panic!("why would you try to get data from a None Object?"),
        }
    }

    pub fn get_mut_data(&mut self) -> &mut ObjectData {
        match self {
            TreeObj(tree) => &mut tree.data,
            GrassObj(grass) => &mut grass.data,
            NoObject => panic!("why would you try to get data from a None Object?"),
        }
    }

    pub fn update(
        &mut self,
        dt: f32,
        day_night_cycle: &DayNightCycle,
        total_game_time: f32,
        rng: &mut ThreadRng,
    ) {
        let data = self.get_mut_data();

        data.is_hovering = false;
        data.shear_x = day_night_cycle.current_shadow_shear;
        data.scale_y = day_night_cycle.current_shadow_scale_y;

        match self {
            TreeObj(tree) => tree.update(dt),
            GrassObj(grass) => grass.update(dt, total_game_time, rng),
            NoObject => (),
        }
    }

    pub fn is_point_intersecting(&self, p: Vector2) -> bool {
        return self.get_data().hover_rect.check_collision_point_rec(p);
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        let sprite = self.current_sprite();

        sprite.draw(d, self.get_data().draw_pos, texture);
    }

    pub fn draw_hover(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        let sprite = self.current_sprite();
        draw_utils::draw_outline(d, sprite, self.get_data().draw_pos, texture);
    }

    pub fn draw_shadow(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        let sprite = self.current_sprite();
        let data = self.get_data();

        draw_utils::draw_shadow(d, sprite, data.draw_pos, data.shear_x, data.scale_y, texture);
    }

    pub fn current_sprite(&self) -> &Sprite {
        match self {
            NoObject => todo!(),
            TreeObj(tree) => tree.sprite(),
            GrassObj(grass) => grass.sprite(),
        }
    }
}
