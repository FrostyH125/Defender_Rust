use basic_raylib_core::{graphics::sprite::Sprite, system::input_handler::InputState};
use rand::rngs::ThreadRng;
use raylib::{
    RaylibHandle, RaylibThread,
    camera::Camera2D,
    color::Color,
    drawing::{RaylibDraw, RaylibMode2DExt, RaylibShaderModeExt, RaylibTextureModeExt},
    ffi::KeyboardKey,
    math::{Rectangle, Vector2},
    shaders::RaylibShader,
    texture::{RenderTexture2D, Texture2D},
};

use crate::{
    ZoomSizes::{FiveX, FourX, SixX, ThreeX, TwoX}, entities::{characters::gatherer::Gatherer, entity_manager::EntityManager}, map::tile_map::TileMap, systems::day_night_cycle::DayNightCycle, utils::{
        map_cord::MapCord, mouse_utils, pathfinding::{self, PathFinder, PathResult},
    },
};

pub mod entities;
pub mod map;
pub mod systems;
pub mod utils;

static PATH_SPRITE: Sprite = Sprite::new(96, 136, 8, 8);

// any of these can be done in any order:
//      add new tree variants
//      pre-requisites for first gatherer-tree interaction implementation:
//          character::move_to(pos) -> requires a_star()
//          cell system for map cells
//          action buttons
//          character_object selecting mode switching
//          character hover
//          character select list
//          object select list

// unrelated:
//  when a in/outlet is being drawn, draw the corners on that tile still if there exists some

pub const TILE_SIZE: f32 = 8.0;

pub struct GameContext {
    total_game_time: f32,
    logical_window_width: u32,
    logical_window_height: u32,
    v_width: u32,
    v_height: u32,
    camera: Camera2D,
    day_night_cycle: DayNightCycle,
    input_state: InputState,
    rng: ThreadRng,
    texture: Texture2D,
    path_finder: PathFinder,
}

fn main() {
    // what the game is pretending the game is running at,
    // to be honest, ideally this will never change unless aspect ratio changes
    let mut window_width_target = 1920;
    let mut window_height_target = 1080;

    // what the game will stretch the render target to, to fill the screen
    let actual_window_width = 2560;
    let actual_window_height = 1440;

    let mut current_zoom = ZoomSizes::FiveX;
    let v_width = current_zoom.v_width(window_width_target);
    let v_height = current_zoom.v_height(window_height_target);

    let mut camera = Camera2D {
        offset: Vector2 {
            x: v_width as f32 / 2.0,
            y: v_height as f32 / 2.0,
        },
        target: Vector2 {
            x: v_width as f32 / 2.0,
            y: v_height as f32 / 2.0,
        },
        rotation: 0.0,
        zoom: 1.0,
    };
    let mut rng = rand::rng();

    let mut camera_pos = camera.target;
    let mut input_state = InputState::new();

    let mut map = TileMap::generate_map(500, 500, &mut rng);
    let mut entity_manager = EntityManager::new(map.map_dimensions);

    let mut day_night_cycle = DayNightCycle::new();

    let (mut rl, thread) = raylib::init()
        .size(actual_window_width as i32, actual_window_height as i32)
        .title("Rust Raylib Starter")
        .build();

    let path_finder = PathFinder::new();

    let texture = rl.load_texture(&thread, "Tileset.png").unwrap();
    let mut shader = rl.load_shader(&thread, None, Some("base_shader.frag"));
    let red_tint_loc = shader.get_shader_location("red_tint");
    let blue_tint_loc = shader.get_shader_location("blue_tint");
    let brightness_modifier_loc = shader.get_shader_location("brightness_modifier");

    let mut render_textures: [RenderTexture2D; 5] = [
        rl.load_render_texture(
            &thread,
            window_width_target as u32 / 2,
            window_height_target as u32 / 2,
        )
        .unwrap(),
        rl.load_render_texture(
            &thread,
            window_width_target as u32 / 3,
            window_height_target as u32 / 3,
        )
        .unwrap(),
        rl.load_render_texture(
            &thread,
            window_width_target as u32 / 4,
            window_height_target as u32 / 4,
        )
        .unwrap(),
        rl.load_render_texture(
            &thread,
            window_width_target as u32 / 5,
            window_height_target as u32 / 5,
        )
        .unwrap(),
        rl.load_render_texture(
            &thread,
            window_width_target as u32 / 6,
            window_height_target as u32 / 6,
        )
        .unwrap(),
    ];

    rl.set_target_fps(60);
    rl.disable_cursor();

    let mut game_context = GameContext {
        total_game_time: 0.0,
        logical_window_width: window_width_target,
        logical_window_height: window_height_target,
        v_width,
        v_height,
        camera,
        day_night_cycle,
        input_state,
        rng,
        texture,
        path_finder,
    };
    //
    // DEBUG START
    //

    entity_manager.add_character(Gatherer::new(Vector2::new(100.0, 100.0)));

    //
    // DEBUG END
    //

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();

        game_context.total_game_time += dt;

        // update input first
        game_context.input_state.update(&mut rl, camera.zoom);

        if game_context.input_state.middle_roll.abs() >= 1.0 {
            let up = game_context.input_state.middle_roll < 0.0;
            current_zoom = current_zoom.change_res(up);
            game_context.v_width = current_zoom.v_width(game_context.logical_window_width);
            game_context.v_height = current_zoom.v_height(game_context.logical_window_height);
        }

        if rl.is_key_pressed(KeyboardKey::KEY_Z) {
            current_zoom = current_zoom.change_res(false);
        } else if rl.is_key_pressed(KeyboardKey::KEY_X) {
            current_zoom = current_zoom.change_res(true);
        }

        if rl.is_key_down(KeyboardKey::KEY_D) {
            camera_pos.x += game_context.v_width as f32 * dt;
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            camera_pos.x -= game_context.v_width as f32 * dt;
        }
        if rl.is_key_down(KeyboardKey::KEY_W) {
            camera_pos.y -= game_context.v_width as f32 * dt;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            camera_pos.y += game_context.v_width as f32 * dt;
        }

        if game_context.input_state.middle_currently_held {
            camera_pos.x -= game_context.input_state.delta.x
                / (window_width_target as f32 / game_context.v_width as f32);
            camera_pos.y -= game_context.input_state.delta.y
                / (window_height_target as f32 / game_context.v_height as f32);
        }

        // keep cam offset synced no MATTER WHAT THIS WAS PISSING ME OFF FOR A WHILE
        game_context.camera.offset.x = current_zoom.v_width(window_width_target) as f32 / 2.0;
        game_context.camera.offset.y = current_zoom.v_height(window_height_target) as f32 / 2.0;

        // remove any floating points from camera pos

        game_context.camera.target.x = camera_pos.x.round();
        game_context.camera.target.y = camera_pos.y.round();

        //--UPDATE BEGINS HERE--//
        map.update(dt);
        entity_manager.update(
            &mut map.map_object_grid,
            &mut game_context,
            current_zoom.zoom(),
            dt,
        );

        game_context.day_night_cycle.update(dt, &mut rl);

        shader.set_shader_value(red_tint_loc, game_context.day_night_cycle.red_tint);
        shader.set_shader_value(blue_tint_loc, game_context.day_night_cycle.blue_tint);
        shader.set_shader_value(
            brightness_modifier_loc,
            game_context.day_night_cycle.brightness_modifier,
        );

        let current_rt = &mut render_textures[current_zoom as usize];
        //--UPDATE ENDS HERE--//

        //--DRAWING BEINGS HERE--//
        {
            let mut d = rl.begin_drawing(&thread);
            {
                let mut render_texture_handle = d.begin_texture_mode(&thread, current_rt);
                render_texture_handle.clear_background(Color::RAYWHITE);
                {
                    let mut cam_handle = render_texture_handle.begin_mode2D(game_context.camera);
                    {
                        let mut shader_handle = cam_handle.begin_shader_mode(&mut shader);

                        map.draw(&mut shader_handle, &game_context);

                        entity_manager.draw(
                            &map.map_object_grid,
                            &mut shader_handle,
                            &game_context.texture,
                        );
                        mouse_utils::draw_mouse(
                            &mut shader_handle,
                            mouse_utils::mouse_world_coords(&game_context),
                            &game_context.texture,
                        );

                        let mouse_cord = mouse_utils::mouse_world_coords(&game_context);

                        if let PathResult::Success { path } = game_context
                            .path_finder
                            .a_star(
                                MapCord::new(0, 0),
                                MapCord::new(mouse_cord.x as i16 / 8, mouse_cord.y as i16 / 8),
                                &map.map_tile_grid,
                                10_000,
                                map.map_dimensions,
                            )
                        {
                            for p in path {
                                PATH_SPRITE.draw(&mut shader_handle, Vector2::new(p.x as f32 * 8.0, p.y as f32 * 8.0), &game_context.texture);
                            }
                        }
                    } // end shader mode - nothing drawn will pass through shader beyond here
                } // end camera mode - nothing drawn will be drawn in world space beyond here
            } // end rt mode - nothing drawn will be drawn on the render texture beyond here

            let source_rec = Rectangle::new(
                0.0,
                0.0,
                current_rt.texture.width as f32,
                -current_rt.texture.height as f32, // Negative height flips it right-side up
            );

            let dest_rec = Rectangle::new(
                0.0,
                0.0,
                actual_window_width as f32,
                actual_window_height as f32,
            );
            let origin = Vector2::new(0.0, 0.0);

            d.draw_texture_pro(current_rt, source_rec, dest_rec, origin, 0.0, Color::WHITE);
            game_context.day_night_cycle.draw_dbg(&mut d);
        }
        //--DRAWING ENDS HERE--//
    }
}

#[repr(usize)]
#[derive(Clone, Copy)]
enum ZoomSizes {
    TwoX,
    ThreeX,
    FourX,
    FiveX,
    SixX,
}

impl ZoomSizes {
    pub fn change_res(self, up: bool) -> Self {
        let current_index = self as usize;

        let add: isize = match up {
            true => -1,
            false => 1,
        };

        let mut idx = (current_index as isize + add) as usize;

        // if you go lower than 0, usize wraps back around, and would wrap back to the other side without this
        if idx > usize::MAX - 1 {
            idx = 0
        }

        return Self::get_zoom_from_index(idx);
    }

    pub fn get_zoom_from_index(idx: usize) -> Self {
        let comp = idx.clamp(0, 5);

        match comp {
            0 => TwoX,
            1 => ThreeX,
            2 => FourX,
            3 => FiveX,
            4 => SixX,
            5.. => SixX,
        }
    }

    pub fn v_width(self, screen_width: u32) -> u32 {
        return screen_width / (self as u32 + 2);
    }

    pub fn v_height(self, screen_height: u32) -> u32 {
        return screen_height / (self as u32 + 2);
    }

    pub fn zoom(self) -> u32 {
        let zoom = self as u32 + 2;
        return zoom;
    }
}

fn change_window_size(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    rt_array: &mut [RenderTexture2D],
    window_width: &mut f32,
    window_height: &mut f32,
    new_width: f32,
    new_height: f32,
) {
    *window_width = new_width;
    *window_height = new_height;
    rl.set_window_size(*window_width as i32, *window_height as i32);

    set_render_textures(rl, thread, rt_array, *window_width, *window_height);
}

fn set_render_textures(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    rt_array: &mut [RenderTexture2D],
    window_width: f32,
    window_height: f32,
) {
    let w_u32 = window_width as u32;
    let h_u32 = window_height as u32;

    let rt_count = rt_array.len();

    for i in 0..rt_count {
        rt_array[i] = rl
            .load_render_texture(thread, w_u32 / (i as u32 + 2), h_u32 / (i as u32 + 2))
            .unwrap();
    }
}
