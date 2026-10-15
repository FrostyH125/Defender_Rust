use rand::{RngExt, rngs::ThreadRng};
use raylib::math::Vector2;

pub fn random_offset_by_one(rng: &mut ThreadRng) -> Vector2 {
    let x = rng.random_range(-1..=1) as f32;
    let y = rng.random_range(-1..=1) as f32;
    return Vector2::new(x, y);
}

pub fn random_offset_in_range(rng: &mut ThreadRng, range: f32) -> Vector2 {
    let x_offset = rng.random_range(-range..=range);
    let y_offset = rng.random_range(-range..=range);
    return Vector2::new(x_offset, y_offset);
}
