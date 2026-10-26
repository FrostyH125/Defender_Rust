// very very similar to enemy. actually, almost identical, except the way the find opponent function
// will work is that itll look within a detection_range and see if any enemy is there, if is, find closest one, and walk to it

use std::collections::HashMap;

use zander_game_core_rs::raylib::sprite::Sprite;

use crate::{
    GameContext,
    entities::{
        character::{CharacterData, CombatState},
        entity_manager::{BasicCharacterInfo, CharID},
    },
};

pub enum FighterKind {}

pub struct FighterData {
    detection_range: f32,
}

pub struct Fighter {
    fighter_kind: FighterKind,
    fighter_data: FighterData,
}

impl Fighter {
    pub fn update(
        &mut self,
        this_characters_data: &mut CharacterData,
        game_context: &mut GameContext,
        character_info: &HashMap<CharID, BasicCharacterInfo>,
    ) {
        // try to select a new opponent continuously until it finds one
        // this update method (as well as the one for enemy) will run
        // regardless of the character state of this character because
        // fighting class units (including enemies) behave a little differently
        // as their main game loop is actually integrated into the character state
        // due to other units being eligible for fighting capabilities, unlike
        // say gatherers who have their whole gathering loop just when theyre in the
        // CharacterState::None state, and that should not be intertwined with
        // combat and moving due to user input, so rightfully thats all separated
        // for them.
        //
        // if you ever find this too limiting, look at the bottom of this file
        // for ways to add unique functionality
        if this_characters_data.current_opponent.is_none() {
            let next_opponent = Fighter::try_select_new_opponent(
                &self.fighter_data,
                this_characters_data,
                character_info,
            );

            // the moment a character gets a new opponent, it enters combat
            // maybe the function name isnt obvious for that,
            if let Some(id) = next_opponent {
                this_characters_data.select_new_opponent_and_enter_combat(id);
            }
        }
    }

    pub fn try_select_new_opponent(
        fighter_data: &FighterData,
        this_characters_data: &mut CharacterData,
        character_info: &HashMap<CharID, BasicCharacterInfo>,
    ) -> Option<CharID> {
        let mut closest: Option<CharID> = None;
        let mut closest_distance: f32 = f32::MAX;

        for (id, ch_info) in character_info {
            let dist_to_char = ch_info.position.distance_to(this_characters_data.pos);

            if dist_to_char > fighter_data.detection_range {
                continue;
            }

            if dist_to_char > closest_distance {
                continue;
            }

            closest = Some(*id);
            closest_distance = dist_to_char;
        }

        return closest;
    }

    pub fn current_sprite(&self, this_character_data: &CharacterData) -> Sprite {
        // if in unique state, then branch for unique logic
        if this_character_data.combat_state == CombatState::InAttack {
            match self.fighter_kind {
                _ => (),
            }
        }

        // if no unique behavior, just return idle
        return this_character_data
            .character_values
            .idle_anim
            .current_sprite();
    }

    /// the actual place where any attack specific actions should happen
    /// this is where fighter specific attack logic should happen
    /// returns a bool when finished
    pub fn attack_update(
        &mut self,
        this_character_data: &mut CharacterData,
        self_id: CharID,
        target_id: CharID,
        game_context: &mut GameContext,
        character_info: &HashMap<CharID, BasicCharacterInfo>,
    ) -> bool { 
        match self.fighter_kind {
            // Fighter::Fighter1(f1) => f1.do_attack()
        }
    }
}




    // on_being_hit function() for unique functionality on being hit, can implement timer and
    // unique animations

    // on_hitting_opponent function() for unique functionality on hitting the opponent, can implement
    // timer and unique animations

    // on_opponent_kill function() for unique functionality,
    // which can implement a timer to do special animations and logic while opponent.is_none()