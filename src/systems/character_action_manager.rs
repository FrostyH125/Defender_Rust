use crate::{
    entities::{
        character::{CharacterKind, CharacterState, CombatState}, entity_manager::{CharID, CharacterEntry},
    }, utils::entity_utils::get_char_by_unique_id,
};

pub enum CharacterAction {
    Attack {
        attacker_id: CharID,
        target_id: CharID,
    },
    TryEngageInCombat {
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
                    let damage = attacker
                        .character
                        .character_data
                        .character_values
                        .attack_power;

                    let target = get_char_by_unique_id(chars, *target_id);
                    target.character.character_data.health -= damage;
                }
                CharacterAction::TryEngageInCombat {
                    attacker_id,
                    target_id,
                } => {
                    let target = &mut get_char_by_unique_id(chars, *target_id).character;

                    // early return and nothing done if character is dying
                    if target.character_data.state == CharacterState::Dying {
                        return;
                    }

                    if target.character_data.current_opponent.is_none() {
                        target.character_data.select_new_opponent_and_enter_combat(*attacker_id);
                    }

                    let attacker = &mut get_char_by_unique_id(chars, *attacker_id).character;

                    if attacker.character_data.current_opponent.is_none() {
                        attacker.character_data.select_new_opponent_and_enter_combat(*target_id);
                    }
                }
            }
        }

        // once all actions are taken care of, the list is cleared so the next frames actions can be stored
        self.actions.clear();
    }
}
