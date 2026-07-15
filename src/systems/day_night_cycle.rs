
// need to add moon phases

use raylib::{RaylibHandle, color::Color, drawing::{RaylibDraw, RaylibDrawHandle}, ffi::KeyboardKey};

pub struct DayNightCycle {
    pub current_time: f32,
    pub current_shadow_shear: f32,
    pub current_shadow_scale_y: f32
}

impl DayNightCycle {
    pub fn new() -> Self {
        return DayNightCycle { current_time: 0.0, current_shadow_shear: 0.0, current_shadow_scale_y: 0.0 }
    }    
    
    pub fn update(&mut self, dt: f32, rl: &mut RaylibHandle) {
        self.current_time += dt;

        if rl.is_key_down(KeyboardKey::KEY_Q) {
            self.current_shadow_shear -= dt;
        }

        if rl.is_key_down(KeyboardKey::KEY_W) {
            self.current_shadow_shear += dt
        }
        if rl.is_key_down(KeyboardKey::KEY_E) {
            self.current_shadow_scale_y -= dt;
        }

        if rl.is_key_down(KeyboardKey::KEY_R) {
            self.current_shadow_scale_y += dt
        }

        // set shadow variables
    }

    pub fn draw_dbg(&self, d: &mut RaylibDrawHandle) {
        d.draw_text(&format!("current time: {}", self.current_time), 5, 0, 30, Color::BLACK);
        d.draw_text(&format!("current shear: {}", self.current_shadow_shear), 5, 40, 30, Color::BLACK);
        d.draw_text(&format!("current scale_y: {}", self.current_shadow_scale_y), 5, 80, 30, Color::BLACK);
    }
}