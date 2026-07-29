use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

use raylib::math::Vector2;

use crate::TILE_SIZE;

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

    pub fn map_pos(self) -> Vector2 {
        return Vector2::new(self.x as f32 * TILE_SIZE, self.y as f32 * TILE_SIZE);
    }

    pub fn dist_to(self, p2: MapCord) -> f32 {
        let dx = (p2.x - self.x) as f32;
        let dy = (p2.y - self.y) as f32;
    
        return (dx * dx + dy * dy).sqrt();
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

impl AddAssign for MapCord {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign for MapCord {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
