use raylib::math::Vector2;
use std::collections::HashMap;
use zander_game_core_rs::{raylib::sprite::Sprite, system::timer::Timer};

use crate::{
    GameContext,
    entities::{
        character::{Affiliation, CharacterData, CombatState},
        characters::enemies::slime::Slime,
        entity_manager::{BasicCharacterInfo, CharID},
    },
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
        all_chars_info: &HashMap<CharID, BasicCharacterInfo>,
    ) {
        self.enemy_data
            .new_opponent_check_timer
            .track(game_context.dt);
        // allows it to check on first frame of being spawned and find an opponent immediately
        // continues to check every second or so after
        if self.enemy_data.new_opponent_check_timer.is_done()
            || this_characters_data.current_opponent.is_none()
        {
            let next_opponent_id =
                Enemy::enemy_get_closest_target(this_characters_data.get_center(), all_chars_info);
            this_characters_data.select_new_opponent_and_enter_combat(next_opponent_id);
            self.enemy_data.new_opponent_check_timer.reset();
        }

        // available for enemies with special functionality
        match &mut self.enemy_kind {
            EnemyKind::Slime(..) => (),
        }
    }

    /// returns a bool when finished
    pub fn attack_update(
        &mut self,
        this_character_data: &mut CharacterData,
        self_id: CharID,
        target_id: CharID,
        game_context: &mut GameContext,
        all_chars_info: &HashMap<CharID, BasicCharacterInfo>,
    ) -> bool {
        match &mut self.enemy_kind {
            EnemyKind::Slime(_) => {
                this_character_data.basic_attack(self_id, target_id, game_context, all_chars_info);
                return true;
            }
        }
    }

    /// is only called when in a unique state
    pub fn current_sprite(&self, this_character_data: &CharacterData) -> Sprite {
        // if in unique state, then branch for unique logic
        if this_character_data.combat_state == CombatState::InAttack {
            match &self.enemy_kind {
                EnemyKind::Slime(_) => (),
            }
        }

        // if no unique behavior, just return idle
        return this_character_data
            .character_values
            .idle_anim
            .current_sprite();
    }

    /// this gets the closest target for an enemy of either a character or in the future a building
    /// this ideally can never fail because if all your characters are gone, you lost the game anyway
    /// and even if something is too far away, should still return a value so the caller can decide how to handle it
    pub fn enemy_get_closest_target(
        current_center_pos: Vector2,
        all_chars_info: &HashMap<CharID, BasicCharacterInfo>,
    ) -> CharID {
        let mut closest_id = CharID(0);
        let mut closest_distance = f32::MAX;

        for (id, char_info) in all_chars_info {
            if char_info.affiliation != Affiliation::Good {
                continue;
            }

            let distance = char_info.center_pos.distance_to(current_center_pos);

            if distance < closest_distance {
                closest_distance = distance;
                closest_id = *id;
            }
        }

        return closest_id;
    }
}
