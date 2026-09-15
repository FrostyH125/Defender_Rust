use crate::{
    map::{
        tile::TileType,
        tile_map::{MapDimensions, MapTileGrid},
    },
    utils::map_cord::MapCord,
};

#[inline]
pub fn get_tile_from_x_y(
    tile_grid: &MapTileGrid,
    map_dimensions: MapDimensions,
    x: i16,
    y: i16,
) -> TileType {
    let idx = y as usize * map_dimensions.width as usize + x as usize;

    return tile_grid[idx];
}

#[inline]
pub fn is_tile_in_bounds(map_dimensions: MapDimensions, cord: MapCord) -> bool {
    let is_x_in_bounds = cord.x >= 0 && cord.x < map_dimensions.width as i16;
    let is_y_in_bounds = cord.y >= 0 && cord.y < map_dimensions.height as i16;

    return is_x_in_bounds && is_y_in_bounds;
}

#[inline]
pub fn get_tile_at_cord(
    tile_grid: &MapTileGrid,
    map_dimensions: MapDimensions,
    cord: MapCord,
) -> TileType {
    let idx = cord.y as usize * map_dimensions.width as usize + cord.x as usize;

    return tile_grid[idx];
}

#[inline]
pub fn tile_is_in_bounds_and_matches(
    tile_grid: &MapTileGrid,
    map_dimensions: MapDimensions,
    cord: MapCord,
    tile_type: TileType,
) -> bool {
    if !is_tile_in_bounds(map_dimensions, cord) {
        return false;
    }
    if get_tile_at_cord(tile_grid, map_dimensions, cord) == tile_type {
        return true;
    } else {
        return false;
    }
}

#[inline]
pub fn tile_not_in_bounds_or_matches(
    tile_grid: &MapTileGrid,
    map_dimensions: MapDimensions,
    cord: MapCord,
    tile_type: TileType,
) -> bool {
    if !is_tile_in_bounds(map_dimensions, cord) {
        return true;
    }
    if get_tile_at_cord(tile_grid, map_dimensions, cord) == tile_type {
        return true;
    } else {
        return false;
    }
}

#[inline]
pub fn tile_not_in_bounds_or_doesnt_match(
    tile_grid: &MapTileGrid,
    map_dimensions: MapDimensions,
    cord: MapCord,
    tile_type: TileType,
) -> bool {
    if !is_tile_in_bounds(map_dimensions, cord) {
        return true;
    }
    if get_tile_at_cord(tile_grid, map_dimensions, cord) != tile_type {
        return true;
    } else {
        return false;
    }
}

#[inline]
pub fn cords_to_index(map_dimensions: MapDimensions, cord: MapCord) -> usize {
    let y_u = cord.y as usize;
    let x_u = cord.x as usize;
    return y_u * map_dimensions.width as usize + x_u;
}