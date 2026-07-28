use basic_raylib_core::graphics::sprite::Sprite;
use rand::rngs::ThreadRng;
use raylib::{
    drawing::RaylibDrawHandle,
    math::{Rectangle, Vector2},
    texture::Texture2D,
};

use crate::{TILE_SIZE, entities::characters::gatherer::Gatherer, map::tile_map::MapDimensions, systems::day_night_cycle::DayNightCycle, utils::{draw_utils, map_cord::MapCord, map_utils}};


pub struct CharacterData {
    pub pos: Vector2,
    draw_offset: Vector2,
    width: f32,
    height: f32,
    shadow_shear_x: f32,
    shadow_scale_y: f32,
    pub is_hovering: bool,
}

impl CharacterData {
    pub fn new(pos: Vector2, draw_offset: Vector2, width: f32, height: f32) -> CharacterData {
        return CharacterData {
            pos,
            draw_offset,
            width,
            height,
            shadow_shear_x: 0.0,
            shadow_scale_y: 0.0,
            is_hovering: false,
        }
    }
}

pub enum Character {
    GathererChar(Gatherer)
}

impl Character {
    pub fn get_data(&self) -> &CharacterData {
        match self {
            Character::GathererChar(gatherer) => &gatherer.data,
        }
    }

    pub fn get_mut_data(&mut self) -> &mut CharacterData {
        match self {
            Character::GathererChar(gatherer) => &mut gatherer.data,
        }
    }

    pub fn update(&mut self, dt: f32) {
        match self {
            Character::GathererChar(gatherer) => gatherer.update(dt),
        }
    }

    pub fn is_point_intersecting(&self, p: Vector2) -> bool {
        return self.get_hover_rect().check_collision_point_rec(p)
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        let sprite = self.current_sprite();
        sprite.draw(d, self.get_draw_pos(), texture);
    }

    pub fn draw_hover(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        let sprite = self.current_sprite();
        draw_utils::draw_outline(d, sprite, self.get_draw_pos(), texture);
    }

    pub fn draw_shadow(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        let sprite = self.current_sprite();
        let data = self.get_data();

        draw_utils::draw_shadow(d, sprite, self.get_draw_pos(), data.shadow_shear_x, data.shadow_scale_y, texture);
    }

    pub fn current_sprite(&self) -> &Sprite {
        match self {
            Character::GathererChar(gatherer) => gatherer.sprite(),
        }
    }

    pub fn get_draw_pos(&self) -> Vector2 {
        let data = self.get_data();
        return data.pos + data.draw_offset;
    }

    pub fn get_hover_rect(&self) -> Rectangle {
        let data = self.get_data();
        let d_pos = self.get_draw_pos();
        return Rectangle::new(d_pos.x, d_pos.y, data.width, data.height);
    }

    pub fn get_tile_index(&self, map_dimensions: MapDimensions) -> usize {
        let pos = self.get_data().pos;
        let cord = MapCord::new(pos.x as i16 / TILE_SIZE as i16, pos.y as i16 / TILE_SIZE as i16);
        return map_utils::cords_to_index(map_dimensions, cord)
    }

    /// characters are rendered one tile later than their actual pos tile.
    /// since rendering uses the tile indices in a single dimension,
    /// objects immediately to the right would draw over the character
    /// when they should be drawn behind it if one wasn't added
    pub fn get_render_tile_index(&self, map_dimensions: MapDimensions) -> usize {
        let idx = self.get_tile_index(map_dimensions);
        return idx + 1;
    }
}
