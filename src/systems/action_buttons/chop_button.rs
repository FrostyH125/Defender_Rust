use basic_raylib_core::graphics::sprite::Sprite;

use crate::{entities::{character::Character, characters::gatherer::{GatherTarget, GathererState}, entity_manager::CharacterEntry, object::Object::{self}}, map::tile_map::MapObjectGrid, utils::entity_utils::get_char_by_index};

pub const CHOP_BUTTON_SPRITE: Sprite = Sprite::new(144, 40, 16, 16);

pub fn on_click(obj_ids: &[usize], char_ids: &[usize], object_grid: &mut MapObjectGrid, chars: &mut [CharacterEntry]) {
    for obj_id in obj_ids {
        let obj = &mut object_grid[*obj_id];

        if let Object::TreeObj(tree) = obj {
            tree.data.is_marked_for_gathering = true;            
        } 
        
    }

    for char_id in char_ids {
        let char = &mut get_char_by_index(chars, *char_id).character;

        if let Character::GathererChar(gatherer) = char {
            gatherer.state = GathererState::LookingForObject(GatherTarget::Tree);
        }
    }
}
