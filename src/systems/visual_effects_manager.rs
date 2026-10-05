use raylib::{
    camera::Camera2D, color::Color, drawing::{RaylibDraw, RaylibDrawHandle}, math::Vector2, text::{Font, RaylibFont}, texture::Texture2D,
};
use zander_game_core_rs::{raylib::sprite::Sprite, system::timer::Timer};

pub enum VisualEffectKind {
    DamageNumber { digit_sprites: Vec<Sprite> },
}

pub struct VisualEffect {
    kind: VisualEffectKind,
    timer: Timer,
    pos: Vector2,
}

impl VisualEffect {
    pub fn update(&mut self, dt: f32) {
        match self.kind {
            VisualEffectKind::DamageNumber{..} => {
                self.pos.y -= 10.0 * dt;
            }
        }
    }
}

pub struct VisualEffectsManager {
    effects: Vec<VisualEffect>,
}

impl VisualEffectsManager {
    pub fn new() -> Self {
        return Self {
            effects: Vec::with_capacity(50),
        };
    }

    pub fn add_damage_number(&mut self, mut pos: Vector2, number: f32) {
        let num_str = number.to_string();
        const NUM_SPRITES_START_X: i32 = 160;
        const NUM_SPRITES_Y: i32 = 0;
        const NUM_SPRITE_WIDTH: i32 = 4;
        const NUM_SPRITE_HEIGHT: i32 = 5;

        let mut digit_sprites = Vec::new();

        // turn the string into a list of sprites
        for digit in num_str.chars() {
            let spr_x = NUM_SPRITES_START_X + (NUM_SPRITE_WIDTH * digit.to_digit(10).unwrap() as i32);
            digit_sprites.push(Sprite::new(spr_x, NUM_SPRITES_Y, NUM_SPRITE_WIDTH, NUM_SPRITE_HEIGHT));
        }
        
        let timer_len = 2.0;
        let half_size = (digit_sprites.len() as i32 * NUM_SPRITE_WIDTH / 2) as f32;
        pos.x -= half_size;

        let kind = VisualEffectKind::DamageNumber { digit_sprites };

        let effect = VisualEffect {
            kind,
            timer: Timer::new(timer_len),
            pos,
        };

        self.effects.push(effect);
    }

    pub fn update_effects(&mut self, dt: f32) {
        for i in (0..self.effects.len()).rev() {
            let effect = &mut self.effects[i];

            effect.timer.track(dt);

            if effect.timer.is_done() {
                self.effects.swap_remove(i);
                continue;
            }

            effect.update(dt);
        }
    }

    pub fn draw_effects(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        for effect in &self.effects {
            match &effect.kind {
                VisualEffectKind::DamageNumber { digit_sprites } => {
                    let mut x_pos = effect.pos.x;
                    
                    for sprite in digit_sprites {
                        sprite.draw(d, Vector2::new(x_pos, effect.pos.y), texture);
                        x_pos += sprite.src_rect.width.ceil();
                    }
                }
            }
        }
    }
}
