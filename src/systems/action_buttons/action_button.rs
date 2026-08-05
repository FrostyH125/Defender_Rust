use basic_raylib_core::graphics::sprite::Sprite;
use rand::RngExt;
use raylib::{
    drawing::RaylibDrawHandle,
    math::{Rectangle, Vector2},
    texture::Texture2D,
};

use crate::{
    GameContext,
    entities::{
        character::Character,
        characters::gatherer::{GatherTarget, GathererState},
        entity_manager::CharacterEntry,
        object::Object,
    },
    map::tile_map::MapObjectGrid,
    utils::{draw_utils, entity_utils::get_char_by_index, mouse_utils::mouse_world_coords},
};

pub const CHOP_BUTTON_SPRITE: Sprite = Sprite::new(144, 40, 16, 16);

pub enum ActionButtonKind {
    ChopButton,
}

pub struct ActionButton {
    kind: ActionButtonKind,
    sprite: Sprite,
    pub rect: Rectangle,
    pub spawn_y_pos: f32,
    total_life_time: f32,
    pub sin_offset: f32,
    pub is_hovering: bool,
}

impl ActionButton {
    pub fn new(kind: ActionButtonKind) -> Self {
        let sprite = match kind {
            ActionButtonKind::ChopButton => CHOP_BUTTON_SPRITE,
        };

        return Self {
            kind,
            sprite,
            spawn_y_pos: f32::default(),
            rect: Rectangle::new(0.0, 0.0, 16.0, 16.0),
            total_life_time: 0.0,
            sin_offset: 0.0,
            is_hovering: false,
        };
    }
}

impl ActionButton {
    pub fn update(&mut self, game_context: &GameContext) {
        self.total_life_time += game_context.dt;

        self.rect.y =
            self.spawn_y_pos + ((self.total_life_time + self.sin_offset) / 2.0).sin() * 2.0;

        self.is_hovering = self
            .rect
            .check_collision_point_rec(mouse_world_coords(game_context));
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        let current_pos = Vector2::new(self.rect.x, self.rect.y);

        match self.is_hovering {
            true => draw_utils::draw_with_extra_brightness(d, &self.sprite, current_pos, texture),
            false => self.sprite.draw(d, current_pos, texture),
        }
    }

    pub fn on_click(
        &mut self,
        obj_ids: &[usize],
        char_ids: &[usize],
        object_grid: &mut MapObjectGrid,
        characters: &mut [CharacterEntry],
    ) {
        match self.kind {
            ActionButtonKind::ChopButton => ActionButton::set_gatherers_to_gather(
                obj_ids,
                char_ids,
                object_grid,
                characters,
                GatherTarget::Tree,
            ),
        }

        //self.make_pop_particles();
    }

    /// sets the gatherers to gather the objects specified in the obj_kind parameter
    pub fn set_gatherers_to_gather(
        obj_ids: &[usize],
        char_ids: &[usize],
        object_grid: &mut MapObjectGrid,
        characters: &mut [CharacterEntry],
        obj_kind: GatherTarget,
    ) {
        for obj_id in obj_ids {
            let obj = &mut object_grid[*obj_id];

            match obj_kind {
                GatherTarget::Tree => {
                    obj.get_mut_data().is_marked_for_gathering = true;
                }
                GatherTarget::Grass => todo!(),
            }
        }

        for char_id in char_ids {
            let char = &mut get_char_by_index(characters, *char_id).character;

            if let Character::GathererChar(gatherer) = char {
                gatherer.state = GathererState::LookingForObject(obj_kind);
            }
        }
    }

    pub fn make_spawn_particles(&self, game_context: &mut GameContext) {
        static BUBBLE_PARTICLE_SPAWN_SPRITE_LARGE: Sprite = Sprite::new(48, 0, 3, 3);
        static BUBBLE_PARTICLE_SPAWN_SPRITE_SMALL: Sprite = Sprite::new(49, 0, 1, 1);

        let rng = &mut game_context.rng;

        for _ in 0..=rng.random_range(30..=40) {
            let x_pos = rng.random_range(self.rect.x..=(self.rect.x + self.rect.width));
            let y_pos =
                rng.random_range((self.rect.y + 5.0)..=(self.rect.y + self.rect.height + 5.0));

            let y_vel = rng.random_range(-5.0..=0.0);
            let x_vel = rng.random_range(-0.5..=0.5);
            let y_acc = rng.random_range(-90.0..=-50.0);
            let life_span = rng.random_range(0.75..=1.25);

            let spr = match rng.random_bool(0.05) {
                true => &BUBBLE_PARTICLE_SPAWN_SPRITE_LARGE,
                false => &BUBBLE_PARTICLE_SPAWN_SPRITE_SMALL,
            };

            game_context.particle_system.emit(
                spr,
                Vector2::new(x_pos, y_pos),
                Vector2::new(x_vel, y_vel),
                Vector2::new(0.0, y_acc),
                life_span,
            );
        }
    }
    pub fn make_pop_particles(&self) {
        todo!()
    }
}
