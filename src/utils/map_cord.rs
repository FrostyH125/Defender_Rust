use std::ops::{Add, Sub};

#[derive(PartialEq, Eq, Hash)]
pub struct MapCord {
    // i16 because then its easy to check when cords are out of bounds without doing weird stuff
    pub x: i16,
    pub y: i16
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
            y: self.y + rhs.y
        }
    }
}

impl Sub for MapCord {

    type Output = MapCord;
    
    fn sub(self, rhs: MapCord) -> MapCord {
        MapCord {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}