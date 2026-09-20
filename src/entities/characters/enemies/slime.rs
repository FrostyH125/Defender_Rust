use zander_game_core_rs::raylib::sprite::Sprite;

use crate::entities::{character::CharacterData, characters::enemy::EnemyData};

pub enum SlimeState {
    Idle,
}

pub struct Slime {
    pub ch_data: CharacterData,
    enemy_data: EnemyData,
    pub slime_state: SlimeState,
}

impl Slime {
    pub fn update(&mut self) {}
    pub fn current_sprite(&self) -> Sprite {
        todo!()
    }
}
