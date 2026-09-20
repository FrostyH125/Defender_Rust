use std::collections::HashMap;

use zander_game_core_rs::raylib::sprite::Sprite;

use crate::{GameContext, entities::{character::CharacterData, characters::enemies::slime::{Slime, SlimeState}, entity_manager::{CharID, CharacterInfo}}, map::tile_map::TileMap};

pub enum Enemy {
    Slime(Slime)
}

/// data specific to the Enemy type characters
pub struct EnemyData {

}

impl Enemy {
    pub fn get_data(&self) -> &CharacterData {
        match self {
            Enemy::Slime(slime) => &slime.ch_data
        }
    }

    pub fn get_mut_data(&mut self) -> &mut CharacterData {
        match self {
            Enemy::Slime(slime) => &mut slime.ch_data,
        }
    }

    pub fn set_idle(&mut self) {
        match self {
            Enemy::Slime(slime) => slime.slime_state = SlimeState::Idle,
        }
    }

    pub fn update(&mut self, game_context: &mut GameContext, map: &mut TileMap, character_info: &HashMap<CharID, CharacterInfo>) {
        match self {
            Enemy::Slime(slime) => slime.update(),
        }
    }

    pub fn current_sprite(&self) -> Sprite {
        match self {
            Enemy::Slime(slime) => slime.current_sprite(),
        }
    }
}
