use basic_raylib_core::graphics::sprite_animation::SpriteAnimationInstance;

pub struct TileMap {
    lake_tile_anim_instance: SpriteAnimationInstance,
    default_tile_anim_instance: SpriteAnimationInstance,
}

impl TileMap {
    pub fn update(&mut self) {
        // update lake tile anim and default tile anim
    }

    pub fn draw(&self) {
        // move tile renderer draw function to this struct
    }
}
