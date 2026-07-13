use crate::utils::map_cord::MapCord;

pub const ORTHOGONAL_DELTAS: [MapCord; 8] = [
    MapCord::new(0, -1),
    MapCord::new(1, -1),
    MapCord::new(1, 0),
    MapCord::new(1, 1),
    MapCord::new(0, 1),
    MapCord::new(-1, 1),
    MapCord::new(-1, 0),
    MapCord::new(-1, -1),
];

pub const CARDINAL_DELTAS: [MapCord; 4] = [
    MapCord::new(0, -1),
    MapCord::new(1, 0),
    MapCord::new(0, 1),
    MapCord::new(-1, 0),
];
