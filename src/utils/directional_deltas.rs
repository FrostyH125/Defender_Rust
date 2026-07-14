use crate::utils::map_cord::MapCord;

#[repr(i8)]
#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    pub fn get_enum_from_repr(repr: i8) -> Direction {
        match repr {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            _ => panic!("not a valid Direction enum value")
        }
    }

    /// returns a new direction one turn to the left
    pub fn turn_left(&self) -> Direction {
        let val = *self as i8;
        let new_val = (val - 1).rem_euclid(4);
        return Self::get_enum_from_repr(new_val);
    }

    // returns a new direction one turn to the right
    pub fn turn_right(&self) -> Direction {
        let val = *self as i8;
        let new_val = (val + 1).rem_euclid(4);
        return Self::get_enum_from_repr(new_val);
    }
}

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
