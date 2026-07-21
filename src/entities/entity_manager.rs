use basic_raylib_core::system::input_handler::InputState;
use raylib::{camera::Camera2D, drawing::RaylibDrawHandle, texture::Texture2D};

use crate::{
    TILE_SIZE,
    entities::object::Object,
    map::tile_map::{MapDimensions, MapObjectGrid, TileMap},
    systems::day_night_cycle::DayNightCycle,
    utils::{map_cord::MapCord, mouse_utils},
};

pub struct EntityManager {
    map_dimensions: MapDimensions,
    start_tile_x: i16,
    start_tile_y: i16,
    end_tile_x: i16,
    end_tile_y: i16,
}

impl EntityManager {
    pub fn new(map_dimensions: MapDimensions) -> Self {
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
        window_width: f32,
        window_height: f32,
        v_width: f32,
        v_height: f32,
        camera: &Camera2D,
        input_state: &InputState,
    ) {
        let mut found_hovering: bool = false;

        let start_x = camera.target.x - v_width / 2.0;
        let start_y = camera.target.y - v_height / 2.0;
        let end_x = start_x + v_width / camera.zoom;
        let end_y = start_y + v_height / camera.zoom;

        self.start_tile_x = (start_x / TILE_SIZE) as i16 - 1;
        self.start_tile_y = (start_y / TILE_SIZE) as i16;
        self.end_tile_x = (end_x / TILE_SIZE) as i16 + 2;
        self.end_tile_y = (end_y / TILE_SIZE) as i16 + 2;

        for y in (self.start_tile_y..=self.end_tile_y).rev() {
            for x in (self.start_tile_x..=self.end_tile_x).rev() {
                let cord = MapCord::new(x, y);

                if !TileMap::is_tile_in_bounds_no_ref(self.map_dimensions, cord) {
                    continue;
                }

                let index = TileMap::cords_to_index(self.map_dimensions, cord);

                if let Object::NoObject = object_grid[index] {
                    continue;
                }

                object_grid[index].update(dt);

                if !found_hovering {
                    if object_grid[index].is_point_intersecting(mouse_utils::mouse_world_coords(
                        input_state.mouse_pos,
                        camera,
                        window_width,
                        window_height,
                        v_width,
                        v_height,
                    )) {
                        found_hovering = true;
                        object_grid[index].get_mut_data().is_hovering = true;
                    }
                }
            }
        }
    }

    pub fn draw(
        &self,
        day_night_cycle: &DayNightCycle,
        object_grid: &MapObjectGrid,
        d: &mut RaylibDrawHandle,
        texture: &Texture2D,
    ) {
        let mut hover_obj: Option<&Object> = None;

        for y in self.start_tile_y..=self.end_tile_y {
            // separate shadow pass specifcialyl so the shadows dont cross over same row objects
            for x in self.start_tile_x..=self.end_tile_x {
                let cord = MapCord::new(x, y);

                if !TileMap::is_tile_in_bounds_no_ref(self.map_dimensions, cord) {
                    continue;
                }

                let index = TileMap::cords_to_index(self.map_dimensions, cord);

                if let Object::NoObject = object_grid[index] {
                    continue;
                }

                object_grid[index].draw_shadow(
                    d,
                    texture,
                    day_night_cycle.current_shadow_shear,
                    day_night_cycle.current_shadow_scale_y,
                );
            }
            for x in self.start_tile_x..=self.end_tile_x {
                let cord = MapCord::new(x, y);

                if !TileMap::is_tile_in_bounds_no_ref(self.map_dimensions, cord) {
                    continue;
                }

                let index = TileMap::cords_to_index(self.map_dimensions, cord);

                if let Object::NoObject = object_grid[index] {
                    continue;
                }

                object_grid[index].draw(d, texture);

                if let None = hover_obj {
                    if object_grid[index].get_data().is_hovering {
                        hover_obj = Some(&object_grid[index]);
                    }
                }
            }
        }

        if let Some(obj) = hover_obj {
            obj.draw_hover(d, texture);
        }
    }
}
