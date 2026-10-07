use std::collections::HashMap;

use raylib::math::Vector2;
use zander_game_core_rs::raylib::sprite::Sprite;

use crate::{GameContext, entities::{character::CharacterData, characters::enemies::slime::{Slime, SlimeState}, entity_manager::{CharID, BasicCharacterInfo}}, map::tile_map::TileMap};

pub enum EnemyKind {
    Slime(Slime)
}

/// data specific to the Enemy type characters
pub struct EnemyData {

}

pub struct Enemy {
    pub enemy_data: EnemyData,
    pub enemy_kind: EnemyKind
}

impl Enemy {
    pub fn set_idle(&mut self) {
        match &mut self.enemy_kind {
            EnemyKind::Slime(slime) => slime.slime_state = SlimeState::Idle,
        }
    }

    pub fn is_idle(&self) -> bool {
        match &self.enemy_kind {
            EnemyKind::Slime(slime) => return slime.slime_state == SlimeState::Idle,
        }
    }

    pub fn update(&mut self, game_context: &mut GameContext, map: &mut TileMap, character_info: &HashMap<CharID, BasicCharacterInfo>) {
        match &mut self.enemy_kind {
            EnemyKind::Slime(slime) => slime.update(character_info),
        }
    }

    pub fn current_sprite(&self, character_data: &CharacterData) -> Sprite {
        match &self.enemy_kind {
            EnemyKind::Slime(slime) => slime.current_sprite(character_data),
        }
    }
}
