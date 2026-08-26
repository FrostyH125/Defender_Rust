use raylib::{drawing::RaylibDrawHandle, math::Vector2, texture::Texture2D};
use zander_game_core_rs::raylib::sprite::Sprite;

use crate::GameContext;

static MOUSE_SPRITE: Sprite = Sprite::new(104, 128, 8, 8);

#[inline]
pub fn mouse_world_coords(game_context: &GameContext) -> Vector2 {
    // essentially, mouse_pos divided by the zoom level
    // mouse_pos * 320 / 1920 = mouse_pos * 1 / 6 = mouse_pos / 6
    // this accounts for the render target zooms exclusively
    let mouse_screen_pos_scaled_with_render_target = Vector2 {
        x: game_context.input_state.mouse_pos.x * game_context.v_width as f32
            / game_context.logical_window_width as f32,
        y: game_context.input_state.mouse_pos.y * game_context.v_height as f32
            / game_context.logical_window_height as f32,
    };

    // parens account for aligning the pos with the offset of the camera
    // then adding camera target accounts for where the camera is in world space
    return Vector2 {
        x: mouse_screen_pos_scaled_with_render_target.x - game_context.camera.offset.x
            + game_context.camera.target.x,
        y: mouse_screen_pos_scaled_with_render_target.y - game_context.camera.offset.y
            + game_context.camera.target.y,
    };

    // the result is very simple in essence
    // step 1: find where the mouse is on the VIRTUAL screen
    // then add the camera offsets and target to find out where it is in world space
}

pub fn draw_mouse(d: &mut RaylibDrawHandle, mouse_world_pos: Vector2, sprite_sheet: &Texture2D) {
    MOUSE_SPRITE.draw(d, mouse_world_pos, sprite_sheet);
}
