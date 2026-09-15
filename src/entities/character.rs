use std::collections::HashMap;

use raylib::{
    drawing::RaylibDrawHandle,
    math::{Rectangle, Vector2},
    texture::Texture2D,
};
use zander_game_core_rs::{
    raylib::{sprite::Sprite, sprite_animation::SpriteAnimationInstance},
    system::timer::Timer,
};

use crate::{
    GameContext, TILE_SIZE,
    entities::{
        characters::gatherer::{Gatherer, GathererState},
        entity_manager::CharacterInfo,
        object::Object,
    },
    map::tile_map::{MapDimensions, TileMap},
    systems::character_action_manager::CharacterActionManager,
    utils::{
        camera_utils,
        direction_utils::FacingDirection,
        draw_utils,
        map_cord::MapCord,
        map_utils,
        pathfinding::PathResult::{self, NoPath},
    },
};

#[derive(Clone, Copy)]
pub enum Affiliation {
    Good,
    Evil,
}

pub enum CharacterMovementResult {
    Success,
    NotArrivedYet,
    NoRoute,
    TooLong,
}

#[derive(Clone, Copy)]
pub enum CharacterState {
    None,
    Moving { target: Vector2 },
    InCombat,
}

pub struct CharacterData {
    pub character_values: CharacterSpecificValues,
    pub path: PathResult,
    pub opponents: Vec<usize>,
    pub pos: Vector2,
    pub target_pos: Option<Vector2>,
    pub facing_direction: FacingDirection,
    pub health: f32,
    pub is_hovering: bool,
    pub is_hovering_for_move: bool,
    pub is_selected: bool,
    pub is_selected_for_move: bool,
    pub state: CharacterState,
    pub char_idx: usize,
    pub attack_timer: Timer,
}

/// this struct is for things that are based on T type character, not characters as a whole and not things managed by the code specifically
/// basically just things that are solely dependent on the type of character holding it (ex: position doesnt count, since that isnt based
/// on the character type)
pub struct CharacterSpecificValues {
    pub idle_anim: SpriteAnimationInstance,
    pub move_anim: SpriteAnimationInstance,
    pub attack_anim: SpriteAnimationInstance,
    pub draw_offset: Vector2,
    pub max_health: f32,
    pub time_between_attacks: f32,
    pub attack_power: f32,
    pub move_speed: f32,
    pub width: f32,
    pub height: f32,
    pub affiliation: Affiliation,
}

impl CharacterData {
    pub fn new(pos: Vector2, character_values: CharacterSpecificValues) -> CharacterData {
        return CharacterData {
            state: CharacterState::None,
            pos,
            target_pos: None,
            path: NoPath,
            facing_direction: FacingDirection::Right,
            health: character_values.max_health,
            is_hovering: false,
            is_hovering_for_move: false,
            is_selected: false,
            is_selected_for_move: false,
            opponents: Vec::new(),
            char_idx: 0,
            attack_timer: Timer::new(character_values.time_between_attacks),
            character_values,
        };
    }

    pub fn move_to(
        &mut self,
        target: Vector2,
        game_context: &mut GameContext,
        map: &TileMap,
    ) -> CharacterMovementResult {
        // compare current target to new target
        if self.target_pos != Some(target) || matches!(self.path, PathResult::NoPath) {
            self.target_pos = Some(target);
            self.path = game_context.path_finder.a_star(
                MapCord::from_vec2(self.pos),
                MapCord::from_vec2(target),
                map,
                100.0,
            );
            if let PathResult::Success { path } = &mut self.path {
                path.push_back(target);

                // if the target path entry is more than or equal to an eigth of a block away from the last
                // real final path entry, remove the real final path entry
                // this reduces backtracking

                if path.len() > 1 {
                    if path[0].distance_to(path[1]) >= 2.0 {
                        path.remove(1);
                    }
                }
            }
        }

        if let PathResult::TooLong = self.path {
            println!("Route is too long for character");
            return CharacterMovementResult::TooLong;
        }

        if let PathResult::NoRoute = self.path {
            println!("There is no viable route for character");
            return CharacterMovementResult::NoRoute;
        }

        if let PathResult::Success { path } = &mut self.path {
            if path.is_empty() {
                return CharacterMovementResult::Success;
            }

            let mut next = &path[0];

            // get the next tile
            if self.pos.distance_to(*next) <= 1.0 {
                if path.len() > 1 {
                    path.pop_front();
                    next = &path[0];
                } else if path.len() == 1 {
                    // if theres only one left and youre running this code,
                    // this means youve made it to the only tile left, which is the target
                    self.pos = target;
                    self.path = NoPath;
                    self.target_pos = None;
                    return CharacterMovementResult::Success;
                }
            }

            let mut delta = *next - self.pos;

            if delta.y.abs() > 1.0 || delta.x.abs() > 1.0 {
                delta.normalize();
            }

            self.pos += delta * self.character_values.move_speed * game_context.dt;

            if delta.x < 0.0 {
                self.facing_direction = FacingDirection::Left;
            } else {
                self.facing_direction = FacingDirection::Right;
            }
        }

        return CharacterMovementResult::NotArrivedYet;
    }
}

pub enum Character {
    GathererChar(Gatherer),
}

impl Character {
    pub fn set_move_to(&mut self, target: Vector2) {
        self.set_idle();
        self.get_mut_data().state = CharacterState::Moving { target };
    }

    pub fn set_idle(&mut self) {
        match self {
            Character::GathererChar(gatherer) => gatherer.state = GathererState::Idle,
        }
    }

    #[inline]
    pub fn get_data(&self) -> &CharacterData {
        match self {
            Character::GathererChar(gatherer) => &gatherer.data,
        }
    }

    #[inline]
    pub fn get_mut_data(&mut self) -> &mut CharacterData {
        match self {
            Character::GathererChar(gatherer) => &mut gatherer.data,
        }
    }

    #[inline]
    pub fn update(
        &mut self,
        game_context: &mut GameContext,
        map: &mut TileMap,
        character_info: &HashMap<usize, CharacterInfo>,
    ) {
        match self.get_data().state {
            CharacterState::None => {
                if self.is_idle() {
                    self.get_mut_data()
                        .character_values
                        .idle_anim
                        .update(game_context.dt);
                }

                match self {
                    Character::GathererChar(gatherer) => gatherer.update(game_context, map),
                }
            }
            CharacterState::Moving { target } => {
                match self.get_mut_data().move_to(target, game_context, map) {
                    CharacterMovementResult::NotArrivedYet => self
                        .get_mut_data()
                        .character_values
                        .move_anim
                        .update(game_context.dt),
                    _ => self.get_mut_data().state = CharacterState::None,
                }
            }
            CharacterState::InCombat => {
                // for now just the first entry is important
                // this grabs the enemy hp (current enemy) at the first opponents char id's key
                // the reason i went with a hashmap is to make it easier for characters to look up this sort of thing
                // especially later on when looking for pos and stuff
                // and it also makes it easier to add a spoecific system (such as if i wanted to find the char with the lowest health, for example)
                // i can simply loop through the opponents list plugging into the hashmap, rather than the obviously idiotic solution
                // of looping through all characters and finding the ones that are contained within the opponents vec
                let enemy_hp = character_info[&self.get_data().opponents[0]].health;

                let data = self.get_mut_data();
                let dt = game_context.dt;

                data.attack_timer.track(dt);

                if data.attack_timer.is_done() {
                    data.character_values.attack_anim.update(dt);
                    if data.character_values.attack_anim.finished_playing {
                        data.character_values.attack_anim.reset();
                        data.attack_timer.reset();
                        self.attack(
                            self.get_data().char_idx,
                            self.get_data().opponents[0],
                            &mut game_context.character_action_manager,
                        );
                    }
                }

                // only switches state when opponents are gone
                if enemy_hp <= 0.0 {
                    self.get_mut_data().opponents.remove(0);
                    if self.get_mut_data().opponents.is_empty() {
                        self.get_mut_data().state = CharacterState::None;
                    }
                }
            }
        }

        let data = self.get_mut_data();

        data.is_hovering = false;
        data.is_hovering_for_move = false;
    }

    #[inline]
    pub fn is_point_intersecting(&self, p: Vector2) -> bool {
        return self.get_hover_rect().check_collision_point_rec(p);
    }

    #[inline]
    pub fn draw(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        let sprite = self.current_sprite();
        sprite.draw(d, self.get_draw_pos(), texture);
    }

    #[inline]
    pub fn draw_hover(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        let sprite = self.current_sprite();
        draw_utils::draw_outline(d, sprite, self.get_draw_pos(), texture);
    }

    #[inline]
    pub fn draw_selected(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        let sprite = self.current_sprite();
        draw_utils::draw_with_extra_brightness(d, sprite, self.get_draw_pos(), texture);
    }

    #[inline]
    pub fn draw_hover_for_move(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        let sprite = self.current_sprite();
        draw_utils::draw_outline_for_move(d, sprite, self.get_draw_pos(), texture);
    }

    #[inline]
    pub fn draw_shadow(
        &self,
        d: &mut RaylibDrawHandle,
        texture: &Texture2D,
        shadow_shear: f32,
        shadow_scale: f32,
    ) {
        let sprite = self.current_sprite();

        draw_utils::draw_shadow(
            d,
            sprite,
            self.get_draw_pos(),
            shadow_shear,
            shadow_scale,
            texture,
        );
    }

    /// the individual implementations per class of `current_sprite()` are
    /// only used when the main character state is None and the character itself
    /// is not idle, thus, unique actions can be implemented cleanly
    pub fn current_sprite(&self) -> Sprite {
        let mut spr = match self.get_data().state {
            CharacterState::None => {
                if self.is_idle() {
                    self.get_data().character_values.idle_anim.current_sprite()
                } else {
                    match self {
                        Character::GathererChar(gatherer) => gatherer.current_sprite(),
                    }
                }
            }
            CharacterState::Moving { .. } => {
                self.get_data().character_values.move_anim.current_sprite()
            }
            CharacterState::InCombat => todo!(),
        };

        if self.get_data().facing_direction == FacingDirection::Left {
            // if i dont -0.1 then the width will apparently be less than the width due
            // to floating point shenanigans, since its drawn to a low res grid its
            // rounded down, i have to add to the width essentially
            spr.src_rect.width = -spr.src_rect.width - 0.1;
        }

        return spr;
    }

    #[inline]
    pub fn get_draw_pos(&self) -> Vector2 {
        let data = self.get_data();
        return data.pos + data.character_values.draw_offset;
    }

    #[inline]
    pub fn get_hover_rect(&self) -> Rectangle {
        let data = self.get_data();
        let d_pos = self.get_draw_pos();
        return Rectangle::new(
            d_pos.x,
            d_pos.y,
            data.character_values.width,
            data.character_values.height,
        );
    }

    #[inline]
    pub fn get_tile_index(&self, map_dimensions: MapDimensions) -> usize {
        let pos = self.get_data().pos;
        let cord = MapCord::new(
            pos.x as i16 / TILE_SIZE as i16,
            pos.y as i16 / TILE_SIZE as i16,
        );
        return map_utils::cords_to_index(map_dimensions, cord);
    }

    /// characters are rendered one tile later than their actual pos tile.
    /// since rendering uses the tile indices in a single dimension,
    /// objects immediately to the right would draw over the character
    /// when they should be drawn behind it if one wasn't added
    #[inline]
    pub fn get_render_tile_index(&self, map_dimensions: MapDimensions) -> usize {
        let idx = self.get_tile_index(map_dimensions);
        return idx + 1;
    }

    #[inline]
    pub fn update_obj_if_out_of_update_range(object: &mut Object, game_context: &mut GameContext) {
        let object_pos = object.get_data().pos;

        if !camera_utils::is_in_update_area(object_pos, game_context) {
            return;
        }

        object.update(game_context, false);
    }

    pub fn is_idle(&self) -> bool {
        match self {
            Character::GathererChar(gatherer) => {
                return gatherer.state == GathererState::Idle;
            }
        }
    }

    pub fn reset_state(&mut self) {
        match self {
            Character::GathererChar(gatherer) => gatherer.state = GathererState::Idle,
        }
    }

    pub fn level_up(&mut self) {
        let data = self.get_mut_data();

        data.character_values.move_speed += 0.05;
        data.character_values.attack_power += 1.0;
        data.character_values.max_health += 5.0;
        data.character_values.time_between_attacks -= 0.01;
        data.attack_timer = Timer::new(data.character_values.time_between_attacks);
    }

    pub fn attack(
        &mut self,
        self_id: usize,
        target_id: usize,
        character_action_manager: &mut CharacterActionManager,
    ) {
        match self {
            Character::GathererChar(..) => character_action_manager.request_attack(
                self_id,
                target_id,
                self.get_data().character_values.attack_power,
            ),
        }
    }
}
