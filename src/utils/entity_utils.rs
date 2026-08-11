use crate::entities::entity_manager::CharacterEntry;

#[inline]
pub fn get_char_by_index(characters: &mut[CharacterEntry], idx: usize) -> &mut CharacterEntry {
    return characters
        .iter_mut()
        .find(|c| c.unique_id == idx)
        .unwrap();
}
