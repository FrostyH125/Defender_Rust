use basic_raylib_core::graphics::sprite::Sprite;
use raylib::math::Vector2;

use crate::{GameContext, entities::character::{Character, CharacterData, CharacterMovementResult}, map::tile_map::TileMap, utils::mouse_utils};

pub static GATHERER_SPRITE: Sprite = Sprite::new(16, 72, 8, 8);

pub struct Gatherer {
    pub data: CharacterData,
}

impl Gatherer {
    pub fn new(pos: Vector2) -> Character {
        let gatherer = Gatherer {
            data: CharacterData::new(pos, Vector2::zero(), 8.0, 8.0, 30.0),
        };

        return Character::GathererChar(gatherer);
    }

    pub fn update(&mut self, game_context: &mut GameContext, map: &TileMap) {

    }

    pub fn sprite(&self) -> &Sprite {
        return &GATHERER_SPRITE;
    }
}
