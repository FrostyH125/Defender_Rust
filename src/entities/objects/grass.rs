use rand::{RngExt, rngs::ThreadRng};
use raylib::math::Vector2;
use zander_game_core_rs::raylib::{animation_data::SpriteAnimationData, sprite::Sprite, sprite_animation::SpriteAnimationInstance};

use crate::{
    GameContext,
    entities::{
        object::{Object, ObjectData},
        objects::grass::GrassType::Wheaty,
    },
    map::{map_cell::MapCell, tile_map::MapDimensions},
    utils::{direction_utils::FacingDirection, map_cord::MapCord, vector2_utils},
};

const SMALL_GRASS_HEIGHT: i32 = 8;
const TALL_GRASS_HEIGHT: i32 = 16;
const GRASS_WIDTH: i32 = 8;
const GRASS_ANIM_SPEED: f32 = 0.75;
const MINIMUM_LEVEL_UP_TIME: f32 = 80.0;
const MAXIMUM_LEVEL_UP_TIME: f32 = 120.0;

static WHEATY_GRASS_ANIMS: [SpriteAnimationData; 3] = [
    SpriteAnimationData {
        frames: &[
            Sprite::new(0, 8, GRASS_WIDTH, SMALL_GRASS_HEIGHT),
            Sprite::new(8, 8, GRASS_WIDTH, SMALL_GRASS_HEIGHT),
        ],
        frame_duration: GRASS_ANIM_SPEED,
        should_loop: true,
    },
    SpriteAnimationData {
        frames: &[
            Sprite::new(0, 16, GRASS_WIDTH, SMALL_GRASS_HEIGHT),
            Sprite::new(8, 16, GRASS_WIDTH, SMALL_GRASS_HEIGHT),
        ],
        frame_duration: GRASS_ANIM_SPEED,
        should_loop: true,
    },
    SpriteAnimationData {
        frames: &[
            Sprite::new(0, 24, GRASS_WIDTH, TALL_GRASS_HEIGHT),
            Sprite::new(8, 24, GRASS_WIDTH, TALL_GRASS_HEIGHT),
        ],
        frame_duration: GRASS_ANIM_SPEED,
        should_loop: true,
    },
];

static FLOWERY_GRASS_ANIMS: [SpriteAnimationData; 3] = [
    SpriteAnimationData {
        frames: &[
            Sprite::new(0, 40, GRASS_WIDTH, SMALL_GRASS_HEIGHT),
            Sprite::new(8, 40, GRASS_WIDTH, SMALL_GRASS_HEIGHT),
        ],
        frame_duration: GRASS_ANIM_SPEED,
        should_loop: true,
    },
    SpriteAnimationData {
        frames: &[
            Sprite::new(0, 48, GRASS_WIDTH, SMALL_GRASS_HEIGHT),
            Sprite::new(8, 48, GRASS_WIDTH, SMALL_GRASS_HEIGHT),
        ],
        frame_duration: GRASS_ANIM_SPEED,
        should_loop: true,
    },
    SpriteAnimationData {
        frames: &[
            Sprite::new(0, 56, GRASS_WIDTH, TALL_GRASS_HEIGHT),
            Sprite::new(8, 56, GRASS_WIDTH, TALL_GRASS_HEIGHT),
        ],
        frame_duration: GRASS_ANIM_SPEED,
        should_loop: true,
    },
];

static BUSHY_GRASS_ANIMS: [SpriteAnimationData; 3] = [
    SpriteAnimationData {
        frames: &[
            Sprite::new(0, 72, GRASS_WIDTH, SMALL_GRASS_HEIGHT),
            Sprite::new(8, 72, GRASS_WIDTH, SMALL_GRASS_HEIGHT),
        ],
        frame_duration: GRASS_ANIM_SPEED,
        should_loop: true,
    },
    SpriteAnimationData {
        frames: &[
            Sprite::new(0, 80, GRASS_WIDTH, SMALL_GRASS_HEIGHT),
            Sprite::new(8, 80, GRASS_WIDTH, SMALL_GRASS_HEIGHT),
        ],
        frame_duration: GRASS_ANIM_SPEED,
        should_loop: true,
    },
    SpriteAnimationData {
        frames: &[
            Sprite::new(0, 88, GRASS_WIDTH, TALL_GRASS_HEIGHT),
            Sprite::new(8, 88, GRASS_WIDTH, TALL_GRASS_HEIGHT),
        ],
        frame_duration: GRASS_ANIM_SPEED,
        should_loop: true,
    },
];

static STALKY_GRASS_ANIMS: [SpriteAnimationData; 3] = [
    SpriteAnimationData {
        frames: &[
            Sprite::new(0, 104, GRASS_WIDTH, SMALL_GRASS_HEIGHT),
            Sprite::new(8, 104, GRASS_WIDTH, SMALL_GRASS_HEIGHT),
        ],
        frame_duration: GRASS_ANIM_SPEED,
        should_loop: true,
    },
    SpriteAnimationData {
        frames: &[
            Sprite::new(0, 112, GRASS_WIDTH, SMALL_GRASS_HEIGHT),
            Sprite::new(8, 112, GRASS_WIDTH, SMALL_GRASS_HEIGHT),
        ],
        frame_duration: GRASS_ANIM_SPEED,
        should_loop: true,
    },
    SpriteAnimationData {
        frames: &[
            Sprite::new(0, 120, GRASS_WIDTH, TALL_GRASS_HEIGHT),
            Sprite::new(8, 120, GRASS_WIDTH, TALL_GRASS_HEIGHT),
        ],
        frame_duration: GRASS_ANIM_SPEED,
        should_loop: true,
    },
];

#[derive(Copy, Clone)]
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
    pub fn new(
        cord: MapCord,
        game_context: &mut GameContext,
        map_dimensions: MapDimensions,
        cells: &mut Vec<MapCell>,
    ) -> Object {
        let grass_level = game_context.rng.random_range(0..=2);
        let grass_type = GrassType::random_type(&mut game_context.rng);

        let (offset_y, height) = match grass_level {
            0 | 1 => (0.0, SMALL_GRASS_HEIGHT),
            2 => (-8.0, TALL_GRASS_HEIGHT),
            _ => panic!("only levels 0..=2 allowed for grass"),
        };

        let data = ObjectData::new(
            cord.map_pos(),
            Vector2::new(0.0, offset_y),
            vector2_utils::random_offset_by_one(&mut game_context.rng),
            cord,
            cells,
            map_dimensions,
            GRASS_WIDTH as f32,
            height as f32,
            100.0,
            0.5,
            0.0,
        );

        let grass = Grass {
            data,
            level_up_time: game_context
                .rng
                .random_range(MINIMUM_LEVEL_UP_TIME..=MAXIMUM_LEVEL_UP_TIME)
                + game_context.total_game_time,
            grass_level,
            grass_type,
            anim_instance: SpriteAnimationInstance {   
                sprite_animation: Self::get_grass_anim(grass_type, grass_level),
                current_frame_time: game_context.rng.random_range(0.0..=GRASS_ANIM_SPEED),
                current_frame_index: game_context
                    .rng
                    .random_range(0..WHEATY_GRASS_ANIMS[0].frames.len())
                    as u8,
                finished_playing: false,
            },
        };

        return Object::GrassObj(grass);
    }

    /// has a 90% chance of being level 0 (small), and from the 10% of the other chance, it has an 80% chance of that to be 1 (medium), else its 2 (large)
    pub fn new_small_likely(
        cord: MapCord,
        game_context: &mut GameContext,
        map_dimensions: MapDimensions,
        cells: &mut Vec<MapCell>,
    ) -> Object {
        let grass_level = if game_context.rng.random_bool(0.9) {
            0
        } else if game_context.rng.random_bool(0.8) {
            1
        } else {
            2
        };

        let grass_type = GrassType::random_type(&mut game_context.rng);

        let (offset_y, height) = match grass_level {
            0 | 1 => (0.0, SMALL_GRASS_HEIGHT),
            2 => (-8.0, TALL_GRASS_HEIGHT),
            _ => panic!("only levels 0..=2 allowed for grass"),
        };

        let data = ObjectData::new(
            cord.map_pos(),
            Vector2::new(0.0, offset_y),
            vector2_utils::random_offset_by_one(&mut game_context.rng),
            cord,
            cells,
            map_dimensions,
            GRASS_WIDTH as f32,
            height as f32,
            100.0,
            0.0,
            0.0,
        );

        let grass = Grass {
            data,
            level_up_time: game_context
                .rng
                .random_range(MINIMUM_LEVEL_UP_TIME..=MAXIMUM_LEVEL_UP_TIME)
                + game_context.total_game_time,
            grass_level,
            grass_type,
            anim_instance: SpriteAnimationInstance {
                sprite_animation: Self::get_grass_anim(grass_type, grass_level),              
                current_frame_time: game_context.rng.random_range(0.0..=GRASS_ANIM_SPEED),
                current_frame_index: game_context
                    .rng
                    .random_range(0..WHEATY_GRASS_ANIMS[0].frames.len())
                    as u8,
                finished_playing: false,
            },
        };

        return Object::GrassObj(grass);
    }

    /// has a 90% chance of being level 2 (large), and from the 10% of the other chance, it has an 80% chance of that to be 1 (medium), else its 2 (small)
    pub fn new_large_likely(
        cord: MapCord,
        game_context: &mut GameContext,
        map_dimensions: MapDimensions,
        cells: &mut Vec<MapCell>,
    ) -> Object {
        let grass_level = if game_context.rng.random_bool(0.9) {
            2
        } else if game_context.rng.random_bool(0.8) {
            1
        } else {
            0
        };

        let grass_type = GrassType::random_type(&mut game_context.rng);

        let (offset_y, height) = match grass_level {
            0 | 1 => (0.0, SMALL_GRASS_HEIGHT),
            2 => (-8.0, TALL_GRASS_HEIGHT),
            _ => panic!("only levels 0..=2 allowed for grass"),
        };

        let data = ObjectData::new(
            cord.map_pos(),
            Vector2::new(0.0, offset_y),
            vector2_utils::random_offset_by_one(&mut game_context.rng),
            cord,
            cells,
            map_dimensions,
            GRASS_WIDTH as f32,
            height as f32,
            100.0,
            0.0,
            0.0,
        );

        let grass = Grass {
            data,
            level_up_time: game_context
                .rng
                .random_range(MINIMUM_LEVEL_UP_TIME..=MAXIMUM_LEVEL_UP_TIME)
                + game_context.total_game_time,
            grass_level,
            grass_type,
            anim_instance: SpriteAnimationInstance {
                sprite_animation: Self::get_grass_anim(grass_type, grass_level),
                current_frame_time: game_context.rng.random_range(0.0..=GRASS_ANIM_SPEED),
                current_frame_index: game_context
                    .rng
                    .random_range(0..WHEATY_GRASS_ANIMS[0].frames.len())
                    as u8,
                finished_playing: false,
            },
        };

        return Object::GrassObj(grass);
    }

    pub fn update(&mut self, game_context: &mut GameContext) {
        // all anims have exact same properties so its simply not necessary to distinguish them
        self.anim_instance.update(game_context.dt);

        // while loop, because if offscreen for extended period of time, may level up multiple times
        while game_context.total_game_time > self.level_up_time {
            if self.grass_level >= 2 {
                // just to make sure. it shouldnt be anything else but if this code saves just one life itll be worth it
                self.grass_level = 2;

                // now this code will never run again hahahaha
                self.level_up_time = f32::MAX;
                break;
            }
            self.level_up_time += game_context
                .rng
                .random_range(MINIMUM_LEVEL_UP_TIME..=MAXIMUM_LEVEL_UP_TIME);
            self.level_up();
        }
    }

    pub fn level_up(&mut self) {
        self.grass_level += 1;
        self.grass_level = self.grass_level.clamp(0, 2);

        if self.grass_level == 2 {
            // this only happens once since the level up cant be called once you reach 2
            // this just adjusts for the increased height of the tall grass
            self.data.draw_pos += Vector2::new(0.0, -8.0);
        }

        self.anim_instance.sprite_animation = Self::get_grass_anim(self.grass_type, self.grass_level);
    }

    pub fn on_hit(&mut self, game_context: &mut GameContext, facing_dir: FacingDirection) {
        static GRASS_EMIT_PARTICLES: [Sprite; 4] = [
            Sprite::new(51, 0, 1, 1),
            Sprite::new(52, 0, 1, 1),
            Sprite::new(53, 0, 1, 1),
            Sprite::new(54, 0, 1, 1),
        ];

        let rect = self.data.hover_rect();
        let half_height = rect.height / 2.0;

        for _ in 0..=game_context.rng.random_range(15..=25) {
            let pos = Vector2::new(
                game_context.rng.random_range(rect.x..=rect.x + rect.width),
                game_context.rng.random_range(
                    rect.y + half_height - half_height / 4.0
                        ..=rect.y + half_height + half_height / 4.0,
                ),
            );

            let mut vel = Vector2::new(
                game_context.rng.random_range(15.0..=30.0),
                game_context.rng.random_range(-10.0..=3.0),
            );

            if facing_dir == FacingDirection::Left {
                vel.x = -vel.x;
            }

            let acc = Vector2::new(-vel.x * game_context.rng.random_range(0.5..=1.5), 10.0);

            let life_span = game_context.rng.random_range(0.75..=1.25);

            let sprite = &GRASS_EMIT_PARTICLES[game_context.rng.random_range(0..=3)];

            game_context
                .particle_system
                .emit(sprite, pos, vel, acc, life_span);
        }
    }

    pub fn sprite(&self) -> Sprite {
        return match self.grass_type {
            Wheaty => {
                WHEATY_GRASS_ANIMS[self.grass_level as usize].frames
                    [self.anim_instance.current_frame_index as usize]
            }
            GrassType::Flowery => {
                FLOWERY_GRASS_ANIMS[self.grass_level as usize].frames
                    [self.anim_instance.current_frame_index as usize]
            }
            GrassType::Bushy => {
                BUSHY_GRASS_ANIMS[self.grass_level as usize].frames
                    [self.anim_instance.current_frame_index as usize]
            }
            GrassType::Stalky => {
                STALKY_GRASS_ANIMS[self.grass_level as usize].frames
                    [self.anim_instance.current_frame_index as usize]
            }
        };
    }

    pub fn get_grass_anim(grass_type: GrassType, level: u8) -> &'static SpriteAnimationData {
        match grass_type {
            Wheaty => &WHEATY_GRASS_ANIMS[level as usize],
            GrassType::Flowery => &FLOWERY_GRASS_ANIMS[level as usize],
            GrassType::Bushy => &BUSHY_GRASS_ANIMS[level as usize],
            GrassType::Stalky => &STALKY_GRASS_ANIMS[level as usize],
        }
    }
}
