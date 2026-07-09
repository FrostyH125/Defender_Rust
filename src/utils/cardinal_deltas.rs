use raylib::math::Vector2;

pub const ORTHOGONAL_DELTAS: [Vector2; 8] = [
    Vector2::new(0.0, -1.0),
    Vector2::new(1.0, -1.0),
    Vector2::new(1.0, 0.0),
    Vector2::new(1.0, 1.0),
    Vector2::new(0.0, 1.0),
    Vector2::new(-1.0, 1.0),
    Vector2::new(-1.0, 0.0),
    Vector2::new(-1.0, -1.0),
];

pub const CARDINAL_DELTAS: [Vector2; 4] = [
    Vector2::new(0.0, -1.0),
    Vector2::new(1.0, 0.0),
    Vector2::new(0.0, 1.0),
    Vector2::new(-1.0, 0.0),
];
