use raylib::{
    drawing::RaylibDrawHandle,
    math::{Rectangle, Vector2},
    texture::Texture2D,
};
use zander_game_core_rs::{raylib::sprite::Sprite, system::timer::Timer};

use crate::{
    GameContext,
    entities::{
        object::Object::*,
        objects::{grass::Grass, tree::Tree},
    },
    utils::{camera_utils, direction_utils::FacingDirection, draw_utils, map_cord::MapCord},
};

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
pub enum ObjectKind {
    Tree,
    Grass,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum ObjectState {
    Idle,
    Breaking,
}

/// This houses data that all objects share, as to not repeat fields between objects
pub struct ObjectData {
    pub object_specific_data: ObjectSpecificData,
    pub pos: Vector2,
    pub draw_pos: Vector2,
    pub cord: MapCord,
    pub sprite_flip: bool,
    pub state: ObjectState,
    is_marked_for_gathering: bool,
    is_occupied: bool,
    is_hovering: bool,
    is_selected: bool,
}

/// this is strictly for data that is solely dependent on the kind of object it is, not just stuff that every object has
/// for example, pos doesnt count, because pos is not dependent on the kind of object, but something like the health would be
/// because different objects will likely start with different amounts of health
pub struct ObjectSpecificData {
    pub situational_draw_offset: Vector2,
    pub draw_offset: Vector2,
    pub width: f32,
    pub height: f32,
    pub disappear_timer: Timer,
    pub health: f32,
    pub object_kind: ObjectKind,
}

impl ObjectData {
    pub fn new(
        pos: Vector2,
        randomized_offset: Vector2,
        cord: MapCord,
        object_specific_data: ObjectSpecificData,
    ) -> Self {
        let true_logical_pos = pos + randomized_offset;
        let final_draw_pos = true_logical_pos + object_specific_data.draw_offset;

        return ObjectData {
            object_specific_data,
            pos: true_logical_pos,
            draw_pos: final_draw_pos,
            cord,
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
        return Rectangle::new(
            self.draw_pos.x,
            self.draw_pos.y,
            self.object_specific_data.width,
            self.object_specific_data.height,
        );
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

    #[inline]
    pub fn update(&mut self, game_context: &mut GameContext, should_deselect: bool) {
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
                    self.delete();
                    return;
                }

                let disappear_timer = &mut self.get_mut_data().object_specific_data.disappear_timer;

                disappear_timer.track(game_context.dt);
                if disappear_timer.is_done() {
                    self.delete();
                    return;
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
            self.get_data().draw_pos + self.get_data().object_specific_data.situational_draw_offset,
            texture,
        );
    }

    #[inline]
    pub fn draw_hover(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        let sprite = self.current_sprite();
        draw_utils::draw_outline(
            d,
            sprite,
            self.get_data().draw_pos + self.get_data().object_specific_data.situational_draw_offset,
            texture,
        );
    }

    #[inline]
    pub fn draw_selected(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        let sprite = self.current_sprite();
        draw_utils::draw_with_extra_brightness(
            d,
            sprite,
            self.get_data().draw_pos + self.get_data().object_specific_data.situational_draw_offset,
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
            data.draw_pos + data.object_specific_data.situational_draw_offset,
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
        data.object_specific_data.health -= damage;
        let health = data.object_specific_data.health;

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

    fn delete(&mut self) {
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

    #[inline]
    /// sets object's data's field `is_hovering` to `true`
    pub fn set_hovering(&mut self) {
        self.get_mut_data().is_hovering = true;
    }

    #[inline]
    pub fn is_hovering(&self) -> bool {
        return self.get_data().is_hovering;
    }

    #[inline]
    /// sets object's data's field `is_selected` to `true`
    pub fn set_selected(&mut self) {
        self.get_mut_data().is_selected = true;
    }

    #[inline]
    pub fn is_selected(&self) -> bool {
        return self.get_data().is_selected;
    }

    #[inline]
    /// sets object's data's field `is_occupied` to `true`
    pub fn set_occupied(&mut self) {
        self.get_mut_data().is_occupied = true;
    }

    #[inline]
    /// sets object's data's field `is_occupied` to `true`
    pub fn set_unoccupied(&mut self) {
        self.get_mut_data().is_occupied = false;
    }

    #[inline]
    pub fn is_occupied(&self) -> bool {
        return self.get_data().is_occupied;
    }

    /// sets object's data's field `is_marked_for_gathering` to `true`
    pub fn mark_for_gathering(&mut self) {
        self.get_mut_data().is_marked_for_gathering = true;
    }

    pub fn unmark_for_gathering(&mut self) {
        self.get_mut_data().is_marked_for_gathering = false;
    }

    pub fn is_marked_for_gathering(&self) -> bool {
        return self.get_data().is_marked_for_gathering;
    }
}
