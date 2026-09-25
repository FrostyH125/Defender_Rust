use crate::entities::{characters::gatherer::GatherTarget, entity_manager::{CharID, CharacterEntry}, object::Object};

#[inline]
pub fn get_char_by_unique_id(characters: &mut[CharacterEntry], idx: CharID) -> &mut CharacterEntry {
    return characters
        .iter_mut()
        .find(|c| c.unique_id == idx)
        .unwrap();
}

pub fn object_matches_gathering_target(gather_target: GatherTarget, obj: &Object) -> bool {
    return match (gather_target, obj) {
        (GatherTarget::Tree, Object::TreeObj(..)) => true,
        (GatherTarget::Grass, Object::GrassObj(..)) => true,
        _ => false
    };
}
