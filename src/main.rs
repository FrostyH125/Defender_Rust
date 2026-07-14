use raylib::{camera::Camera2D, color::Color, drawing::RaylibDraw, math::Vector2};

use crate::map::tile_map::TileMap;

pub mod map;
pub mod utils;

pub const V_WIDTH: f32 = 320.0;
pub const V_HEIGHT: f32 = 180.0;

pub const TILE_SIZE: f32 = 8.0;

fn main() {
    let camera = Camera2D {
        offset: Vector2::new(-V_WIDTH / 2.0, -V_HEIGHT as f32 / 2.0),
        target: Vector2::new(V_WIDTH / 2.0, V_HEIGHT as f32 / 2.0),
        rotation: 0.0,
        zoom: 1.0,
    };

    let mut map = TileMap::new(500, 500);

    let (mut rl, thread) = raylib::init()
        .size(V_WIDTH as i32, V_HEIGHT as i32)
        .title("Rust Raylib Starter")
        .build();

    let texture = rl.load_texture(&thread, "Tileset.png").unwrap();
    
    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();

        map.update(dt);

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::RAYWHITE);
        map.draw(&mut d, &camera, &texture);
    }

    println!("Hello, world!");
}
