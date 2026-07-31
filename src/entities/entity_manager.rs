use basic_raylib_core::system::input_handler;
use raylib::{drawing::RaylibDrawHandle, math::Rectangle, texture::Texture2D};

use crate::{
    GameContext, TILE_SIZE, entities::{
        character::Character,
        object::{self, Object},
    }, map::tile_map::{self, MapDimensions, MapObjectGrid, TileMap}, systems::{
        entity_selecting_manager::{EntitySelectingManager, SelectingMode},
        select_rect::SelectRect,
    }, utils::{
        entity_utils::get_char_by_index, map_cord::MapCord, map_utils, mouse_utils::{self, mouse_world_coords},
    },
};

/// num of tiles to the left and top of the cam view where objects are still being updated and drawn
/// only 1 tile to the right and bottom are extended, this is due to the fact that the position
/// being tested is the top left corner of each object's visible rectangle.
pub const UPDATE_MARGIN: f32 = 4.0;
pub const DRAW_SHADOW_EXTRA_MARGIN: f32 = 2.0;

/// houses the character itself as well as the appropriate render index
/// honestly, I know that render_index could be a field inside of CharacterData,
/// however, since the entity manager is the only thing reading it and manipulating it,
/// I decided to abstract it. This struct + one method for getting the render_index on a character
/// are the only things that will ever have to worry about it. If you're reading this,
/// feek free to let me know what you think about this design choice
pub struct CharacterEntry {
    pub character: Character,
    pub unique_id: usize,
    render_index: usize,
}

pub struct EntityManager {
    next_character_id: usize,
    characters: Vec<CharacterEntry>,
    map_dimensions: MapDimensions,
    start_tile_x: i16,
    start_tile_y: i16,
    end_tile_x: i16,
    end_tile_y: i16,
}

impl EntityManager {
    pub fn new(map_dimensions: MapDimensions) -> Self {
        return EntityManager {
            next_character_id: 0,
            characters: Vec::with_capacity(200),
            map_dimensions,
            start_tile_x: 0,
            start_tile_y: 0,
            end_tile_x: 0,
            end_tile_y: 0,
        };
    }

    pub fn add_character(&mut self, character: Character) {
        let render_index = character.get_render_tile_index(self.map_dimensions);

        self.characters.push(CharacterEntry {
            character,
            render_index,
            unique_id: self.next_character_id,
        });

        self.next_character_id += 1;
    }

    pub fn update(
        &mut self,
        map: &mut TileMap,
        game_context: &mut GameContext,
        selector: &mut EntitySelectingManager,
        select_rect: &SelectRect,
        zoom: u32,
    ) {
        if game_context.input_state.left_clicked_once {
            match selector.selecting_mode {
                SelectingMode::Objects => selector.deselect_objs(&mut map.map_object_grid),
                SelectingMode::Characters => selector.deselect_chars(&mut self.characters),
            }
        }

        let v_width = (game_context.logical_window_width / zoom) as f32;
        let v_height = (game_context.logical_window_height / zoom) as f32;

        // add margins to the update area
        let start_x = (game_context.camera.target.x - v_width / 2.0) - TILE_SIZE * UPDATE_MARGIN;
        let start_y = (game_context.camera.target.y - v_height / 2.0) - TILE_SIZE * UPDATE_MARGIN;

        // add a 1 block margin on the right and 2.0 to bottom as well
        // the reason for the 2.0 is because, unlike the x axis, some objects
        // actually contain a y offset that separates their logical pos and their
        // visible pos. so, if the object (which max atm is going to be 2.0 x 2.0 tiles large)
        // is 2 tiles tall, containing this offset, such as the tree, which logically should be drawn
        // at an offset so the base of it is at the bottom of the logical tile its on, instead of the one
        // below it, then if i only added 1 to the bottom instead of 2, the logical pos and the visible
        // rect would move out of update area and camera view respectively on the exact same frame.
        let end_x = start_x + v_width + TILE_SIZE * (UPDATE_MARGIN + 1.0);
        let end_y = start_y + v_height + TILE_SIZE * (UPDATE_MARGIN + 2.0);

        self.start_tile_x = (start_x / TILE_SIZE) as i16;
        self.start_tile_y = (start_y / TILE_SIZE) as i16;
        self.end_tile_x = (end_x / TILE_SIZE) as i16;
        self.end_tile_y = (end_y / TILE_SIZE) as i16;

        // update with the actual tiles being updated, not just the actual rectangle being used for those values
        game_context.update_rect = Rectangle::new(
            self.start_tile_x as f32 * TILE_SIZE,
            self.start_tile_y as f32 * TILE_SIZE,
            (self.end_tile_x - self.start_tile_x) as f32 * TILE_SIZE,
            (self.end_tile_y - self.start_tile_y) as f32 * TILE_SIZE,
        );

        // used for single target selections when no select rect
        let mut hover_char: Option<usize> = None;

        // used for when the select rect is dragging
        let mut hover_chars: Vec<usize> = Vec::new();

        for character in &mut self.characters {
            character.character.update(game_context, map);

            character.render_index = character
                .character
                .get_render_tile_index(self.map_dimensions);

            // only do this shit if the selecting mode is proper, otherwise theres no point
            if let SelectingMode::Characters = selector.selecting_mode {
                match select_rect.select_range_active {
                    // count each character inside of the rectangle if its dragging, they should all be drawing with hover
                    true => {
                        if character
                            .character
                            .get_hover_rect()
                            .check_collision_recs(&select_rect.rectangle)
                        {
                            character.character.get_mut_data().is_hovering = true;
                            hover_chars.push(character.unique_id);
                        }
                    }
                    // if the select rect is not currently active, then just go as usual with the normal single slot
                    false => {
                        if character
                            .character
                            .get_hover_rect()
                            .check_collision_point_rec(mouse_world_coords(game_context))
                        {
                            hover_char = Some(character.unique_id);
                        }
                    }
                }
            }
        }
        
        if let SelectingMode::Characters = selector.selecting_mode {
            if let Some(char_idx) = hover_char {
                get_char_by_index(&mut self.characters, char_idx)
                    .character
                    .get_mut_data()
                    .is_hovering = true;

                if game_context.input_state.left_clicked_once {
                    selector.select_single_char(&mut self.characters, char_idx);
                }
            }
        }

        let mut hover_obj: Option<usize> = None;
        let mut hover_objs: Vec<usize> = Vec::new();

        for y in self.start_tile_y..=self.end_tile_y {
            for x in self.start_tile_x..=self.end_tile_x {
                let cord = MapCord::new(x, y);

                if !map_utils::is_tile_in_bounds(self.map_dimensions, cord) {
                    continue;
                }

                let index = map_utils::cords_to_index(self.map_dimensions, cord);

                let obj = &mut map.map_object_grid[index];

                obj.update(game_context);

                if let Object::NoObject = obj {
                    continue;
                }

                if let SelectingMode::Objects = selector.selecting_mode {
                    match select_rect.select_range_active {
                        // put all objects in drag rect into the rectangle
                        true => {
                            if select_rect.rectangle.check_collision_recs(&obj.get_data().hover_rect) {
                                obj.get_mut_data().is_hovering = true;
                                hover_objs.push(index);
                            }
                        },
                        // carry on as normal if not dragging
                        false => {
                            if map.map_object_grid[index].is_point_intersecting(
                                mouse_utils::mouse_world_coords(&game_context),
                            ) {
                                hover_obj = Some(index);
                            }
                        }
                    }
                }
            }
        }

        //println!("hover_objs: {}", hover_objs.len());

        if let SelectingMode::Objects = selector.selecting_mode {
            if let Some(idx) = hover_obj {
                map.map_object_grid[idx].get_mut_data().is_hovering = true;

                if game_context.input_state.left_clicked_once {
                    selector.select_single_obj(&mut map.map_object_grid, idx);
                }
            }
        }

        if select_rect.is_selecting_this_frame {
            match selector.selecting_mode {
                SelectingMode::Objects => {
                    selector.select_multiple_objs(&mut map.map_object_grid, hover_objs);
                },
                SelectingMode::Characters => {
                    selector.select_multiple_chars(&mut self.characters, hover_chars);
                },
            }
        }

        // sort the characters by tile index (important)
        self.characters.sort_by_key(|c| c.render_index);
    }

    pub fn draw(&self, object_grid: &MapObjectGrid, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        let mut current_char_list_index = 0;
        let mut last_row_final_char_index = current_char_list_index;

        // draw objects within update view,
        // however, extend the draw range for shadows specifically in the right and bottom
        let shadow_draw_end_x = self.end_tile_x + DRAW_SHADOW_EXTRA_MARGIN as i16;

        // subtract 1 because the original end tile y is one tile further out from viewing area than end tile x
        let shadow_draw_end_y = self.end_tile_y + DRAW_SHADOW_EXTRA_MARGIN as i16 - 1;

        for y in self.start_tile_y..=shadow_draw_end_y {
            // separate shadow pass specifcially so the shadows dont cross over same row objects
            for x in self.start_tile_x..=shadow_draw_end_x {
                let cord = MapCord::new(x, y);

                if !map_utils::is_tile_in_bounds(self.map_dimensions, cord) {
                    continue;
                }

                let current_tile_index = map_utils::cords_to_index(self.map_dimensions, cord);

                match object_grid[current_tile_index] {
                    Object::NoObject => (),
                    _ => object_grid[current_tile_index].draw_shadow(d, texture),
                }

                while current_char_list_index < self.characters.len() {
                    // characters are rendered one tile later than their actual pos tile.
                    // since rendering uses the tile indices in a single dimension,
                    // objects immediately to the right would draw over the character
                    // when they should be drawn behind it
                    let next_char_tile_index =
                        self.characters[current_char_list_index].render_index;

                    // if the next character index hasnt happened yet, then break, its too early to draw
                    if next_char_tile_index > current_tile_index {
                        break;
                    }

                    // wow! the character index is currently matching the current tile index! yes! draw the character!
                    if next_char_tile_index == current_tile_index {
                        self.characters[current_char_list_index]
                            .character
                            .draw_shadow(d, texture);
                    }

                    // inc the list index so that in the next iteration, the next char is being compared
                    current_char_list_index += 1;
                }
            }

            current_char_list_index = last_row_final_char_index;

            for x in self.start_tile_x..=self.end_tile_x {
                let cord = MapCord::new(x, y);

                if !map_utils::is_tile_in_bounds(self.map_dimensions, cord) {
                    continue;
                }

                let current_tile_index = map_utils::cords_to_index(self.map_dimensions, cord);

                match object_grid[current_tile_index] {
                    Object::NoObject => (),
                    _ => {
                        let object = &object_grid[current_tile_index];
                        let object_data = object.get_data();

                        object.draw(d, texture);

                        if object_data.is_hovering {
                            object.draw_hover(d, texture);
                        }

                        if object_data.is_selected {
                            object.draw_selected(d, texture);
                        }
                    }
                }

                while current_char_list_index < self.characters.len() {
                    let next_char_tile_index =
                        self.characters[current_char_list_index].render_index;

                    if next_char_tile_index > current_tile_index {
                        break;
                    }

                    if next_char_tile_index == current_tile_index {
                        let character = &self.characters[current_char_list_index].character;
                        let character_data = character.get_data();

                        character.draw(d, texture);

                        if character_data.is_hovering {
                            character.draw_hover(d, texture);
                        }

                        if character_data.is_selected {
                            character.draw_selected(d, texture);
                        }
                    }

                    current_char_list_index += 1;
                }
            }

            last_row_final_char_index = current_char_list_index;
        }
    }
}
