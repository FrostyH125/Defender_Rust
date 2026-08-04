use basic_raylib_core::graphics::sprite::Sprite;
use rand::rngs::ThreadRng;
use raylib::math::Vector2;

use crate::{
    TILE_SIZE, entities::object::{Object, ObjectData, ObjectState}, map::{map_cell::MapCell, tile_map::MapDimensions}, utils::{map_cord::MapCord, vector2_utils},
};

static TREE_SPRITE: Sprite = Sprite::new(144, 24, 8, 16);

pub struct Tree {
    pub data: ObjectData,
}

impl Tree {
    pub fn new(
        cord: MapCord,
        rng: &mut ThreadRng,
        map_dimensions: MapDimensions,
        cells: &mut Vec<MapCell>,
    ) -> Object {
        let tree = Tree {
            data: ObjectData::new(
                cord.map_pos(),
                Vector2::new(0.0, -TILE_SIZE),
                vector2_utils::random_offset_by_one(rng),
                cells,
                map_dimensions,
                8.0,
                16.0,
                100.0,
                0.1,
                1.0
            ),
        };

        return Object::TreeObj(tree);
    }

    pub fn update(&mut self, dt: f32) {
        if let ObjectState::GettingHit = self.data.state {
            self.data.situational_draw_offset.x = 1.0;
        } else {
            self.data.situational_draw_offset.x = 0.0;
        }
    }

    pub fn sprite(&self) -> &Sprite {
        return &TREE_SPRITE;
    }
}
