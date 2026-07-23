use basic_raylib_core::system::input_handler::InputState;
use raylib::{
    camera::Camera2D, color::Color, drawing::{RaylibDraw, RaylibMode2DExt, RaylibShaderModeExt, RaylibTextureModeExt}, ffi::KeyboardKey, math::{Rectangle, Vector2}, shaders::RaylibShader, texture::RenderTexture2D,
};

use crate::{
    ZoomSizes::{FiveX, FourX, OneX, ThreeX, TwoX},
    entities::entity_manager::EntityManager,
    map::tile_map::TileMap,
    systems::day_night_cycle::DayNightCycle,
    utils::mouse_utils,
};

pub mod entities;
pub mod map;
pub mod systems;
pub mod utils;

// any of these can be done in any order:
//  enter moon phase info into DayNightCycle
//  add grass
//  add new tree variants
pub const TILE_SIZE: f32 = 8.0;

fn main() {
    let mut window_width = 1920.0;
    let mut window_height = 1080.0;

    let mut current_zoom = ZoomSizes::FiveX;
    let mut camera = Camera2D {
        offset: Vector2 {
            x: current_zoom.v_width() / 2.0,
            y: current_zoom.v_height() / 2.0,
        },
        target: Vector2 {
            x: current_zoom.v_width() / 2.0,
            y: current_zoom.v_height() / 2.0,
        },
        rotation: 0.0,
        zoom: 1.0,
    };

    let mut camera_pos = camera.target;

    let mut input_state = InputState::new();

    let mut map = TileMap::generate_map(500, 500);
    let mut entity_manager = EntityManager::new(map.map_dimensions);

    let mut day_night_cycle = DayNightCycle::new();

    let (mut rl, thread) = raylib::init()
        .size(window_width as i32, window_height as i32)
        .title("Rust Raylib Starter")
        .build();

    let texture = rl.load_texture(&thread, "Tileset.png").unwrap();
    let mut shader = rl.load_shader(&thread, None, Some("outline.frag"));
    let red_tint_loc = shader.get_shader_location("red_tint");
    let blue_tint_loc = shader.get_shader_location("blue_tint");
    let brightness_modifier_loc = shader.get_shader_location("brightness_modifier");

    let mut render_textures: [RenderTexture2D; 5] = [
        rl.load_render_texture(&thread, 1920, 1080).unwrap(),
        rl.load_render_texture(&thread, 960, 540).unwrap(),
        rl.load_render_texture(&thread, 640, 360).unwrap(),
        rl.load_render_texture(&thread, 480, 270).unwrap(),
        rl.load_render_texture(&thread, 320, 180).unwrap(),
    ];

    rl.set_target_fps(60);
    rl.disable_cursor();

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();

        // update input first
        input_state.update(&mut rl, camera.zoom);

        if input_state.middle_roll.abs() >= 1.0 {
            if input_state.middle_roll < 0.0 {
                current_zoom = current_zoom.change_res(false);
            } else {
                current_zoom = current_zoom.change_res(true);
            }

            camera.offset.x = current_zoom.v_width() / 2.0;
            camera.offset.y = current_zoom.v_height() / 2.0;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_Z) {
            current_zoom = current_zoom.change_res(false);
            camera.offset.x = current_zoom.v_width() / 2.0;
            camera.offset.y = current_zoom.v_height() / 2.0;
        } else if rl.is_key_pressed(KeyboardKey::KEY_X) {
            current_zoom = current_zoom.change_res(true);
            camera.offset.x = current_zoom.v_width() / 2.0;
            camera.offset.y = current_zoom.v_height() / 2.0;
        }

        if rl.is_key_down(KeyboardKey::KEY_D) {
            camera_pos.x += current_zoom.v_width() * dt;
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            camera_pos.x -= current_zoom.v_width() * dt;
        }
        if rl.is_key_down(KeyboardKey::KEY_W) {
            camera_pos.y -= current_zoom.v_width() * dt;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            camera_pos.y += current_zoom.v_width() * dt;
        }

        if input_state.middle_currently_held {
            camera_pos.x -= input_state.delta.x / (window_width / current_zoom.v_width());
            camera_pos.y -= input_state.delta.y / (window_height / current_zoom.v_height());
        }

        // remove any floating points from camera pos
        camera.target.x = camera_pos.x.round();
        camera.target.y = camera_pos.y.round();

        //--UPDATE BEGINS HERE--//
        map.update(dt);
        entity_manager.update(
            &mut map.map_object_grid,
            dt,
            window_width,
            window_height,
            current_zoom.v_width(),
            current_zoom.v_height(),
            &camera,
            &input_state,
        );
        day_night_cycle.update(dt, &mut rl);
        
        shader.set_shader_value(red_tint_loc, day_night_cycle.red_tint);
        shader.set_shader_value(blue_tint_loc, day_night_cycle.blue_tint);
        shader.set_shader_value(brightness_modifier_loc, day_night_cycle.brightness_modifier);

        //--UPDATE ENDS HERE--//
        let current_rt = &mut render_textures[current_zoom as usize];

        //--DRAWING BEINGS HERE--//
        {
            let mut d = rl.begin_drawing(&thread);
            {
                let mut render_texture_handle = d.begin_texture_mode(&thread, current_rt);
                render_texture_handle.clear_background(Color::RAYWHITE);
                {
                    let mut cam_handle = render_texture_handle.begin_mode2D(camera);
                    {
                        let mut shader_handle = cam_handle.begin_shader_mode(&mut shader);

                        map.draw(
                            &mut shader_handle,
                            &camera,
                            current_zoom.v_width(),
                            current_zoom.v_height(),
                            &texture,
                        );

                        entity_manager.draw(
                            &day_night_cycle,
                            &map.map_object_grid,
                            &mut shader_handle,
                            &texture,
                        );
                        mouse_utils::draw_mouse(
                            &mut shader_handle,
                            mouse_utils::mouse_world_coords(
                                input_state.mouse_pos,
                                &camera,
                                window_width,
                                window_height,
                                current_zoom.v_width(),
                                current_zoom.v_height(),
                            ),
                            &texture,
                        );
                    } // end shader mode - nothing drawn will pass through shader beyond here
                } // end camera mode - nothing drawn will be drawn in world space beyond here
            } // end rt mode - nothing drawn will be drawn on the render texture beyond here

            let source_rec = Rectangle::new(
                0.0,
                0.0,
                current_rt.texture.width as f32,
                -current_rt.texture.height as f32, // Negative height flips it right-side up
            );

            let dest_rec = Rectangle::new(0.0, 0.0, window_width, window_height);
            let origin = Vector2::new(0.0, 0.0);

            d.draw_texture_pro(current_rt, source_rec, dest_rec, origin, 0.0, Color::WHITE);
            day_night_cycle.draw_dbg(&mut d);
        }
        //--DRAWING ENDS HERE--//
    }
}

#[repr(usize)]
#[derive(Clone, Copy)]
enum ZoomSizes {
    OneX,   // 1920x1080
    TwoX,   // 960x540
    ThreeX, // 640x360
    FourX,  // 480x270
    FiveX,  // 320x180
}

impl ZoomSizes {
    pub fn change_res(self, up: bool) -> Self {
        let current_index = self as usize;

        let add: isize = match up {
            true => 1,
            false => -1,
        };

        let mut idx = (current_index as isize + add) as usize;

        if idx > 6 {
            idx = 0
        }

        return Self::get_zoom_from_index(idx);
    }

    pub fn get_zoom_from_index(idx: usize) -> Self {
        let comp = idx.clamp(0, 4);

        match comp {
            0 => OneX,
            1 => TwoX,
            2 => ThreeX,
            3 => FourX,
            4 => FiveX,
            5.. => FiveX,
        }
    }

    pub fn v_width(self) -> f32 {
        match self {
            OneX => 1920.0,
            TwoX => 960.0,
            ThreeX => 640.0,
            FourX => 480.0,
            FiveX => 320.0,
        }
    }

    pub fn v_height(self) -> f32 {
        match self {
            OneX => 1080.0,
            TwoX => 540.0,
            ThreeX => 360.0,
            FourX => 270.0,
            FiveX => 180.0,
        }
    }
}
