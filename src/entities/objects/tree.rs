use basic_raylib_core::graphics::sprite::Sprite;
use raylib::{color::Color, drawing::RaylibDrawHandle, math::Vector2, texture::Texture2D};

use crate::{TILE_SIZE, entities::object::ObjectData, utils::{draw_utils, map_cord::MapCord}};

static TREE_SPRITE: Sprite = Sprite::new(144, 24, 8, 16);

pub struct Tree {
    pub data: ObjectData,
}

impl Tree {
    pub fn new(map_cord: MapCord) -> Self {
        let x_pos = map_cord.x as f32 * TILE_SIZE;
        let y_pos = map_cord.y as f32 * TILE_SIZE;

        return Tree {
            data: ObjectData::new(Vector2::new(x_pos, y_pos)),
        };
    }

    pub fn update(&mut self, dt: f32) {
        
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        // need to draw it at an offset since its 2 tiles tall, its real position
        // is still the tile that its on
        TREE_SPRITE.draw(d, self.data.pos - Vector2::new(0.0, TILE_SIZE), texture);
    }

    pub fn draw_hover(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        draw_utils::draw_outline(d, &TREE_SPRITE, self.data.pos - Vector2::new(0.0, TILE_SIZE), texture);
    }
}
