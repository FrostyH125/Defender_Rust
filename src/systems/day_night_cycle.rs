// need to add moon phases

use basic_raylib_core::utils::math_utils::{self, smooth_lerp_min_max};
use raylib::{
    RaylibHandle,
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
    ffi::KeyboardKey,
};

use crate::systems::day_night_cycle::MoonPhase::{FirstQuarter, NewMoon, WaxingCrescent};



enum MoonPhase {
    NewMoon,
    WaxingCrescent,
    FirstQuarter,
    WaxingGibbous,
    FullMoon,
    WaningGibbous,
    LastQuarter,
    WaningCrescent,
}

struct NightDetails {
    shadow_shear_x: f32,
    shadow_scale_y: f32,
    brightness: f32,
    moon_phase: MoonPhase,
}

// pub static NIGHTS: [NightDetails; 8] = [
//     NightDetails {
//         shadow_shear_x: 0.0,
//         shadow_scale_y: 0.0,
//         brightness: -0.4,
//         moon_phase: NewMoon,
//     },
//     NightDetails {
//         shadow_shear_x: 2.0,
//         shadow_scale_y: 0.5,
//         brightness: -0.3,
//         moon_phase: WaxingCrescent,
//     },
//     NightDetails {
//         shadow_shear_x: 4.0,
//         shadow_scale_y: 0.8,
//         brightness: -0.25,
//         moon_phase: FirstQuarter,
//     },
//     NightDetails {
//         shadow_shear_x: todo!(),
//         shadow_scale_y: todo!(),
//         brightness: todo!(),
//         moon_phase: todo!(),
//     },
//     NightDetails {
//         shadow_shear_x: todo!(),
//         shadow_scale_y: todo!(),
//         brightness: todo!(),
//         moon_phase: todo!(),
//     },
//     NightDetails {
//         shadow_shear_x: todo!(),
//         shadow_scale_y: todo!(),
//         brightness: todo!(),
//         moon_phase: todo!(),
//     },
//     NightDetails {
//         shadow_shear_x: todo!(),
//         shadow_scale_y: todo!(),
//         brightness: todo!(),
//         moon_phase: todo!(),
//     },
//     NightDetails {
//         shadow_shear_x: todo!(),
//         shadow_scale_y: todo!(),
//         brightness: todo!(),
//         moon_phase: todo!(),
//     },
// ];

pub struct DayNightCycle {
    pub current_time: f32,
    pub current_shadow_shear: f32,
    pub current_shadow_scale_y: f32,
    pub red_tint: f32,
    pub blue_tint: f32,
    pub brightness_modifier: f32,
}

impl DayNightCycle {
    pub fn new() -> Self {
        return DayNightCycle {
            current_time: 0.0,
            current_shadow_shear: 0.0,
            current_shadow_scale_y: 0.0,
            red_tint: 0.0,
            blue_tint: 0.0,
            brightness_modifier: 0.0,
        };
    }

    pub fn update(&mut self, dt: f32, rl: &mut RaylibHandle) {
        self.current_time += dt * 4.0;
        if self.current_time > 360.0 {
            self.current_time -= 360.0;
        }

        self.update_shadow_values();
        self.update_sky_colors();
    }

    pub fn draw_dbg(&self, d: &mut RaylibDrawHandle) {
        d.draw_text(
            &format!("current time: {}", self.current_time),
            5,
            0,
            30,
            Color::BLACK,
        );
        d.draw_text(
            &format!("current shear: {}", self.current_shadow_shear),
            5,
            40,
            30,
            Color::BLACK,
        );
        d.draw_text(
            &format!("current scale_y: {}", self.current_shadow_scale_y),
            5,
            80,
            30,
            Color::BLACK,
        );
    }

    fn update_shadow_values(&mut self) {
        const MAX_SHEAR: f32 = -10.0;
        const MIN_SCALE_Y: f32 = 0.7;
        const MAX_SCALE_Y: f32 = 0.3;

        let (shear, scale) = match self.current_time {
            0.0..=90.0 => (
                smooth_lerp_min_max(-MAX_SHEAR, 0.0, self.current_time, 0.0, 90.0),
                smooth_lerp_min_max(MIN_SCALE_Y, MAX_SCALE_Y, self.current_time, 0.0, 90.0),
            ),
            90.0..=180.0 => (
                smooth_lerp_min_max(0.0, MAX_SHEAR, self.current_time, 90.0, 180.0),
                smooth_lerp_min_max(MAX_SCALE_Y, MIN_SCALE_Y, self.current_time, 90.0, 180.0),
            ),
            180.0..=360.0 => (0.0, 0.0),
            _ => (0.0, 0.0),
        };

        self.current_shadow_scale_y = scale;
        self.current_shadow_shear = shear;
    }

    fn update_sky_colors(&mut self) {
        // these will be replaced eventually with current_night.red/blue/darkness
        const MAX_BLUE_DAYTIME: f32 = 0.3;
        const MAX_RED_DAYTIME: f32 = 0.2;
        const MIN_BRIGHTNESS_DAYTIME: f32 = -0.2;

        let (blue, red, light) = match self.current_time {
            0.0..=30.0 => (
                smooth_lerp_min_max(MAX_BLUE_DAYTIME, 0.0, self.current_time, 0.0, 30.0),
                smooth_lerp_min_max(MAX_RED_DAYTIME, 0.0, self.current_time, 0.0, 30.0),
                smooth_lerp_min_max(MIN_BRIGHTNESS_DAYTIME, 0.0, self.current_time, 0.0, 30.0),
            ),
            30.0..=150.0 => (0.0, 0.0, 0.0),
            150.0..=180.0 => (
                smooth_lerp_min_max(0.0, MAX_BLUE_DAYTIME, self.current_time, 150.0, 180.0),
                smooth_lerp_min_max(0.0, MAX_RED_DAYTIME, self.current_time, 150.0, 180.0),
                smooth_lerp_min_max(0.0, MIN_BRIGHTNESS_DAYTIME, self.current_time, 150.0, 180.0),
            ),
            180.0..=360.0 => (0.0, 0.0, 0.0),
            _ => (0.0, 0.0, 0.0),
        };

        self.blue_tint = blue;
        self.red_tint = red;
        self.brightness_modifier = light;
    }
}
