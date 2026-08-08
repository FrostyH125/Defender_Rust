use basic_raylib_core::graphics::sprite::Sprite;
use raylib::{
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
    math::{Rectangle, Vector2},
    texture::Texture2D,
};

use crate::utils::directional_deltas::CARDINAL_DELTAS;

/// scale_y is -1.0..=1.0, meaning how far the sprite is scaled down, 0 being not at all
/// 1.0 exact dimensions of base sprite, -1.0 being exact dimensions of base sprite, flipped upside down, mirrored over x axis
pub fn draw_shadow(
    d: &mut RaylibDrawHandle,
    sprite: Sprite,
    pos: Vector2,
    shear_x: f32,
    scale_y: f32,
    texture: &Texture2D,
) {

    const SHADOW_TRIGGER_COLOR: Color = Color::new(255, 255, 0, 255);
    
    let sprite_pivot_x = pos.x + sprite.src_rect.width / 2.0;
    let sprite_pivot_y = pos.y + sprite.src_rect.height;

    let scale_pixels_y = sprite.src_rect.height - (sprite.src_rect.height * scale_y.abs());
    let shadow_in_front = scale_y < 0.0;

    // this is not necessary but makes the shear_x value more logical when upside down
    let mut final_shear_x = shear_x;
    if shadow_in_front {
        final_shear_x = -final_shear_x;
    }
    
    let shear_matrix = [
        1.0, 0.0, 0.0, 0.0, -final_shear_x, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    ];
    
    let mut dest_rect = Rectangle {
        x: pos.x,
        y: pos.y + scale_pixels_y,
        width: sprite.src_rect.width,
        height: sprite.src_rect.height - scale_pixels_y,
    };

    let mut src_rect = sprite.src_rect;

    // move the top to the base of the sprite being shadowed
    // flip src to flip it across the x axis
    if shadow_in_front {
        dest_rect.y = pos.y + src_rect.height;
        src_rect.height = -src_rect.height;
    }
    
    unsafe {
        raylib::ffi::rlPushMatrix();

        raylib::ffi::rlTranslatef(sprite_pivot_x, sprite_pivot_y, 0.0);

        raylib::ffi::rlMultMatrixf(shear_matrix.as_ptr());

        raylib::ffi::rlTranslatef(-sprite_pivot_x, -sprite_pivot_y, 0.0);
    }


    d.draw_texture_pro(
        texture,
        src_rect,
        dest_rect,
        Vector2::zero(),
        0.0,
        SHADOW_TRIGGER_COLOR
    );

    unsafe {
        raylib::ffi::rlPopMatrix();
    }
}

pub fn draw_outline(d: &mut RaylibDrawHandle, sprite: Sprite, pos: Vector2, texture: &Texture2D) {
    const OUTLINE_TRIGGER_COLOR: Color = Color::new(255, 255, 255, 0);
    
    for dir in CARDINAL_DELTAS {
        let draw_pos = Vector2::new(pos.x + dir.x as f32, pos.y + dir.y as f32);
        sprite.draw_col(d, draw_pos, texture, OUTLINE_TRIGGER_COLOR);
    }

    sprite.draw(d, pos, texture);
}

pub fn draw_outline_for_move(d: &mut RaylibDrawHandle, sprite: Sprite, pos: Vector2, texture: &Texture2D) {
    const MOVE_OUTLINE_TRIGGER_COLOR: Color = Color::new(0, 255, 255, 255);
    
    for dir in CARDINAL_DELTAS {
        let draw_pos = Vector2::new(pos.x + dir.x as f32, pos.y + dir.y as f32);
        sprite.draw_col(d, draw_pos, texture, MOVE_OUTLINE_TRIGGER_COLOR);
    }

    sprite.draw(d, pos, texture);
}

pub fn draw_with_extra_brightness(d: &mut RaylibDrawHandle, sprite: Sprite, pos: Vector2, texture: &Texture2D) {
    const EXTRA_BRIGHTNESS_TRIGGER_COLOR: Color = Color::new(255, 0, 255, 255);
    
    sprite.draw_col(d, pos, texture, EXTRA_BRIGHTNESS_TRIGGER_COLOR);
}
