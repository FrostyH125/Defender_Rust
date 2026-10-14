use std::collections::HashMap;

use raylib::math::Vector2;
use zander_game_core_rs::{raylib::sprite::Sprite, system::timer::Timer};

use crate::{
    GameContext, entities::{
        character::{CharacterData, CharacterMovementResult, CharacterState, CombatState}, characters::enemies::slime::Slime, entity_manager::{BasicCharacterInfo, CharID},
    }, map::tile_map::TileMap, systems::character_action_manager::CharacterAction, utils::entity_utils::enemy_get_closest_target,
};

pub enum EnemyKind {
    Slime(Slime),
}

pub enum EnemyState {
    SelectingOpponent,
}

/// data specific to the Enemy type characters
pub struct EnemyData {
    pub enemy_state: EnemyState,
    pub new_opponent_check_timer: Timer,
}

impl EnemyData {
    pub fn new(new_opponent_check_interval: f32) -> Self {
        return Self {
            enemy_state: EnemyState::SelectingOpponent,
            new_opponent_check_timer: Timer::new(new_opponent_check_interval),
        };
    }
}

pub struct Enemy {
    pub enemy_data: EnemyData,
    pub enemy_kind: EnemyKind,
}

impl Enemy {
    pub fn update(
        &mut self,
        this_characters_data: &mut CharacterData,
        game_context: &mut GameContext,
        map: &mut TileMap,
        character_info: &HashMap<CharID, BasicCharacterInfo>,
    ) {
        match self.enemy_data.enemy_state {
            EnemyState::SelectingOpponent => {
                this_characters_data.current_opponent = Some(enemy_get_closest_target(
                    this_characters_data.pos,
                    character_info,
                ));
                this_characters_data.state = CharacterState::InCombat;
                this_characters_data.combat_state = CombatState::MovingToTarget;
            }
        };

        // available for enemies with special functionality
        match &mut self.enemy_kind {
            EnemyKind::Slime(..) => (),
        }
    }
}
