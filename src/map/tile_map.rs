use std::collections::HashMap;

use basic_raylib_core::graphics::sprite_animation::SpriteAnimationInstance;
use raylib::{
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
    math::{Rectangle, Vector2},
};

use crate::{
    GameContext, TILE_SIZE,
    entities::object::Object,
    map::{
        map_cell::{CELL_SIZE, MapCell},
        map_gen_functions,
        tile::{
            LakeSpriteData, RiverSpriteData,
            TileType::{self},
        },
        tile_map_animation_data::{
            GRASS_TILE, INLET_ANIMS, LAKE_TILE_ANIM, LAKE_TILE_CORNER_ANIMATION_REFERENCE,
            LAKE_TILE_SHORE_ANIMATION_REFERENCE, OUTLETS_ANIMS, REGULAR_TILE_FRAME_DURATION,
            RIVER_TILE_CORNER_ANIMS, RIVER_TILE_STRAIGHT_ANIMS, RIVER_TILE_T_SECTION_ANIMS,
            RiverType::{self},
            SHORE_AND_CORNER_AND_RIVER_FRAME_DURATION, SpriteFlip,
        },
    },
    utils::{
        direction_utils::ORTHOGONAL_DELTAS, map_cord::MapCord, map_utils::cords_to_index,
        vector2_utils,
    },
};

pub type MapTileGrid = Vec<TileType>;
pub type MapObjectGrid = Vec<Object>;

#[derive(Clone, Copy)]
pub struct MapDimensions {
    pub width: u16,
    pub height: u16,
}

impl MapDimensions {
    pub fn total_tiles(&self) -> usize {
        return self.width as usize * self.height as usize;
    }
}

pub struct TileMap {
    pub map_tile_grid: MapTileGrid,
    pub map_object_grid: MapObjectGrid,
    pub map_cell_grid: Vec<MapCell>,
    pub map_dimensions: MapDimensions,
    lake_sprite_data: HashMap<MapCord, LakeSpriteData>,
    river_sprite_data: HashMap<MapCord, RiverSpriteData>,
    lake_shore_corner_tile_anim_instance: SpriteAnimationInstance,
    default_tile_anim_instance: SpriteAnimationInstance,
    river_tile_anim_instance: SpriteAnimationInstance,
}

impl TileMap {
    pub fn generate_map(map_width: u16, map_height: u16, game_context: &mut GameContext) -> Self {
        let map_dimensions = MapDimensions {
            width: map_width,
            height: map_height,
        };

        let mut map_cell_grid = map_gen_functions::generate_cell_grid(map_dimensions);

        let total_map_length = map_dimensions.total_tiles();

        let mut tile_grid: MapTileGrid = vec![TileType::Grass; total_map_length];

        let mut object_grid = Vec::new();
        object_grid.resize_with(total_map_length, || Object::NoObject);

        let (forest_lake_tiles, grass_lake_tiles) =
            map_gen_functions::create_lakes(&mut tile_grid, map_dimensions, &mut game_context.rng);
        println!("Lakes created!");

        let lake_sprite_data =
            map_gen_functions::set_lake_shore_and_corner_sprites(&tile_grid, map_dimensions);
        println!("Lake sprites added!");

        let all_river_tiles = map_gen_functions::create_rivers(
            &mut tile_grid,
            &lake_sprite_data,
            map_dimensions,
            &mut game_context.rng,
        );
        println!("River generated!");

        let river_sprite_data = map_gen_functions::set_river_tile_animations(
            &all_river_tiles,
            &tile_grid,
            map_dimensions,
        );
        println!("River sprites added!");

        map_gen_functions::spawn_forests_around_lakes(
            &tile_grid,
            &mut object_grid,
            forest_lake_tiles,
            map_dimensions,
            &mut map_cell_grid,
            &mut game_context.rng,
        );
        println!("Made forest lakes!");

        map_gen_functions::spawn_standalone_forests(
            &tile_grid,
            &mut object_grid,
            map_dimensions,
            &mut map_cell_grid,
            &mut game_context.rng,
        );
        println!("Forests created!");

        map_gen_functions::spawn_standalone_trees(
            &tile_grid,
            &mut object_grid,
            map_dimensions,
            &mut map_cell_grid,
            &mut game_context.rng,
        );
        println!("Standalone trees created!");

        map_gen_functions::spawn_standalone_grass(
            &tile_grid,
            &mut object_grid,
            map_dimensions,
            &mut map_cell_grid,
            game_context,
        );
        println!("Standalone grass created!");

        map_gen_functions::spawn_grass_around_lakes(
            &tile_grid,
            &mut object_grid,
            grass_lake_tiles,
            map_dimensions,
            &mut map_cell_grid,
            game_context,
        );
        println!("Made grass around lakes!");

        map_gen_functions::spawn_grass_around_rivers(
            &tile_grid,
            &mut object_grid,
            &river_sprite_data,
            map_dimensions,
            &mut map_cell_grid,
            game_context,
        );
        println!("Made grass around rivers!");

        map_gen_functions::spawn_fields_of_grass(
            &tile_grid,
            &mut object_grid,
            map_dimensions,
            &mut map_cell_grid,
            game_context,
        );
        println!("Made fields of grass!");

        //SpawnGrassAroundSomeTrees();
        //SetGrassTileGrowMultiplier();
        return TileMap {
            map_tile_grid: tile_grid,
            map_object_grid: object_grid,
            map_cell_grid,
            map_dimensions,
            lake_sprite_data,
            river_sprite_data: river_sprite_data,
            lake_shore_corner_tile_anim_instance: SpriteAnimationInstance::new(),
            default_tile_anim_instance: SpriteAnimationInstance::new(),
            river_tile_anim_instance: SpriteAnimationInstance::new(),
        };
    }

    pub fn update(&mut self, dt: f32) {
        self.lake_shore_corner_tile_anim_instance.current_frame_time += dt;
        if self.lake_shore_corner_tile_anim_instance.current_frame_time
            >= SHORE_AND_CORNER_AND_RIVER_FRAME_DURATION
        {
            self.lake_shore_corner_tile_anim_instance
                .current_frame_index += 1;
            self.lake_shore_corner_tile_anim_instance
                .current_frame_index %= 2;
            self.lake_shore_corner_tile_anim_instance.current_frame_time = 0.0;
        }

        self.default_tile_anim_instance.current_frame_time += dt;
        if self.default_tile_anim_instance.current_frame_time >= REGULAR_TILE_FRAME_DURATION {
            self.default_tile_anim_instance.current_frame_index += 1;
            self.default_tile_anim_instance.current_frame_index %= 4;
            self.default_tile_anim_instance.current_frame_time = 0.0;
        }

        self.river_tile_anim_instance.current_frame_time += dt;
        if self.river_tile_anim_instance.current_frame_time
            >= SHORE_AND_CORNER_AND_RIVER_FRAME_DURATION
        {
            self.river_tile_anim_instance.current_frame_index += 1;
            self.river_tile_anim_instance.current_frame_index %= 4;
            self.river_tile_anim_instance.current_frame_time = 0.0;
        }

        // spawn grass randomly over time
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, game_context: &GameContext) {
        let start = game_context.camera.target - game_context.camera.offset;
        let end = start + Vector2::new(game_context.v_width as f32, game_context.v_height as f32);

        let start_tile_x = (start.x / TILE_SIZE) as i16 - 1;
        let start_tile_y = (start.y / TILE_SIZE) as i16;
        let end_tile_x = (end.x / TILE_SIZE) as i16 + 2;
        let end_tile_y = (end.y / TILE_SIZE) as i16 + 2;

        for y in start_tile_y..=end_tile_y {
            for x in start_tile_x..=end_tile_x {
                let pos = Vector2::new(
                    (x as f32 * TILE_SIZE).floor(),
                    (y as f32 * TILE_SIZE).floor(),
                );

                if !self.is_tile_in_bounds(x, y) {
                    continue;
                }

                let cord = MapCord::new(x, y);

                let tile_type = self.get_tile_from_x_y(x as u16, y as u16);

                match tile_type {
                    TileType::Grass => GRASS_TILE.draw(d, pos, &game_context.texture),
                    TileType::Lake => {
                        // draw base
                        LAKE_TILE_ANIM.draw(
                            &self.default_tile_anim_instance,
                            d,
                            pos,
                            &game_context.texture,
                        );

                        if let Some(lake_data) = self.lake_sprite_data.get(&cord) {
                            if lake_data.shore_animation_index != 0 {
                                LAKE_TILE_SHORE_ANIMATION_REFERENCE
                                    [lake_data.shore_animation_index as usize - 1]
                                    .draw(
                                        &self.lake_shore_corner_tile_anim_instance,
                                        d,
                                        pos,
                                        &game_context.texture,
                                    );
                            }
                            if lake_data.corner_animation_index != 0 {
                                LAKE_TILE_CORNER_ANIMATION_REFERENCE
                                    [lake_data.corner_animation_index as usize - 1]
                                    .draw(
                                        &self.lake_shore_corner_tile_anim_instance,
                                        d,
                                        pos,
                                        &game_context.texture,
                                    );
                            }
                        }
                    }
                    TileType::River => {
                        let riv_data = self.river_sprite_data.get(&cord).unwrap();

                        match riv_data.river_type {
                            RiverType::Straight => {
                                let anim = &RIVER_TILE_STRAIGHT_ANIMS
                                    [riv_data.river_sprite_index as usize];

                                let (flp_h, flp_v) = match anim.1 {
                                    SpriteFlip::None => (false, false),
                                    SpriteFlip::Horizontal => (true, false),
                                    SpriteFlip::Vertical => (false, true),
                                };

                                anim.0.draw_flp(
                                    &self.river_tile_anim_instance,
                                    d,
                                    pos,
                                    &game_context.texture,
                                    flp_h,
                                    flp_v,
                                );
                            }
                            RiverType::Corner => {
                                let anim =
                                    &RIVER_TILE_CORNER_ANIMS[riv_data.river_sprite_index as usize];

                                let (flp_h, flp_v) = match anim.1 {
                                    SpriteFlip::None => (false, false),
                                    SpriteFlip::Horizontal => (true, false),
                                    SpriteFlip::Vertical => (false, true),
                                };

                                anim.0.draw_flp(
                                    &self.river_tile_anim_instance,
                                    d,
                                    pos,
                                    &game_context.texture,
                                    flp_h,
                                    flp_v,
                                );
                            }
                            RiverType::TSection => {
                                let anim = &RIVER_TILE_T_SECTION_ANIMS
                                    [riv_data.river_sprite_index as usize];

                                let (flp_h, flp_v) = match anim.1 {
                                    SpriteFlip::None => (false, false),
                                    SpriteFlip::Horizontal => (true, false),
                                    SpriteFlip::Vertical => (false, true),
                                };
                                anim.0.draw_flp(
                                    &self.river_tile_anim_instance,
                                    d,
                                    pos,
                                    &game_context.texture,
                                    flp_h,
                                    flp_v,
                                );
                            }
                            RiverType::Inlet => {
                                let anim = &INLET_ANIMS[riv_data.river_sprite_index as usize];

                                let (flp_h, flp_v) = match anim.1 {
                                    SpriteFlip::None => (false, false),
                                    SpriteFlip::Horizontal => (true, false),
                                    SpriteFlip::Vertical => (false, true),
                                };

                                anim.0.draw_flp(
                                    &self.river_tile_anim_instance,
                                    d,
                                    pos,
                                    &game_context.texture,
                                    flp_h,
                                    flp_v,
                                );
                            }
                            RiverType::Outlet => {
                                let anim = &OUTLETS_ANIMS[riv_data.river_sprite_index as usize];

                                let (flp_h, flp_v) = match anim.1 {
                                    SpriteFlip::None => (false, false),
                                    SpriteFlip::Horizontal => (true, false),
                                    SpriteFlip::Vertical => (false, true),
                                };

                                anim.0.draw_flp(
                                    &self.river_tile_anim_instance,
                                    d,
                                    pos,
                                    &game_context.texture,
                                    flp_h,
                                    flp_v,
                                );
                            }
                        }
                    }
                }
            }
        }
        //self.dbg_cells(d);
    }

    fn dbg_cells(&self, d: &mut RaylibDrawHandle<'_>) {
        for i in 0..self.map_cell_grid.len() {
            let cells_wide = self.map_dimensions.width / CELL_SIZE;
            let cell_y = i as u16 / cells_wide;
            let cell_x = i as u16 % cells_wide;

            let pos = Vector2::new(
                (cell_x * CELL_SIZE) as f32 * TILE_SIZE,
                (cell_y * CELL_SIZE) as f32 * TILE_SIZE,
            );
            let rect = Rectangle::new(
                pos.x,
                pos.y,
                CELL_SIZE as f32 * TILE_SIZE,
                CELL_SIZE as f32 * TILE_SIZE,
            );

            d.draw_rectangle_lines_ex(rect, 3.0, Color::WHITE);
            d.draw_text(
                &format!("{}", i),
                pos.x as i32 + 5,
                pos.y as i32 + 5,
                10,
                Color::WHITE,
            );

            // REMOVE &MUT SELF
            let cell = self.get_cell_at_cord(MapCord::from_vec2(pos));
            d.draw_text(
                &format!("{}", cell.unwrap().objects_in_cell.len()),
                pos.x as i32 + 5,
                pos.y as i32 + 20,
                10,
                Color::BLACK,
            );
        }
    }

    pub fn get_tile_from_x_y(&self, x: u16, y: u16) -> TileType {
        let index = y as usize * self.map_dimensions.width as usize + x as usize;
        return self.map_tile_grid[index as usize];
    }

    pub fn is_tile_in_bounds(&self, x: i16, y: i16) -> bool {
        let x_in_bounds = x >= 0 && x < self.map_dimensions.width as i16;
        let y_in_bounds = y >= 0 && y < self.map_dimensions.height as i16;
        return x_in_bounds && y_in_bounds;
    }

    pub fn is_cord_in_bounds(&self, cord: MapCord) -> bool {
        let x_in_bounds = cord.x >= 0 && cord.x < self.map_dimensions.width as i16;
        let y_in_bounds = cord.y >= 0 && cord.y < self.map_dimensions.height as i16;
        return x_in_bounds && y_in_bounds;
    }

    /// returns an optional mutable reference to the cell at the cord, if oob, returns None
    pub fn get_mut_cell_at_cord(&mut self, cord: MapCord) -> Option<&mut MapCell> {
        if !self.is_cord_in_bounds(cord) {
            return None;
        }

        let cell_x = cord.x as u16 / CELL_SIZE;
        let cell_y = cord.y as u16 / CELL_SIZE;
        let num_of_cells_wide = self.map_dimensions.width / CELL_SIZE;

        return Some(&mut self.map_cell_grid[(cell_y * num_of_cells_wide + cell_x) as usize]);
    }

    pub fn get_cell_at_cord(&self, cord: MapCord) -> Option<&MapCell> {
        if !self.is_cord_in_bounds(cord) {
            return None;
        }

        let cell_x = cord.x as u16 / CELL_SIZE;
        let cell_y = cord.y as u16 / CELL_SIZE;
        let num_of_cells_wide = self.map_dimensions.width / CELL_SIZE;

        return Some(&self.map_cell_grid[(cell_y * num_of_cells_wide + cell_x) as usize]);
    }

    pub fn get_3_x_3_cell_grid(&self, cord: MapCord) -> Vec<&MapCell> {
        let mut cells: Vec<&MapCell> = Vec::new();

        // add the current cell
        cells.push(self.get_cell_at_cord(cord).unwrap());

        for dir in ORTHOGONAL_DELTAS {
            let check_cord = cord + (dir * CELL_SIZE as i32);
            if let Some(c) = self.get_cell_at_cord(check_cord) {
                cells.push(c);
            }
        }

        return cells;
    }

    /// adds the objects index (where it is on the map) to the cell's array of indices
    pub fn add_obj_to_cell(&mut self, cord_of_obj: MapCord) {
        let m_d = self.map_dimensions;
        let map_cell = self.get_mut_cell_at_cord(cord_of_obj).unwrap();
        let idx = cords_to_index(m_d, cord_of_obj);
        map_cell.add_obj(idx);
    }
}
