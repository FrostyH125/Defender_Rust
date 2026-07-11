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