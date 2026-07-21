use basic_raylib_core::graphics::sprite::Sprite;
use raylib::{camera::Camera2D, drawing::RaylibDrawHandle, math::Vector2, texture::Texture2D};

static MOUSE_SPRITE: Sprite = Sprite::new(104, 128, 8, 8);

pub fn mouse_world_coords(
    mouse_pos: Vector2,
    camera: &Camera2D,
    window_width: f32,
    window_height: f32,
    virtual_width: f32,
    virtual_height: f32,
) -> Vector2 {
    // essentially, mouse_pos divided by the zoom level
    // mouse_pos * 320 / 1920 = mouse_pos * 1 / 6 = mouse_pos / 6
    // this accounts for the render target zooms exclusively
    let mouse_screen_pos_scaled_with_render_target = Vector2 {
        x: mouse_pos.x * virtual_width / window_width,
        y: mouse_pos.y * virtual_height / window_height,
    };

    // parens account for aligning the pos with the offset of the camera
    // then adding camera target accounts for where the camera is in world space
    return Vector2 {
        x: mouse_screen_pos_scaled_with_render_target.x - camera.offset.x + camera.target.x,
        y: mouse_screen_pos_scaled_with_render_target.y - camera.offset.y + camera.target.y,
    };

    // the result is very simple in essence
    // step 1: find where the mouse is on the VIRTUAL screen
    // then add the camera offsets and target to find out where it is in world space
}

pub fn draw_mouse(d: &mut RaylibDrawHandle, mouse_world_pos: Vector2, sprite_sheet: &Texture2D) {
    MOUSE_SPRITE.draw(d, mouse_world_pos, sprite_sheet);
}
