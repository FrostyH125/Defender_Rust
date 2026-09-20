use crate::entities::entity_manager::{CharID, CharacterEntry};

#[inline]
pub fn get_char_by_unique_id(characters: &mut[CharacterEntry], idx: CharID) -> &mut CharacterEntry {
    return characters
        .iter_mut()
        .find(|c| c.unique_id == idx)
        .unwrap();
}
