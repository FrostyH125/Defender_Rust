use crate::{
    entities::{character::CharacterState, entity_manager::CharacterEntry},
    utils::entity_utils::get_char_by_index,
};

pub enum CharacterAction {
    Attack {
        // even though its currently unused, attacker ID could eventually be used very easily, so im going to keep it for now
        attacker_id: usize,
        target_id: usize,
        damage: f32,
    },
    EngageInCombat {
        attacker_id: usize,
        target_id: usize,
    },
}

/// Handles storing and resolving the actions requested by characters.
/// This prevents characters from needing direct mutable handles to eachother during the update loop
pub struct CharacterActionManager {
    actions: Vec<CharacterAction>,
}

impl CharacterActionManager {
    pub fn new() -> Self {
        return Self {
            actions: Vec::new(),
        };
    }

    pub fn request_attack(&mut self, attacker_id: usize, target_id: usize, damage: f32) {
        self.actions.push(CharacterAction::Attack {
            attacker_id,
            target_id,
            damage,
        });
    }

    pub fn request_engage_in_combat(&mut self, attacker_id: usize, target_id: usize) {
        self.actions.push(CharacterAction::EngageInCombat {
            attacker_id,
            target_id,
        });
    }

    pub fn resolve_actions(&mut self, chars: &mut [CharacterEntry]) {
        for action in &mut self.actions {
            match action {
                CharacterAction::Attack {
                    attacker_id,
                    target_id,
                    damage,
                } => {
                    let attacker = get_char_by_index(chars, *attacker_id);
                    let target = get_char_by_index(chars, *target_id);

                    todo!("target.take_damage(damage)")
                }
                CharacterAction::EngageInCombat {
                    attacker_id,
                    target_id,
                } => {
                    let attacker = &mut get_char_by_index(chars, *attacker_id).character;

                    // eventually you may encounter a bug where characters who are currently dying are added and then its not valid anymore or something
                    // either fix that here, or fix it when the character themselves go to resolve their state from incombat and check if the character is alive or exists
                    // or have the function get_char_by_index return an Optional, (probably the best approach)

                    attacker.get_mut_data().opponents.push(*target_id);
                    attacker.get_mut_data().state = CharacterState::InCombat;

                    let target = &mut get_char_by_index(chars, *target_id).character;

                    target.get_mut_data().opponents.push(*attacker_id);
                    target.get_mut_data().state = CharacterState::InCombat;
                }
            }
        }

        // once all actions are taken care of, the list is cleared so the next frames actions can be stored
        self.actions.clear();
    }
}
