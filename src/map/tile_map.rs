use std::{collections::{HashMap, VecDeque}, ops::ControlFlow};

use basic_raylib_core::graphics::sprite_animation::SpriteAnimationInstance;
use rand::{RngExt, rngs::ThreadRng};
use raylib::math::Vector2;

use crate::{
    map::{
        tile::{LakeSpriteData, TileType},
        tile_map_animation_data::RiverType,
    },
    utils::{
        self,
        array_utils::shuffle_vec,
        directional_deltas::{self, CARDINAL_DELTAS},
        map_cord::MapCord,
    },
};

type MapData = Vec<TileType>;

const LAKE_CHANCE: f32 = 0.001;

pub struct TileMap {
    map: MapData,
    map_width: u16,
    map_height: u16,
    lake_sprite_data: HashMap<MapCord, LakeSpriteData>,

    // pos , river variant, and index for its variant of animations
    river_sprite_data: HashMap<MapCord, (RiverType, u8)>,
    lake_tile_anim_instance: SpriteAnimationInstance,
    default_tile_anim_instance: SpriteAnimationInstance,
    rng: ThreadRng,
}

impl TileMap {
    pub fn new(map_width: u16, map_height: u16) -> Self {
        let map = TileMap::generate_map(map_width, map_height);
        return map;
    }

    pub fn update(&mut self) {
        // update lake tile anim and default tile anim
        // spawn grass randomly over time
    }

    pub fn draw(&self) {
        // move tile renderer draw function to this struct
    }

    fn generate_map(map_width: u16, map_height: u16) -> TileMap {
        let mut rng = rand::rng();

        // the map starts out as a grass filled map, this is because the rest
        // of the functions rely on checking for grass, and so that the tiles are
        // initialized with a value and i dont need to add "None" to TileType just
        // for this one singular purpose
        let mut map: MapData = vec![TileType::Grass; (map_width * map_height) as usize];

        Self::create_lakes(&mut map, map_width, map_height, &mut rng);
        let lake_sprite_data = Self::set_lake_shore_and_corner_sprites(&map, map_width, map_height);
        let all_river_tiles = Self::create_rivers(&mut map, &lake_sprite_data, map_width, map_height, &mut rng);
        //SetRiverTileAnimations();
        //CheckForRiverCorner(); //also checks for t sections as it involves the corner variables anyway
        //CheckInletsAndOutlets();
        //SetGrassTileGrowMultiplier();
        //CreateForests();
        //CreateStandAloneTrees();
        //CreateGrass();
        //SpawnGrassAroundLakesAndRivers();
        return TileMap {
            map,
            map_width,
            map_height,
            lake_sprite_data,
            river_sprite_data: todo!(),
            lake_tile_anim_instance: SpriteAnimationInstance::new(),
            default_tile_anim_instance: SpriteAnimationInstance::new(),
            rng,
        };
    }

    fn create_lakes(map: &mut MapData, map_width: u16, map_height: u16, rng: &mut ThreadRng) {
        let map_len = map.len() as f32;
        let variance_bound = map_len * LAKE_CHANCE / 5.0;
        let final_variance = rng.random_range(-variance_bound..=variance_bound);
        let num_of_cycles = (map_len * LAKE_CHANCE + final_variance) as i32;

        for _ in 0..num_of_cycles {
            let is_forest_lake = rng.random_bool(0.03);

            // yes width and height are swapped. i sat down with a notebook and pencil
            // to figure that one out, apparently it wasn't obvious that was the right way
            // but it makes sense since a = w * h, so w = a / h and h = a / w
            let start_x = rng.random_range(0..(map.len() / map_height as usize)) as i16;
            let start_y = rng.random_range(0..(map.len() / map_width as usize)) as i16;

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

                if !Self::is_tile_in_bounds_no_self(
                    map_width,
                    map_height,
                    current.x,
                    current.y,
                ) {
                    continue;
                }

                if Self::get_tile_at_cords_no_self(
                    map,
                    map_width,
                    current.x,
                    current.y,
                ) == TileType::Lake
                {
                    continue;
                }

                let tile_index = Self::cords_to_index(map_width, current.x, current.y);

                map[tile_index] = TileType::Lake;

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
                Self::create_forest_around_lake(lake_tiles);
            }
        }
    }

    fn set_lake_shore_and_corner_sprites(
        map: &MapData,
        map_width: u16,
        map_height: u16,
    ) -> HashMap<MapCord, LakeSpriteData> {
        let mut lake_sprite_data: HashMap<MapCord, LakeSpriteData> = HashMap::new();

        // rolling with i16 here because if its less than 0 it needs to be caught
        for y in 0..map_height as i16 {
            for x in 0..map_width as i16 {
                let current = MapCord::new(x, y);

                if Self::get_tile_at_cords_no_self(map, map_width, x as i16, y as i16)
                    != TileType::Lake
                {
                    continue;
                }

                let mut shore_bitmask = 0;

                for i in 0..CARDINAL_DELTAS.len() {
                    let neighbor = current + CARDINAL_DELTAS[i];

                    if !Self::is_tile_in_bounds_no_self(
                        map_width, map_height, neighbor.x, neighbor.y,
                    ) {
                        continue;
                    }

                    if Self::get_tile_at_cords_no_self(map, map_width, neighbor.x, neighbor.y)
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

                    if !Self::is_tile_in_bounds_no_self(map_width, map_height, diag_x, diag_y) {
                        continue;
                    }

                    // tile at the diagonal does not allow for a corner, stop checking RIGHT NOWWWWWW
                    if Self::get_tile_at_cords_no_self(map, map_width, diag_x, diag_y)
                        == TileType::Lake
                    {
                        continue;
                    }

                    // check if should be shore on these specific edges, because that would mean no
                    // corner on those edges
                    if Self::get_tile_at_cords_no_self(map, map_width, diag_x, current.y)
                        != TileType::Lake
                        && Self::get_tile_at_cords_no_self(map, map_width, current.x, diag_y)
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
        map: &mut MapData,
        lake_data: &HashMap<MapCord, LakeSpriteData>,
        map_width: u16,
        map_height: u16,
        rng: &mut ThreadRng,
    ) -> HashMap<MapCord, u8> {
        let dir_change_chance = 0.01;
        let river_chance = 0.01;
        let ok_shore_masks: [u8; 4] = [1, 2, 4, 8];

        // (cord, flow dir)
        let mut all_rivers: HashMap<MapCord, u8> = HashMap::new();

        for (cord, data) in lake_data {
            if !ok_shore_masks.contains(&lake_data.get(&cord).unwrap().shore_animation_index) {
                // not viable
                continue;
            }

            // since we found a viable candidate, lets see if it can turn into a river
            if !rng.random_bool(river_chance) {
                continue;
            }

            // get the direction for the river to start in
            let mut direction: i8 = match data.shore_animation_index {
                1 => 0,
                2 => 1,
                4 => 2,
                8 => 3,
                _ => panic!("river dir is not any of the ok bit masks, should not have made it past the viability check")
            };

            // set the current tile to the coordinate of the viable tile
            let mut current_tile = MapCord::new(cord.x, cord.y);
            let mut just_turned = false;

            // --river creation algorithm here-- //

            // set up the current river for this loop iteration
            let mut current_river: HashMap<MapCord, u8> = HashMap::new();

            // start the river with this coordinate
            current_river.insert(current_tile, direction as u8);

            // the main river tile creation loop
            loop {

                // skip the random turning change if you just turned, skips it for exactly 1 iteration
                if just_turned {
                    just_turned = false;
                }
                else if rng.random_bool(dir_change_chance) {
                    let change: i8 = match rng.random_bool(0.5) {
                        true => -1,
                        false => 1,
                    };
                    
                    direction = (direction + change).rem_euclid(4);
                    just_turned = true;
                }
                 
                let check_tile = current_tile + CARDINAL_DELTAS[direction as usize];

                if current_river.contains_key(&check_tile) {
                    // i dont personally want river loops from one origin
                    current_river.clear();
                    break;
                }
                
                if !Self::is_tile_in_bounds_no_self(map_width, map_height, check_tile.x, check_tile.y) {
                    // end river it reached the end
                    for riv_tile in &current_river {
                        all_rivers.insert(*riv_tile.0, *riv_tile.1);
                    }
                    break;
                }

                let check_tile_type = Self::get_tile_at_cords_no_self(map, map_width, check_tile.x, check_tile.y);
                
                if check_tile_type == TileType::River {
                    break;
                } else if check_tile_type == TileType::Lake {
                    let lake_sh = lake_data.get(&check_tile).unwrap().shore_animation_index;

                    if ok_shore_masks.contains(&lake_sh) {
                        // end river here, but add it
                        current_river.insert(check_tile, direction as u8);
                        for riv_tile in &current_river {
                            all_rivers.insert(*riv_tile.0, *riv_tile.1);
                        }
                        break;
                    }
                    else {
                        // scrap river
                        // i didnt program it to connect with this tile
                        // im too lazy too so scrap it
                        current_river.clear();
                        break;
                    }
                }

                // get the info on the tiles to the left and right of the current tile (not the check tile)
                let dir_left = CARDINAL_DELTAS[((direction - 1) % 4) as usize];
                let dir_right = CARDINAL_DELTAS[((direction + 1) % 4) as usize];
                let tile_to_left = current_tile + dir_left;
                let tile_to_right = current_tile + dir_right;
                let left_type = Self::get_tile_at_cords_no_self(map, map_width, tile_to_left.x, tile_to_left.y);
                let right_type = Self::get_tile_at_cords_no_self(map, map_width, tile_to_right.x, tile_to_right.y);

                if left_type == TileType::River || right_type == TileType::River {
                    // end and keep river
                    for riv_tile in &current_river {
                        all_rivers.insert(*riv_tile.0, *riv_tile.1);
                    }
                    break;
                }

                // prepare for next iteration
                current_river.insert(check_tile, direction as u8);
                current_tile = check_tile;
            }
        }

        return all_rivers;
    }

    fn set_river_tile_animations() -> HashMap<MapCord, (RiverType, u8)> {
        
    }

    pub fn get_tile_at_cords(&self, x: u16, y: u16) -> TileType {
        let index = y * self.map_width + x;
        return self.map[index as usize];
    }

    pub fn is_tile_in_bounds(&self, x: i16, y: i16) -> bool {
        let x_in_bounds = x > 0 && x < self.map_width as i16;
        let y_in_bounds = x > 0 && y < self.map_height as i16;
        return x_in_bounds && y_in_bounds;
    }

    // helpers for map creation
    fn is_tile_in_bounds_no_self(map_width: u16, map_height: u16, x: i16, y: i16) -> bool {
        let is_x_in_bounds = x > 0 && x < map_width as i16;
        let is_y_in_bounds = y > 0 && y < map_height as i16;

        return is_x_in_bounds && is_y_in_bounds;
    }

    fn get_tile_at_cords_no_self(map: &MapData, map_width: u16, x: i16, y: i16) -> TileType {
        let idx = (y * map_width as i16 + x) as usize;

        return map[idx];
    }

    fn cords_to_index(map_width: u16, x: i16, y: i16) -> usize {
        let y_u = y as u16;
        let x_u = x as u16;
        return (y_u * map_width + x_u) as usize;
    }

    // helpers for world building specifically
    fn create_forest_around_lake(lake_tiles: Vec<MapCord>) {
        println!(
            "MAKE FOREST AROUND LAKE FUNCTION: num of lake tiles {}",
            lake_tiles.len()
        )
    }
}

