use std::collections::{HashMap, VecDeque};

use basic_raylib_core::graphics::{sprite::Sprite, sprite_animation::SpriteAnimationInstance};
use rand::{RngExt, rngs::ThreadRng};
use raylib::{camera::Camera2D, drawing::RaylibDrawHandle, math::Vector2, texture::Texture2D};

use crate::{
    TILE_SIZE,
    entities::{
        object::Object::{self, TreeObj},
        objects::tree::Tree,
    },
    map::{
        tile::{
            LakeSpriteData, RiverSpriteData,
            TileType::{self},
        },
        tile_map_animation_data::{
            FlowDirection, GRASS_TILE, INLET_ANIMS, LAKE_TILE_ANIM,
            LAKE_TILE_CORNER_ANIMATION_REFERENCE, LAKE_TILE_SHORE_ANIMATION_REFERENCE,
            OUTLETS_ANIMS, REGULAR_TILE_FRAME_DURATION, RIVER_CORNER_ANIM_KEY,
            RIVER_T_SECTION_ANIM_KEY, RIVER_TILE_CORNER_ANIMS, RIVER_TILE_STRAIGHT_ANIMS,
            RIVER_TILE_T_SECTION_ANIMS,
            RiverType::{self},
            SHORE_AND_CORNER_AND_RIVER_FRAME_DURATION, SpriteFlip,
        },
    },
    utils::{
        directional_deltas::{CARDINAL_DELTAS, Direction},
        map_cord::MapCord,
    },
};

type MapTileGrid = Vec<TileType>;
pub type MapObjectGrid = Vec<Object>;
// type MapObjectData = Vec<Object>;

const LAKE_CHANCE: f32 = 0.001;

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
    map_tile_grid: MapTileGrid,
    pub map_object_grid: MapObjectGrid,
    pub map_dimensions: MapDimensions,
    lake_sprite_data: HashMap<MapCord, LakeSpriteData>,
    river_sprite_data: HashMap<MapCord, RiverSpriteData>,
    lake_shore_corner_tile_anim_instance: SpriteAnimationInstance,
    default_tile_anim_instance: SpriteAnimationInstance,
    river_tile_anim_instance: SpriteAnimationInstance,
    rng: ThreadRng,
}

impl TileMap {
    pub fn generate_map(map_width: u16, map_height: u16) -> Self {
        let map_dimensions = MapDimensions {
            width: map_width,
            height: map_height,
        };

        let mut rng = rand::rng();

        let total_map_length = map_dimensions.total_tiles();

        let mut map_tile_grid: MapTileGrid = vec![TileType::Grass; total_map_length];

        let mut map_object_grid = Vec::new();
        map_object_grid.resize_with(total_map_length, || Object::NoObject);

        // ok this looks bad (it is) because the functions purpose is to actually create lakes
        // but it returns a vec of tiles to make forest lakes of which isnt used until way later.
        // ive acknowledged the poor choice. if someone wants to fix it, let me know what you think
        let forest_lake_tiles = Self::create_lakes(&mut map_tile_grid, map_dimensions, &mut rng);
        println!("Lakes created!");

        let lake_sprite_data =
            Self::set_lake_shore_and_corner_sprites(&map_tile_grid, map_dimensions);
        println!("Lake sprites added!");

        let all_river_tiles = Self::create_rivers(
            &mut map_tile_grid,
            &lake_sprite_data,
            map_dimensions,
            &mut rng,
        );
        println!("River generated!");

        let river_sprite_data =
            Self::set_river_tile_animations(&all_river_tiles, &map_tile_grid, map_dimensions);
        println!("River sprites added!");

        Self::create_forest_around_lake(
            &map_tile_grid,
            &mut map_object_grid,
            forest_lake_tiles,
            map_dimensions,
            &mut rng,
        );
        println!("Made forest lakes!");
        //CreateForests();
        //CreateStandAloneTrees();
        //CreateGrass() // maybe if a tile is a tree theres a chance to spawn a bunch of grass around it ... ?
        //SpawnGrassAroundLakes();
        //SpawnGrassAroundRivers(); // i remember the og code had some cracked af math for this one, just use that
        //SetGrassTileGrowMultiplier();
        return TileMap {
            map_tile_grid,
            map_object_grid,
            map_dimensions,
            lake_sprite_data,
            river_sprite_data: river_sprite_data,
            lake_shore_corner_tile_anim_instance: SpriteAnimationInstance::new(),
            default_tile_anim_instance: SpriteAnimationInstance::new(),
            river_tile_anim_instance: SpriteAnimationInstance::new(),
            rng,
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

    pub fn draw(
        &self,
        d: &mut RaylibDrawHandle,
        camera: &Camera2D,
        screen_width: f32,
        screen_height: f32,
        texture: &Texture2D,
    ) {
        let start_x = camera.target.x - (screen_width / camera.zoom) / 2.0;
        let start_y = camera.target.y - (screen_height / camera.zoom) / 2.0;
        let end_x = start_x + screen_width / camera.zoom;
        let end_y = start_y + screen_height / camera.zoom;

        let start_tile_x = (start_x / TILE_SIZE) as i16 - 1;
        let start_tile_y = (start_y / TILE_SIZE) as i16;
        let end_tile_x = (end_x / TILE_SIZE) as i16 + 2;
        let end_tile_y = (end_y / TILE_SIZE) as i16 + 2;

        static OOB_SP: Sprite = Sprite::new(96, 128, 8, 8);

        for y in start_tile_y..=end_tile_y {
            for x in start_tile_x..=end_tile_x {
                let cord = MapCord::new(x, y);
                let pos = Vector2::new(
                    (x as f32 * TILE_SIZE).floor(),
                    (y as f32 * TILE_SIZE).floor(),
                );

                if !self.is_tile_in_bounds(x, y) {
                    OOB_SP.draw(d, pos, texture);
                    continue;
                }

                let tile_type = self.get_tile_from_x_y(x as u16, y as u16);

                match tile_type {
                    TileType::Grass => GRASS_TILE.draw(d, pos, texture),
                    TileType::Lake => {
                        // draw base
                        LAKE_TILE_ANIM.draw(&self.default_tile_anim_instance, d, pos, texture);

                        if let Some(lake_data) = self.lake_sprite_data.get(&cord) {
                            if lake_data.shore_animation_index != 0 {
                                LAKE_TILE_SHORE_ANIMATION_REFERENCE
                                    [lake_data.shore_animation_index as usize - 1]
                                    .draw(
                                        &self.lake_shore_corner_tile_anim_instance,
                                        d,
                                        pos,
                                        texture,
                                    );
                            }
                            if lake_data.corner_animation_index != 0 {
                                LAKE_TILE_CORNER_ANIMATION_REFERENCE
                                    [lake_data.corner_animation_index as usize - 1]
                                    .draw(
                                        &self.lake_shore_corner_tile_anim_instance,
                                        d,
                                        pos,
                                        texture,
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
                                    texture,
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
                                    texture,
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
                                    texture,
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
                                    texture,
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
                                    texture,
                                    flp_h,
                                    flp_v,
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    fn create_lakes(
        tile_grid: &mut MapTileGrid,
        map_dimensions: MapDimensions,
        rng: &mut ThreadRng,
    ) -> Vec<MapCord> {
        let map_len = tile_grid.len() as f32;
        let variance_bound = map_len * LAKE_CHANCE / 5.0;
        let final_variance = rng.random_range(-variance_bound..=variance_bound);
        let num_of_cycles = (map_len * LAKE_CHANCE + final_variance) as i32;
        let mut tree_lake_tiles: Vec<MapCord> = Vec::new();

        for _ in 0..num_of_cycles {
            let is_forest_lake = rng.random_bool(0.03);

            // yes width and height are swapped. i sat down with a notebook and pencil
            // to figure that one out, apparently it wasn't obvious that was the right way
            // but it makes sense since a = w * h, so w = a / h and h = a / w (excuse my dumbness)
            let start_x =
                rng.random_range(0..(tile_grid.len() / map_dimensions.height as usize)) as i16;
            let start_y =
                rng.random_range(0..(tile_grid.len() / map_dimensions.width as usize)) as i16;

            // will only reach this size as long as the queue doesnt run out of tiles
            let target_lake_size = rng.random_range(60..=100);

            let mut tiles_placed = 0;

            let mut next_tiles: VecDeque<MapCord> = VecDeque::new();
            next_tiles.push_back(MapCord::new(start_x, start_y));

            let mut lake_tiles: Vec<MapCord> = Vec::new();

            while next_tiles.len() > 0 && tiles_placed < target_lake_size {
                // guaranteed to be Some(tile) because of the while condition
                let current = next_tiles.pop_front().unwrap();

                // cant use self.is_tile_in_bounds because theres no self yet
                // to avoid inconvenient ass function parameters, im going
                // to just do it manually here

                if !Self::is_tile_in_bounds_no_ref(map_dimensions, current) {
                    continue;
                }

                if Self::get_tile_at_cord_no_self(tile_grid, map_dimensions, current)
                    == TileType::Lake
                {
                    continue;
                }

                let tile_index = Self::cords_to_index(map_dimensions, current);

                tile_grid[tile_index] = TileType::Lake;

                if is_forest_lake {
                    lake_tiles.push(current);
                }

                tiles_placed += 1;

                let chance = 0.8;

                if next_tiles.len() >= target_lake_size {
                    continue;
                }

                for i in 0..CARDINAL_DELTAS.len() {
                    if rng.random_bool(chance) {
                        let dir = CARDINAL_DELTAS[i];
                        next_tiles.push_back(current + dir);
                    }
                }
            }

            if is_forest_lake {
                tree_lake_tiles.append(&mut lake_tiles);
            }
        }

        return tree_lake_tiles;
    }

    fn set_lake_shore_and_corner_sprites(
        map: &MapTileGrid,
        map_dimensions: MapDimensions,
    ) -> HashMap<MapCord, LakeSpriteData> {
        let mut lake_sprite_data: HashMap<MapCord, LakeSpriteData> = HashMap::new();

        // rolling with i16 here because if its less than 0 it needs to be caught
        for y in 0..map_dimensions.height as i16 {
            for x in 0..map_dimensions.width as i16 {
                let current = MapCord::new(x, y);

                if Self::get_tile_at_cord_no_self(map, map_dimensions, current) != TileType::Lake {
                    continue;
                }

                let mut shore_bitmask = 0;

                for i in 0..CARDINAL_DELTAS.len() {
                    let neighbor = current + CARDINAL_DELTAS[i];

                    if !Self::is_tile_in_bounds_no_ref(map_dimensions, neighbor) {
                        continue;
                    }

                    if Self::get_tile_at_cord_no_self(map, map_dimensions, neighbor)
                        == TileType::Lake
                    {
                        continue;
                    }

                    // shore found! add it to the bitmask
                    shore_bitmask |= 1 << i;
                }

                let mut corner_bitmask = 0;

                let corner_checks: [(i16, i16, u8); 4] = [
                    // (x, y, bit)
                    (-1, -1, 0), //NW
                    (1, -1, 1),  //NE
                    (1, 1, 2),   //SE
                    (-1, 1, 3),  //SW
                ];

                for corner in corner_checks {
                    let diag_x = x + corner.0;
                    let diag_y = y + corner.1;

                    if !Self::is_tile_in_bounds_no_ref(map_dimensions, MapCord::new(diag_x, diag_y))
                    {
                        continue;
                    }

                    // tile at the diagonal does not allow for a corner, stop checking RIGHT NOWWWWWW
                    if Self::get_tile_from_x_y_no_self(map, map_dimensions, diag_x, diag_y)
                        == TileType::Lake
                    {
                        continue;
                    }

                    // check if should be shore on these specific edges, because that would mean no
                    // corner on those edges
                    if Self::get_tile_from_x_y_no_self(map, map_dimensions, diag_x, current.y)
                        != TileType::Lake
                        || Self::get_tile_from_x_y_no_self(map, map_dimensions, current.x, diag_y)
                            != TileType::Lake
                    {
                        continue;
                    }

                    // corner found!
                    corner_bitmask |= 1 << corner.2;
                }

                // all lakes are going to have this data
                // if the bitmask is 0 on a field, its simply ignored, as theres no index for 0
                // this is how we'll tell if a lake should use this data or not
                // in drawing, we'll subtract 1 from the index. im doing it this way so that we can
                // keep memory usage as low as possible, because over millions of tiles, even if 10% of them
                // are lakes, thats still a lot of extra data, otherwise id do Option<u8> in LakeSpriteData

                lake_sprite_data.insert(
                    current,
                    LakeSpriteData {
                        shore_animation_index: shore_bitmask,
                        corner_animation_index: corner_bitmask,
                    },
                );
            }
        }

        return lake_sprite_data;
    }

    fn create_rivers(
        map: &mut MapTileGrid,
        lake_data: &HashMap<MapCord, LakeSpriteData>,
        map_dimensions: MapDimensions,
        rng: &mut ThreadRng,
    ) -> HashMap<MapCord, Direction> {
        let dir_change_chance = 0.01;
        let river_chance = 0.05;
        let ok_shore_masks: [u8; 4] = [1, 2, 4, 8];

        // (cord, flow dir)
        let estimated_num_of_river_tiles = map_dimensions.total_tiles() / 50;
        let mut all_rivers: HashMap<MapCord, Direction> =
            HashMap::with_capacity(estimated_num_of_river_tiles);

        for (cord, data) in lake_data {
            if !ok_shore_masks.contains(&data.shore_animation_index) {
                // not viable
                continue;
            }

            // since we found a viable candidate, lets see if it can turn into a river
            if !rng.random_bool(river_chance) {
                continue;
            }

            // get the direction for the river to start in
            let mut direction: Direction = match data.shore_animation_index {
                1 => Direction::North,
                2 => Direction::East,
                4 => Direction::South,
                8 => Direction::West,
                _ => panic!(
                    "river dir is not any of the ok bit masks, should not have made it past the viability check"
                ),
            };

            // set the current tile to the coordinate of the viable tile
            let mut current_tile = MapCord::new(cord.x, cord.y);
            let mut just_turned = false;

            // --river creation algorithm here-- //

            // set up the current river for this loop iteration
            let mut current_river: HashMap<MapCord, Direction> = HashMap::new();

            // start the river with this coordinate
            current_river.insert(current_tile, direction);

            // the main river tile creation loop
            loop {
                // skip the random turning change if you just turned, skips it for exactly 1 iteration
                if just_turned {
                    just_turned = false;
                } else if rng.random_bool(dir_change_chance) {
                    let new_dir = match rng.random_bool(0.5) {
                        true => direction.turn_left(),
                        false => direction.turn_right(),
                    };

                    direction = new_dir;
                    just_turned = true;
                }

                let check_tile = current_tile + CARDINAL_DELTAS[direction as usize];

                if current_river.contains_key(&check_tile) {
                    // i dont personally want river loops from one origin
                    break;
                }

                if !Self::is_tile_in_bounds_no_ref(map_dimensions, check_tile) {
                    // end river it reached the end
                    Self::add_river(&mut current_river, &mut all_rivers, map, map_dimensions);
                    break;
                }

                let check_tile_type =
                    Self::get_tile_at_cord_no_self(map, map_dimensions, check_tile);

                if check_tile_type == TileType::River {
                    let check_tile_two = check_tile + CARDINAL_DELTAS[direction as usize];

                    if !Self::is_tile_in_bounds_no_ref(map_dimensions, check_tile_two) {
                        // not a cross section because the point past the river is out of bounds, just add this river
                        Self::add_river(&mut current_river, &mut all_rivers, map, map_dimensions);
                        break;
                    }

                    if Self::get_tile_at_cord_no_self(map, map_dimensions, check_tile_two)
                        == TileType::River
                    {
                        // i dont want a cross section piece
                        break;
                    }
                    // if that isnt the case, then add the river, because it means theres no cross section
                    Self::add_river(&mut current_river, &mut all_rivers, map, map_dimensions);
                    break;
                } else if check_tile_type == TileType::Lake {
                    let lake_sh = lake_data.get(&check_tile).unwrap().shore_animation_index;

                    if ok_shore_masks.contains(&lake_sh) {
                        // end river here, but add shore tile to river for inlet/outlet
                        current_river.insert(check_tile, direction);
                        Self::add_river(&mut current_river, &mut all_rivers, map, map_dimensions);
                        break;
                    } else {
                        // scrap river
                        // i didnt program it to connect with this tile
                        // im too lazy too so scrap it
                        break;
                    }
                }

                // get the info on the tiles to the left and right of the current tile (not the check tile)
                let dir_left = CARDINAL_DELTAS[direction.turn_left() as usize];
                let dir_right = CARDINAL_DELTAS[direction.turn_right() as usize];
                let tile_to_left = current_tile + dir_left;
                let tile_to_right = current_tile + dir_right;

                let left_type = match Self::is_tile_in_bounds_no_ref(map_dimensions, tile_to_left) {
                    true => Some(Self::get_tile_at_cord_no_self(
                        map,
                        map_dimensions,
                        tile_to_left,
                    )),
                    false => None,
                };

                let right_type = match Self::is_tile_in_bounds_no_ref(map_dimensions, tile_to_right)
                {
                    true => Some(Self::get_tile_at_cord_no_self(
                        map,
                        map_dimensions,
                        tile_to_right,
                    )),
                    false => None,
                };

                match (left_type, right_type) {
                    (None, Some(TileType::River))
                    | (Some(TileType::River), None)
                    | (Some(TileType::River), Some(TileType::River)) => {
                        Self::add_river(&mut current_river, &mut all_rivers, map, map_dimensions);
                        break;
                    }
                    _ => (),
                }

                // prepare for next iteration
                current_river.insert(check_tile, direction);
                current_tile = check_tile;
            }
        }

        return all_rivers;
    }

    fn add_river(
        current_river: &mut HashMap<MapCord, Direction>,
        all_rivers: &mut HashMap<MapCord, Direction>,
        map: &mut MapTileGrid,
        map_dimensions: MapDimensions,
    ) {
        for riv_tile in current_river {
            map[Self::cords_to_index(map_dimensions, *riv_tile.0)] = TileType::River;
            all_rivers.insert(*riv_tile.0, *riv_tile.1);
        }
    }

    fn set_river_tile_animations(
        all_rivers: &HashMap<MapCord, Direction>,
        map: &MapTileGrid,
        map_dimensions: MapDimensions,
    ) -> HashMap<MapCord, RiverSpriteData> {
        let mut river_data: HashMap<MapCord, RiverSpriteData> =
            HashMap::with_capacity(all_rivers.iter().count());

        for (cord, dir) in all_rivers {
            // find the number of neighboring river tiles around this tile
            let mut num_of_neighbors: u8 = 0;
            for direction in CARDINAL_DELTAS {
                let check_tile = *cord + direction;

                if !Self::is_tile_in_bounds_no_ref(map_dimensions, check_tile) {
                    continue;
                }

                if Self::get_tile_at_cord_no_self(map, map_dimensions, check_tile)
                    == TileType::River
                {
                    num_of_neighbors += 1;
                }
            }

            match num_of_neighbors {
                1 => {
                    let check_dir = CARDINAL_DELTAS[*dir as usize];
                    let check_tile = *cord + check_dir;

                    if !Self::is_tile_in_bounds_no_ref(map_dimensions, check_tile) {
                        // actually a straight but just had 1 neighbor due to oob
                        river_data.insert(
                            *cord,
                            RiverSpriteData {
                                river_type: RiverType::Straight,
                                river_sprite_index: *dir as u8,
                            },
                        );
                        continue;
                    }

                    let river_type =
                        if Self::get_tile_at_cord_no_self(map, map_dimensions, check_tile)
                            == TileType::Lake
                        {
                            RiverType::Inlet
                        } else {
                            RiverType::Outlet
                        };

                    let index = if let RiverType::Inlet = river_type {
                        (*dir as u8 + 2) % 4
                    } else {
                        *dir as u8
                    };

                    river_data.insert(
                        *cord,
                        RiverSpriteData {
                            river_type: river_type,
                            river_sprite_index: index,
                        },
                    );
                }
                2 => {
                    for i in 0..CARDINAL_DELTAS.len() {
                        let first_tile = *cord + CARDINAL_DELTAS[i];

                        if !Self::is_tile_in_bounds_no_ref(map_dimensions, first_tile) {
                            continue;
                        }

                        if Self::get_tile_at_cord_no_self(map, map_dimensions, first_tile)
                            != TileType::River
                        {
                            continue;
                        }
                        // river found! now, determine whether its a straight or a corner (both have 2 neighbors)

                        let straight_check_tile = *cord + CARDINAL_DELTAS[(i + 2) % 4];

                        // if its not in bounds, wont check which tile it is, because this has to be a corner
                        if Self::is_tile_in_bounds_no_ref(map_dimensions, straight_check_tile)
                            && Self::get_tile_at_cord_no_self(
                                map,
                                map_dimensions,
                                straight_check_tile,
                            ) == TileType::River
                        {
                            river_data.insert(
                                *cord,
                                RiverSpriteData {
                                    river_type: RiverType::Straight,
                                    river_sprite_index: *dir as u8,
                                },
                            );
                        } else {
                            // corner found, now need to find second tile (first is known)
                            let mut flow_direction = FlowDirection::UpStream; // default value will be overriden eventually

                            // if any of the neigbors are flowing south, that means this piece as a whole will flow south
                            // default starts as north, and will become south if any neighbors are south
                            if *all_rivers.get(&first_tile).unwrap() == Direction::South {
                                flow_direction = FlowDirection::DownStream;
                            }

                            // check for second tile
                            for j in (i + 1)..CARDINAL_DELTAS.len() {
                                let second_tile = *cord + CARDINAL_DELTAS[j];

                                if !Self::is_tile_in_bounds_no_ref(map_dimensions, second_tile) {
                                    continue;
                                }

                                if Self::get_tile_at_cord_no_self(map, map_dimensions, second_tile)
                                    != TileType::River
                                {
                                    continue;
                                }

                                if *all_rivers.get(&second_tile).unwrap() == Direction::South {
                                    flow_direction = FlowDirection::DownStream
                                }

                                let index = RIVER_CORNER_ANIM_KEY
                                    .get(&(
                                        Direction::get_enum_from_repr(i as i8),
                                        Direction::get_enum_from_repr(j as i8),
                                        flow_direction,
                                    ))
                                    .unwrap();

                                river_data.insert(
                                    *cord,
                                    RiverSpriteData {
                                        river_type: RiverType::Corner,
                                        river_sprite_index: *index,
                                    },
                                );
                                break;
                            }
                        }
                    }
                }
                3 => {
                    let mut flow_direction = FlowDirection::UpStream;

                    for i in 0..CARDINAL_DELTAS.len() {
                        let first_tile = *cord + CARDINAL_DELTAS[i];

                        if !Self::is_tile_in_bounds_no_ref(map_dimensions, first_tile) {
                            continue;
                        }

                        if Self::get_tile_at_cord_no_self(map, map_dimensions, first_tile)
                            != TileType::River
                        {
                            continue;
                        }

                        // first tile found!
                        if *all_rivers.get(&first_tile).unwrap() == Direction::South {
                            flow_direction = FlowDirection::DownStream;
                        }

                        for j in (i + 1)..CARDINAL_DELTAS.len() {
                            let second_tile = *cord + CARDINAL_DELTAS[j];

                            if !Self::is_tile_in_bounds_no_ref(map_dimensions, second_tile) {
                                continue;
                            }

                            if Self::get_tile_at_cord_no_self(map, map_dimensions, second_tile)
                                != TileType::River
                            {
                                continue;
                            }

                            // second tile found!
                            if *all_rivers.get(&second_tile).unwrap() == Direction::South {
                                flow_direction = FlowDirection::DownStream;
                            }

                            for k in (j + 1)..CARDINAL_DELTAS.len() {
                                let third_tile = *cord + CARDINAL_DELTAS[k];

                                if !Self::is_tile_in_bounds_no_ref(map_dimensions, third_tile) {
                                    continue;
                                }

                                if Self::get_tile_at_cord_no_self(map, map_dimensions, third_tile)
                                    != TileType::River
                                {
                                    continue;
                                }

                                // third tile found!
                                if *all_rivers.get(&third_tile).unwrap() == Direction::South {
                                    flow_direction = FlowDirection::DownStream;
                                }

                                let index = RIVER_T_SECTION_ANIM_KEY
                                    .get(&(
                                        Direction::get_enum_from_repr(i as i8),
                                        Direction::get_enum_from_repr(j as i8),
                                        Direction::get_enum_from_repr(k as i8),
                                        flow_direction,
                                    ))
                                    .unwrap();

                                river_data.insert(
                                    *cord,
                                    RiverSpriteData {
                                        river_type: RiverType::TSection,
                                        river_sprite_index: *index,
                                    },
                                );
                            }
                        }
                    }
                }

                _ => panic!(
                    "the only valid number of neighbors for a river are 1, 2, 3, this tile should not have made it to the neighbor check"
                ),
            }
        }

        return river_data;
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

    // helpers for map creation
    pub fn is_tile_in_bounds_no_ref(map_dimensions: MapDimensions, cord: MapCord) -> bool {
        let is_x_in_bounds = cord.x >= 0 && cord.x < map_dimensions.width as i16;
        let is_y_in_bounds = cord.y >= 0 && cord.y < map_dimensions.height as i16;

        return is_x_in_bounds && is_y_in_bounds;
    }

    fn get_tile_at_cord_no_self(
        map: &MapTileGrid,
        map_dimensions: MapDimensions,
        cord: MapCord,
    ) -> TileType {
        let idx = cord.y as usize * map_dimensions.width as usize + cord.x as usize;

        return map[idx];
    }

    fn get_tile_from_x_y_no_self(
        map: &MapTileGrid,
        map_dimensions: MapDimensions,
        x: i16,
        y: i16,
    ) -> TileType {
        let idx = y as usize * map_dimensions.width as usize + x as usize;

        return map[idx];
    }

    pub fn cords_to_index(map_dimensions: MapDimensions, cord: MapCord) -> usize {
        let y_u = cord.y as usize;
        let x_u = cord.x as usize;
        return y_u * map_dimensions.width as usize + x_u;
    }

    // helpers for world building specifically
    fn create_forest_around_lake(
        tile_grid: &MapTileGrid,
        object_grid: &mut MapObjectGrid,
        lake_tiles: Vec<MapCord>,
        map_dimensions: MapDimensions,
        rng: &mut ThreadRng,
    ) {
        for lake_tile in lake_tiles {
            let range = rng.random_range(1..=10);

            for dir in CARDINAL_DELTAS {
                for r in 0..=range {
                    let tree_target_tile = lake_tile + (dir * r);

                    if !Self::is_tile_in_bounds_no_ref(map_dimensions, tree_target_tile) {
                        continue;
                    }

                    if Self::get_tile_at_cord_no_self(tile_grid, map_dimensions, tree_target_tile)
                        != TileType::Grass
                    {
                        continue;
                    }

                    //test if you will succeed in placing a tree on this tile
                    if !rng.random_bool(0.1) {
                        continue;
                    }

                    let index = Self::cords_to_index(map_dimensions, tree_target_tile);

                    if let Object::NoObject = object_grid[index] {
                        object_grid[index] = TreeObj(Tree::new(tree_target_tile))
                    }
                }
            }
        }
    }
}
