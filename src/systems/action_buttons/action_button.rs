use basic_raylib_core::graphics::sprite::Sprite;
use raylib::{drawing::RaylibDrawHandle, math::Vector2, texture::Texture2D};

use crate::{entities::entity_manager::CharacterEntry, systems::action_buttons::chop_button::ChopButton};

pub enum ActionButton {
    ChopButton(ChopButton),
}

pub struct ActionButtonData {
    spawn_pos: Vector2,
    current_pos: Vector2,
    total_life_time: f32,
    sin_offset: f32,
}

impl ActionButtonData {
    pub fn new(pos: Vector2, sin_offset: f32) -> Self {
        return Self {
            spawn_pos: pos,
            current_pos: pos,
            total_life_time: 0.0,
            sin_offset,
        };
    }
}

impl ActionButton {
    pub fn get_mut_data(&mut self) -> &mut ActionButtonData {
        return match self {
            ActionButton::ChopButton(chop_button) => &mut chop_button.data,
        };
    }

    pub fn get_data(&self) -> &ActionButtonData {
        return match self {
            ActionButton::ChopButton(chop_button) => &chop_button.data,
        };
    }

    pub fn update(&mut self, dt: f32) {
        let data = self.get_mut_data();
        data.total_life_time += dt;
        
        data.current_pos.y = data.spawn_pos.y + (data.total_life_time + data.sin_offset).sin()
    }

    pub fn current_sprite(&self) -> &Sprite {
        match self {
            ActionButton::ChopButton(chop_button) => todo!(),
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        let spr = self.current_sprite();

        spr.draw(d, self.get_data().current_pos, texture);
    }

    pub fn on_click(&mut self, obj_ids: &[usize], chars: &[&mut CharacterEntry]) {
        match self {
            ActionButton::ChopButton(chop_button) => todo!(),
        }
    }
}
