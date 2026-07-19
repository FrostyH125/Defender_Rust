use basic_raylib_core::graphics::sprite::Sprite;
use raylib::{color::Color, drawing::RaylibDrawHandle, math::{Rectangle, Vector2}, texture::Texture2D};

use crate::utils::directional_deltas::CARDINAL_DELTAS;

pub fn draw_shadow(
    d: &mut RaylibDrawHandle,
    sprite: &Sprite,
    pos: Vector2,
    shear_x: f32,
    scale_y: f32,
    texture: &Texture2D,
) {
    let shear_matrix = [
        1.0, 0.0, 0.0, 0.0, shear_x, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    ];

    unsafe {
        raylib::ffi::rlPushMatrix();

        let sprite_pivot_x = pos.x + sprite.src_rect.width / 2.0;
        let sprite_pivot_y = pos.y + sprite.src_rect.height;

        // Translate to the sprite's world position pivot
        raylib::ffi::rlTranslatef(sprite_pivot_x, sprite_pivot_y, 0.0);
        // Apply shear matrix
        raylib::ffi::rlMultMatrixf(shear_matrix.as_ptr());
        // Translate back to local origin
        raylib::ffi::rlTranslatef(-sprite_pivot_x, -sprite_pivot_y, 0.0);
    }

    let dest_rect = Rectangle {
        x: pos.x,
        y: pos.y + scale_y,
        width: sprite.src_rect.width,
        height: sprite.src_rect.height - scale_y,
    };

    sprite.draw_pro(d, dest_rect, Vector2::zero(), 0.0, texture, Color::new(255, 255, 0, 255));

    unsafe {
        raylib::ffi::rlPopMatrix();
    }
}

pub fn draw_outline(d: &mut RaylibDrawHandle, sprite: &Sprite, pos: Vector2, texture: &Texture2D) {
    for dir in CARDINAL_DELTAS {
        let draw_pos = Vector2::new(pos.x + dir.x as f32, pos.y + dir.y as f32);
        sprite.draw_col(d, draw_pos, texture, Color::new(255, 255, 255, 0));
    }
}
