use rand::{RngExt, rngs::ThreadRng};
use raylib::math::Vector2;
use zander_game_core_rs::{
    raylib::{
        animation_data::SpriteAnimationData, sprite::Sprite,
        sprite_animation::SpriteAnimationInstance,
    },
    system::timer::Timer,
};

use crate::{
    GameContext, TILE_SIZE,
    entities::object::{Object, ObjectData, ObjectKind, ObjectSpecificData, ObjectState},
    utils::{map_cord::MapCord, vector2_utils},
};

enum TreeVariant {
    One,
    Two,
}

static TREE_SPRITE_ONE: Sprite = Sprite::new(144, 24, 8, 16);

static TREE_FALL_ANIM_ONE: SpriteAnimationData = SpriteAnimationData {
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

static TREE_FALL_ANIM_TWO: SpriteAnimationData = SpriteAnimationData {
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
    out_of_hit_pos_timer: Timer
}

impl Tree {
    pub fn new(cord: MapCord, rng: &mut ThreadRng) -> Object {
        let variant = match rng.random_range(0..=1) {
            0 => TreeVariant::One,
            1 => TreeVariant::Two,
            _ => TreeVariant::One,
        };

        let anim = match variant {
            TreeVariant::One => &TREE_FALL_ANIM_ONE,
            TreeVariant::Two => &TREE_FALL_ANIM_TWO,
        };

        let object_specific_data = ObjectSpecificData {
            situational_draw_offset: Vector2::zero(),
            draw_offset: Vector2::new(0.0, -TILE_SIZE),
            width: 8.0,
            height: 16.0,
            disappear_timer: Timer::new(
                TREE_FALL_ANIM_ONE.frame_duration * TREE_FALL_ANIM_ONE.frames.len() as f32,
            ),
            health: 100.0,
            object_kind: ObjectKind::Tree,
        };

        let mut tree = Tree {
            data: ObjectData::new(
                cord.map_pos(),
                vector2_utils::random_offset_by_one(rng),
                cord,
                object_specific_data,
            ),
            variant,
            out_of_hit_pos_timer: Timer::new(0.1),
            falling_anim: SpriteAnimationInstance::new(anim),
        };

        if rng.random_bool(0.5) {
            tree.data.sprite_flip = true;
        }
        
        return Object::TreeObj(tree);
    }

    pub fn update(&mut self, game_context: &mut GameContext) {
        if let ObjectState::Breaking = self.data.state {
            // doesnt matter which one to use because update data is same
            self.falling_anim.update(game_context.dt);

            if self.data.sprite_flip {
                self.data.object_specific_data.situational_draw_offset.x = -8.0;
            } else {
                self.data.object_specific_data.situational_draw_offset.x = 0.0;
            }

            // returning because the out of hit pos timer would reset the draw offset
            // and none of that is necessary since those get overridden on breaking here
            return;
        }

        if self.out_of_hit_pos_timer.is_playing() {
            self.out_of_hit_pos_timer.track(game_context.dt);
            if self.out_of_hit_pos_timer.is_done() {
                self.data.object_specific_data.situational_draw_offset.x = 0.0;
            }
        }
    }

    pub fn on_hit(&mut self, rng: &mut ThreadRng) {
        self.data.object_specific_data.situational_draw_offset.x = match rng.random_bool(0.5) {
            true => 1.0,
            false => -1.0,
        };

        self.out_of_hit_pos_timer.reset();
        self.out_of_hit_pos_timer.set_playing();
    }

    pub fn sprite(&self) -> Sprite {
        return match self.data.state {
            ObjectState::Breaking => self.falling_anim.current_sprite(),
            _ => match self.variant {
                TreeVariant::One => TREE_SPRITE_ONE,
                TreeVariant::Two => TREE_SPRITE_TWO,
            },
        };
    }
}
