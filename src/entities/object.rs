
use raylib::{
    drawing::RaylibDrawHandle,
    math::{Rectangle, Vector2},
    texture::Texture2D,
};
use zander_game_core_rs::{raylib::sprite::Sprite, system::timer::Timer};

use crate::{
    GameContext, entities::{
        object::{Object::*, ObjectState::GettingHit},
        objects::{grass::Grass, tree::Tree},
    }, map::{map_cell::MapCell, tile_map::MapDimensions}, utils::{
        camera_utils,
        direction_utils::FacingDirection,
        draw_utils,
        map_cord::MapCord,
        map_utils::{cords_to_index, get_cell_at_cord},
    },
};

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum ObjectState {
    Idle,
    GettingHit,
    Breaking,
}

/// This houses data that all objects share, as to not repeat fields between objects
pub struct ObjectData {
    pub pos: Vector2,
    pub draw_pos: Vector2,
    pub situational_draw_offset: Vector2,
    width: f32,
    height: f32,
    hit_timer: Timer,
    disappear_timer: Timer,
    health: f32,
    pub cord: MapCord,
    pub is_hovering: bool,
    pub is_selected: bool,
    pub is_occupied: bool,
    pub is_marked_for_gathering: bool,
    pub state: ObjectState,
    pub sprite_flip: bool,
}

impl ObjectData {
    pub fn new(
        pos: Vector2,
        draw_offset: Vector2,
        randomized_offset: Vector2,
        cord: MapCord,
        map_cells: &mut Vec<MapCell>,
        map_dimensions: MapDimensions,
        width: f32,
        height: f32,
        health: f32,
        hit_timer_duration: f32,
        disappear_timer_duration: f32,
    ) -> Self {
        let true_pos = pos + randomized_offset;
        let draw_pos = true_pos + draw_offset;

        // add current index of object to appropriate cell
        let map_cord = MapCord::from_vec2(pos);
        let cell = get_cell_at_cord(map_cells, map_dimensions, map_cord).unwrap();
        cell.add_obj_from_cord(map_dimensions, map_cord);

        return ObjectData {
            pos: true_pos,
            draw_pos,
            situational_draw_offset: Vector2::default(),
            width,
            height,
            health,
            cord,
            hit_timer: Timer::new(hit_timer_duration),
            disappear_timer: Timer::new(disappear_timer_duration),
            is_hovering: false,
            is_selected: false,
            is_occupied: false,
            is_marked_for_gathering: false,
            state: ObjectState::Idle,
            sprite_flip: false,
        };
    }

    #[inline]
    pub fn hover_rect(&self) -> Rectangle {
        return Rectangle::new(self.draw_pos.x, self.draw_pos.y, self.width, self.height);
    }
}

pub enum Object {
    NoObject,
    TreeObj(Tree),
    GrassObj(Grass),
}

impl Object {
    #[inline]
    pub fn get_data(&self) -> &ObjectData {
        match self {
            TreeObj(tree) => &tree.data,
            GrassObj(grass) => &grass.data,
            NoObject => panic!("why would you try to get data from a None Object?"),
        }
    }

    #[inline]
    pub fn get_mut_data(&mut self) -> &mut ObjectData {
        match self {
            TreeObj(tree) => &mut tree.data,
            GrassObj(grass) => &mut grass.data,
            NoObject => panic!("why would you try to get data from a None Object?"),
        }
    }

    #[inline]
    pub fn update(
        &mut self,
        game_context: &mut GameContext,
        should_deselect: bool,
        cells: &mut [MapCell],
        map_dimensions: MapDimensions,
    ) {
        match self {
            TreeObj(tree) => tree.update(game_context),
            GrassObj(grass) => grass.update(game_context),
            // pass if none
            NoObject => return,
        }

        let data = self.get_mut_data();

        if should_deselect {
            data.is_selected = false;
        }

        match data.state {
            ObjectState::Idle => {
                data.is_hovering = false;
            }
            ObjectState::Breaking => {
                // only remove if out of camera view, otherwise, carry to completion
                if !camera_utils::is_in_camera_view(&self.hover_rect(), game_context) {
                    self.delete(map_dimensions, cells);
                    return;
                }

                let disappear_timer = &mut self.get_mut_data().disappear_timer;

                disappear_timer.track(game_context.dt);
                if disappear_timer.is_done() {
                    self.delete(map_dimensions, cells);
                    return;
                }
            }
            ObjectState::GettingHit => {
                let hit_timer = &mut self.get_mut_data().hit_timer;

                hit_timer.track(game_context.dt);

                if hit_timer.is_done() {
                    hit_timer.reset();
                    self.get_mut_data().state = ObjectState::Idle;
                    self.on_out_of_hit();
                }
            }
        }
    }

    #[inline]
    pub fn is_point_intersecting(&self, p: Vector2) -> bool {
        return self.hover_rect().check_collision_point_rec(p);
    }

    #[inline]
    pub fn draw(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        let sprite = self.current_sprite();

        sprite.draw(
            d,
            self.get_data().draw_pos + self.get_data().situational_draw_offset,
            texture,
        );
    }

    #[inline]
    pub fn draw_hover(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        let sprite = self.current_sprite();
        draw_utils::draw_outline(
            d,
            sprite,
            self.get_data().draw_pos + self.get_data().situational_draw_offset,
            texture,
        );
    }

    #[inline]
    pub fn draw_selected(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        let sprite = self.current_sprite();
        draw_utils::draw_with_extra_brightness(
            d,
            sprite,
            self.get_data().draw_pos + self.get_data().situational_draw_offset,
            texture,
        );
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
        let data = self.get_data();

        draw_utils::draw_shadow(
            d,
            sprite,
            data.draw_pos + data.situational_draw_offset,
            shadow_shear,
            shadow_scale,
            texture,
        );
    }

    pub fn current_sprite(&self) -> Sprite {
        let mut spr = match self {
            NoObject => todo!(),
            TreeObj(tree) => tree.sprite(),
            GrassObj(grass) => grass.sprite(),
        };

        if self.get_data().sprite_flip {
            spr.src_rect.width = -spr.src_rect.width - 0.1;
        }

        return spr;
    }

    pub fn take_hit(
        &mut self,
        damage: f32,
        game_context: &mut GameContext,
        facing_dir: FacingDirection,
    ) {
        let data = self.get_mut_data();
        data.health -= damage;
        let health = data.health;

        match data.state {
            GettingHit => {
                data.hit_timer.reset();
            }
            _ => {
                data.state = GettingHit;
            }
        }

        self.on_hit(game_context, facing_dir);

        if health <= 0.0 {
            self.get_mut_data().state = ObjectState::Breaking;
            self.get_mut_data().is_marked_for_gathering = false;
        }
    }

    /// describes a one time action that should be taken the moment something is hit
    fn on_hit(&mut self, game_context: &mut GameContext, facing_dir: FacingDirection) {
        match self {
            NoObject => (),
            TreeObj(tree) => tree.on_hit(&mut game_context.rng),
            GrassObj(grass) => grass.on_hit(game_context, facing_dir),
        }
    }

    /// describes a one time action that should be taken the moment something comes out of the hit state
    fn on_out_of_hit(&mut self) {
        match self {
            NoObject => (),
            TreeObj(tree) => tree.on_out_of_hit(),
            GrassObj(grass) => (),
        }
    }

    fn delete(&mut self, map_dimensions: MapDimensions, cells: &mut [MapCell]) {
        let cord = MapCord::from_vec2(self.get_data().pos);
        let idx = cords_to_index(map_dimensions, cord);

        let cell = get_cell_at_cord(cells, map_dimensions, cord).unwrap();
        cell.remove_obj(idx);
        *self = Self::NoObject
    }

    pub fn should_not_be_used_again_by_anything(&self) -> bool {
        let state = self.get_data().state;
        return state == ObjectState::Breaking;
    }

    #[inline]
    pub fn hover_rect(&self) -> Rectangle {
        return self.get_data().hover_rect();
    }
}
