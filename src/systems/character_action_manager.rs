use crate::{
    entities::{character::CharacterState, entity_manager::{CharID, CharacterEntry}}, utils::entity_utils::get_char_by_unique_id,
};

pub enum CharacterAction {
    Attack {
        attacker_id: CharID,
        target_id: CharID,
    },
    EngageInCombat {
        attacker_id: CharID,
        target_id: CharID,
    },
}

/// Handles storing and resolving the actions requested by characters.
/// This prevents characters from needing direct mutable access to eachother during the update loop
/// it also prevents characters from needing to independently contain the business logic for doing so
pub struct CharacterActionManager {
    actions: Vec<CharacterAction>,
}

impl CharacterActionManager {
    pub fn new() -> Self {
        return Self {
            actions: Vec::new(),
        };
    }

    pub fn push_action(&mut self, action: CharacterAction) {
        self.actions.push(action);
    }

    pub fn resolve_actions(&mut self, chars: &mut [CharacterEntry]) {
        for action in &mut self.actions {
            match action {
                CharacterAction::Attack {
                    attacker_id,
                    target_id,
                } => {
                    let attacker = get_char_by_unique_id(chars, *attacker_id);
                    let damage = attacker.character.character_data.character_values.attack_power;

                    let target = get_char_by_unique_id(chars, *target_id);
                    target.character.character_data.health -= damage;
                }
                CharacterAction::EngageInCombat {
                    attacker_id,
                    target_id,
                } => {

                    let target = &mut get_char_by_unique_id(chars, *target_id).character;

                    // early return and nothing done if character is dying
                    if target.character_data.health <= 0.0 {
                        return;
                    }
                    
                    target.character_data.opponents.push(*attacker_id);
                    target.character_data.state = CharacterState::InCombat;
                    
                    let attacker = &mut get_char_by_unique_id(chars, *attacker_id).character;

                    attacker.character_data.opponents.push(*target_id);
                    attacker.character_data.state = CharacterState::InCombat;

                }
            }
        }

        // once all actions are taken care of, the list is cleared so the next frames actions can be stored
        self.actions.clear();
    }
}
