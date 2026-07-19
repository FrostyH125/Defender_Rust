use crate::map::tile_map_animation_data::RiverType;

#[derive(Copy, Clone)]

#[derive(PartialEq, Eq)]
pub enum TileType {
    Grass,
    Lake,
    River
}

pub struct LakeSpriteData {
    pub shore_animation_index: u8,
    pub corner_animation_index: u8
}

pub struct RiverSpriteData {
    pub river_type: RiverType,
    pub river_sprite_index: u8
}