use basic_raylib_core::graphics::sprite::Sprite;
use raylib::math::Vector2;

use crate::entities::character::{Character, CharacterData};

pub struct Gatherer {
    pub data: CharacterData,
}

impl Gatherer {
    pub fn new(pos: Vector2) -> Character {
        todo!()
    }

    pub fn update(&mut self, dt: f32) {
        todo!()
    }

    pub fn sprite(&self) -> &Sprite {
        todo!()
    }
}