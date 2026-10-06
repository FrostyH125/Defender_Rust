use raylib::{
    drawing::RaylibDrawHandle,
    math::{Rectangle, Vector2},
    texture::Texture2D,
};
use zander_game_core_rs::{raylib::sprite::Sprite, system::timer::Timer};

use crate::{
    GameContext,
    entities::{
        object::ObjectKind::*,
        objects::{grass::Grass, tree::Tree},
    },
    utils::{camera_utils, direction_utils::FacingDirection, draw_utils, map_cord::MapCord},
};

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
pub enum SimpleObjectKind {
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
    pub object_kind: SimpleObjectKind,
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

pub enum ObjectKind {
    TreeObj(Tree),
    GrassObj(Grass),
}

pub struct Object {
    pub object_data: ObjectData,
    pub object_kind: ObjectKind
}

impl Object {
    #[inline]
    pub fn update(&mut self, game_context: &mut GameContext, should_deselect: bool) {
        match &mut self.object_kind {
            TreeObj(tree) => tree.update(&mut self.object_data, game_context),
            GrassObj(grass) => grass.update(&mut self.object_data, game_context),
        }

        if should_deselect {
            self.object_data.is_selected = false;
        }

        match self.object_data.state {
            ObjectState::Idle => {
                self.object_data.is_hovering = false;
            }
            ObjectState::Breaking => {
                let disappear_timer = &mut self.object_data.object_specific_data.disappear_timer;

                disappear_timer.track(game_context.dt);

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
            self.object_data.draw_pos + self.object_data.object_specific_data.situational_draw_offset,
            texture,
        );
    }

    #[inline]
    pub fn draw_hover(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        let sprite = self.current_sprite();
        draw_utils::draw_outline(
            d,
            sprite,
            self.object_data.draw_pos + self.object_data.object_specific_data.situational_draw_offset,
            texture,
        );
    }

    #[inline]
    pub fn draw_selected(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        let sprite = self.current_sprite();
        draw_utils::draw_with_extra_brightness(
            d,
            sprite,
            self.object_data.draw_pos + self.object_data.object_specific_data.situational_draw_offset,
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

        draw_utils::draw_shadow(
            d,
            sprite,
            self.object_data.draw_pos + self.object_data.object_specific_data.situational_draw_offset,
            shadow_shear,
            shadow_scale,
            texture,
        );
    }

    pub fn current_sprite(&self) -> Sprite {
        let mut spr = match &self.object_kind {
            TreeObj(tree) => tree.sprite(&self.object_data),
            GrassObj(grass) => grass.sprite(),
        };

        if self.object_data.sprite_flip {
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
        self.object_data.object_specific_data.health -= damage;
        let health = self.object_data.object_specific_data.health;

        self.on_hit(game_context, facing_dir);

        if health <= 0.0 {
            self.object_data.state = ObjectState::Breaking;
            self.object_data.is_marked_for_gathering = false;
        }
    }

    /// describes a one time action that should be taken the moment something is hit
    fn on_hit(&mut self, game_context: &mut GameContext, facing_dir: FacingDirection) {
        match &mut self.object_kind {
            TreeObj(tree) => tree.on_hit(&mut self.object_data, &mut game_context.rng),
            GrassObj(grass) => grass.on_hit(&self.object_data, game_context, facing_dir),
        }
    }

    pub fn should_not_be_used_again_by_anything(&self) -> bool {
        let state = self.object_data.state;
        return state == ObjectState::Breaking;
    }

    #[inline]
    pub fn hover_rect(&self) -> Rectangle {
        return self.object_data.hover_rect();
    }

    /// sets object's data's field `is_hovering` to `true`
    #[inline]
    pub fn set_hovering(&mut self) {
        self.object_data.is_hovering = true;
    }

    #[inline]
    pub fn is_hovering(&self) -> bool {
        return self.object_data.is_hovering;
    }

    /// sets object's data's field `is_selected` to `true`
    #[inline]
    pub fn set_selected(&mut self) {
        self.object_data.is_selected = true;
    }

    #[inline]
    pub fn is_selected(&self) -> bool {
        return self.object_data.is_selected;
    }

    /// sets object's data's field `is_occupied` to `true`
    #[inline]
    pub fn set_occupied(&mut self) {
        self.object_data.is_occupied = true;
    }

    /// sets object's data's field `is_occupied` to `true`
    #[inline]
    pub fn set_unoccupied(&mut self) {
        self.object_data.is_occupied = false;
    }

    #[inline]
    pub fn is_occupied(&self) -> bool {
        return self.object_data.is_occupied;
    }

    /// sets object's data's field `is_marked_for_gathering` to `true`
    #[inline]
    pub fn mark_for_gathering(&mut self) {
        self.object_data.is_marked_for_gathering = true;
    }

    #[inline]
    pub fn unmark_for_gathering(&mut self) {
        self.object_data.is_marked_for_gathering = false;
    }
    #[inline]
    pub fn is_marked_for_gathering(&self) -> bool {
        return self.object_data.is_marked_for_gathering;
    }

    #[inline]
    pub fn get_center_pos(&self) -> Vector2 {
        let visual_rect = self.hover_rect();

        return Vector2::new(
            visual_rect.x + (visual_rect.width / 2.0),
            visual_rect.y + (visual_rect.height / 2.0),
        );
    }
}
