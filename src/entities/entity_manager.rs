use rand::RngExt;
use raylib::{
    drawing::RaylibDrawHandle,
    math::{Rectangle, Vector2},
    texture::Texture2D,
};
use zander_game_core_rs::raylib::sprite::Sprite;

use crate::{
    GameContext, TILE_SIZE, entities::{character::Character, object::Object}, map::tile_map::{MapDimensions, MapObjectGrid, TileMap}, systems::{
        action_button_manager::ActionButtonManager,
        entity_selecting_manager::{EntitySelectingManager, SelectingMode},
        select_rect::SelectRect,
    }, utils::{map_cord::MapCord, map_utils, mouse_utils::mouse_world_coords, rectangle_utils::center_of_rect},
};

/// num of tiles to the left and top of the cam view where objects are still being updated and drawn
/// only 1 tile to the right and bottom are extended, this is due to the fact that the position
/// being tested is the top left corner of each object's visible rectangle.
pub const UPDATE_MARGIN: f32 = 4.0;
pub const DRAW_SHADOW_EXTRA_MARGIN: f32 = 2.0;

static HOVER_SELECT_PARTICLE_SPRITE: Sprite = Sprite::new(64, 72, 1, 1);

pub struct CharacterEntry {
    pub character: Character,
    pub unique_id: usize,
    render_index: usize,
}

pub struct EntityManager {
    next_character_id: usize,
    pub characters: Vec<CharacterEntry>,
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
        action_button_manager: &mut ActionButtonManager,
        zoom: u32,
    ) {
        // update the action buttons first so that they can interrupt and do their on_click before the next batch of selections starts
        // since left clicking ALWAYS removes the selected objects and clears the action buttons active
        action_button_manager.update(
            game_context,
            selector,
            &mut map.map_object_grid,
            &mut self.characters,
        );

        let right_clicked = game_context.input_state.right_clicked_once;
        let left_clicked = game_context.input_state.left_clicked_once;
        let mouse_pos = mouse_world_coords(game_context);

        let mut was_anything_selected_this_frame = false;

        let are_any_action_buttons_hovering =
            action_button_manager.check_for_buttons_being_hovered();

        if left_clicked {
            match selector.selecting_mode {
                SelectingMode::Objects => selector.deselect_objs(),
                SelectingMode::Characters => selector.deselect_chars(),
            }

            // need to deselect all objs and chars so they dont stay selected even after the button is activated
            if are_any_action_buttons_hovering {
                selector.deselect_chars();
                selector.deselect_objs();
                selector.deselect_move();
            }

            action_button_manager.clear_buttons();

            selector.deselect_move();
        }

        if right_clicked {
            selector.deselect_move();
            selector.deselect_chars();
            selector.deselect_objs();
        }

        self.update_update_rect(game_context, zoom);

        let mut hover_char: Option<&mut CharacterEntry> = None;
        let mut hover_char_for_move: Option<&mut CharacterEntry> = None;
        let mut hover_chars: Vec<&mut CharacterEntry> = Vec::new();
        let mut hover_chars_for_move: Vec<&mut CharacterEntry> = Vec::new();

        let mut moved_anyone = false;
        for character in &mut self.characters {
            let hover_rect = character.character.get_hover_rect();

            if selector.is_deselecting_chars {
                character.character.get_mut_data().is_selected = false;
            }

            if right_clicked {
                if character.character.get_mut_data().is_selected_for_move {
                    character.character.set_move_to(mouse_pos);
                    moved_anyone = true;
                }
            }

            if selector.is_deselecting_move {
                // deselecting move means right mouse button was clicked
                // this means move the character to a new pos
                character.character.get_mut_data().is_selected_for_move = false;
            }

            character.character.update(game_context, map);

            let should_spawn_new_selected_for_move_particle =
                character.character.get_data().is_selected_for_move
                    && game_context.rng.random_bool(game_context.dt as f64 * 10.0);

            if should_spawn_new_selected_for_move_particle {
                spawn_character_selected_for_potential_move_particle(game_context, character);
            }

            character.render_index = character
                .character
                .get_render_tile_index(self.map_dimensions);

            if select_rect.move_select_range_active {
                if hover_rect.check_collision_recs(&select_rect.rectangle) {
                    character.character.get_mut_data().is_hovering_for_move = true;
                    hover_chars_for_move.push(character);
                    continue;
                }
            }

            if let SelectingMode::Characters = selector.selecting_mode {
                match select_rect.select_range_active {
                    // count each character inside of the rectangle if its dragging, they should all be drawing with hover
                    true => {
                        if hover_rect.check_collision_recs(&select_rect.rectangle) {
                            character.character.get_mut_data().is_hovering = true;
                            hover_chars.push(character);
                            continue;
                        }
                    }
                    // if the select rect is not currently active, then just go as usual with the normal single slot
                    false => {
                        let should_be_hovered = !are_any_action_buttons_hovering
                            && !right_clicked
                            && hover_rect.check_collision_point_rec(mouse_pos);

                        if should_be_hovered {
                            hover_char = Some(character);
                            continue;
                        }
                    }
                }
            }

            let should_be_hovered_for_move =
                !are_any_action_buttons_hovering && hover_rect.check_collision_point_rec(mouse_pos);

            if should_be_hovered_for_move {
                hover_char_for_move = Some(character);
            }
        }

        if moved_anyone {
            spawn_mouse_selected_move_particles(game_context, mouse_pos);
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

                obj.update(
                    game_context,
                    selector.is_deselecting_objs,
                    &mut map.map_cell_grid,
                    map.map_dimensions,
                );

                if let Object::NoObject = obj {
                    continue;
                }

                if let SelectingMode::Objects = selector.selecting_mode {
                    match select_rect.select_range_active {
                        // put all objects in drag rect into the rectangle
                        true => {
                            let should_hover_obj = select_rect
                                .rectangle
                                .check_collision_recs(&obj.hover_rect());

                            if should_hover_obj {
                                obj.get_mut_data().is_hovering = true;
                                hover_objs.push(index);
                            }
                        }
                        // carry on as normal if not dragging
                        false => {
                            let should_hover_obj = !are_any_action_buttons_hovering
                                && obj.is_point_intersecting(mouse_pos);

                            if should_hover_obj {
                                hover_obj = Some(index);
                            }
                        }
                    }
                }
            }
        }

        match selector.selecting_mode {
            SelectingMode::Objects => {
                if select_rect.is_selecting_this_frame {
                    selector.select_multiple_objs(&mut map.map_object_grid, hover_objs);
                    was_anything_selected_this_frame = true;
                } else {
                    if let Some(idx) = hover_obj {
                        let obj = &mut map.map_object_grid[idx];

                        obj.get_mut_data().is_hovering = true;

                        if left_clicked {
                            selector.select_single_obj(obj, idx);
                            was_anything_selected_this_frame = true;
                        }
                    }
                }
            }
            SelectingMode::Characters => {
                if select_rect.is_selecting_this_frame {
                    selector.select_multiple_chars(hover_chars);
                    was_anything_selected_this_frame = true;
                } else {
                    if let Some(ch) = hover_char {
                        ch.character.get_mut_data().is_hovering = true;

                        if left_clicked {
                            selector.select_single_char(ch);
                            was_anything_selected_this_frame = true;
                        }
                    }
                }
            }
        }

        if select_rect.is_selecting_for_move_this_frame {
            selector.select_multiple_moves(hover_chars_for_move);
        } else {
            if let Some(ch) = hover_char_for_move {
                ch.character.get_mut_data().is_hovering_for_move = true;

                if right_clicked {
                    selector.select_single_move(ch);
                }
            }
        }

        if was_anything_selected_this_frame {
            let button_base_pos = match select_rect.is_selecting_this_frame {
                true => center_of_rect(select_rect.rectangle),
                false => mouse_world_coords(game_context),
            };

            // this function will reset the action buttons and then check if there are any matches between selected objects and selected characters
            // if it finds any number of matches, it will spawn them at the position passed as an argument
            action_button_manager.try_trigger_match(
                game_context,
                button_base_pos,
                &selector,
                &map.map_object_grid,
                &mut self.characters,
            );
        }

        // sort the characters by tile index (important)
        self.characters.sort_by_key(|c| c.render_index);
    }

    pub fn draw(&self, object_grid: &MapObjectGrid, d: &mut RaylibDrawHandle, texture: &Texture2D, shear_x: f32, scale_y: f32) {
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
                    _ => object_grid[current_tile_index].draw_shadow(d, texture, shear_x, scale_y),
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
                            .draw_shadow(d, texture, shear_x, scale_y);
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

                        if character_data.is_hovering_for_move {
                            character.draw_hover_for_move(d, texture);
                        }
                    }

                    current_char_list_index += 1;
                }
            }

            last_row_final_char_index = current_char_list_index;
        }
    }

    fn update_update_rect(&mut self, game_context: &mut GameContext, zoom: u32) {
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

        let new_update_rect = Rectangle::new(
            self.start_tile_x as f32 * TILE_SIZE,
            self.start_tile_y as f32 * TILE_SIZE,
            (self.end_tile_x - self.start_tile_x) as f32 * TILE_SIZE,
            (self.end_tile_y - self.start_tile_y) as f32 * TILE_SIZE,
        );

        // update with the actual tiles being updated, not just the actual rectangle being used for those values
        game_context.update_rect = new_update_rect;
    }
}

fn spawn_mouse_selected_move_particles(game_context: &mut GameContext, mouse_pos: Vector2) {
    for _ in 0..=game_context.rng.random_range(20..=30) {
        let area_of_effect = Rectangle::new(
            mouse_pos.x - TILE_SIZE / 2.0,
            mouse_pos.y,
            TILE_SIZE,
            TILE_SIZE,
        );
        let p_pos = Vector2::new(
            game_context
                .rng
                .random_range(area_of_effect.x..=area_of_effect.x + area_of_effect.width),
            game_context
                .rng
                .random_range(area_of_effect.y..=area_of_effect.y + area_of_effect.height),
        );
        let p_vel = Vector2::new(0.0, game_context.rng.random_range(-50.0..=-30.0));
        let p_acc = Vector2::new(
            game_context.rng.random_range(-3.0..=3.0),
            game_context.rng.random_range(40.0..=65.0),
        );
        game_context.particle_system.emit(
            &HOVER_SELECT_PARTICLE_SPRITE,
            p_pos,
            p_vel,
            p_acc,
            game_context.rng.random_range(0.5..=0.75),
        );
    }
}

fn spawn_character_selected_for_potential_move_particle(
    game_context: &mut GameContext,
    character: &mut CharacterEntry,
) {
    let data = character.character.get_data();

    let height = character.character.get_hover_rect().height;

    let p_pos = Vector2::new(
        game_context
            .rng
            .random_range(data.pos.x..=data.pos.x + character.character.get_hover_rect().width),
        game_context
            .rng
            .random_range(data.pos.y + height / 2.0..=data.pos.y + height),
    );

    let p_vel = Vector2::new(0.0, game_context.rng.random_range(-50.0..=-45.0));
    let p_acc = Vector2::new(
        game_context.rng.random_range(-3.0..=3.0),
        game_context.rng.random_range(70.0..=80.0),
    );

    game_context.particle_system.emit(
        &HOVER_SELECT_PARTICLE_SPRITE,
        p_pos,
        p_vel,
        p_acc,
        game_context.rng.random_range(0.5..=0.75),
    );
}
