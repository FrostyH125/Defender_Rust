use rand::{RngExt, rngs::ThreadRng};
use raylib::math::Vector2;

use crate::{TILE_SIZE, utils::map_cord::MapCord};

pub fn random_offset_by_one(rng: &mut ThreadRng) -> Vector2 {
    let x = rng.random_range(-1..=1) as f32;
    let y = rng.random_range(-1..=1) as f32;

    return Vector2::new(x, y);
}

pub fn v2_to_cord(v: Vector2) -> MapCord {
    let x = v.x as i16 / TILE_SIZE as i16;
    let y = v.y as i16 / TILE_SIZE as i16;

    return MapCord::new(x, y);
}

pub fn cord_to_v2(c: MapCord) -> Vector2 {
    let x = c.x as f32 * TILE_SIZE;
    let y = c.y as f32 * TILE_SIZE;

    return Vector2::new(x, y);
}