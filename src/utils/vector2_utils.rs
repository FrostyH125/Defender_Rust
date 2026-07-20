use rand::{RngExt, rngs::ThreadRng};
use raylib::math::Vector2;

pub fn random_offset_by_one(rng: &mut ThreadRng) -> Vector2 {
    let x = rng.random_range(-1..=1) as f32;
    let y = rng.random_range(-1..=1) as f32;

    return Vector2::new(x, y);
}