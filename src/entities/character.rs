use raylib::{
    drawing::RaylibDrawHandle,
    math::{Rectangle, Vector2},
    texture::Texture2D,
};
use zander_game_core_rs::raylib::sprite::Sprite;

use crate::{
    GameContext, TILE_SIZE,
    entities::{
        characters::gatherer::{Gatherer, GathererState},
        object::Object,
    },
    map::tile_map::{MapDimensions, TileMap},
    utils::{
        camera_utils,
        direction_utils::FacingDirection,
        draw_utils,
        map_cord::MapCord,
        map_utils,
        pathfinding::PathResult::{self, NoPath},
    },
};

pub enum CharacterMovementResult {
    Success,
    NotArrivedYet,
    NoRoute,
    TooLong,
}

pub struct CharacterData {
    pub path: PathResult,
    pub pos: Vector2,
    draw_offset: Vector2,
    pub target_pos: Option<Vector2>,
    width: f32,
    height: f32,
    move_speed: f32,
    pub facing_direction: FacingDirection,
    pub is_moving_to_pos: bool,
    pub is_hovering: bool,
    pub is_hovering_for_move: bool,
    pub is_selected: bool,
    pub is_selected_for_move: bool,
}

impl CharacterData {
    pub fn new(
        pos: Vector2,
        draw_offset: Vector2,
        width: f32,
        height: f32,
        move_speed: f32,
    ) -> CharacterData {
        return CharacterData {
            pos,
            draw_offset,
            target_pos: None,
            path: NoPath,
            width,
            height,
            move_speed,
            facing_direction: FacingDirection::Right,
            is_hovering: false,
            is_hovering_for_move: false,
            is_selected: false,
            is_selected_for_move: false,
            is_moving_to_pos: false,
        };
    }

    pub fn move_to(
        &mut self,
        target: Vector2,
        game_context: &mut GameContext,
        map: &TileMap,
    ) -> CharacterMovementResult {
        // compare current target to new target
        if self.target_pos != Some(target) || matches!(self.path, PathResult::NoPath) {
            self.target_pos = Some(target);
            self.path = game_context.path_finder.a_star(
                MapCord::from_vec2(self.pos),
                MapCord::from_vec2(target),
                map,
                100.0,
            );
            if let PathResult::Success { path } = &mut self.path {
                path.push_back(target);

                // if the target path entry is more than or equal to an eigth of a block away from the last
                // real final path entry, remove the real final path entry
                // this reduces backtracking

                if path.len() > 1 {
                    if path[0].distance_to(path[1]) >= 2.0 {
                        path.remove(1);
                    }
                }
            }
        }

        if let PathResult::TooLong = self.path {
            println!("Route is too long for character");
            return CharacterMovementResult::TooLong;
        }

        if let PathResult::NoRoute = self.path {
            println!("There is no viable route for character");
            return CharacterMovementResult::NoRoute;
        }

        if let PathResult::Success { path } = &mut self.path {
            if path.is_empty() {
                return CharacterMovementResult::Success;
            }

            let mut next = &path[0];

            // get the next tile
            if self.pos.distance_to(*next) <= 1.0 {
                if path.len() > 1 {
                    path.pop_front();
                    next = &path[0];
                } else if path.len() == 1 {
                    // if theres only one left and youre running this code,
                    // this means youve made it to the only tile left, which is the target
                    self.pos = target;
                    self.path = NoPath;
                    self.target_pos = None;
                    return CharacterMovementResult::Success;
                }
            }

            let mut delta = *next - self.pos;

            if delta.y.abs() > 1.0 || delta.x.abs() > 1.0 {
                delta.normalize();
            }

            self.pos += delta * self.move_speed * game_context.dt;

            if delta.x < 0.0 {
                self.facing_direction = FacingDirection::Left;
            } else {
                self.facing_direction = FacingDirection::Right;
            }
        }

        return CharacterMovementResult::NotArrivedYet;
    }
}

pub enum Character {
    GathererChar(Gatherer),
}

impl Character {
    pub fn start_moving_to(&mut self, pos: Vector2) {
        self.reset_state();

        let data = self.get_mut_data();
        data.is_moving_to_pos = true;

        // need to manually clear the path because the algorithm wont actually
        // make a new path when i manually set the target pos. I would have to
        // call 'move_to()' manually and theres just not really any reason
        // to do that since id have to pass game_context and map and i dont want to
        data.path = NoPath;
        data.target_pos = Some(pos)
    }

    #[inline]
    pub fn get_data(&self) -> &CharacterData {
        match self {
            Character::GathererChar(gatherer) => &gatherer.data,
        }
    }

    #[inline]
    pub fn get_mut_data(&mut self) -> &mut CharacterData {
        match self {
            Character::GathererChar(gatherer) => &mut gatherer.data,
        }
    }

    #[inline]
    pub fn update(&mut self, game_context: &mut GameContext, map: &mut TileMap) {
        match self {
            Character::GathererChar(gatherer) => {
                if let GathererState::MovingToObject { .. } = gatherer.state {
                    gatherer.data.is_moving_to_pos = false;
                }
                gatherer.update(game_context, map)
            }
        }
        let data = self.get_mut_data();

        if data.is_moving_to_pos {
            match data.move_to(data.target_pos.unwrap(), game_context, map) {
                CharacterMovementResult::NotArrivedYet => (),
                _ => {
                    data.is_moving_to_pos = false;
                    data.target_pos = None;
                }
            }
        }

        data.is_hovering = false;
        data.is_hovering_for_move = false;
    }

    #[inline]
    pub fn is_point_intersecting(&self, p: Vector2) -> bool {
        return self.get_hover_rect().check_collision_point_rec(p);
    }

    #[inline]
    pub fn draw(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        let sprite = self.current_sprite();
        sprite.draw(d, self.get_draw_pos(), texture);
    }

    #[inline]
    pub fn draw_hover(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        let sprite = self.current_sprite();
        draw_utils::draw_outline(d, sprite, self.get_draw_pos(), texture);
    }

    #[inline]
    pub fn draw_selected(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        let sprite = self.current_sprite();
        draw_utils::draw_with_extra_brightness(d, sprite, self.get_draw_pos(), texture);
    }

    #[inline]
    pub fn draw_hover_for_move(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        let sprite = self.current_sprite();
        draw_utils::draw_outline_for_move(d, sprite, self.get_draw_pos(), texture);
    }

    #[inline]
    pub fn draw_shadow(
        &self,
        d: &mut RaylibDrawHandle,
        texture: &Texture2D,
        shadow_shear: f32,
        shadow_scale: f32,
    ) {
        let sprite = self.current_sprite();

        draw_utils::draw_shadow(
            d,
            sprite,
            self.get_draw_pos(),
            shadow_shear,
            shadow_scale,
            texture,
        );
    }

    pub fn current_sprite(&self) -> Sprite {
        let mut spr = match self {
            Character::GathererChar(gatherer) => gatherer.sprite(),
        };

        if self.get_data().facing_direction == FacingDirection::Left {
            // if i dont -0.1 then the width will apparently be less than the width due
            // to floating point shenanigans, since its drawn to a low res grid its
            // rounded down, i have to add to the width essentially
            spr.src_rect.width = -spr.src_rect.width - 0.1;
        }

        return spr;
    }

    #[inline]
    pub fn get_draw_pos(&self) -> Vector2 {
        let data = self.get_data();
        return data.pos + data.draw_offset;
    }

    #[inline]
    pub fn get_hover_rect(&self) -> Rectangle {
        let data = self.get_data();
        let d_pos = self.get_draw_pos();
        return Rectangle::new(d_pos.x, d_pos.y, data.width, data.height);
    }

    #[inline]
    pub fn get_tile_index(&self, map_dimensions: MapDimensions) -> usize {
        let pos = self.get_data().pos;
        let cord = MapCord::new(
            pos.x as i16 / TILE_SIZE as i16,
            pos.y as i16 / TILE_SIZE as i16,
        );
        return map_utils::cords_to_index(map_dimensions, cord);
    }

    /// characters are rendered one tile later than their actual pos tile.
    /// since rendering uses the tile indices in a single dimension,
    /// objects immediately to the right would draw over the character
    /// when they should be drawn behind it if one wasn't added
    #[inline]
    pub fn get_render_tile_index(&self, map_dimensions: MapDimensions) -> usize {
        let idx = self.get_tile_index(map_dimensions);
        return idx + 1;
    }

    #[inline]
    pub fn update_obj_if_out_of_update_range(
        object: &mut Object,
        game_context: &mut GameContext,
        map: &mut TileMap,
    ) {
        let object_pos = object.get_data().pos;

        if !camera_utils::is_in_update_area(object_pos, game_context) {
            return;
        }

        object.update(
            game_context,
            false,
            &mut map.map_cell_grid,
            map.map_dimensions,
        );
    }

    pub fn reset_state(&mut self) {
        match self {
            Character::GathererChar(gatherer) => gatherer.state = GathererState::Idle,
        }
    }
}
