use crate::entities::{characters::gatherer::GatherTarget, entity_manager::{CharID, CharacterEntry}, object::Object};

#[inline]
pub fn get_char_by_unique_id(characters: &mut[CharacterEntry], idx: CharID) -> &mut CharacterEntry {
    return characters
        .iter_mut()
        .find(|c| c.unique_id == idx)
        .unwrap();
}

// for those reading this
// originally i had something like this
// return match (gather_target, obj) {
//    (GatherTarget::Tree, Object::TreeObj(..)) => true,
//    (GatherTarget::Grass, Object::GrassObj(..)) => true,
//    // etc etc
//    _ => false
//};
// 
// however i wanted the compiler to warn me when i add a new gather target to the game to update this
// so i opted for a slightly messier function with exhaustive enum check on gather target
pub fn object_matches_gathering_target(gather_target: GatherTarget, obj: &Object) -> bool {
    return match gather_target {
        GatherTarget::Tree => match obj {
            Object::TreeObj(..) => true,
            _ => false
        },
        GatherTarget::Grass => match obj {
            Object::GrassObj(..) => true,
            _ => false
        },
    };
}
