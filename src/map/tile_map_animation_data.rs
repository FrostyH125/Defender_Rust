use std::{collections::HashMap, sync::LazyLock};

use basic_raylib_core::graphics::{animation_data::AnimationData, sprite::Sprite};

const SHORE_AND_CORNER_FRAME_DURATION: f32 = 0.4;
const REGULAR_TILE_FRAME_DURATION: f32 = 0.2;

pub enum SpriteFlip {
    None,
    Horizontal, // flips across y axis -> <-
    Vertical,   // flips across x axis ^ v
}

pub enum RiverType {
    Straight,
    Corner,
    TSection,
    Inlet,
    Outlet,
}

static RIVER_CORNER_ANIM_KEY: LazyLock<HashMap<(u8, u8, bool), usize>> = LazyLock::new(|| {
    HashMap::from([
        ((2, 1, true), 0),
        ((2, 1, false), 1),
        ((2, 3, true), 2),
        ((2, 3, false), 3),
        ((0, 3, true), 4),
        ((0, 3, false), 5),
        ((0, 1, true), 6),
        ((0, 1, false), 7),
    ])
});

static RIVER_T_SECTION_ANIM_KEY: LazyLock<HashMap<(u8, u8, u8, bool), usize>> = LazyLock::new(|| {
    HashMap::from([
        ((0, 1, 2, true), 0),
        ((0, 2, 3, true), 1),
        ((0, 1, 2, false), 2),
        ((0, 2, 3, false), 3),
        ((0, 1, 3, true), 4),
        ((0, 1, 3, false), 5),
        ((1, 2, 3, true), 6),
        ((1, 2, 3, false), 7),
    ])
});

// (ANIM_NAME, bitmask)
static LAKE_TILE_SHORE_ANIMATION_REFERENCE: [AnimationData; 15] = [
    // TOP, 1
    AnimationData {
        frames: &[Sprite::new(16, 17, 8, 8), Sprite::new(24, 17, 8, 8)],
        frame_duration: SHORE_AND_CORNER_FRAME_DURATION,
        should_loop: true,
    },
    // RIGHT, 2
    AnimationData {
        frames: &[Sprite::new(16, 24, 8, 8), Sprite::new(24, 24, 8, 8)],
        frame_duration: SHORE_AND_CORNER_FRAME_DURATION,
        should_loop: true,
    },
    // TOP AND RIGHT EDGE, 3
    AnimationData {
        frames: &[Sprite::new(16, 32, 8, 8), Sprite::new(24, 32, 8, 8)],
        frame_duration: SHORE_AND_CORNER_FRAME_DURATION,
        should_loop: true,
    },
    // BOTTOM EDGE, 4
    AnimationData {
        frames: &[Sprite::new(16, 40, 8, 8), Sprite::new(24, 40, 8, 8)],
        frame_duration: SHORE_AND_CORNER_FRAME_DURATION,
        should_loop: true,
    },
    // TOP AND BOTTOM EDGE, 5
    AnimationData {
        frames: &[Sprite::new(16, 48, 8, 8), Sprite::new(24, 48, 8, 8)],
        frame_duration: SHORE_AND_CORNER_FRAME_DURATION,
        should_loop: true,
    },
    // RIGHT AND BOTTOM EDGE, 6
    AnimationData {
        frames: &[Sprite::new(16, 56, 8, 8), Sprite::new(24, 56, 8, 8)],
        frame_duration: SHORE_AND_CORNER_FRAME_DURATION,
        should_loop: true,
    },
    // TOP, RIGHT, AND BOTTOM EDGE, 7
    AnimationData {
        frames: &[Sprite::new(16, 64, 8, 8), Sprite::new(24, 64, 8, 8)],
        frame_duration: SHORE_AND_CORNER_FRAME_DURATION,
        should_loop: true,
    },
    // LEFT, 8
    AnimationData {
        frames: &[Sprite::new(16, 8, 8, 8), Sprite::new(24, 8, 8, 8)],
        frame_duration: SHORE_AND_CORNER_FRAME_DURATION,
        should_loop: true,
    },
    // TOP AND LEFT, 9
    AnimationData {
        frames: &[Sprite::new(32, 16, 8, 8), Sprite::new(40, 16, 8, 8)],
        frame_duration: SHORE_AND_CORNER_FRAME_DURATION,
        should_loop: true,
    },
    // LEFT AND RIGHT, 10
    AnimationData {
        frames: &[Sprite::new(32, 24, 8, 8), Sprite::new(40, 24, 8, 8)],
        frame_duration: SHORE_AND_CORNER_FRAME_DURATION,
        should_loop: true,
    },
    // LEFT, RIGHT, AND TOP EDGE, 11
    AnimationData {
        frames: &[Sprite::new(32, 32, 8, 8), Sprite::new(40, 32, 8, 8)],
        frame_duration: SHORE_AND_CORNER_FRAME_DURATION,
        should_loop: true,
    },
    // LEFT AND BOTTOM EDGE, 12
    AnimationData {
        frames: &[Sprite::new(32, 40, 8, 8), Sprite::new(40, 40, 8, 8)],
        frame_duration: SHORE_AND_CORNER_FRAME_DURATION,
        should_loop: true,
    },
    // TOP, BOTTOM, AND LEFT EDGE, 13
    AnimationData {
        frames: &[Sprite::new(32, 48, 8, 8), Sprite::new(40, 48, 8, 8)],
        frame_duration: SHORE_AND_CORNER_FRAME_DURATION,
        should_loop: true,
    },
    // LEFT, RIGHT, AND BOTTOM EDGE, 14
    AnimationData {
        frames: &[Sprite::new(32, 56, 8, 8), Sprite::new(40, 56, 8, 8)],
        frame_duration: SHORE_AND_CORNER_FRAME_DURATION,
        should_loop: true,
    },
    // ALL SIDES COVERED, 15
    AnimationData {
        frames: &[Sprite::new(32, 64, 8, 8), Sprite::new(40, 64, 8, 8)],
        frame_duration: SHORE_AND_CORNER_FRAME_DURATION,
        should_loop: true,
    },
];

// (ANIM_NAME, bitmask)
static LAKE_TILE_CORNER_ANIMATION_REFERENCE: [AnimationData; 15] = [
    // TOP LEFT CORNER, 1
    AnimationData {
        frames: &[Sprite::new(48, 16, 8, 8), Sprite::new(56, 16, 8, 8)],
        frame_duration: SHORE_AND_CORNER_FRAME_DURATION,
        should_loop: true,
    },
    // TOP RIGHT CORNER, 2
    AnimationData {
        frames: &[Sprite::new(48, 24, 8, 8), Sprite::new(56, 24, 8, 8)],
        frame_duration: SHORE_AND_CORNER_FRAME_DURATION,
        should_loop: true,
    },
    // TOP LEFT AND TOP RIGHT CORNER, 3
    AnimationData {
        frames: &[Sprite::new(48, 32, 8, 8), Sprite::new(56, 32, 8, 8)],
        frame_duration: SHORE_AND_CORNER_FRAME_DURATION,
        should_loop: true,
    },
    // BOTTOM RIGHT CORNER, 4
    AnimationData {
        frames: &[Sprite::new(80, 32, 8, 8), Sprite::new(88, 32, 8, 8)],
        frame_duration: SHORE_AND_CORNER_FRAME_DURATION,
        should_loop: true,
    },
    // TOP LEFT CORNER AND BOTTOM RIGHT CORNER, 5
    AnimationData {
        frames: &[Sprite::new(64, 40, 8, 8), Sprite::new(72, 40, 8, 8)],
        frame_duration: SHORE_AND_CORNER_FRAME_DURATION,
        should_loop: true,
    },
    // BOTTOM RIGHT AND TOP RIGHT CORNER, 6
    AnimationData {
        frames: &[Sprite::new(48, 48, 8, 8), Sprite::new(56, 48, 8, 8)],
        frame_duration: SHORE_AND_CORNER_FRAME_DURATION,
        should_loop: true,
    },
    // TOP LEFT CORNER, TOP RIGHT CORNER, BOTTOM RIGHT CORNER, 7
    AnimationData {
        frames: &[Sprite::new(80, 48, 8, 8), Sprite::new(88, 48, 8, 8)],
        frame_duration: SHORE_AND_CORNER_FRAME_DURATION,
        should_loop: true,
    },
    // BOTTOM LEFT CORNER, 8
    AnimationData {
        frames: &[Sprite::new(64, 16, 8, 8), Sprite::new(72, 16, 8, 8)],
        frame_duration: SHORE_AND_CORNER_FRAME_DURATION,
        should_loop: true,
    },
    // TOP LEFT AND BOTTOM LEFT CORNER, 9
    AnimationData {
        frames: &[Sprite::new(80, 16, 8, 8), Sprite::new(88, 16, 8, 8)],
        frame_duration: SHORE_AND_CORNER_FRAME_DURATION,
        should_loop: true,
    },
    // TOP RIGHT AND BOTTOM LEFT CORNER, 10
    AnimationData {
        frames: &[Sprite::new(64, 24, 8, 8), Sprite::new(72, 24, 8, 8)],
        frame_duration: SHORE_AND_CORNER_FRAME_DURATION,
        should_loop: true,
    },
    // TOP LEFT, BOTTOM LEFT, AND TOP RIGHT CORNER, 11
    AnimationData {
        frames: &[Sprite::new(64, 32, 8, 8), Sprite::new(64, 32, 8, 8)],
        frame_duration: SHORE_AND_CORNER_FRAME_DURATION,
        should_loop: true,
    },
    // BOTTOM RIGHT AND BOTTOM LEFT CORNER, 12
    AnimationData {
        frames: &[Sprite::new(48, 40, 8, 8), Sprite::new(56, 40, 8, 8)],
        frame_duration: SHORE_AND_CORNER_FRAME_DURATION,
        should_loop: true,
    },
    // TOP LEFT, BOTTOM LEFT, AND BOTTOM RIGHT CORNER, 13
    AnimationData {
        frames: &[Sprite::new(80, 40, 8, 8), Sprite::new(88, 40, 8, 8)],
        frame_duration: SHORE_AND_CORNER_FRAME_DURATION,
        should_loop: true,
    },
    // TOP RIGHT, BOTTOM RIGHT, AND BOTTOM LEFT CORNER, 14
    AnimationData {
        frames: &[Sprite::new(64, 48, 8, 8), Sprite::new(72, 48, 8, 8)],
        frame_duration: SHORE_AND_CORNER_FRAME_DURATION,
        should_loop: true,
    },
    // ALL FOUR CORNERS, 15
    AnimationData {
        frames: &[Sprite::new(48, 56, 8, 8), Sprite::new(56, 56, 8, 8)],
        frame_duration: SHORE_AND_CORNER_FRAME_DURATION,
        should_loop: true,
    },
];

static LAKE_TILE_ANIM: AnimationData = AnimationData {
    frames: &[
        Sprite::new(16, 0, 8, 8),
        Sprite::new(24, 0, 8, 8),
        Sprite::new(32, 0, 8, 8),
        Sprite::new(40, 0, 8, 8),
    ],
    frame_duration: REGULAR_TILE_FRAME_DURATION,
    should_loop: true,
};

static RIVER_TILE_STRAIGHT_ANIMS: [(AnimationData, SpriteFlip); 4] = [
    // NORTH
    (
        AnimationData {
            frames: &[
                Sprite::new(96, 8, 8, 8),
                Sprite::new(104, 8, 8, 8),
                Sprite::new(112, 8, 8, 8),
                Sprite::new(120, 8, 8, 8),
            ],
            frame_duration: REGULAR_TILE_FRAME_DURATION,
            should_loop: true,
        },
        SpriteFlip::None,
    ),
    // EAST
    (
        AnimationData {
            frames: &[
                Sprite::new(96, 0, 8, 8),
                Sprite::new(104, 0, 8, 8),
                Sprite::new(112, 0, 8, 8),
                Sprite::new(120, 0, 8, 8),
            ],
            frame_duration: REGULAR_TILE_FRAME_DURATION,
            should_loop: true,
        },
        SpriteFlip::None,
    ),
    // SOUTH
    (
        AnimationData {
            frames: &[
                Sprite::new(96, 8, 8, 8),
                Sprite::new(104, 8, 8, 8),
                Sprite::new(112, 8, 8, 8),
                Sprite::new(120, 8, 8, 8),
            ],
            frame_duration: REGULAR_TILE_FRAME_DURATION,
            should_loop: true,
        },
        SpriteFlip::Vertical,
    ),
    // WEST
    (
        AnimationData {
            frames: &[
                Sprite::new(96, 0, 8, 8),
                Sprite::new(104, 0, 8, 8),
                Sprite::new(112, 0, 8, 8),
                Sprite::new(120, 0, 8, 8),
            ],
            frame_duration: REGULAR_TILE_FRAME_DURATION,
            should_loop: true,
        },
        SpriteFlip::Horizontal,
    ),
];

static RIVER_TILE_CORNER_ANIMS: [(AnimationData, SpriteFlip); 8] = [
    // DOWN TO RIGHT, 0
    (
        AnimationData {
            frames: &[
                Sprite::new(96, 16, 8, 8),
                Sprite::new(104, 16, 8, 8),
                Sprite::new(112, 16, 8, 8),
                Sprite::new(120, 16, 8, 8),
            ],
            frame_duration: REGULAR_TILE_FRAME_DURATION,
            should_loop: true,
        },
        SpriteFlip::None,
    ),
    // RIGHT TO DOWN, 1
    (
        AnimationData {
            frames: &[
                Sprite::new(96, 24, 8, 8),
                Sprite::new(104, 24, 8, 8),
                Sprite::new(112, 24, 8, 8),
                Sprite::new(120, 24, 8, 8),
            ],
            frame_duration: REGULAR_TILE_FRAME_DURATION,
            should_loop: true,
        },
        SpriteFlip::None,
    ),
    // DOWN TO LEFT, 2
    (
        AnimationData {
            frames: &[
                Sprite::new(96, 16, 8, 8),
                Sprite::new(104, 16, 8, 8),
                Sprite::new(112, 16, 8, 8),
                Sprite::new(120, 16, 8, 8),
            ],
            frame_duration: REGULAR_TILE_FRAME_DURATION,
            should_loop: true,
        },
        SpriteFlip::Horizontal,
    ),
    // LEFT TO DOWN, 3
    (
        AnimationData {
            frames: &[
                Sprite::new(96, 24, 8, 8),
                Sprite::new(104, 24, 8, 8),
                Sprite::new(112, 24, 8, 8),
                Sprite::new(120, 24, 8, 8),
            ],
            frame_duration: REGULAR_TILE_FRAME_DURATION,
            should_loop: true,
        },
        SpriteFlip::Horizontal,
    ),
    // LEFT TO UP, 4
    (
        AnimationData {
            frames: &[
                Sprite::new(96, 32, 8, 8),
                Sprite::new(104, 32, 8, 8),
                Sprite::new(112, 32, 8, 8),
                Sprite::new(120, 32, 8, 8),
            ],
            frame_duration: REGULAR_TILE_FRAME_DURATION,
            should_loop: true,
        },
        SpriteFlip::Horizontal,
    ),
    // UP TO LEFT, 5
    (
        AnimationData {
            frames: &[
                Sprite::new(96, 40, 8, 8),
                Sprite::new(104, 40, 8, 8),
                Sprite::new(112, 40, 8, 8),
                Sprite::new(120, 40, 8, 8),
            ],
            frame_duration: REGULAR_TILE_FRAME_DURATION,
            should_loop: true,
        },
        SpriteFlip::Horizontal,
    ),
    // RIGHT TO UP, 6
    (
        AnimationData {
            frames: &[
                Sprite::new(96, 32, 8, 8),
                Sprite::new(104, 32, 8, 8),
                Sprite::new(112, 32, 8, 8),
                Sprite::new(120, 32, 8, 8),
            ],
            frame_duration: REGULAR_TILE_FRAME_DURATION,
            should_loop: true,
        },
        SpriteFlip::None,
    ),
    // UP TO RIGHT, 7
    (
        AnimationData {
            frames: &[
                Sprite::new(96, 40, 8, 8),
                Sprite::new(104, 40, 8, 8),
                Sprite::new(112, 40, 8, 8),
                Sprite::new(120, 40, 8, 8),
            ],
            frame_duration: REGULAR_TILE_FRAME_DURATION,
            should_loop: true,
        },
        SpriteFlip::None,
    ),
];

static RIVER_TILE_T_SECTION_ANIMS: [(AnimationData, SpriteFlip); 8] = [
    // IN RIGHT FLOWING UP
    (
        AnimationData {
            frames: &[
                Sprite::new(96, 48, 8, 8),
                Sprite::new(104, 48, 8, 8),
                Sprite::new(112, 48, 8, 8),
                Sprite::new(120, 48, 8, 8),
            ],
            frame_duration: REGULAR_TILE_FRAME_DURATION,
            should_loop: true,
        },
        SpriteFlip::None,
    ),
    // IN LEFT FLOWING UP
    (
        AnimationData {
            frames: &[
                Sprite::new(96, 48, 8, 8),
                Sprite::new(104, 48, 8, 8),
                Sprite::new(112, 48, 8, 8),
                Sprite::new(120, 48, 8, 8),
            ],
            frame_duration: REGULAR_TILE_FRAME_DURATION,
            should_loop: true,
        },
        SpriteFlip::Horizontal,
    ),
    // IN RIGHT FLOWING DOWN
    (
        AnimationData {
            frames: &[
                Sprite::new(96, 56, 8, 8),
                Sprite::new(104, 56, 8, 8),
                Sprite::new(112, 56, 8, 8),
                Sprite::new(120, 56, 8, 8),
            ],
            frame_duration: REGULAR_TILE_FRAME_DURATION,
            should_loop: true,
        },
        SpriteFlip::None,
    ),
    // IN LEFT FLOWING DOWN
    (
        AnimationData {
            frames: &[
                Sprite::new(96, 56, 8, 8),
                Sprite::new(104, 56, 8, 8),
                Sprite::new(112, 56, 8, 8),
                Sprite::new(120, 56, 8, 8),
            ],
            frame_duration: REGULAR_TILE_FRAME_DURATION,
            should_loop: true,
        },
        SpriteFlip::Horizontal,
    ),
    // IN UP FLOWING RIGHT
    (
        AnimationData {
            frames: &[
                Sprite::new(96, 64, 8, 8),
                Sprite::new(104, 64, 8, 8),
                Sprite::new(112, 64, 8, 8),
                Sprite::new(120, 64, 8, 8),
            ],
            frame_duration: REGULAR_TILE_FRAME_DURATION,
            should_loop: true,
        },
        SpriteFlip::None,
    ),
    // IN UP FLOWING LEFT
    (
        AnimationData {
            frames: &[
                Sprite::new(96, 64, 8, 8),
                Sprite::new(104, 64, 8, 8),
                Sprite::new(112, 64, 8, 8),
                Sprite::new(120, 64, 8, 8),
            ],
            frame_duration: REGULAR_TILE_FRAME_DURATION,
            should_loop: true,
        },
        SpriteFlip::Horizontal,
    ),
    // IN DOWN FLOWING RIGHT
    (
        AnimationData {
            frames: &[
                Sprite::new(96, 72, 8, 8),
                Sprite::new(104, 72, 8, 8),
                Sprite::new(112, 72, 8, 8),
                Sprite::new(120, 72, 8, 8),
            ],
            frame_duration: REGULAR_TILE_FRAME_DURATION,
            should_loop: true,
        },
        SpriteFlip::None,
    ),
    // IN DOWN FLOWING LEFT
    (
        AnimationData {
            frames: &[
                Sprite::new(96, 72, 8, 8),
                Sprite::new(104, 72, 8, 8),
                Sprite::new(112, 72, 8, 8),
                Sprite::new(120, 72, 8, 8),
            ],
            frame_duration: REGULAR_TILE_FRAME_DURATION,
            should_loop: true,
        },
        SpriteFlip::Horizontal,
    ),
];

// moving out of the lake INTO the river
static OUTLETS_ANIMS: [(AnimationData, SpriteFlip); 4] = [
    // OUT UP (land on top)
    (
        AnimationData {
            frames: &[
                Sprite::new(96, 88, 8, 8),
                Sprite::new(104, 88, 8, 8),
                Sprite::new(112, 88, 8, 8),
                Sprite::new(120, 88, 8, 8),
            ],
            frame_duration: REGULAR_TILE_FRAME_DURATION,
            should_loop: true,
        },
        SpriteFlip::None,
    ),
    // OUT RIGHT (land on right)
    (
        AnimationData {
            frames: &[
                Sprite::new(96, 96, 8, 8),
                Sprite::new(104, 96, 8, 8),
                Sprite::new(112, 96, 8, 8),
                Sprite::new(120, 96, 8, 8),
            ],
            frame_duration: REGULAR_TILE_FRAME_DURATION,
            should_loop: true,
        },
        SpriteFlip::None,
    ),
    // OUT DOWN
    (
        AnimationData {
            frames: &[
                Sprite::new(96, 120, 8, 8),
                Sprite::new(104, 120, 8, 8),
                Sprite::new(112, 120, 8, 8),
                Sprite::new(120, 120, 8, 8),
            ],
            frame_duration: REGULAR_TILE_FRAME_DURATION,
            should_loop: true,
        },
        SpriteFlip::None,
    ),
    // OUT LEFT
    (
        AnimationData {
            frames: &[
                Sprite::new(96, 96, 8, 8),
                Sprite::new(104, 96, 8, 8),
                Sprite::new(112, 96, 8, 8),
                Sprite::new(120, 96, 8, 8),
            ],
            frame_duration: REGULAR_TILE_FRAME_DURATION,
            should_loop: true,
        },
        SpriteFlip::Horizontal,
    ),
];

// moving INTO the lake FROM the river
static INLET_ANIMS: [(AnimationData, SpriteFlip); 4] = [
    // IN UP
    (
        AnimationData {
            frames: &[
                Sprite::new(96, 80, 8, 8),
                Sprite::new(104, 80, 8, 8),
                Sprite::new(112, 80, 8, 8),
                Sprite::new(120, 80, 8, 8),
            ],
            frame_duration: REGULAR_TILE_FRAME_DURATION,
            should_loop: true,
        },
        SpriteFlip::None,
    ),
    // IN RIGHT
    (
        AnimationData {
            frames: &[
                Sprite::new(96, 104, 8, 8),
                Sprite::new(104, 104, 8, 8),
                Sprite::new(112, 104, 8, 8),
                Sprite::new(120, 104, 8, 8),
            ],
            frame_duration: REGULAR_TILE_FRAME_DURATION,
            should_loop: true,
        },
        SpriteFlip::None,
    ),
    // IN DOWN
    (
        AnimationData {
            frames: &[
                Sprite::new(96, 112, 8, 8),
                Sprite::new(104, 112, 8, 8),
                Sprite::new(112, 112, 8, 8),
                Sprite::new(120, 112, 8, 8),
            ],
            frame_duration: REGULAR_TILE_FRAME_DURATION,
            should_loop: true,
        },
        SpriteFlip::None,
    ),
    // IN LEFT
    (
        AnimationData {
            frames: &[
                Sprite::new(96, 104, 8, 8),
                Sprite::new(104, 104, 8, 8),
                Sprite::new(112, 104, 8, 8),
                Sprite::new(120, 104, 8, 8),
            ],
            frame_duration: REGULAR_TILE_FRAME_DURATION,
            should_loop: true,
        },
        SpriteFlip::Horizontal,
    ),
];
