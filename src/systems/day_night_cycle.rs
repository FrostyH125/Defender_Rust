// need to add moon phases

use basic_raylib_core::utils::math_utils::{self, smooth_lerp_min_max};
use raylib::{
    RaylibHandle,
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
    ffi::KeyboardKey,
};

use crate::systems::day_night_cycle::MoonPhase::{FirstQuarter, FullMoon, LastQuarter, NewMoon, WaningCrescent, WaningGibbous, WaxingCrescent, WaxingGibbous};

const CRESCENT_GIBBOUS_SHEAR_X: f32 = 2.0;
const CRESCENT_GIBBOUS_SCALE_Y: f32 = 0.5;
const FULL_NEW_SHEAR_X: f32 = 0.0;
const FULL_NEW_SCALE_Y: f32 = 1.0;
const QUARTER_SHEAR_X: f32 = 8.0;
const QUARTER_SCALE_Y: f32 = 0.3;
const NEW_BRIGHTNESS_MODIFIER: f32 = -0.45;
const CRESCENT_BRIGHTNESS_MODIFIER: f32 = -0.4;
const QUARTER_BRIGHTNESS_MODIFIER: f32 = -0.35;
const GIBBOUS_BRIGHTNESS_MODIFIER: f32 = -0.3;
const FULL_BRIGHTNESS_MODIFIER: f32 = -0.25;

#[derive(Debug)]
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
    brightness_modifier: f32,
    moon_phase: MoonPhase,
}

static NIGHTS: [NightDetails; 8] = [
    NightDetails {
        shadow_shear_x: FULL_NEW_SHEAR_X,
        shadow_scale_y: FULL_NEW_SCALE_Y,
        brightness_modifier: NEW_BRIGHTNESS_MODIFIER,
        moon_phase: NewMoon,
    },
    NightDetails {
        shadow_shear_x: -CRESCENT_GIBBOUS_SHEAR_X,
        shadow_scale_y: CRESCENT_GIBBOUS_SCALE_Y,
        brightness_modifier: CRESCENT_BRIGHTNESS_MODIFIER,
        moon_phase: WaxingCrescent,
    },
    NightDetails {
        shadow_shear_x: -QUARTER_SHEAR_X,
        shadow_scale_y: QUARTER_SCALE_Y,
        brightness_modifier: QUARTER_BRIGHTNESS_MODIFIER,
        moon_phase: FirstQuarter,
    },
    NightDetails {
        shadow_shear_x: -CRESCENT_GIBBOUS_SHEAR_X,
        shadow_scale_y: -CRESCENT_GIBBOUS_SCALE_Y,
        brightness_modifier: GIBBOUS_BRIGHTNESS_MODIFIER,
        moon_phase: WaxingGibbous,
    },
    NightDetails {
        shadow_shear_x: FULL_NEW_SHEAR_X,
        shadow_scale_y: -FULL_NEW_SCALE_Y,
        brightness_modifier: FULL_BRIGHTNESS_MODIFIER,
        moon_phase: FullMoon,
    },
    NightDetails {
        shadow_shear_x: CRESCENT_GIBBOUS_SHEAR_X,
        shadow_scale_y: -CRESCENT_GIBBOUS_SCALE_Y,
        brightness_modifier: GIBBOUS_BRIGHTNESS_MODIFIER,
        moon_phase: WaningGibbous,
    },
    NightDetails {
        shadow_shear_x: QUARTER_SHEAR_X,
        shadow_scale_y: -QUARTER_SCALE_Y,
        brightness_modifier: QUARTER_BRIGHTNESS_MODIFIER,
        moon_phase: LastQuarter,
    },
    NightDetails {
        shadow_shear_x: CRESCENT_GIBBOUS_SHEAR_X,
        shadow_scale_y: CRESCENT_GIBBOUS_SCALE_Y,
        brightness_modifier: CRESCENT_BRIGHTNESS_MODIFIER,
        moon_phase: WaningCrescent,
    },
];

pub struct DayNightCycle {
    pub current_night: usize,
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
            current_night: 0,
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

        if rl.is_key_pressed(KeyboardKey::KEY_C) {
            self.current_night += 1;
            self.current_night %= 8;
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
        d.draw_text(
            &format!("current night: {:?}", NIGHTS[self.current_night].moon_phase),
            5,
            120,
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
            180.0..=360.0 => {
                let current_night = &NIGHTS[self.current_night];

                (current_night.shadow_shear_x, current_night.shadow_scale_y)
            },
            _ => (0.0, 0.0),
        };

        self.current_shadow_scale_y = scale;
        self.current_shadow_shear = shear;
    }

    fn update_sky_colors(&mut self) {
        // these will be replaced eventually with current_night.red/blue/darkness
        const MAX_BLUE: f32 = 0.3;
        const MAX_RED: f32 = 0.2;
        let current_night_brightness_modifier = NIGHTS[self.current_night].brightness_modifier;

        

        let (blue, red, light) = match self.current_time {
            0.0..=30.0 => (
                smooth_lerp_min_max(MAX_BLUE, 0.0, self.current_time, 0.0, 30.0),
                smooth_lerp_min_max(MAX_RED, 0.0, self.current_time, 0.0, 30.0),
                smooth_lerp_min_max(current_night_brightness_modifier, 0.0, self.current_time, 0.0, 30.0),
            ),
            30.0..=150.0 => (0.0, 0.0, 0.0),
            150.0..=180.0 => (
                smooth_lerp_min_max(0.0, MAX_BLUE, self.current_time, 150.0, 180.0),
                smooth_lerp_min_max(0.0, MAX_RED, self.current_time, 150.0, 180.0),
                smooth_lerp_min_max(0.0, current_night_brightness_modifier, self.current_time, 150.0, 180.0),
            ),
            180.0..=360.0 => {
                let current_night = &NIGHTS[self.current_night];

                (MAX_BLUE, MAX_RED, current_night.brightness_modifier)
            },
            _ => (0.0, 0.0, 0.0),
        };

        self.blue_tint = blue;
        self.red_tint = red;
        self.brightness_modifier = light;
    }
}
