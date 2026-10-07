use raylib::{
    camera::Camera2D,
    ffi::GetWorldToScreen2D,
    math::{Rectangle, Vector2},
};

use crate::{GameContext, ZoomSizes};

/// checks if an object is within the visible view
#[inline]
pub fn is_in_camera_view(visual_rect: Rectangle, game_context: &GameContext) -> bool {
    // draw_pos.x, draw_pos.y, w, h, which is what we need to see if obj is in view
    let cam_pos = game_context.camera.target - game_context.camera.offset;

    // get the cam size
    let cam_width = game_context.v_width as f32;
    let cam_height = game_context.v_height as f32;

    // actual camera view rect
    let cam_rect = Rectangle::new(cam_pos.x, cam_pos.y, cam_width, cam_height);

    // return false if the camera is out of view, whether or not its still in the update area,
    return cam_rect.check_collision_recs(&visual_rect);
}

/// checks if an object is within the area being updated
#[inline]
pub fn is_in_update_area(object_pos: Vector2, game_context: &GameContext) -> bool {
    return game_context
        .update_rect
        .check_collision_point_rec(object_pos);
}

#[inline]
pub fn world_to_screen(world_pos: Vector2, camera: &Camera2D, zoom: ZoomSizes) -> Vector2 {
    let scale = zoom.zoom() as f32;
    let ffi_v2: raylib::ffi::Vector2 = world_pos.into();
    let ffi_camera: raylib::ffi::Camera2D = camera.into();

    unsafe {
        let screen_raw: Vector2 = GetWorldToScreen2D(ffi_v2, ffi_camera).into();
        return screen_raw * scale;
    }
}
