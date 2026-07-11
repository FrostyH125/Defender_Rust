use std::collections::{HashMap, VecDeque};

use basic_raylib_core::graphics::sprite_animation::SpriteAnimationInstance;
use rand::{RngExt, rngs::ThreadRng};
use raylib::math::Vector2;

use crate::{
    map::{
        tile::{LakeSpriteData, TileType},
        tile_map_animation_data::RiverType,
    },
    utils::{
        directional_deltas::{self, CARDINAL_DELTAS},
        map_cord::MapCord,
    },
};

type MapTiles = Vec<TileType>;

const LAKE_CHANCE: f32 = 0.001;

pub struct TileMap {
    map: MapTiles,
    map_width: u16,
    map_height: u16,
    lake_sprite_data: HashMap<MapCord, LakeSpriteData>,

    // pos , river variant, and index for its variant of animations
    river_sprite_data: HashMap<MapCord, RiverType, u8>,
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
        let mut map: MapTiles = vec![TileType::Grass; (map_width * map_height) as usize];

        Self::create_lakes(&mut map, map_width, map_height, &mut rng);
        let lake_sprite_data = Self::set_lake_shore_and_corner_sprites(&map, map_width, map_height);
        //CreateRivers();
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

    fn create_lakes(map: &mut MapTiles, map_width: u16, map_height: u16, rng: &mut ThreadRng) {
        let map_len = map.len() as f32;
        let variance_bound = map_len * LAKE_CHANCE / 5.0;
        let final_variance = rng.random_range(-variance_bound..=variance_bound);
        let num_of_cycles = (map_len * LAKE_CHANCE + final_variance) as i32;

        for _ in 0..num_of_cycles {
            let is_forest_lake = rng.random_bool(0.03);

            // yes width and height are swapped. i sat down with a notebook and pencil
            // to figure that one out, apparently it wasn't obvious that was the right way
            // but it makes sense since a = w * h, so w = a / h and h = a / w
            let start_x = rng.random_range(0..(map.len() / map_height as usize)) as f32;
            let start_y = rng.random_range(0..(map.len() / map_width as usize)) as f32;

            // will only reach this size as long as the queue doesnt run out of tiles
            let target_lake_size = rng.random_range(60..=100);

            let mut tiles_placed = 0;

            let mut next_tiles: VecDeque<Vector2> = VecDeque::new();
            next_tiles.push_back(Vector2::new(start_x, start_y));

            let mut lake_tiles: Vec<Vector2> = Vec::new();

            while next_tiles.len() > 0 && tiles_placed < target_lake_size {
                // guaranteed to be Some(tile) because of the while condition
                let current = next_tiles.pop_front().unwrap();

                // cant use self.is_tile_in_bounds because theres no self yet
                // to avoid inconvenient ass function parameters, im going
                // to just do it manually here

                if !Self::is_tile_in_bounds_no_self(
                    map_width,
                    map_height,
                    current.x as i16,
                    current.y as i16,
                ) {
                    continue;
                }

                if Self::get_tile_at_cords_no_self(
                    map,
                    map_width,
                    current.x as i16,
                    current.y as i16,
                ) == TileType::Lake
                {
                    continue;
                }

                let tile_index = Self::cords_to_index_f(map_width, current.x, current.y);

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
                        next_tiles.push_back(Vector2::new(
                            current.x + dir.0 as f32,
                            current.y + dir.1 as f32,
                        ));
                    }
                }
            }

            if is_forest_lake {
                Self::create_forest_around_lake(lake_tiles);
            }
        }
    }

    fn set_lake_shore_and_corner_sprites(
        map: &MapTiles,
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
                    let neighbor = MapCord::new(
                        current.x + CARDINAL_DELTAS[i].0,
                        current.y + CARDINAL_DELTAS[i].1,
                    );

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

    pub fn get_tile_at_cords(&self, x: u16, y: u16) -> TileType {
        let index = y * self.map_width + x;
        return self.map[index as usize];
    }

    pub fn is_tile_in_bounds(&self, x: i16, y: i16) -> bool {
        let x_in_bounds = x > 0 && x < self.map_width as i16;
        let y_in_bounds = x > 0 && x < self.map_height as i16;
        return x_in_bounds && y_in_bounds;
    }

    // helpers for map creation
    fn is_tile_in_bounds_no_self(map_width: u16, map_height: u16, x: i16, y: i16) -> bool {
        let is_x_in_bounds = x > 0 && x < map_width as i16;
        let is_y_in_bounds = y > 0 && y < map_height as i16;

        return is_x_in_bounds && is_y_in_bounds;
    }

    fn get_tile_at_cords_no_self(map: &MapTiles, map_width: u16, x: i16, y: i16) -> TileType {
        let idx = (y * map_width as i16 + x) as usize;

        return map[idx];
    }

    fn cords_to_index_f(map_width: u16, x: f32, y: f32) -> usize {
        let y_u = y as u16;
        let x_u = x as u16;
        return (y_u * map_width + x_u) as usize;
    }

    // helpers for world building specifically
    fn create_forest_around_lake(lake_tiles: Vec<Vector2>) {
        println!(
            "MAKE FOREST AROUND LAKE FUNCTION: num of lake tiles {}",
            lake_tiles.len()
        )
    }
}
