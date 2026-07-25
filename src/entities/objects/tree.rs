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
    pub fn new(cord: MapCord, rng: &mut ThreadRng) -> Object {
        let tree = Tree {
            data: ObjectData::new(
                cord.map_pos(),
                Vector2::new(0.0, -TILE_SIZE),
                vector2_utils::random_offset_by_one(rng),
                8.0,
                16.0,
            ),
        };

        return Object::TreeObj(tree);
    }

    pub fn update(&mut self, dt: f32) {}

    pub fn sprite(&self) -> &Sprite {
        return &TREE_SPRITE;
    }
}
