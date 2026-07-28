use raylib::{drawing::RaylibDrawHandle, texture::Texture2D};

use crate::{
    GameContext, TILE_SIZE,
    entities::{character::Character, object::Object},
    map::tile_map::{self, MapDimensions, MapObjectGrid},
    utils::{map_cord::MapCord, map_utils, mouse_utils},
};

pub struct EntityManager {
    characters: Vec<Character>,
    map_dimensions: MapDimensions,
    start_tile_x: i16,
    start_tile_y: i16,
    end_tile_x: i16,
    end_tile_y: i16,
}

impl EntityManager {
    pub fn new(map_dimensions: MapDimensions) -> Self {
        return EntityManager {
            characters: Vec::with_capacity(200),
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
        game_context: &mut GameContext,
        zoom: u32,
        dt: f32,
    ) {
        let mut found_hovering: bool = false;
        let v_width = (game_context.window_width / zoom) as i16;
        let v_height = (game_context.window_height / zoom) as i16;

        let start_x = game_context.camera.target.x as i16 - v_width / 2;
        let start_y = game_context.camera.target.y as i16 - v_height / 2;
        let end_x = start_x + v_width;
        let end_y = start_y + v_height;

        self.start_tile_x = start_x / TILE_SIZE as i16 - 1;
        self.start_tile_y = start_y / TILE_SIZE as i16;
        self.end_tile_x = end_x / TILE_SIZE as i16 + 2;
        self.end_tile_y = end_y / TILE_SIZE as i16 + 2;

        // sort the characters by tile index (important)
        self.characters.sort_by_key(|c| {
            map_utils::cords_to_index(
                self.map_dimensions,
                MapCord::new(c.get_data().pos.x as i16 / 8, c.get_data().pos.y as i16 / 8),
            )
        });

        for y in (self.start_tile_y..=self.end_tile_y).rev() {
            for x in (self.start_tile_x..=self.end_tile_x).rev() {
                let cord = MapCord::new(x, y);

                if !map_utils::is_tile_in_bounds(self.map_dimensions, cord) {
                    continue;
                }

                let index = map_utils::cords_to_index(self.map_dimensions, cord);

                if let Object::NoObject = object_grid[index] {
                    continue;
                }

                object_grid[index].update(
                    dt,
                    &game_context.day_night_cycle,
                    game_context.total_game_time,
                    &mut game_context.rng,
                );

                if !found_hovering {
                    if object_grid[index]
                        .is_point_intersecting(mouse_utils::mouse_world_coords(&game_context))
                    {
                        found_hovering = true;
                        object_grid[index].get_mut_data().is_hovering = true;
                    }
                }
            }
        }
    }

    pub fn draw(&self, object_grid: &MapObjectGrid, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        let mut hover_obj: Option<&Object> = None;

        let mut current_char_list_index = 0;
        let mut last_row_final_char_index = current_char_list_index;
        
        for y in self.start_tile_y..=self.end_tile_y {

            
            // separate shadow pass specifcialyl so the shadows dont cross over same row objects
            for x in self.start_tile_x..=self.end_tile_x {
                let cord = MapCord::new(x, y);

                if !map_utils::is_tile_in_bounds(self.map_dimensions, cord) {
                    continue;
                }

                let current_tile_index = map_utils::cords_to_index(self.map_dimensions, cord);

                while current_char_list_index < self.characters.len() {

                    // get the next character tile index
                    let next_char_tile_index = self.characters[current_char_list_index].get_tile_index(self.map_dimensions);

                    // if the next character index hasnt happened yet, then break, its too early to draw
                    if next_char_tile_index > current_tile_index {
                        break;
                    }

                    // wow! the character index is currently matching the current tile index! yes! draw the character!
                    if next_char_tile_index == current_tile_index {
                        self.characters[current_char_list_index].draw_shadow(d, texture);
                    }

                    // inc the list index so that in the next iteration, the next char is being compared
                    current_char_list_index += 1;
                }

                if let Object::NoObject = object_grid[current_tile_index] {
                    continue;
                }

                object_grid[current_tile_index].draw_shadow(d, texture);
            }

            current_char_list_index = last_row_final_char_index;
            
            for x in self.start_tile_x..=self.end_tile_x {
                let cord = MapCord::new(x, y);

                if !map_utils::is_tile_in_bounds(self.map_dimensions, cord) {
                    continue;
                }

                let current_tile_index = map_utils::cords_to_index(self.map_dimensions, cord);

                while current_char_list_index < self.characters.len() {
                    let next_char_tile_index = self.characters[current_char_list_index].get_tile_index(self.map_dimensions);

                    if next_char_tile_index > current_tile_index {
                        break;
                    }

                    if next_char_tile_index == current_tile_index {
                        self.characters[current_char_list_index].draw(d, texture);
                    }

                    current_char_list_index += 1;
                }

                if let Object::NoObject = object_grid[current_tile_index] {
                    continue;
                }

                object_grid[current_tile_index].draw(d, texture);

                if let None = hover_obj {
                    if object_grid[current_tile_index].get_data().is_hovering {
                        hover_obj = Some(&object_grid[current_tile_index]);
                    }
                }
            }

            last_row_final_char_index = current_char_list_index;
        }

        if let Some(obj) = hover_obj {
            obj.draw_hover(d, texture);
        }
    }
}
