use crate::{map::{tile::TileType, tile_map::{MapDimensions, MapTileGrid}}, utils::map_cord::MapCord};

pub fn get_tile_from_x_y(map: &MapTileGrid, map_dimensions: MapDimensions, x: i16, y: i16) -> TileType {
    let idx = y as usize * map_dimensions.width as usize + x as usize;

    return map[idx];
}

pub fn is_tile_in_bounds(map_dimensions: MapDimensions, cord: MapCord) -> bool {
    let is_x_in_bounds = cord.x >= 0 && cord.x < map_dimensions.width as i16;
    let is_y_in_bounds = cord.y >= 0 && cord.y < map_dimensions.height as i16;

    return is_x_in_bounds && is_y_in_bounds;
}

pub fn get_tile_at_cord(map: &MapTileGrid, map_dimensions: MapDimensions, cord: MapCord) -> TileType {
    let idx = cord.y as usize * map_dimensions.width as usize + cord.x as usize;

    return map[idx];
}

pub fn cords_to_index(map_dimensions: MapDimensions, cord: MapCord) -> usize {
    let y_u = cord.y as usize;
    let x_u = cord.x as usize;
    return y_u * map_dimensions.width as usize + x_u;
}