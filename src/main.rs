use basic_raylib_core::system::input_handler::InputState;
use raylib::{
    camera::Camera2D,
    color::Color,
    drawing::{RaylibDraw, RaylibMode2DExt},
    math::Vector2,
};

use crate::map::tile_map::TileMap;

pub mod map;
pub mod utils;
pub mod entities;

// Object::update() -> match update()
// Object::draw(WHITE) -> match draw(WHITE) // actually draws normally since WHITE is the whitelisted color from being changed by shader
// Object::draw_shadow(Color::0,0,0,0) -> match draw(Color::0,0,0,0)
// Sprite::draw_col(... ... ... Color) -> draw_texture_pro(... ... ... Color)

pub const V_WIDTH: f32 = 320.0;
pub const V_HEIGHT: f32 = 180.0;

pub const TILE_SIZE: f32 = 8.0;

fn main() {
    const INITIAL_ZOOM: f32 = 6.0;
    let screen_width = V_WIDTH * INITIAL_ZOOM;
    let screen_height = V_HEIGHT * INITIAL_ZOOM;

    let mut camera = Camera2D {
        offset: Vector2::zero(),
        target: Vector2 {
            x: screen_width / 2.0,
            y: screen_height / 2.0,
        },
        rotation: 0.0,
        zoom: INITIAL_ZOOM,
    };

    let mut input_state = InputState::new();

    let mut map = TileMap::new(500, 500);

    let (mut rl, thread) = raylib::init()
        .size(V_WIDTH as i32 * 6, V_HEIGHT as i32 * 6)
        .title("Rust Raylib Starter")
        .build();

    let texture = rl.load_texture(&thread, "Tileset.png").unwrap();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        input_state.update(&mut rl, camera.zoom);

        if input_state.middle_currently_held {
            println!("INPUT HELD");
            camera.target -= input_state.delta / camera.zoom;
        }

        camera.zoom += input_state.middle_roll;
        camera.zoom = camera.zoom.clamp(1.0, 10.0);

        camera.offset = Vector2 {
            x: screen_width / 2.0,
            y: screen_height / 2.0,
        };

        map.update(dt);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);

        {
            let mut cam_handle = d.begin_mode2D(camera);
            map.draw(
                &mut cam_handle,
                &camera,
                screen_width,
                screen_height,
                &texture,
            );
        }
    }

    println!("Hello, world!");
}
