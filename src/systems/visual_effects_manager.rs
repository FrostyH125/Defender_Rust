use rand::{RngExt, rngs::ThreadRng};
use raylib::{
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
    math::Vector2,
    text::{Font, RaylibFont},
};
use zander_game_core_rs::system::timer::Timer;

use crate::{GameContext, utils::{camera_utils::world_to_screen_ui, vector2_utils::{floor, random_offset_in_range}}};

pub enum VisualEffectKind {
    DamageNumber {
        offset: Vector2,
        num_str: String,
        font_size: f32,
        rotation: f32
    },
}

pub struct VisualEffect {
    kind: VisualEffectKind,
    timer: Timer,
    pos: Vector2,
}

impl VisualEffect {
    pub fn update(&mut self, dt: f32) {
        match self.kind {
            VisualEffectKind::DamageNumber { .. } => {
                let inverse_progress = 1.0 - self.timer.progress();
                let speed = 20.0;

                self.pos.y -= (speed * inverse_progress) * dt;
            }
        }
    }
}

pub struct VisualEffectsManager {
    world_space_effects: Vec<VisualEffect>,
    ui_space_effects: Vec<VisualEffect>,
}

impl VisualEffectsManager {
    pub fn new() -> Self {
        return Self {
            world_space_effects: Vec::with_capacity(50),
            ui_space_effects: Vec::with_capacity(50),
        };
    }

    pub fn add_damage_number(&mut self, pos: Vector2, number: f32, font: &Font, rng: &mut ThreadRng) {
        let font_size = 40.0;

        let num_str = number.to_string();
        let string_size = font.measure_text(&num_str, font_size, 0.0);

        let offset = string_size / 2.0;
        let timer_len = 2.0;
        self.ui_space_effects.push(VisualEffect {
            kind: VisualEffectKind::DamageNumber {
                offset,
                num_str,
                font_size,
                rotation: rng.random_range(-15.0..=15.0)
            },
            timer: Timer::new(timer_len),
            pos: floor(pos)  + random_offset_in_range(rng, 2.0),
        });
    }

    pub fn update_effects(&mut self, dt: f32) {
        for i in (0..self.world_space_effects.len()).rev() {
            let effect = &mut self.world_space_effects[i];

            effect.timer.track(dt);

            if effect.timer.is_done() {
                self.world_space_effects.swap_remove(i);
                continue;
            }

            effect.update(dt);
        }

        for i in (0..self.ui_space_effects.len()).rev() {
            let effect = &mut self.ui_space_effects[i];

            effect.timer.track(dt);

            if effect.timer.is_done() {
                self.ui_space_effects.swap_remove(i);
                continue;
            }

            effect.update(dt);
        }
    }

    pub fn draw_ui_effects(
        &self,
        d: &mut RaylibDrawHandle,
        game_context: &GameContext,
        ui_rt_width: i32,
    ) {
        for effect in &self.ui_space_effects {
            match &effect.kind {
                VisualEffectKind::DamageNumber {
                    offset,
                    num_str,
                    font_size,
                    rotation
                } => {
                    let screen_pos = world_to_screen_ui(
                        effect.pos,
                        &game_context.camera,
                        game_context.v_width,
                        ui_rt_width,
                    );

                    d.draw_text_pro(
                        &game_context.damage_number_font,
                        num_str,
                        screen_pos,
                        offset,
                        *rotation,
                        *font_size,
                        0.0,
                        Color::WHITE,
                    );
                }
            }
        }
    }
}
