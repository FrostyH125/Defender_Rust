use std::collections::{HashMap, VecDeque};

use rand::{RngExt, rngs::ThreadRng};

use crate::{
    GameContext, entities::{
        object::Object::{self},
        objects::{grass::Grass, tree::Tree},
    }, map::{
        map_cell::{CELL_SIZE, MapCell},
        tile::{LakeSpriteData, RiverSpriteData, TileType},
        tile_map::{MapDimensions, MapObjectGrid, MapTileGrid},
        tile_map_animation_data::{
            FlowDirection, RIVER_CORNER_ANIM_KEY, RIVER_T_SECTION_ANIM_KEY, RiverType,
        },
    }, utils::{
        direction_utils::{CARDINAL_DELTAS, Direction, ORTHOGONAL_DELTAS}, map_cord::MapCord, map_utils::{self, get_tile_at_cord, is_tile_in_bounds, tile_not_in_bounds_or_doesnt_match, tile_not_in_bounds_or_matches},
    },
};

pub fn generate_cell_grid(map_dimensions: MapDimensions) -> Vec<MapCell> {
    let mut map_cells: Vec<MapCell> = Vec::new();

    let num_of_cells_wide = map_dimensions.width / CELL_SIZE;
    let num_of_cells_tall = map_dimensions.height / CELL_SIZE;

    for _ in 0..num_of_cells_tall {
        for _ in 0..num_of_cells_wide {
            map_cells.push(MapCell::new());
        }
    }

    return map_cells;
}

pub fn create_lakes(
    tile_grid: &mut MapTileGrid,
    map_dimensions: MapDimensions,
    rng: &mut ThreadRng,
) -> (Vec<MapCord>, Vec<MapCord>) {
    const LAKE_CHANCE: f32 = 0.001;

    let map_len = tile_grid.len() as f32;
    let variance_bound = map_len * LAKE_CHANCE / 5.0;
    let final_variance = rng.random_range(-variance_bound..=variance_bound);
    let num_of_cycles = (map_len * LAKE_CHANCE + final_variance) as i32;
    let mut tree_lake_tiles: Vec<MapCord> = Vec::new();
    let mut grass_lake_tiles: Vec<MapCord> = Vec::new();

    for _ in 0..num_of_cycles {
        let is_surrounded_by_trees = rng.random_bool(0.03);
        let is_surrounded_by_grass = rng.random_bool(0.25);

        let start_x = rng.random_range(0..map_dimensions.width) as i16;
        let start_y = rng.random_range(0..map_dimensions.height) as i16;

        let max_lake_size = rng.random_range(60..=100);

        let mut tiles_placed = 0;

        let mut next_tiles: VecDeque<MapCord> = VecDeque::new();
        next_tiles.push_back(MapCord::new(start_x, start_y));

        let mut current_lake: Vec<MapCord> = Vec::with_capacity(100);

        while next_tiles.len() > 0 && tiles_placed < max_lake_size {
            let current_tile = next_tiles.pop_front().unwrap();

            if !map_utils::is_tile_in_bounds(map_dimensions, current_tile) {
                continue;
            }

            if map_utils::get_tile_at_cord(tile_grid, map_dimensions, current_tile) == TileType::Lake {
                continue;
            }

            let tile_index = map_utils::cords_to_index(map_dimensions, current_tile);

            tile_grid[tile_index] = TileType::Lake;

            current_lake.push(current_tile);

            tiles_placed += 1;

            let chance_for_new_tile = 0.8;

            if next_tiles.len() >= max_lake_size {
                continue;
            }

            for i in 0..CARDINAL_DELTAS.len() {
                if rng.random_bool(chance_for_new_tile) {
                    let dir = CARDINAL_DELTAS[i];
                    next_tiles.push_back(current_tile + dir);
                }
            }
        }

        if is_surrounded_by_trees {
            let mut lake_copy = current_lake.clone();
            tree_lake_tiles.append(&mut lake_copy);
        }

        if is_surrounded_by_grass {
            grass_lake_tiles.append(&mut current_lake);
        }
    }

    return (tree_lake_tiles, grass_lake_tiles);
}

pub fn set_lake_shore_and_corner_sprites(
    tile_grid: &MapTileGrid,
    map_dimensions: MapDimensions,
) -> HashMap<MapCord, LakeSpriteData> {
    let mut lake_sprite_data: HashMap<MapCord, LakeSpriteData> = HashMap::new();

    // rolling with i16 here because if its less than 0 it needs to be caught
    for y in 0..map_dimensions.height as i16 {
        for x in 0..map_dimensions.width as i16 {
            let current = MapCord::new(x, y);

            if map_utils::get_tile_at_cord(tile_grid, map_dimensions, current) != TileType::Lake {
                continue;
            }

            let mut shore_bitmask = 0;

            for i in 0..CARDINAL_DELTAS.len() {
                let neighbor = current + CARDINAL_DELTAS[i];

                if map_utils::tile_not_in_bounds_or_matches(tile_grid, map_dimensions, neighbor, TileType::Lake) {
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

                let check = MapCord::new(diag_x, diag_y);

                if map_utils::tile_not_in_bounds_or_matches(tile_grid, map_dimensions, check, TileType::Lake) {
                    continue;
                }

                // check if should be shore on these specific edges, because that would mean no
                // corner on those edges
                if map_utils::get_tile_from_x_y(tile_grid, map_dimensions, diag_x, current.y)
                    != TileType::Lake
                    || map_utils::get_tile_from_x_y(tile_grid, map_dimensions, current.x, diag_y)
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

pub fn create_rivers(
    map: &mut MapTileGrid,
    lake_data: &HashMap<MapCord, LakeSpriteData>,
    map_dimensions: MapDimensions,
    rng: &mut ThreadRng,
) -> HashMap<MapCord, Direction> {
    const DIR_CHANGE_CHANCE: f64 = 0.01;
    const RIVER_CHANCE_ADJUSTMENT_FOR_CANCELLED_RIVERS: f64 = 0.002;
    const RIVER_CHANCE: f64 = 0.05 + RIVER_CHANCE_ADJUSTMENT_FOR_CANCELLED_RIVERS;
    static OK_SHORE_MASKS: [u8; 4] = [1, 2, 4, 8];

    // (cord, flow dir)
    let estimated_num_of_river_tiles = map_dimensions.total_tiles() / 50;
    let mut all_rivers: HashMap<MapCord, Direction> =
        HashMap::with_capacity(estimated_num_of_river_tiles);

    for (cord, data) in lake_data {
        if !OK_SHORE_MASKS.contains(&data.shore_animation_index) {
            // not viable
            continue;
        }

        // since we found a viable candidate, lets see if it can turn into a river
        if !rng.random_bool(RIVER_CHANCE) {
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
            } else if rng.random_bool(DIR_CHANGE_CHANCE) {
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

            if !is_tile_in_bounds(map_dimensions, check_tile) {
                // end river it reached the end
                add_river(&mut current_river, &mut all_rivers, map, map_dimensions);
                break;
            }

            let check_tile_type = get_tile_at_cord(map, map_dimensions, check_tile);

            if check_tile_type == TileType::River {
                let check_tile_two = check_tile + CARDINAL_DELTAS[direction as usize];

                if !is_tile_in_bounds(map_dimensions, check_tile_two) {
                    // not a cross section because the point past the river is out of bounds, just add this river
                    add_river(&mut current_river, &mut all_rivers, map, map_dimensions);
                    break;
                }

                if get_tile_at_cord(map, map_dimensions, check_tile_two) == TileType::River {
                    // i dont want a cross section piece
                    break;
                }
                // if that isnt the case, then add the river, because it means theres no cross section
                add_river(&mut current_river, &mut all_rivers, map, map_dimensions);
                break;
            } else if check_tile_type == TileType::Lake {
                let lake_sh = lake_data.get(&check_tile).unwrap().shore_animation_index;

                if OK_SHORE_MASKS.contains(&lake_sh) {
                    // end river here, but add shore tile to river for inlet/outlet

                    // check to see if lake tiles are all around it, because if so, scrap it
                    // the problem is that it can have 0 neighbors and thats undesirable
                    let mut counter = 0;

                    for dir in CARDINAL_DELTAS {
                        let t = dir + current_tile;

                        if !is_tile_in_bounds(map_dimensions, t) {
                            // please delete this river its cursed as it is.
                            counter = 4;
                            break;
                        }

                        if let TileType::Lake = map_utils::get_tile_at_cord(map, map_dimensions, t)
                        {
                            counter += 1;
                        }
                    }

                    if counter >= 3 {
                        // end it here, river has no neighbors, its going to crash the neighbor check
                        // 3 because i lowkenuinely want to avoid rivers inside of lakes if possible
                        // i know it kind of looks cool in a way but its not the result im looking for
                        break;
                    }

                    current_river.insert(check_tile, direction);
                    add_river(&mut current_river, &mut all_rivers, map, map_dimensions);
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

            let left_type = match map_utils::is_tile_in_bounds(map_dimensions, tile_to_left) {
                true => Some(map_utils::get_tile_at_cord(
                    map,
                    map_dimensions,
                    tile_to_left,
                )),
                false => None,
            };

            let right_type = match map_utils::is_tile_in_bounds(map_dimensions, tile_to_right) {
                true => Some(map_utils::get_tile_at_cord(
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
                    add_river(&mut current_river, &mut all_rivers, map, map_dimensions);
                    break;
                }
                _ => (),
            }

            // this little section just makes sure a river tile with 4 river neighbors is impossible before adding a tile
            let mut counter = 0;

            for dir in CARDINAL_DELTAS {
                let r = check_tile + dir;

                if !is_tile_in_bounds(map_dimensions, r) {
                    // impossible to have all 4 neighbors so doesnt even need to count it
                    break;
                }

                if get_tile_at_cord(map, map_dimensions, r) == TileType::River {
                    counter += 1;
                }
            }

            if counter == 4 {
                // throw this river away. its not going to work
                // i originally was going to have the game just restart this river but i figured in particularly unlucky
                // circumstances, it would probably end up as an endless loop
                // i will simply raise the average river rate to account for this rare occurrence
                break;
            }

            // prepare for next iteration
            current_river.insert(check_tile, direction);
            current_tile = check_tile;
        }
    }

    return all_rivers;
}

pub fn add_river(
    current_river: &mut HashMap<MapCord, Direction>,
    all_rivers: &mut HashMap<MapCord, Direction>,
    map: &mut MapTileGrid,
    map_dimensions: MapDimensions,
) {
    for riv_tile in current_river {
        map[map_utils::cords_to_index(map_dimensions, *riv_tile.0)] = TileType::River;
        all_rivers.insert(*riv_tile.0, *riv_tile.1);
    }
}

pub fn set_river_tile_animations(
    all_rivers: &HashMap<MapCord, Direction>,
    tile_grid: &MapTileGrid,
    map_dimensions: MapDimensions,
) -> HashMap<MapCord, RiverSpriteData> {
    let mut river_data: HashMap<MapCord, RiverSpriteData> =
        HashMap::with_capacity(all_rivers.iter().count());

    for (cord, dir) in all_rivers {
        // find the number of neighboring river tiles around this tile
        let mut num_of_neighbors: u8 = 0;
        for direction in CARDINAL_DELTAS {
            let check_tile = *cord + direction;

            if map_utils::tile_is_in_bounds_and_matches(tile_grid, map_dimensions, check_tile, TileType::River) {
                num_of_neighbors += 1;
            }
        }

        match num_of_neighbors {
            1 => {
                let check_dir = CARDINAL_DELTAS[*dir as usize];
                let check_tile = *cord + check_dir;

                if !map_utils::is_tile_in_bounds(map_dimensions, check_tile) {
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

                let river_type = if map_utils::get_tile_at_cord(tile_grid, map_dimensions, check_tile)
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

                    if tile_not_in_bounds_or_doesnt_match(tile_grid, map_dimensions, first_tile, TileType::River) {
                        continue;
                    }
                    
                    // river found! now, determine whether its a straight or a corner (both have 2 neighbors)

                    let straight_check_tile = *cord + CARDINAL_DELTAS[(i + 2) % 4];

                    // if its not in bounds, wont check which tile it is, because this has to be a corner
                    if map_utils::is_tile_in_bounds(map_dimensions, straight_check_tile)
                        && map_utils::get_tile_at_cord(tile_grid, map_dimensions, straight_check_tile)
                            == TileType::River
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

                            if tile_not_in_bounds_or_doesnt_match(tile_grid, map_dimensions, second_tile, TileType::River) {
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

                    if tile_not_in_bounds_or_doesnt_match(tile_grid, map_dimensions, first_tile, TileType::River) {
                        continue;
                    }

                    // first tile found!
                    if *all_rivers.get(&first_tile).unwrap() == Direction::South {
                        flow_direction = FlowDirection::DownStream;
                    }

                    for j in (i + 1)..CARDINAL_DELTAS.len() {
                        let second_tile = *cord + CARDINAL_DELTAS[j];

                        if tile_not_in_bounds_or_doesnt_match(tile_grid, map_dimensions, second_tile, TileType::River) {
                            continue;
                        }

                        // second tile found!
                        if *all_rivers.get(&second_tile).unwrap() == Direction::South {
                            flow_direction = FlowDirection::DownStream;
                        }

                        for k in (j + 1)..CARDINAL_DELTAS.len() {
                            let third_tile = *cord + CARDINAL_DELTAS[k];

                            if tile_not_in_bounds_or_doesnt_match(tile_grid, map_dimensions, third_tile, TileType::River) {
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
                "the only valid number of neighbors for a river are 1, 2, 3, this tile should not have made it to the neighbor check, num of neighbors: {}",
                num_of_neighbors
            ),
        }
    }

    return river_data;
}

// helpers for world building specifically
pub fn spawn_forests_around_lakes(
    tile_grid: &MapTileGrid,
    object_grid: &mut MapObjectGrid,
    lake_tiles: Vec<MapCord>,
    map_dimensions: MapDimensions,
    map_cells: &mut Vec<MapCell>,
    rng: &mut ThreadRng,
) {
    for lake_tile in lake_tiles {
        let range = rng.random_range(1..=10);

        for dir in CARDINAL_DELTAS {
            for r in 0..=range {
                let try_tree_tile = lake_tile + (dir * r);

                if tile_not_in_bounds_or_doesnt_match(tile_grid, map_dimensions, try_tree_tile, TileType::Grass) {
                    continue;
                }

                //test if you will succeed in placing a tree on this tile
                if !rng.random_bool(0.1) {
                    continue;
                }

                let index = map_utils::cords_to_index(map_dimensions, try_tree_tile);

                if let Object::NoObject = object_grid[index] {
                    object_grid[index] =
                        Tree::new(try_tree_tile, rng, map_dimensions, map_cells);
                }
            }
        }
    }
}

pub fn spawn_standalone_forests(
    tile_grid: &MapTileGrid,
    object_grid: &mut MapObjectGrid,
    map_dimensions: MapDimensions,
    map_cells: &mut Vec<MapCell>,
    rng: &mut ThreadRng,
) {
    let total_tiles = map_dimensions.total_tiles();
    let cycles = rng.random_range(total_tiles / 10_000..=total_tiles / 5_000);

    for _ in 0..=cycles {
        let horizontal_dir = match rng.random_bool(0.5) {
            true => Direction::West,
            false => Direction::East,
        };

        let vertical_dir = match rng.random_bool(0.5) {
            true => Direction::North,
            false => Direction::South,
        };

        // check if forest will be vertical or horizontal
        let (dir_1, dir_2) = match rng.random_bool(0.5) {
            true => (vertical_dir, horizontal_dir),
            false => (horizontal_dir, vertical_dir),
        };

        let mut start_pos = MapCord::new(
            rng.random_range(0..map_dimensions.width) as i16,
            rng.random_range(0..map_dimensions.height) as i16,
        );

        // height of a column of trees
        let mut height = 1;

        // how many columns there are
        let mut length = 0;

        while height > 0 {
            // make a line of trees from 0 to height
            for j in 0..height {
                let try_tree_tile = start_pos + CARDINAL_DELTAS[dir_1 as usize] * j;

                if tile_not_in_bounds_or_doesnt_match(tile_grid, map_dimensions, try_tree_tile, TileType::Grass) {
                    continue;
                }

                let idx = map_utils::cords_to_index(map_dimensions, try_tree_tile);

                if let Object::NoObject = object_grid[idx] {
                    object_grid[idx] = Tree::new(try_tree_tile, rng, map_dimensions, map_cells);
                }
            }

            // move it one tile to the left or right
            start_pos += CARDINAL_DELTAS[dir_2 as usize];

            // move randomly, primarily in the direction of the first dir
            if rng.random_bool(0.5) {
                if rng.random_bool(0.2) {
                    start_pos -= CARDINAL_DELTAS[dir_1 as usize];
                } else {
                    start_pos += CARDINAL_DELTAS[dir_1 as usize];
                }
            }

            length += 1;

            // change height based on length based weights
            match length {
                ..=11 => {
                    if rng.random_bool(0.2) {
                        height += 1;
                    }
                    if rng.random_bool(0.05) {
                        height -= 1;
                    }
                }
                ..25 => {
                    if rng.random_bool(0.8) {
                        height += 1;
                    }
                    if rng.random_bool(0.8) {
                        height -= 1;
                    }
                }
                25.. => {
                    if rng.random_bool(0.2) {
                        height -= 1;
                    }
                    if rng.random_bool(0.05) {
                        height += 1;
                    }
                }
            }
        }
    }
}

pub fn spawn_standalone_trees(
    tile_grid: &MapTileGrid,
    object_grid: &mut MapObjectGrid,
    map_dimensions: MapDimensions,
    map_cells: &mut Vec<MapCell>,
    rng: &mut ThreadRng,
) {
    let num_of_trees =
        (map_dimensions.total_tiles() as f32 * rng.random_range(0.001..=0.002)) as i32;

    for _ in 0..=num_of_trees {
        loop {
            let rand_x = rng.random_range(0..map_dimensions.width);
            let rand_y = rng.random_range(0..map_dimensions.height);

            let try_tree_tile = MapCord::new(rand_x as i16, rand_y as i16);

            // keep trying until you find a grass tile
            if map_utils::get_tile_at_cord(tile_grid, map_dimensions, try_tree_tile) != TileType::Grass {
                continue;
            }

            let idx = map_utils::cords_to_index(map_dimensions, try_tree_tile);

            if let Object::NoObject = object_grid[idx] {
                object_grid[idx] = Tree::new(try_tree_tile, rng, map_dimensions, map_cells);
                break;
            }
        }
    }
}

pub fn spawn_standalone_grass(
    tile_grid: &MapTileGrid,
    object_grid: &mut MapObjectGrid,
    map_dimensions: MapDimensions,
    cells: &mut Vec<MapCell>,
    game_context: &mut GameContext,
) {
    let num_of_grass =
        (map_dimensions.total_tiles() as f32 * game_context.rng.random_range(0.01..=0.02)) as i32;

    for _ in 0..=num_of_grass {
        loop {
            let rand_x = game_context.rng.random_range(0..map_dimensions.width);
            let rand_y = game_context.rng.random_range(0..map_dimensions.height);

            let try_grass_tile = MapCord::new(rand_x as i16, rand_y as i16);

            // keep trying until you find a grass tile
            if map_utils::get_tile_at_cord(tile_grid, map_dimensions, try_grass_tile) != TileType::Grass {
                continue;
            }

            let idx = map_utils::cords_to_index(map_dimensions, try_grass_tile);

            if let Object::NoObject = object_grid[idx] {
                object_grid[idx] = Grass::new(try_grass_tile, game_context, map_dimensions, cells);
                break;
            }
        }
    }
}

pub fn spawn_grass_around_lakes(
    tile_grid: &MapTileGrid,
    object_grid: &mut MapObjectGrid,
    lake_tiles: Vec<MapCord>,
    map_dimensions: MapDimensions,
    cells: &mut Vec<MapCell>,
    game_context: &mut GameContext,
) {
    for lake_tile in lake_tiles {
        let mut range = game_context.rng.random_range(4..=12);

        let extra_large_grass_patch = game_context.rng.random_bool(0.4);

        if extra_large_grass_patch {
            range += game_context.rng.random_range(2..=5);
        }

        for dir in ORTHOGONAL_DELTAS {
            for range_out in 1..=range {
                let normalized_range = range_out as f64 / range as f64;

                if !game_context.rng.random_bool(0.1) {
                    continue;
                }

                let try_grass_tile = dir * range_out + lake_tile;

                if tile_not_in_bounds_or_doesnt_match(tile_grid, map_dimensions, try_grass_tile, TileType::Grass) {
                    continue;
                }

                let idx = map_utils::cords_to_index(map_dimensions, try_grass_tile);

                let obj = &mut object_grid[idx];

                if let Object::NoObject = obj {
                    // spawn different grass levels on average depending on distance from the water
                    match normalized_range {
                        0.0..=0.2 => {
                            *obj = Grass::new_large_likely(
                                try_grass_tile,
                                game_context,
                                map_dimensions,
                                cells,
                            )
                        }
                        0.7..=1.0 => {
                            *obj = Grass::new_small_likely(
                                try_grass_tile,
                                game_context,
                                map_dimensions,
                                cells,
                            )
                        }
                        _ => *obj = Grass::new(try_grass_tile, game_context, map_dimensions, cells),
                    }
                }
            }
        }
    }
}

pub fn spawn_grass_around_rivers(
    tile_grid: &MapTileGrid,
    object_grid: &mut MapObjectGrid,
    river_tiles: &HashMap<MapCord, RiverSpriteData>,
    map_dimensions: MapDimensions,
    cells: &mut Vec<MapCell>,
    game_context: &mut GameContext,
) {
    for (cord, _) in river_tiles {
        let range = game_context.rng.random_range(1..=8);

        for dir in CARDINAL_DELTAS {
            for range_out in 1..=range {
                let normalized_range = range_out as f64 / range as f64;

                // make it more likely to spawn the closer it is to the river
                if !game_context
                    .rng
                    .random_bool(0.5 + (0.2 - (normalized_range / 5.0)))
                {
                    continue;
                }

                let try_grass_tile = dir * range_out + *cord;

                if tile_not_in_bounds_or_doesnt_match(tile_grid, map_dimensions, try_grass_tile, TileType::Grass) {
                    continue;
                }

                let idx = map_utils::cords_to_index(map_dimensions, try_grass_tile);

                let obj = &mut object_grid[idx];

                if let Object::NoObject = obj {
                    // spawn different grass levels on average depending on distance from the water
                    match normalized_range {
                        0.0..=0.2 => {
                            *obj = Grass::new_large_likely(
                                try_grass_tile,
                                game_context,
                                map_dimensions,
                                cells,
                            )
                        }
                        0.7..=1.0 => {
                            *obj = Grass::new_small_likely(
                                try_grass_tile,
                                game_context,
                                map_dimensions,
                                cells,
                            )
                        }
                        _ => *obj = Grass::new(try_grass_tile, game_context, map_dimensions, cells),
                    }
                }
            }
        }
    }
}

pub fn spawn_fields_of_grass(
    tile_grid: &MapTileGrid,
    object_grid: &mut MapObjectGrid,
    map_dimensions: MapDimensions,
    map_cells: &mut Vec<MapCell>,
    game_context: &mut GameContext,
) {
    let total_tiles = map_dimensions.total_tiles();
    let cycles = game_context
        .rng
        .random_range(total_tiles / 10_000..=total_tiles / 5_000);

    for _ in 0..=cycles {
        let horizontal_dir = match game_context.rng.random_bool(0.5) {
            true => Direction::West,
            false => Direction::East,
        };

        let vertical_dir = match game_context.rng.random_bool(0.5) {
            true => Direction::North,
            false => Direction::South,
        };

        // check if forest will be vertical or horizontal
        let (dir_1, dir_2) = match game_context.rng.random_bool(0.5) {
            true => (vertical_dir, horizontal_dir),
            false => (horizontal_dir, vertical_dir),
        };

        let mut start_pos = MapCord::new(
            game_context.rng.random_range(0..map_dimensions.width) as i16,
            game_context.rng.random_range(0..map_dimensions.height) as i16,
        );

        // height of a column of trees
        let mut height = 1;

        // how many columns there are
        let mut length = 0;

        while height > 0 {
            // make a line of trees from 0 to height
            for j in 0..height {
                let normalized_progress = j as f64 / height as f64;

                // if at edges, just plain have a chance not to spawn one
                if normalized_progress < 0.05 || normalized_progress > 0.95 {
                    if game_context.rng.random_bool(0.15) {
                        continue;
                    }
                }

                let try_grass_tile = start_pos + CARDINAL_DELTAS[dir_1 as usize] * j;

                if tile_not_in_bounds_or_doesnt_match(tile_grid, map_dimensions, try_grass_tile, TileType::Grass) {
                    continue;
                }

                let idx = map_utils::cords_to_index(map_dimensions, try_grass_tile);

                let obj = &mut object_grid[idx];

                if let Object::NoObject = obj {
                    // meant to be used on any thin parts (typically the end)
                    if height <= 4 {
                        if game_context.rng.random_bool(0.03) {
                            continue;
                        }
                        *obj = Grass::new_small_likely(
                            try_grass_tile,
                            game_context,
                            map_dimensions,
                            map_cells,
                        )
                    } else {
                        // if its on the edges of the field, make it likely to be small, if in middle, its likely to be large, else, random size
                        match normalized_progress {
                            0.0..=0.15 | 0.85..=1.0 => {
                                // even though the spot is viable, if its on the edges, give it some variation by just not spawning one
                                if game_context.rng.random_bool(0.05) {
                                    continue;
                                }
                                *obj = Grass::new_small_likely(
                                    try_grass_tile,
                                    game_context,
                                    map_dimensions,
                                    map_cells,
                                )
                            }
                            0.4..=0.6 => {
                                *obj = Grass::new_large_likely(
                                    try_grass_tile,
                                    game_context,
                                    map_dimensions,
                                    map_cells,
                                )
                            }
                            _ => {
                                *obj = Grass::new(
                                    try_grass_tile,
                                    game_context,
                                    map_dimensions,
                                    map_cells,
                                )
                            }
                        }
                    }
                }
            }

            // move it one tile to the left or right
            start_pos += CARDINAL_DELTAS[dir_2 as usize];

            length += 1;

            let rng = &mut game_context.rng;

            // change height based on length based weights
            match length {
                ..=15 => {
                    if rng.random_bool(0.4) {
                        height += rng.random_range(2..=5);
                        start_pos -= CARDINAL_DELTAS[dir_1 as usize] * rng.random_range(1..=3)
                    }
                }
                ..35 => {
                    if rng.random_bool(0.8) {
                        height += rng.random_range(1..=2);
                        start_pos -= CARDINAL_DELTAS[dir_1 as usize] * rng.random_range(1..=3)
                    }
                    if rng.random_bool(0.8) {
                        height -= rng.random_range(1..=2);
                        start_pos += CARDINAL_DELTAS[dir_1 as usize] * rng.random_range(1..=3)
                    }
                }
                35.. => {
                    if rng.random_bool(0.4) {
                        height -= rng.random_range(2..=5);
                        start_pos += CARDINAL_DELTAS[dir_1 as usize] * rng.random_range(1..=3)
                    }
                }
            }
        }
    }
}
