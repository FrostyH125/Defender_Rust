use std::ops::{Add, Mul, Sub};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct MapCord {
    // i16 because then its easy to check when cords are out of bounds without doing weird stuff
    pub x: i16,
    pub y: i16,
}

impl MapCord {
    pub const fn new(x: i16, y: i16) -> Self {
        MapCord { x, y }
    }
}

impl Add for MapCord {
    type Output = MapCord;

    fn add(self, rhs: MapCord) -> MapCord {
        MapCord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for MapCord {
    type Output = MapCord;

    fn sub(self, rhs: MapCord) -> MapCord {
        MapCord {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<i32> for MapCord {
    type Output = MapCord;

    fn mul(self, rhs: i32) -> Self::Output {
        MapCord {
            x: self.x * rhs as i16,
            y: self.y * rhs as i16,
        }
    }
}
