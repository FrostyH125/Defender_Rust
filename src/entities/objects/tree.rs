use basic_raylib_core::graphics::sprite::Sprite;
use rand::rngs::ThreadRng;
use raylib::{drawing::RaylibDrawHandle, math::Vector2, texture::Texture2D};

use crate::{
    TILE_SIZE,
    entities::object::{Object, ObjectData},
    utils::{draw_utils, map_cord::MapCord, vector2_utils},
};

static TREE_SPRITE: Sprite = Sprite::new(144, 24, 8, 16);

pub struct Tree {
    pub data: ObjectData,
}

impl Tree {
    pub fn new(map_cord: MapCord, rng: &mut ThreadRng) -> Object {
        let x_pos = map_cord.x as f32 * TILE_SIZE;
        let y_pos = map_cord.y as f32 * TILE_SIZE;

        let tree = Tree {
            data: ObjectData::new(
                Vector2::new(x_pos, y_pos),
                Vector2::new(0.0, -TILE_SIZE),
                vector2_utils::random_offset_by_one(rng),
                8.0,
                16.0,
            ),
        };

        return Object::TreeObj(tree);
    }

    pub fn update(&mut self, dt: f32) {}

    pub fn draw(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        // need to draw it at an offset since its 2 tiles tall, its real position
        // is still the tile that its on
        TREE_SPRITE.draw(d, self.data.draw_pos, texture);
    }

    pub fn draw_hover(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        draw_utils::draw_outline(d, &TREE_SPRITE, self.data.draw_pos, texture);
    }

    pub fn draw_shadow(
        &self,
        d: &mut RaylibDrawHandle,
        texture: &Texture2D,
        shear_x: f32,
        scale_y: f32,
    ) {
        draw_utils::draw_shadow(
            d,
            &TREE_SPRITE,
            self.data.draw_pos,
            shear_x,
            scale_y,
            texture,
        );
    }
}
