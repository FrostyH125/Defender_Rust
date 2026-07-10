#[derive(Copy, Clone)]

#[derive(PartialEq, Eq)]
pub enum TileType {
    Grass,
    Lake,
    River
}

pub struct LakeSpriteData {
    base_animation_index: u8,
    shore_animation_index: u8,
    corner_animation_index: u8
}