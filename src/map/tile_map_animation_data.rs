use basic_raylib_core::graphics::{animation_data::AnimationData, sprite::Sprite};

const LAKE_TILE_FRAME_DURATION: f32 = 0.4;

// (ANIM_NAME, bitmask)
static LAKE_TILE_SHORE_ANIMATION_REFERENCE: [AnimationData; 15] = [
    // TOP, 1
    AnimationData {
        frames: &[Sprite::new(16, 17, 8, 8), Sprite::new(24, 17, 8, 8)],
        frame_duration: LAKE_TILE_FRAME_DURATION,
        should_loop: true,
    },
    // RIGHT, 2
    AnimationData {
        frames: &[Sprite::new(16, 24, 8, 8), Sprite::new(24, 24, 8, 8)],
        frame_duration: LAKE_TILE_FRAME_DURATION,
        should_loop: true,
    },
    // TOP AND RIGHT EDGE, 3
    AnimationData {
        frames: &[Sprite::new(16, 32, 8, 8), Sprite::new(24, 32, 8, 8)],
        frame_duration: LAKE_TILE_FRAME_DURATION,
        should_loop: true,
    },
    // BOTTOM EDGE, 4
    AnimationData {
        frames: &[Sprite::new(16, 40, 8, 8), Sprite::new(24, 40, 8, 8)],
        frame_duration: LAKE_TILE_FRAME_DURATION,
        should_loop: true,
    },
    // TOP AND BOTTOM EDGE, 5
    AnimationData {
        frames: &[Sprite::new(16, 48, 8, 8), Sprite::new(24, 48, 8, 8)],
        frame_duration: LAKE_TILE_FRAME_DURATION,
        should_loop: true,
    },
    // RIGHT AND BOTTOM EDGE, 6
    AnimationData {
        frames: &[Sprite::new(16, 56, 8, 8), Sprite::new(24, 56, 8, 8)],
        frame_duration: LAKE_TILE_FRAME_DURATION,
        should_loop: true,
    },
    // TOP, RIGHT, AND BOTTOM EDGE, 7
    AnimationData {
        frames: &[Sprite::new(16, 64, 8, 8), Sprite::new(24, 64, 8, 8)],
        frame_duration: LAKE_TILE_FRAME_DURATION,
        should_loop: true,
    },
    // LEFT, 8
    AnimationData {
        frames: &[Sprite::new(16, 8, 8, 8), Sprite::new(24, 8, 8, 8)],
        frame_duration: LAKE_TILE_FRAME_DURATION,
        should_loop: true,
    },
    // TOP AND LEFT, 9
    AnimationData {
        frames: &[Sprite::new(32, 16, 8, 8), Sprite::new(40, 16, 8, 8)],
        frame_duration: LAKE_TILE_FRAME_DURATION,
        should_loop: true,
    },
    // LEFT AND RIGHT, 10
    AnimationData {
        frames: &[Sprite::new(32, 24, 8, 8), Sprite::new(40, 24, 8, 8)],
        frame_duration: LAKE_TILE_FRAME_DURATION,
        should_loop: true,
    },
    // LEFT, RIGHT, AND TOP EDGE, 11
    AnimationData {
        frames: &[Sprite::new(32, 32, 8, 8), Sprite::new(40, 32, 8, 8)],
        frame_duration: LAKE_TILE_FRAME_DURATION,
        should_loop: true,
    },
    // LEFT AND BOTTOM EDGE, 12
    AnimationData {
        frames: &[Sprite::new(32, 40, 8, 8), Sprite::new(40, 40, 8, 8)],
        frame_duration: LAKE_TILE_FRAME_DURATION,
        should_loop: true,
    },
    // TOP, BOTTOM, AND LEFT EDGE, 13
    AnimationData {
        frames: &[Sprite::new(32, 48, 8, 8), Sprite::new(40, 48, 8, 8)],
        frame_duration: LAKE_TILE_FRAME_DURATION,
        should_loop: true,
    },
    // LEFT, RIGHT, AND BOTTOM EDGE, 14
    AnimationData {
        frames: &[Sprite::new(32, 56, 8, 8), Sprite::new(40, 56, 8, 8)],
        frame_duration: LAKE_TILE_FRAME_DURATION,
        should_loop: true,
    },
    // ALL SIDES COVERED, 15
    AnimationData {
        frames: &[Sprite::new(32, 64, 8, 8), Sprite::new(40, 64, 8, 8)],
        frame_duration: LAKE_TILE_FRAME_DURATION,
        should_loop: true,
    },
];

// (ANIM_NAME, bitmask)
static LAKE_TILE_CORNER_ANIMATION_REFERENCE: [AnimationData; 15] = [
    // TOP LEFT CORNER, 1
    AnimationData {
        frames: &[Sprite::new(48, 16, 8, 8), Sprite::new(56, 16, 8, 8)],
        frame_duration: LAKE_TILE_FRAME_DURATION,
        should_loop: true,
    },
    // TOP RIGHT CORNER, 2
    AnimationData {
        frames: &[Sprite::new(48, 24, 8, 8), Sprite::new(56, 24, 8, 8)],
        frame_duration: LAKE_TILE_FRAME_DURATION,
        should_loop: true,
    },
    // TOP LEFT AND TOP RIGHT CORNER, 3
    AnimationData {
        frames: &[Sprite::new(48, 32, 8, 8), Sprite::new(56, 32, 8, 8)],
        frame_duration: LAKE_TILE_FRAME_DURATION,
        should_loop: true,
    },
    // BOTTOM RIGHT CORNER, 4
    AnimationData {
        frames: &[Sprite::new(80, 32, 8, 8), Sprite::new(88, 32, 8, 8)],
        frame_duration: LAKE_TILE_FRAME_DURATION,
        should_loop: true,
    },
    // TOP LEFT CORNER AND BOTTOM RIGHT CORNER, 5
    AnimationData {
        frames: &[Sprite::new(64, 40, 8, 8), Sprite::new(72, 40, 8, 8)],
        frame_duration: LAKE_TILE_FRAME_DURATION,
        should_loop: true,
    },
    // BOTTOM RIGHT AND TOP RIGHT CORNER, 6
    AnimationData {
        frames: &[Sprite::new(48, 48, 8, 8), Sprite::new(56, 48, 8, 8)],
        frame_duration: LAKE_TILE_FRAME_DURATION,
        should_loop: true,
    },
    // TOP LEFT CORNER, TOP RIGHT CORNER, BOTTOM RIGHT CORNER, 7
    AnimationData {
        frames: &[Sprite::new(80, 48, 8, 8), Sprite::new(88, 48, 8, 8)],
        frame_duration: LAKE_TILE_FRAME_DURATION,
        should_loop: true,
    },
    // BOTTOM LEFT CORNER, 8
    AnimationData {
        frames: &[Sprite::new(64, 16, 8, 8), Sprite::new(72, 16, 8, 8)],
        frame_duration: LAKE_TILE_FRAME_DURATION,
        should_loop: true,
    },
    // TOP LEFT AND BOTTOM LEFT CORNER, 9
    AnimationData {
        frames: &[Sprite::new(80, 16, 8, 8), Sprite::new(88, 16, 8, 8)],
        frame_duration: LAKE_TILE_FRAME_DURATION,
        should_loop: true,
    },
    // TOP RIGHT AND BOTTOM LEFT CORNER, 10
    AnimationData {
        frames: &[Sprite::new(64, 24, 8, 8), Sprite::new(72, 24, 8, 8)],
        frame_duration: LAKE_TILE_FRAME_DURATION,
        should_loop: true,
    },
    // TOP LEFT, BOTTOM LEFT, AND TOP RIGHT CORNER, 11
    AnimationData {
        frames: &[Sprite::new(64, 32, 8, 8), Sprite::new(64, 32, 8, 8)],
        frame_duration: LAKE_TILE_FRAME_DURATION,
        should_loop: true,
    },
    // BOTTOM RIGHT AND BOTTOM LEFT CORNER, 12
    AnimationData {
        frames: &[Sprite::new(48, 40, 8, 8), Sprite::new(56, 40, 8, 8)],
        frame_duration: LAKE_TILE_FRAME_DURATION,
        should_loop: true,
    },
    // TOP LEFT, BOTTOM LEFT, AND BOTTOM RIGHT CORNER, 13
    AnimationData {
        frames: &[Sprite::new(80, 40, 8, 8), Sprite::new(88, 40, 8, 8)],
        frame_duration: LAKE_TILE_FRAME_DURATION,
        should_loop: true,
    },
    // TOP RIGHT, BOTTOM RIGHT, AND BOTTOM LEFT CORNER, 14
    AnimationData {
        frames: &[Sprite::new(64, 48, 8, 8), Sprite::new(72, 48, 8, 8)],
        frame_duration: LAKE_TILE_FRAME_DURATION,
        should_loop: true,
    },
    // ALL FOUR CORNERS, 15
    AnimationData {
        frames: &[Sprite::new(48, 56, 8, 8), Sprite::new(56, 56, 8, 8)],
        frame_duration: LAKE_TILE_FRAME_DURATION,
        should_loop: true,
    },
];