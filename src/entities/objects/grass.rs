use basic_raylib_core::graphics::{
    animation_data::AnimationData, sprite::Sprite, sprite_animation::SpriteAnimationInstance,
};
use rand::{RngExt, rngs::ThreadRng};
use raylib::{drawing::RaylibDrawHandle, math::Vector2, rgui::RaylibDrawGui, texture::Texture2D};

use crate::{
    entities::{
        object::{Object, ObjectData},
        objects::grass::GrassType::Wheaty,
    },
    utils::{draw_utils, map_cord::MapCord, vector2_utils},
};

const SMALL_GRASS_HEIGHT: i32 = 8;
const TALL_GRASS_HEIGHT: i32 = 16;
const GRASS_WIDTH: i32 = 8;
const GRASS_ANIM_SPEED: f32 = 0.2;
const MINIMUM_LEVEL_UP_TIME: f32 = 80.0;
const MAXIMUM_LEVEL_UP_TIME: f32 = 120.0;

static WHEATY_GRASS_ANIMS: [AnimationData; 3] = [
    AnimationData {
        frames: &[
            Sprite::new(0, 8, GRASS_WIDTH, SMALL_GRASS_HEIGHT),
            Sprite::new(8, 8, GRASS_WIDTH, SMALL_GRASS_HEIGHT),
        ],
        frame_duration: GRASS_ANIM_SPEED,
        should_loop: true,
    },
    AnimationData {
        frames: &[
            Sprite::new(0, 16, GRASS_WIDTH, SMALL_GRASS_HEIGHT),
            Sprite::new(8, 16, GRASS_WIDTH, SMALL_GRASS_HEIGHT),
        ],
        frame_duration: GRASS_ANIM_SPEED,
        should_loop: true,
    },
    AnimationData {
        frames: &[
            Sprite::new(0, 24, GRASS_WIDTH, TALL_GRASS_HEIGHT),
            Sprite::new(8, 24, GRASS_WIDTH, TALL_GRASS_HEIGHT),
        ],
        frame_duration: GRASS_ANIM_SPEED,
        should_loop: true,
    },
];

static FLOWERY_GRASS_ANIMS: [AnimationData; 3] = [
    AnimationData {
        frames: &[
            Sprite::new(0, 40, GRASS_WIDTH, SMALL_GRASS_HEIGHT),
            Sprite::new(8, 40, GRASS_WIDTH, SMALL_GRASS_HEIGHT),
        ],
        frame_duration: GRASS_ANIM_SPEED,
        should_loop: true,
    },
    AnimationData {
        frames: &[
            Sprite::new(0, 48, GRASS_WIDTH, SMALL_GRASS_HEIGHT),
            Sprite::new(8, 48, GRASS_WIDTH, SMALL_GRASS_HEIGHT),
        ],
        frame_duration: GRASS_ANIM_SPEED,
        should_loop: true,
    },
    AnimationData {
        frames: &[
            Sprite::new(0, 56, GRASS_WIDTH, TALL_GRASS_HEIGHT),
            Sprite::new(8, 56, GRASS_WIDTH, TALL_GRASS_HEIGHT),
        ],
        frame_duration: GRASS_ANIM_SPEED,
        should_loop: true,
    },
];

static BUSHY_GRASS_ANIMS: [AnimationData; 3] = [
    AnimationData {
        frames: &[
            Sprite::new(0, 72, GRASS_WIDTH, SMALL_GRASS_HEIGHT),
            Sprite::new(8, 72, GRASS_WIDTH, SMALL_GRASS_HEIGHT),
        ],
        frame_duration: GRASS_ANIM_SPEED,
        should_loop: true,
    },
    AnimationData {
        frames: &[
            Sprite::new(0, 80, GRASS_WIDTH, SMALL_GRASS_HEIGHT),
            Sprite::new(8, 80, GRASS_WIDTH, SMALL_GRASS_HEIGHT),
        ],
        frame_duration: GRASS_ANIM_SPEED,
        should_loop: true,
    },
    AnimationData {
        frames: &[
            Sprite::new(0, 88, GRASS_WIDTH, TALL_GRASS_HEIGHT),
            Sprite::new(8, 88, GRASS_WIDTH, TALL_GRASS_HEIGHT),
        ],
        frame_duration: GRASS_ANIM_SPEED,
        should_loop: true,
    },
];

static STALKY_GRASS_ANIMS: [AnimationData; 3] = [
    AnimationData {
        frames: &[
            Sprite::new(0, 104, GRASS_WIDTH, SMALL_GRASS_HEIGHT),
            Sprite::new(8, 104, GRASS_WIDTH, SMALL_GRASS_HEIGHT),
        ],
        frame_duration: GRASS_ANIM_SPEED,
        should_loop: true,
    },
    AnimationData {
        frames: &[
            Sprite::new(0, 112, GRASS_WIDTH, SMALL_GRASS_HEIGHT),
            Sprite::new(8, 112, GRASS_WIDTH, SMALL_GRASS_HEIGHT),
        ],
        frame_duration: GRASS_ANIM_SPEED,
        should_loop: true,
    },
    AnimationData {
        frames: &[
            Sprite::new(0, 120, GRASS_WIDTH, TALL_GRASS_HEIGHT),
            Sprite::new(8, 120, GRASS_WIDTH, TALL_GRASS_HEIGHT),
        ],
        frame_duration: GRASS_ANIM_SPEED,
        should_loop: true,
    },
];

enum GrassType {
    Wheaty,
    Flowery,
    Bushy,
    Stalky,
}

impl GrassType {
    pub fn random_type(rng: &mut ThreadRng) -> Self {
        let k = rng.random_range(0..=3);
        return match k {
            0 => Self::Wheaty,
            1 => Self::Flowery,
            2 => Self::Bushy,
            3 => Self::Stalky,
            _ => panic!("only 0..=3 allowed for GrassType::random_type()"),
        };
    }
}

pub struct Grass {
    pub data: ObjectData,
    level_up_time: f32,
    grass_level: u8,
    grass_type: GrassType,
    anim_instance: SpriteAnimationInstance,
}

impl Grass {
    pub fn new(cord: MapCord, rng: &mut ThreadRng, total_game_time: f32) -> Object {
        let grass_level = rng.random_range(0..=2);
        let grass_type = GrassType::random_type(rng);

        let (offset_y, height) = match grass_level {
            0 | 1 => (0.0, SMALL_GRASS_HEIGHT),
            2 => (-8.0, TALL_GRASS_HEIGHT),
            _ => panic!("only levels 0..=2 allowed for grass"),
        };

        let data = ObjectData::new(
            cord.map_pos(),
            Vector2::new(0.0, offset_y),
            vector2_utils::random_offset_by_one(rng),
            GRASS_WIDTH as f32,
            height as f32,
        );

        let grass = Grass {
            data,
            level_up_time: rng.random_range(MINIMUM_LEVEL_UP_TIME..=MAXIMUM_LEVEL_UP_TIME)
                + total_game_time,
            grass_level,
            grass_type,
            anim_instance: SpriteAnimationInstance::new(),
        };

        return Object::GrassObj(grass);
    }

    pub fn update(&mut self, dt: f32, total_game_time: f32, rng: &mut ThreadRng) {
        // all anims have exact same properties so its simply not necessary to distinguish them
        WHEATY_GRASS_ANIMS[0].update(&mut self.anim_instance, dt);

        if self.grass_level < 2 {
            // while loop, because if offscreen for extended period of time, may level up multiple times
            while total_game_time > self.level_up_time {
                self.level_up_time +=
                    rng.random_range(MINIMUM_LEVEL_UP_TIME..=MAXIMUM_LEVEL_UP_TIME);
                self.level_up();
            }
        }
    }

    pub fn sprite(&self) -> &Sprite {
        return match self.grass_type {
            Wheaty => {
                &WHEATY_GRASS_ANIMS[self.grass_level as usize].frames
                    [self.anim_instance.current_frame_index as usize]
            }
            GrassType::Flowery => {
                &FLOWERY_GRASS_ANIMS[self.grass_level as usize].frames
                    [self.anim_instance.current_frame_index as usize]
            }
            GrassType::Bushy => {
                &BUSHY_GRASS_ANIMS[self.grass_level as usize].frames
                    [self.anim_instance.current_frame_index as usize]
            }
            GrassType::Stalky => {
                &STALKY_GRASS_ANIMS[self.grass_level as usize].frames
                    [self.anim_instance.current_frame_index as usize]
            }
        };
    }

    pub fn level_up(&mut self) {
        self.grass_level += 1;
        self.grass_level = self.grass_level.clamp(0, 2);

        if self.grass_level == 2 {
            // this only happens once since the level up cant be called once you reach 2
            // this just adjusts for the increased height of the tall grass
            self.data.draw_pos += Vector2::new(0.0, -8.0);
        }
    }
}
