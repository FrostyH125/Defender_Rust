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
use zander_game_core_rs::{
    raylib::sprite::Sprite,
    system::{input_handler::InputState, sprite_particle_system::SpriteParticleSystem},
};

use crate::{
    ZoomSizes::{FiveX, FourX, SixX, ThreeX, TwoX},
    entities::{characters::gatherer::Gatherer, entity_manager::EntityManager},
    map::tile_map::TileMap,
    systems::{
        action_button_manager::ActionButtonManager,
        character_action_manager::CharacterActionManager, day_night_cycle::DayNightCycle,
        entity_selecting_manager::EntitySelectingManager, select_rect::SelectRect,
    },
    utils::{
        direction_utils::ORTHOGONAL_DELTAS,
        mouse_utils::{self, mouse_world_coords},
        pathfinding::PathFinder,
    },
};

pub mod entities;
pub mod map;
pub mod systems;
pub mod utils;

// any of these can be done in any order:
//      grass visual upon disappearing, maybe extra particles or something, maybe just draw the anim with a shear making it fall down, maybe both
//      make it so that the selectors never select or even hover enemies for movement
//      draw grass with a shear when its hit
//      add a type alias for usize : CharID
//      add a vfx manager for adding things like slashes in a fire and forget kind of way
//      make shadows not appear so suddenly at night, even if they ease into the position quickly, would still be better
//      add clouds
//      gather all button
//      wobble shader effect on the action buttons (will later be used on building buttons too)
//      cool shader for background instead of no tiles -> use that one steam tool it was sick
//      ALL the sounds from the github repo
//      fighter struct sprint
//          -- enemy: enum { EnemyKindOne, EnemyKindTwo, etc }
//          -- fighter: enum { FighterKindOne, FighterKindTwo, etc..}
//              -- Affiliation enum { Good, Bad }
//          -- fighter_data: struct
//          -- FighterState::Idle
//          -- FighterState::LookingForEnemy
//          -- FighterState::MovingToEnemy
//          -- FighterState::InCombat
//          After all of this is added, i need to test the new combat system in place.
//          have a fighter request engaging in combat with an enemy after moving toward it
//          have them hit eachother (draw hp over each one to see),
//          then print the state of the fighter, and see if it switches back to none
//          print as well the count of its opponent list

pub const TILE_SIZE: f32 = 8.0;

pub struct GameContext {
    total_game_time: f32,
    logical_window_width: u32,
    logical_window_height: u32,
    v_width: u32,
    v_height: u32,
    dt: f32,
    camera: Camera2D,
    day_night_cycle: DayNightCycle,
    input_state: InputState,
    rng: ThreadRng,
    texture: Texture2D,
    path_finder: PathFinder,
    update_rect: Rectangle,
    particle_system: SpriteParticleSystem,
    character_action_manager: CharacterActionManager,
}

fn main() {
    // what the game is pretending the game is running at,
    // to be honest, ideally this will never change unless aspect ratio changes
    let window_width_target = 1920;
    let window_height_target = 1080;

    // what the game will stretch the render target to, to fill the screen
    let actual_window_width = 1920;
    let actual_window_height = 1080;

    let mut current_zoom = ZoomSizes::FiveX;
    let v_width = current_zoom.v_width(window_width_target);
    let v_height = current_zoom.v_height(window_height_target);

    let (mut rl, thread) = raylib::init()
        .size(actual_window_width as i32, actual_window_height as i32)
        .title("Defender_Rust")
        .build();

    let camera = Camera2D {
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

    let mut select_rect = SelectRect::new();
    let sprite_particle_system = SpriteParticleSystem::new(1000);
    let rng = rand::rng();
    let mut camera_pos = camera.target;
    let input_state = InputState::new();
    let mut entity_selecting_manager = EntitySelectingManager::new();
    let mut action_button_manager = ActionButtonManager::new();
    let map_width = 500;
    let map_height = 500;
    let day_night_cycle = DayNightCycle::new();
    let path_finder = PathFinder::new(map_width, map_height);
    let character_action_manager = CharacterActionManager::new();
    let texture = rl.load_texture(&thread, "Tileset.png").unwrap();

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
        particle_system: sprite_particle_system,
        update_rect: Rectangle::default(),
        character_action_manager,
        dt: 0.0,
    };

    let mut map = TileMap::generate_map(map_width, map_height, &mut game_context);
    let mut entity_manager = EntityManager::new(map.map_dimensions);

    let mut char_and_object_multi_shader = rl.load_shader(&thread, None, Some("char_and_obj_multi_shader.frag"));
    let mut shadow_fix_shader = rl.load_shader(&thread, None, Some("object_rt_shadow_fix.frag"));
    let mut ground_time_of_day_shader = rl.load_shader(&thread, None, Some("ground_time_of_day_shader.frag"));
    
    let red_tint_loc_ch_obj_shader = char_and_object_multi_shader.get_shader_location("red_tint");
    let blue_tint_loc_ch_obj_shader = char_and_object_multi_shader.get_shader_location("blue_tint");
    let brightness_modifier_loc_ch_obj_shader =
        char_and_object_multi_shader.get_shader_location("brightness_modifier");

    let red_tint_loc_ground_shader = ground_time_of_day_shader.get_shader_location("red_tint");
    let blue_tint_loc_ground_shader = ground_time_of_day_shader.get_shader_location("blue_tint");
    let brightness_modifier_loc_ground_shader =
        ground_time_of_day_shader.get_shader_location("brightness_modifier");

    let mut ground_render_textures: [RenderTexture2D; 5] = [
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

    let mut object_and_character_render_textures: [RenderTexture2D; 5] = [
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

    //
    // DEBUG START
    //

    entity_manager.add_character(Gatherer::new(Vector2::new(100.0, 100.0)));

    //
    // DEBUG END
    //

    while !rl.window_should_close() {
        game_context.dt = rl.get_frame_time();

        game_context.total_game_time += game_context.dt;

        // update input first
        game_context.input_state.update(&mut rl, camera.zoom);

        if game_context.input_state.left_clicked_once {
            let pos = mouse_world_coords(&game_context);
            make_mouse_click_particles(pos, &mut game_context.particle_system);
        }

        if game_context.input_state.right_clicked_once {
            let pos = mouse_world_coords(&game_context);
            make_mouse_right_click_particles(pos, &mut game_context.particle_system);
        }

        select_rect.update(&game_context);

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
            camera_pos.x += game_context.v_width as f32 * game_context.dt;
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            camera_pos.x -= game_context.v_width as f32 * game_context.dt;
        }
        if rl.is_key_down(KeyboardKey::KEY_W) {
            camera_pos.y -= game_context.v_width as f32 * game_context.dt;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            camera_pos.y += game_context.v_width as f32 * game_context.dt;
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

        // update map first
        map.update(game_context.dt);

        // entity selecting manager updated next in order to properly maintain good order with
        // select modes
        entity_selecting_manager.update(&mut rl);

        // with proper select modes, can now update entity manager itself
        entity_manager.update(
            &mut map,
            &mut game_context,
            &mut entity_selecting_manager,
            &select_rect,
            &mut action_button_manager,
            current_zoom.zoom(),
        );

        // particle system updates next since the particle system could be used by the buttons
        game_context.particle_system.update(game_context.dt);

        // this just updates the values used for the shader
        game_context
            .day_night_cycle
            .update(game_context.dt, &mut rl);

        char_and_object_multi_shader
            .set_shader_value(red_tint_loc_ch_obj_shader, game_context.day_night_cycle.red_tint);
        char_and_object_multi_shader
            .set_shader_value(blue_tint_loc_ch_obj_shader, game_context.day_night_cycle.blue_tint);
        char_and_object_multi_shader.set_shader_value(
            brightness_modifier_loc_ch_obj_shader,
            game_context.day_night_cycle.brightness_modifier,
        );

        ground_time_of_day_shader
            .set_shader_value(red_tint_loc_ground_shader, game_context.day_night_cycle.red_tint);
        ground_time_of_day_shader
            .set_shader_value(blue_tint_loc_ground_shader, game_context.day_night_cycle.blue_tint);
        ground_time_of_day_shader.set_shader_value(
            brightness_modifier_loc_ground_shader,
            game_context.day_night_cycle.brightness_modifier,
        );

        let current_ground_rt = &mut ground_render_textures[current_zoom as usize];
        let current_object_and_character_rt =
            &mut object_and_character_render_textures[current_zoom as usize];
        //--UPDATE ENDS HERE--//

        //--DRAWING BEINGS HERE--//
        let mut d = rl.begin_drawing(&thread);

        {
            let mut ground_rt = d.begin_texture_mode(&thread, current_ground_rt);
            ground_rt.clear_background(Color::DARKCYAN);

            {
                let mut cam = ground_rt.begin_mode2D(game_context.camera);

                let mut shader = cam.begin_shader_mode(&mut ground_time_of_day_shader);

                map.draw(&mut shader, &game_context);

                game_context
                    .particle_system
                    .draw(&mut shader, &game_context.texture);
            } // end camera drawing
        } // end ground rt drawing

        {
            let mut object_rt = d.begin_texture_mode(&thread, current_object_and_character_rt);

            object_rt.clear_background(Color::BLANK);

            {
                let mut cam = object_rt.begin_mode2D(game_context.camera);

                let mut shader = cam.begin_shader_mode(&mut char_and_object_multi_shader);

                entity_manager.draw(
                    &map.map_object_grid,
                    &mut shader,
                    &game_context.texture,
                    game_context.day_night_cycle.current_shadow_shear,
                    game_context.day_night_cycle.current_shadow_scale,
                );

                select_rect.draw(&mut shader);
                action_button_manager.draw(&mut shader, &game_context);

                mouse_utils::draw_mouse(
                    &mut shader,
                    mouse_utils::mouse_world_coords(&game_context),
                    &game_context.texture,
                );
            } // end camera and shader drawing
        } // end object rt drawing

        let source_rec = Rectangle::new(
            0.0,
            0.0,
            current_ground_rt.texture.width as f32,
            -current_ground_rt.texture.height as f32, // Negative height flips it right-side up
        );

        let dest_rec = Rectangle::new(
            0.0,
            0.0,
            actual_window_width as f32,
            actual_window_height as f32,
        );
        let origin = Vector2::new(0.0, 0.0);

        d.draw_texture_pro(
            current_ground_rt,
            source_rec,
            dest_rec,
            origin,
            0.0,
            Color::WHITE,
        );

        {
            let mut shadow_fix_shader_handle = d.begin_shader_mode(&mut shadow_fix_shader);

            shadow_fix_shader_handle.draw_texture_pro(
                current_object_and_character_rt,
                source_rec,
                dest_rec,
                origin,
                0.0,
                Color::WHITE,
            );
        }

        game_context.day_night_cycle.draw_dbg(&mut d);
        entity_selecting_manager.draw(&mut d);
    }
    //--DRAWING ENDS HERE--//
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

// if you change window to a new aspect ratio, youll need to set_render_textures to accomodate the new
// aspect ratio otherwise it will shrink or stretch
fn change_window_size(
    rl: &mut RaylibHandle,
    window_width: &mut f32,
    window_height: &mut f32,
    new_width: f32,
    new_height: f32,
) {
    *window_width = new_width;
    *window_height = new_height;
    rl.set_window_size(*window_width as i32, *window_height as i32);
}

/// sets render textures based on a window width target.
/// for example, if you pass in 1920, 1080, the zooms will be based on that
fn set_render_textures(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    ground_rt_array: &mut [RenderTexture2D],
    obj_rt_array: &mut [RenderTexture2D],
    window_width_target: f32,
    window_height_target: f32,
) {
    let w_u32 = window_width_target as u32;
    let h_u32 = window_height_target as u32;

    let rt_count = ground_rt_array.len();

    for i in 0..rt_count {
        ground_rt_array[i] = rl
            .load_render_texture(thread, w_u32 / (i as u32 + 2), h_u32 / (i as u32 + 2))
            .unwrap();
    }
    for i in 0..rt_count {
        obj_rt_array[i] = rl
            .load_render_texture(thread, w_u32 / (i as u32 + 2), h_u32 / (i as u32 + 2))
            .unwrap();
    }
}

pub fn make_mouse_click_particles(click_pos: Vector2, particle_system: &mut SpriteParticleSystem) {
    static CLICK_PARTICLE_SPRITE: Sprite = Sprite::new(48, 1, 2, 1);
    for dir in ORTHOGONAL_DELTAS {
        const SPEED: f32 = 60.0;

        let pos = click_pos + dir.as_vec2() * 2.0;
        let delta = (pos - click_pos).normalized();
        let angle = delta.y.atan2(delta.x);
        let velocity = delta * SPEED;
        let acceleration = -delta * SPEED * 3.0;

        particle_system.emit_ex(
            &CLICK_PARTICLE_SPRITE,
            pos,
            velocity,
            acceleration,
            0.0,
            angle.to_degrees(),
            0.3,
            false,
        );
    }
}

pub fn make_mouse_right_click_particles(
    click_pos: Vector2,
    particle_system: &mut SpriteParticleSystem,
) {
    static CLICK_PARTICLE_SPRITE: Sprite = Sprite::new(48, 4, 2, 1);
    for dir in ORTHOGONAL_DELTAS {
        const SPEED: f32 = 60.0;

        let pos = click_pos + dir.as_vec2() * 2.0;
        let delta = (pos - click_pos).normalized();
        let angle = delta.y.atan2(delta.x);
        let velocity = delta * SPEED;
        let acceleration = -delta * SPEED * 3.0;

        particle_system.emit_ex(
            &CLICK_PARTICLE_SPRITE,
            pos,
            velocity,
            acceleration,
            0.0,
            angle.to_degrees(),
            0.3,
            false,
        );
    }
}
