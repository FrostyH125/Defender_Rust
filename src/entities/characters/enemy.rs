use std::collections::HashMap;

use raylib::math::Vector2;
use zander_game_core_rs::{raylib::sprite::Sprite, system::timer::Timer};

use crate::{
    GameContext,
    entities::{
        character::{CharacterData, CharacterMovementResult},
        characters::{
            enemies::slime::{Slime, SlimeState},
            enemy::EnemyState::SelectingOpponent,
        },
        entity_manager::{BasicCharacterInfo, CharID},
    },
    map::tile_map::TileMap,
    systems::character_action_manager::CharacterAction,
    utils::entity_utils::enemy_get_closest_target,
};

pub enum EnemyKind {
    Slime(Slime),
}

pub enum EnemyState {
    SelectingOpponent,
    Moving { target_pos: Vector2 },
    Attacking,
}

/// data specific to the Enemy type characters
pub struct EnemyData {
    pub enemy_state: EnemyState,
    pub new_opponent_check_timer: Timer,
    pub range_of_attack: f32,
}

impl EnemyData {
    pub fn new(new_opponent_check_interval: f32, attack_range: f32) -> Self {
        return Self {
            enemy_state: EnemyState::SelectingOpponent,
            new_opponent_check_timer: Timer::new(new_opponent_check_interval),
            range_of_attack: attack_range,
        };
    }
}

pub struct Enemy {
    pub enemy_data: EnemyData,
    pub enemy_kind: EnemyKind,
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
                self.enemy_data.enemy_state = EnemyState::Moving {
                    target_pos: character_info
                        .get(&this_characters_data.current_opponent.unwrap())
                        .unwrap()
                        .position,
                };
            }
            EnemyState::Moving { target_pos } => {
                this_characters_data
                    .character_values
                    .move_anim
                    .update(game_context.dt);

                // check every so often if new opponent is closer
                self.enemy_data
                    .new_opponent_check_timer
                    .track(game_context.dt);
                if self.enemy_data.new_opponent_check_timer.is_done() {
                    self.enemy_data.new_opponent_check_timer.reset();
                    self.enemy_data.enemy_state = EnemyState::SelectingOpponent;
                }

                match this_characters_data.move_to(target_pos, game_context, map) {
                    CharacterMovementResult::Success | CharacterMovementResult::NotArrivedYet => {
                        if target_pos.distance_to(this_characters_data.pos)
                            <= self.enemy_data.range_of_attack
                        {
                            self.enemy_data.enemy_state = EnemyState::Attacking;
                        }
                    }
                    CharacterMovementResult::NoRoute | CharacterMovementResult::TooLong => {
                        todo!("make enemy disappear,  its clearly having some sort of issue")
                    }
                }
            }
            EnemyState::Attacking => {
                game_context.character_action_manager.push_action(
                    CharacterAction::EngageInCombat {
                        attacker_id: this_characters_data.unique_char_id,
                        target_id: this_characters_data.current_opponent.unwrap(),
                    },
                );
            }
        }

        // available for enemies with special functionality
        match &mut self.enemy_kind {
            EnemyKind::Slime(..) => (),
        }
    }

    pub fn current_sprite(&self, character_data: &CharacterData) -> Sprite {
        match &self.enemy_kind {
            EnemyKind::Slime(slime) => slime.current_sprite(character_data),
        }
    }
}
