use std::collections::HashSet;

use basic_raylib_core::graphics::sprite::Sprite;
use rand::rngs::ThreadRng;
use raylib::{
    drawing::RaylibDrawHandle,
    math::{Rectangle, Vector2},
    texture::Texture2D,
};

use crate::{
    GameContext, entities::{
        object::Object::*,
        objects::{grass::Grass, tree::Tree},
    }, map::{map_cell::MapCell, tile_map::{MapDimensions, TileMap}}, systems::day_night_cycle::DayNightCycle, utils::{camera_utils, draw_utils, map_cord::MapCord, map_utils::get_cell_at_cord, vector2_utils::v2_to_cord},
};

#[derive(PartialEq, Eq)]
pub enum ObjectState {
    Idle,
    Breaking,
    WaitingForDeletion,
}

/// This houses data that all objects share, as to not repeat fields between objects
pub struct ObjectData {
    pub pos: Vector2,
    pub draw_pos: Vector2,
    pub hover_rect: Rectangle,
    shadow_shear_x: f32,
    shadow_scale_y: f32,
    pub is_hovering: bool,
    pub is_selected: bool,
    state: ObjectState
}

impl ObjectData {
    pub fn new(
        pos: Vector2,
        draw_offset: Vector2,
        randomized_offset: Vector2,
        map_cells: &mut Vec<MapCell>,
        map_dimensions: MapDimensions,
        width: f32,
        height: f32,
    ) -> Self {
        let true_pos = pos + randomized_offset;
        let draw_pos = true_pos + draw_offset;

        let hover_rect = Rectangle::new(draw_pos.x, draw_pos.y, width, height);

        // add current index of object to appropriate cell
        let map_cord = v2_to_cord(pos);
        let cell = get_cell_at_cord(map_cells, map_dimensions, map_cord).unwrap();
        cell.add_obj_from_cord(map_dimensions, map_cord);

        return ObjectData {
            pos: true_pos,
            draw_pos,
            hover_rect,
            shadow_shear_x: 0.0,
            shadow_scale_y: 0.0,
            is_hovering: false,
            is_selected: false,
            state: ObjectState::Idle
        };
    }
}

pub enum Object {
    NoObject,
    TreeObj(Tree),
    GrassObj(Grass),
}

impl Object {
    pub fn get_data(&self) -> &ObjectData {
        match self {
            TreeObj(tree) => &tree.data,
            GrassObj(grass) => &grass.data,
            NoObject => panic!("why would you try to get data from a None Object?"),
        }
    }

    pub fn get_mut_data(&mut self) -> &mut ObjectData {
        match self {
            TreeObj(tree) => &mut tree.data,
            GrassObj(grass) => &mut grass.data,
            NoObject => panic!("why would you try to get data from a None Object?"),
        }
    }

    pub fn update(
        &mut self,
        game_context: &mut GameContext
        
    ) {

        match self {
            TreeObj(tree) => tree.update(game_context.dt),
            GrassObj(grass) => grass.update(game_context),
            // pass if none
            NoObject => return,
        }

        
        
        let data = self.get_mut_data();
        
        match data.state {
        ObjectState::Idle => {
            data.is_hovering = false;
            data.shadow_shear_x = game_context.day_night_cycle.current_shadow_shear;
            data.shadow_scale_y = game_context.day_night_cycle.current_shadow_scale;
        },
            ObjectState::Breaking => {
                // only remove if out of camera view, otherwise, carry to completion
                if !camera_utils::is_in_camera_view(&data.hover_rect, game_context) {
                    *self = Self::NoObject
                }
            },
            ObjectState::WaitingForDeletion => {
                // remove no matter what, this object is ready to go
                // can be removed on same frame as set for deletion, so, no
                // worry about going out of bounds in this state (which would be a 1 frame window otherwise)
                *self = Self::NoObject
            },
        }
    }

    pub fn is_point_intersecting(&self, p: Vector2) -> bool {
        return self.get_data().hover_rect.check_collision_point_rec(p);
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        let sprite = self.current_sprite();

        sprite.draw(d, self.get_data().draw_pos, texture);
    }

    pub fn draw_hover(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        let sprite = self.current_sprite();
        draw_utils::draw_outline(d, sprite, self.get_data().draw_pos, texture);
    }

    pub fn draw_shadow(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        let sprite = self.current_sprite();
        let data = self.get_data();

        draw_utils::draw_shadow(d, sprite, data.draw_pos, data.shadow_shear_x, data.shadow_scale_y, texture);
    }

    pub fn current_sprite(&self) -> &Sprite {
        match self {
            NoObject => todo!(),
            TreeObj(tree) => tree.sprite(),
            GrassObj(grass) => grass.sprite(),
        }
    }
}
