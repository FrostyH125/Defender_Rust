use std::collections::HashMap;

use raylib::math::Vector2;

use crate::entities::{character::Affiliation, characters::gatherer::GatherTarget, entity_manager::{BasicCharacterInfo, CharID, CharacterEntry}, object::ObjectKind};

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
pub fn object_matches_gathering_target(gather_target: GatherTarget, obj: &ObjectKind) -> bool {
    return match gather_target {
        GatherTarget::Tree => match obj {
            ObjectKind::TreeObj(..) => true,
            _ => false
        },
        GatherTarget::Grass => match obj {
            ObjectKind::GrassObj(..) => true,
            _ => false
        },
    };
}

/// this gets the closest target for an enemy of either a character or in the future a building
/// this ideally can never fail because if all your characters are gone, you lost the game anyway
/// and even if something is too far away, should still return a value so the caller can decide how to handle it
pub fn enemy_get_closest_target(current_pos: Vector2, character_info: &HashMap<CharID, BasicCharacterInfo>, target_affiliation: Affiliation) -> CharID {

    let mut closest_id = CharID(0);
    let mut closest_distance = f32::MAX;
    
    for (id, char_info) in character_info {
        if char_info.affiliation != target_affiliation {
            continue;
        }
        
        let distance = char_info.position.distance_to(current_pos);

        if distance < closest_distance {
            closest_distance = distance;
            closest_id = *id;
        }
    }

    return closest_id;
}
