use basic_raylib_core::graphics::{
    animation_data::AnimationData, sprite::Sprite, sprite_animation::SpriteAnimationInstance,
};
use rand::{RngExt, rngs::ThreadRng};
use raylib::math::Vector2;

use crate::{
    GameContext, TILE_SIZE,
    entities::object::{Object, ObjectData, ObjectState},
    map::{map_cell::MapCell, tile_map::MapDimensions},
    utils::{map_cord::MapCord, vector2_utils},
};

enum TreeVariant {
    One,
    Two,
}

static TREE_SPRITE_ONE: Sprite = Sprite::new(144, 24, 8, 16);

static TREE_FALL_ANIM_ONE: AnimationData = AnimationData {
    frames: &[
        Sprite::new(152, 24, 16, 16),
        Sprite::new(168, 24, 16, 16),
        Sprite::new(184, 24, 16, 16),
        Sprite::new(200, 24, 16, 16),
        Sprite::new(200, 24, 16, 16),
        Sprite::new(200, 24, 16, 16),
        Sprite::new(200, 24, 16, 16),
        Sprite::new(200, 24, 16, 16),
    ],
    frame_duration: 0.25,
    should_loop: false,
};

static TREE_SPRITE_TWO: Sprite = Sprite::new(144, 152, 8, 16);

static TREE_FALL_ANIM_TWO: AnimationData = AnimationData {
    frames: &[
        Sprite::new(152, 152, 16, 16),
        Sprite::new(168, 152, 16, 16),
        Sprite::new(184, 152, 16, 16),
        Sprite::new(200, 152, 16, 16),
        Sprite::new(200, 152, 16, 16),
        Sprite::new(200, 152, 16, 16),
        Sprite::new(200, 152, 16, 16),
        Sprite::new(200, 152, 16, 16),
    ],
    frame_duration: 0.25,
    should_loop: false,
};

pub struct Tree {
    pub data: ObjectData,
    falling_anim: SpriteAnimationInstance,
    variant: TreeVariant,
    has_set_offset: bool,
}

impl Tree {
    pub fn new(
        cord: MapCord,
        rng: &mut ThreadRng,
        map_dimensions: MapDimensions,
        cells: &mut Vec<MapCell>,
    ) -> Object {
        let mut tree = Tree {
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
                TREE_FALL_ANIM_ONE.frame_duration
                    * TREE_FALL_ANIM_ONE.frames.len() as f32,
            ),
            has_set_offset: false,
            falling_anim: SpriteAnimationInstance::new(),
            variant: match rng.random_range(0..=1) {
                0 => TreeVariant::One,
                1 => TreeVariant::Two,
                _ => TreeVariant::One,
            }
        };

        if rng.random_bool(0.5) {
            tree.data.sprite_flip = true;
        }

        return Object::TreeObj(tree);
    }

    pub fn update(&mut self, game_context: &mut GameContext) {
        if let ObjectState::GettingHit = self.data.state {
            if !self.has_set_offset {
                self.data.situational_draw_offset.x = match game_context.rng.random_bool(0.5) {
                    true => 1.0,
                    false => -1.0,
                };
                self.has_set_offset = true;
            }
        } else {
            self.data.situational_draw_offset.x = 0.0;
            self.has_set_offset = false;
        }

        if let ObjectState::Breaking = self.data.state {
            // doesnt matter which one to use because update data is same
            TREE_FALL_ANIM_ONE.update(&mut self.falling_anim, game_context.dt);
            
            if self.data.sprite_flip {
                self.data.situational_draw_offset.x = -8.0;
            }
        }
    }

    pub fn sprite(&self) -> Sprite {
        return match self.data.state {
            ObjectState::Breaking => match self.variant {
                TreeVariant::One => TREE_FALL_ANIM_ONE.frames[self.falling_anim.current_frame_index as usize],
                TreeVariant::Two => TREE_FALL_ANIM_TWO.frames[self.falling_anim.current_frame_index as usize],
            }
            _ => match self.variant {
                TreeVariant::One => TREE_SPRITE_ONE,
                TreeVariant::Two => TREE_SPRITE_TWO,
            },
        };
    }
}
