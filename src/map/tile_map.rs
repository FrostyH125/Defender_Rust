use std::collections::VecDeque;

use basic_raylib_core::graphics::sprite_animation::SpriteAnimationInstance;
use rand::{RngExt, rngs::ThreadRng};
use raylib::math::Vector2;

use crate::{map::tile::TileType, utils::cardinal_deltas::{self, CARDINAL_DELTAS}};

type MapTiles = Vec<TileType>;

const LAKE_CHANCE: f32 = 0.001;

pub struct TileMap {
    map: MapTiles,
    map_width: u16,
    map_height: u16,
    lake_tile_anim_instance: SpriteAnimationInstance,
    default_tile_anim_instance: SpriteAnimationInstance,
    rng: ThreadRng
}

impl TileMap {
    pub fn new(map_width: u16, map_height: u16) {
        let map = TileMap::generate_map(map_width, map_height);
    }

    pub fn update(&mut self) {
        // update lake tile anim and default tile anim
        // spawn grass randomly over time
    }

    pub fn draw(&self) {
        // move tile renderer draw function to this struct
    }
    
    fn generate_map(map_width: u16, map_height: u16) -> MapTiles {

        let mut rng = rand::rng();

        // the map starts out as a grass filled map, this is because the rest
        // of the functions rely on checking for grass, and so that the tiles are
        // initialized with a value and i dont need to add "None" to TileType just
        // for this one singular purpose
        let mut map: MapTiles = vec![TileType::Grass; (map_width * map_height) as usize];

        Self::create_lakes(&mut map, map_width, map_height, &mut rng);
        //SetLakeShoreSprites();
        //SetLakeCornerSprites();
        //CreateRivers();
        //SetRiverTileAnimations();
        //CheckForRiverCorner(); //also checks for t sections as it involves the corner variables anyway
        //CheckInletsAndOutlets();
        //SetGrassTileGrowMultiplier();
        //CreateForests();
        //CreateStandAloneTrees();
        //CreateGrass();
        //SpawnGrassAroundLakesAndRivers();

        return map;
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

                if !Self::is_tile_in_bounds_no_self(map, map_width, map_height, current.x, current.y) {
                    continue;
                }

                if Self::get_tile_at_cords_no_self(map, map_width, current.x, current.y) == TileType::Lake {
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
                        next_tiles.push_back(Vector2::new(current.x + dir.x, current.y + dir.y));
                    }
                }
            }

            if is_forest_lake {
                Self::create_forest_around_lake(lake_tiles);
            }
        }
    }

    fn set_lake_shore_sprites();
    
    pub fn get_tile_at_cords(&self, x: u16, y: u16) -> TileType {
        let index = y * self.map_width + x;
        return self.map[index as usize];
    }

    pub fn is_tile_in_bounds(&self, x: u16, y: u16) -> bool {
        let x_in_bounds = x > 0 && x < self.map_width;
        let y_in_bounds = x > 0 && x < self.map_height;
        return x_in_bounds && y_in_bounds;
    }

    // helpers for map creation
    fn is_tile_in_bounds_no_self(map: &MapTiles, map_width: u16, map_height: u16, x: f32, y: f32) -> bool {
        let x_u = x as u16;
        let y_u = y as u16;

        let is_x_in_bounds = x_u > 0 && x_u < map_width;
        let is_y_in_bounds = y_u > 0 && y_u < map_height;

        return is_x_in_bounds && is_y_in_bounds;
    }

    fn get_tile_at_cords_no_self(map: &MapTiles, map_width: u16, x: f32, y: f32) -> TileType {
        let x_u = x as u16;
        let y_u = y as u16;

        let idx = (y_u * map_width + x_u) as usize;

        return map[idx];
    }

    fn cords_to_index_f(map_width: u16, x: f32, y: f32) -> usize {
        let y_u = y as u16;
        let x_u = x as u16;
        return (y_u * map_width + x_u) as usize;
    }

    // helpers for world building specifically
    fn create_forest_around_lake(lake_tiles: Vec<Vector2>) {
        println!("MAKE FOREST AROUND LAKE FUNCTION: num of lake tiles {}", lake_tiles.len())
    }
}
