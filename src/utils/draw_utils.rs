use basic_raylib_core::graphics::sprite::Sprite;
use raylib::{color::Color, drawing::RaylibDrawHandle, math::Vector2, texture::Texture2D};

use crate::utils::directional_deltas::CARDINAL_DELTAS;

pub fn draw_with_shear(
    d: &mut RaylibDrawHandle,
    sprite: &Sprite,
    pos: Vector2,
    shear_x: f32,
    shear_y: f32,
    texture: &Texture2D,
) {
}

pub fn draw_outline(d: &mut RaylibDrawHandle, sprite: &Sprite, pos: Vector2, texture: &Texture2D) {
    for dir in CARDINAL_DELTAS {
        let draw_pos = Vector2::new(pos.x + dir.x as f32, pos.y + dir.y as f32);
        sprite.draw_col(d, draw_pos, texture, Color::new(255, 255, 255, 0));
    }
}
