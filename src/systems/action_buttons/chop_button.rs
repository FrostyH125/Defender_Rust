use basic_raylib_core::graphics::sprite::Sprite;

use crate::{entities::entity_manager::CharacterEntry, map::tile_map::MapObjectGrid};

pub const CHOP_BUTTON_SPRITE: Sprite = Sprite::new(144, 40, 16, 16);

pub fn on_click(obj_ids: &[usize], char_ids: &[usize], object_grid: &mut MapObjectGrid, chars: &mut [CharacterEntry]) {
    todo!()
}
