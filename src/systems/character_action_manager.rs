use crate::{entities::entity_manager::CharacterEntry, utils::entity_utils::get_char_by_index};

pub enum CharacterAction {
    Attack {
        // even though its currently unused, attacker ID could eventually be used very easily, so im going to keep it for now
        attacker_id: usize,
        target_id: usize,
        damage: f32
    }
}

/// Handles storing and resolving the actions requested by characters.
/// This prevents characters from needing direct mutable handles to eachother during the update loop
pub struct CharacterActionManager {
    actions: Vec<CharacterAction>
}

impl CharacterActionManager {
    pub fn new() -> Self {
        return Self {
            actions: Vec::new()
        }
    }

    pub fn resolve_actions(&mut self, chars: &mut [CharacterEntry]) {
        for action in &mut self.actions {
            match action {
                CharacterAction::Attack { attacker_id, target_id, damage } => {
                    let attacker = get_char_by_index(chars, *attacker_id);
                    let target = get_char_by_index(chars, *target_id);

                    todo!("target.take_damage(damage)")
                },
            }
        }

        // once all actions are taken care of, the list is cleared so the next frames actions can be stored
        self.actions.clear();
    }
}