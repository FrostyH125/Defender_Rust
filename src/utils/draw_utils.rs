use basic_raylib_core::graphics::sprite::Sprite;
use raylib::{
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
    math::{Rectangle, Vector2},
    texture::Texture2D,
};

use crate::utils::directional_deltas::CARDINAL_DELTAS;

/// scale_y is 0..=1.0, meaning how far the sprite is scaled down, 0 being not at all
/// 1.0 being all the way to the base
/// 1.0..=2.0 for scale_y will put the shadow in front of the sprite
pub fn draw_shadow(
    d: &mut RaylibDrawHandle,
    sprite: &Sprite,
    pos: Vector2,
    shear_x: f32,
    scale_y: f32,
    texture: &Texture2D,
) {
    let shear_matrix = [
        1.0, 0.0, 0.0, 0.0, -shear_x, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    ];
    let sprite_pivot_x = pos.x + sprite.src_rect.width / 2.0;
    let sprite_pivot_y = pos.y + sprite.src_rect.height;

    let local_scale_y = scale_y * sprite.src_rect.height;
    
    let shadow_in_front = local_scale_y < -sprite.src_rect.height;
    let real_scale_y = local_scale_y.abs();

    unsafe {
        raylib::ffi::rlPushMatrix();

        raylib::ffi::rlTranslatef(sprite_pivot_x, sprite_pivot_y, 0.0);

        raylib::ffi::rlMultMatrixf(shear_matrix.as_ptr());

        raylib::ffi::rlTranslatef(-sprite_pivot_x, -sprite_pivot_y, 0.0);
    }

    let mut dest_rect = Rectangle {
        x: pos.x,
        y: pos.y + real_scale_y,
        width: sprite.src_rect.width,
        height: sprite.src_rect.height - real_scale_y,
    };

    let mut src_rect = sprite.src_rect;

    if shadow_in_front {
        dest_rect.y = pos.y + src_rect.height;
        src_rect.height = -src_rect.height;
    }

    d.draw_texture_pro(
        texture,
        src_rect,
        dest_rect,
        Vector2::zero(),
        0.0,
        Color::new(255, 255, 0, 255),
    );

    unsafe {
        raylib::ffi::rlPopMatrix();
    }
}

pub fn draw_outline(d: &mut RaylibDrawHandle, sprite: &Sprite, pos: Vector2, texture: &Texture2D) {
    for dir in CARDINAL_DELTAS {
        let draw_pos = Vector2::new(pos.x + dir.x as f32, pos.y + dir.y as f32);
        sprite.draw_col(d, draw_pos, texture, Color::new(255, 255, 255, 0));
    }

    sprite.draw(d, pos, texture);
}
