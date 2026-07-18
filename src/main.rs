use basic_raylib_core::system::input_handler::InputState;
use raylib::{
    camera::Camera2D,
    color::Color,
    drawing::{RaylibDraw, RaylibMode2DExt, RaylibShaderModeExt, RaylibTextureModeExt},
    math::{Rectangle, Vector2},
};

use crate::{
    entities::entity_manager::EntityManager, map::tile_map::TileMap,
    systems::day_night_cycle::DayNightCycle,
};

pub mod entities;
pub mod map;
pub mod systems;
pub mod utils;

// Object::update() -> match update()
// Object::draw(WHITE) -> match draw(WHITE) // actually draws normally since WHITE is the whitelisted color from being changed by shader
// Object::draw_shadow(Color::0,0,0,0) -> match draw(Color::0,0,0,0)

// DayNightCycle::update_shadow_info() -> changes shear and scale based on time_of_day (which goes from 0.0..=360.0), 180.0..=360.0 is night and will use the night time shear and scale
// DayNightCycle::update_day_night_color() -> more tint info to be passed to the shader

// shadows_and_lighting.fs

pub const V_WIDTH: f32 = 320.0;
pub const V_HEIGHT: f32 = 180.0;

pub const TILE_SIZE: f32 = 8.0;

fn main() {
    let mut screen_width = 1920.0;
    let mut screen_height = 1080.0;

    const INITIAL_ZOOM: f32 = 1.0;

    let mut camera = Camera2D {
        offset: Vector2 {
            x: V_WIDTH / 2.0,
            y: V_HEIGHT / 2.0,
        },
        target: Vector2 {
            x: V_WIDTH / 2.0,
            y: V_HEIGHT / 2.0,
        },
        rotation: 0.0,
        zoom: INITIAL_ZOOM,
    };

    let mut input_state = InputState::new();

    let mut map = TileMap::new(500, 500);
    let mut entity_manager = EntityManager::new(500, 500);

    let mut day_night_cycle = DayNightCycle::new();

    let (mut rl, thread) = raylib::init()
        .size(screen_width as i32, screen_height as i32)
        .title("Rust Raylib Starter")
        .build();

    let texture = rl.load_texture(&thread, "Tileset.png").unwrap();
    let mut outline_shader = rl.load_shader(&thread, None, Some("outline.frag"));
    let mut render_texture = rl
        .load_render_texture(&thread, V_WIDTH as u32, V_HEIGHT as u32)
        .unwrap();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();

        // update input first
        input_state.update(&mut rl, camera.zoom);
        if input_state.middle_currently_held {
            camera.target.x -= input_state.delta.x / (screen_width / V_WIDTH);
            camera.target.y -= input_state.delta.y / (screen_height / V_HEIGHT);
        }

        //--UPDATE BEGINS HERE--//
        map.update(dt);
        entity_manager.update(&mut map.map_object_grid, dt, V_WIDTH, V_HEIGHT, &camera);
        day_night_cycle.update(dt, &mut rl);

        //--UPDATE ENDS HERE--//

        //--DRAWING BEINGS HERE--//
        {
            let mut d = rl.begin_drawing(&thread);
            {
                let mut rt_handle = d.begin_texture_mode(&thread, &mut render_texture);
                rt_handle.clear_background(Color::RAYWHITE);
                {
                    let mut cam_handle = rt_handle.begin_mode2D(camera);
                    map.draw(&mut cam_handle, &camera, V_WIDTH, V_HEIGHT, &texture);

                    {
                        let mut outline_shader_handle =
                            cam_handle.begin_shader_mode(&mut outline_shader);
                        entity_manager.draw(
                            &map.map_object_grid,
                            &mut outline_shader_handle,
                            &texture,
                        );
                    } // end shader mode - nothing drawn will pass through shader beyond here
                } // end camera mode - nothing drawn will be drawn in world space beyond here
            } // end rt mode - nothing drawn will be drawn on the render texture beyond here
            let source_rec = Rectangle::new(
                0.0,
                0.0,
                render_texture.texture.width as f32,
                -render_texture.texture.height as f32, // Negative height flips it right-side up
            );

            let dest_rec = Rectangle::new(0.0, 0.0, screen_width, screen_height);
            let origin = Vector2::new(0.0, 0.0);

            d.draw_texture_pro(
                &render_texture,
                source_rec,
                dest_rec,
                origin,
                0.0,
                Color::WHITE,
            );
            day_night_cycle.draw_dbg(&mut d);
        }
        //--DRAWING ENDS HERE--//
    }
}
