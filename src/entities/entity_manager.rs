use std::iter::MapWhile;

use raylib::{camera::Camera2D, drawing::RaylibDrawHandle, texture::Texture2D};

use crate::{
    TILE_SIZE,
    map::tile_map::{MapDimensions, MapObjectGrid, TileMap},
    utils::map_cord::MapCord,
};

pub struct EntityManager {
    map_dimensions: MapDimensions,
    start_tile_x: i16,
    start_tile_y: i16,
    end_tile_x: i16,
    end_tile_y: i16,
}

impl EntityManager {
    pub fn new(map_width: u16, map_height: u16) -> Self {
        let map_dimensions = MapDimensions {
            width: map_width,
            height: map_height,
        };

        return EntityManager {
            map_dimensions,
            start_tile_x: 0,
            start_tile_y: 0,
            end_tile_x: 0,
            end_tile_y: 0,
        };
    }

    pub fn update(
        &mut self,
        object_grid: &mut MapObjectGrid,
        dt: f32,
        screen_width: f32,
        screen_height: f32,
        camera: &Camera2D,
    ) {
        let start_x = camera.target.x - (screen_width / camera.zoom) / 2.0;
        let start_y = camera.target.y - (screen_height / camera.zoom) / 2.0;
        let end_x = start_x + screen_width / camera.zoom;
        let end_y = start_y + screen_height / camera.zoom;

        self.start_tile_x = (start_x / TILE_SIZE) as i16 - 1;
        self.start_tile_y = (start_y / TILE_SIZE) as i16;
        self.end_tile_x = (end_x / TILE_SIZE) as i16 + 2;
        self.end_tile_y = (end_y / TILE_SIZE) as i16 + 2;

        for y in self.start_tile_y..=self.end_tile_y {
            for x in self.start_tile_x..=self.end_tile_x {
                let cord = MapCord::new(x, y);

                if !TileMap::is_tile_in_bounds_no_ref(self.map_dimensions, cord) {
                    continue;
                }

                let index = TileMap::cords_to_index(self.map_dimensions, cord);

                object_grid[index].update(dt);
            }
        }
    }

    pub fn draw(&self, object_grid: &MapObjectGrid, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        for y in self.start_tile_y..=self.end_tile_y {
            for x in self.start_tile_x..=self.end_tile_x {
                let cord = MapCord::new(x, y);

                if !TileMap::is_tile_in_bounds_no_ref(self.map_dimensions, cord) {
                    continue;
                }

                let index = TileMap::cords_to_index(self.map_dimensions, cord);

                object_grid[index].draw(d, texture);
            }
        }
    }
}
