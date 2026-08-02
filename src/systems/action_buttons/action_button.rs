use basic_raylib_core::graphics::sprite::Sprite;
use raylib::{drawing::RaylibDrawHandle, math::{Rectangle, Vector2}, texture::Texture2D};

use crate::{
    entities::entity_manager::CharacterEntry,
    systems::action_buttons::chop_button::{self, CHOP_BUTTON_SPRITE},
    utils::draw_utils,
};

pub enum ActionButtonKind {
    ChopButton,
}

pub struct ActionButton {
    kind: ActionButtonKind,
    sprite: Sprite,
    pub hover_rect: Rectangle,
    pub spawn_pos: Vector2,
    current_pos: Vector2,
    total_life_time: f32,
    pub sin_offset: f32,
}

impl ActionButton {
    pub fn new(kind: ActionButtonKind) -> Self {
        let sprite = match kind {
            ActionButtonKind::ChopButton => CHOP_BUTTON_SPRITE,
        };

        return Self {
            kind,
            sprite,
            spawn_pos: Vector2::default(),
            current_pos: Vector2::default(),
            hover_rect: Rectangle::new(0.0, 0.0, 16.0, 16.0),
            total_life_time: 0.0,
            sin_offset: 0.0,
        };
    }
}

impl ActionButton {
    pub fn update(&mut self, dt: f32) {
        self.total_life_time += dt;

        self.current_pos.y = self.spawn_pos.y + ((self.total_life_time + self.sin_offset) / 2.0).sin() * 2.0;
        self.current_pos.x = self.spawn_pos.x;
        self.hover_rect.x = self.current_pos.x;
        self.hover_rect.y = self.current_pos.y
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, texture: &Texture2D, hover: bool) {
        match hover {
            true => {
                draw_utils::draw_with_extra_brightness(d, &self.sprite, self.current_pos, texture)
            }
            false => self.sprite.draw(d, self.current_pos, texture),
        }
    }

    pub fn on_click(&mut self, obj_ids: &[usize], chars: &[&mut CharacterEntry]) {
        match self.kind {
            ActionButtonKind::ChopButton => chop_button::on_click(obj_ids, chars),
        }

        self.make_pop_particles();
    }

    pub fn make_spawn_particles(&self) {
        todo!()
    }
    pub fn make_pop_particles(&self) {
        todo!()
    }
}
