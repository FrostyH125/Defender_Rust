use std::collections::HashMap;
use zander_game_core_rs::system::timer::Timer;

use crate::{
    GameContext,
    entities::{
        character::{CharacterData, CharacterState, CombatState},
        characters::enemies::slime::Slime,
        entity_manager::{BasicCharacterInfo, CharID},
    },
    map::tile_map::TileMap,
    utils::entity_utils::enemy_get_closest_target,
};

pub enum EnemyKind {
    Slime(Slime),
}

/// data specific to the Enemy type characters
pub struct EnemyData {
    pub new_opponent_check_timer: Timer,
}

impl EnemyData {
    pub fn new(new_opponent_check_interval: f32) -> Self {
        return Self {
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
        // allows it to check on first frame of being spawned and find an opponent immediately
        // continues to check every second or so after
        // otherwise it just checks no matter what if theres no opponent
        if let None = this_characters_data.current_opponent {
            self.select_new_opponent(this_characters_data, character_info);
            self.enemy_data.new_opponent_check_timer.reset();
        } else {
            self.enemy_data.new_opponent_check_timer.track(game_context.dt);
            
            if self.enemy_data.new_opponent_check_timer.is_done() {
                self.select_new_opponent(this_characters_data, character_info);
                self.enemy_data.new_opponent_check_timer.reset();
            }
        }

        // available for enemies with special functionality
        match &mut self.enemy_kind {
            EnemyKind::Slime(..) => (),
        }
    }

    pub fn select_new_opponent(
        &mut self,
        this_characters_data: &mut CharacterData,
        character_info: &HashMap<CharID, BasicCharacterInfo>,
    ) {
        this_characters_data.current_opponent = Some(enemy_get_closest_target(
            this_characters_data.pos,
            character_info,
        ));
        this_characters_data.state = CharacterState::InCombat;
        this_characters_data.combat_state = CombatState::MovingToTarget;

        // available for enemies with special functionality
        match &mut self.enemy_kind {
            EnemyKind::Slime(..) => (),
        }
    }
}
