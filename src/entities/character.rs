use std::collections::{HashMap, HashSet};

use raylib::{
    drawing::RaylibDrawHandle,
    ffi::__bool_true_false_are_defined,
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
        characters::{
            enemy::Enemy,
            gatherer::{Gatherer, GathererState},
        },
        entity_manager::{BasicCharacterInfo, CharID},
        object::Object,
    },
    map::tile_map::{MapDimensions, TileMap},
    systems::character_action_manager::{CharacterAction, CharacterActionManager},
    utils::{
        camera_utils,
        direction_utils::FacingDirection,
        draw_utils,
        map_cord::MapCord,
        map_utils,
        pathfinding::{
            Path,
            PathError::{self},
        },
    },
};

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
pub enum SimpleCharacterKind {
    Gatherer,
    Enemy,
}

#[derive(Clone, Copy, PartialEq)]
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

#[derive(Clone, Copy)]
pub enum CombatState {
    // for handling the attack animation
    PreAttack,

    // for handling the attack action itself
    Attack,

    // for handling an optional post attack cooldown animation
    PostAttack,
}

pub struct CharacterData {
    pub character_values: CharacterSpecificData,
    pub path: Option<Path>,
    pub opponents: HashSet<CharID>,
    current_opponent: Option<CharID>,
    pub pos: Vector2,
    pub target_pos: Option<Vector2>,
    pub facing_direction: FacingDirection,
    pub health: f32,
    pub is_hovering: bool,
    pub is_hovering_for_move: bool,
    pub is_selected: bool,
    pub is_selected_for_move: bool,
    pub state: CharacterState,
    pub combat_state: CombatState,
    pub unique_char_id: CharID,
    pub attack_timer: Timer,
}

/// this struct is for things that are based on T type character, not characters as a whole and not things managed by the code specifically
/// basically just things that are solely dependent on the type of character holding it (ex: position doesnt count, since that isnt based
/// on the character type)
pub struct CharacterSpecificData {
    pub idle_anim: SpriteAnimationInstance,
    pub move_anim: SpriteAnimationInstance,
    pub attack_anim: SpriteAnimationInstance,
    pub post_attack_anim: SpriteAnimationInstance,
    pub draw_offset: Vector2,
    pub max_health: f32,
    pub time_between_attacks: f32,
    pub attack_power: f32,
    pub move_speed: f32,
    pub width: f32,
    pub height: f32,
    pub affiliation: Affiliation,
    pub character_kind: SimpleCharacterKind,
}

impl CharacterData {
    pub fn new(pos: Vector2, character_values: CharacterSpecificData) -> CharacterData {
        return CharacterData {
            state: CharacterState::None,
            combat_state: CombatState::PreAttack,
            current_opponent: None,
            pos,
            target_pos: None,
            path: None,
            facing_direction: FacingDirection::Right,
            health: character_values.max_health,
            is_hovering: false,
            is_hovering_for_move: false,
            is_selected: false,
            is_selected_for_move: false,
            opponents: HashSet::new(),
            unique_char_id: CharID(0),
            attack_timer: Timer::new(character_values.time_between_attacks),
            character_values,
        };
    }

    pub fn set_idle_character_state(&mut self) {
        self.state = CharacterState::None;
    }

    pub fn move_to(
        &mut self,
        target: Vector2,
        game_context: &mut GameContext,
        map: &TileMap,
    ) -> CharacterMovementResult {
        // compare current target to new target
        if self.target_pos != Some(target) || matches!(self.path, None) {
            self.target_pos = Some(target);

            match game_context.path_finder.a_star(
                MapCord::from_vec2(self.pos),
                MapCord::from_vec2(target),
                map,
                100.0,
            ) {
                Ok(path) => self.path = Some(path),
                Err(path_error) => match path_error {
                    PathError::TooLong => {
                        println!("Route is too long for character");
                        return CharacterMovementResult::TooLong;
                    }
                    PathError::NoRoute => {
                        println!("There is no viable route for character");
                        return CharacterMovementResult::NoRoute;
                    }
                },
            }

            if let Some(path) = &mut self.path {
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

        if let Some(path) = &mut self.path {
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
                    self.path = None;
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

pub enum CharacterKind {
    Gatherer(Gatherer),
    Enemy(Enemy),
}

pub struct Character {
    pub character_data: CharacterData,
    pub character_kind: CharacterKind,
}

impl Character {
    pub fn set_move_to(&mut self, target: Vector2) {
        self.set_idle_to_unique_states();
        self.character_data.state = CharacterState::Moving { target };
    }

    pub fn set_idle_to_unique_states(&mut self) {
        match &mut self.character_kind {
            CharacterKind::Gatherer(gatherer) => gatherer.gatherer_state = GathererState::Idle,
            CharacterKind::Enemy(enemy) => enemy.set_idle(),
        }
    }

    #[inline]
    pub fn update(
        &mut self,
        game_context: &mut GameContext,
        map: &mut TileMap,
        character_info: &HashMap<CharID, BasicCharacterInfo>,
    ) {
        match self.character_data.state {
            CharacterState::None => {
                if self.is_idle() {
                    self.character_data
                        .character_values
                        .idle_anim
                        .update(game_context.dt);
                }

                match &mut self.character_kind {
                    CharacterKind::Gatherer(gatherer) => {
                        gatherer.update(&mut self.character_data, game_context, map)
                    }
                    CharacterKind::Enemy(enemy) => enemy.update(game_context, map, character_info),
                }
            }
            CharacterState::Moving { target } => {
                match self.character_data.move_to(target, game_context, map) {
                    CharacterMovementResult::NotArrivedYet => self
                        .character_data
                        .character_values
                        .move_anim
                        .update(game_context.dt),
                    _ => {
                        self.character_data.character_values.move_anim.reset();
                        self.character_data.state = CharacterState::None;
                    }
                }
            }
            CharacterState::InCombat => {
                let dt = game_context.dt;

                // unfortunately doing this every frame just made a lot of sense
                // between attack animation times, and many things potentially affecting a potential opponent
                // i just dont think checking once in a state would make sense
                // it doesnt make sense to attack an opponent that is potentially dead, or wind
                // an attack up for one thats dead, etc, id basically be checking in every state anyway every frame
                if let Some(id) = self.character_data.current_opponent {
                    let opponent_is_invalid = match character_info.get(&id) {
                        Some(c_info) => c_info.health <= 0.0,
                        None => true,
                    };

                    if opponent_is_invalid {
                        self.character_data.current_opponent = None;
                    }
                }

                // if list isnt empty, checks list and assigns next opponent
                if self.character_data.current_opponent.is_none() {
                    self.clean_opponents_list(character_info);

                    // checks after resolving any issues with the current opponent and cleaning the list if the list is empty
                    if self.character_data.opponents.is_empty() {
                        // reset all relevant values
                        self.character_data.state = CharacterState::None;
                        self.character_data.combat_state = CombatState::PreAttack;
                        self.character_data.attack_timer.reset();
                        self.character_data.character_values.attack_anim.reset();
                        self.character_data
                            .character_values
                            .post_attack_anim
                            .reset();
                        return;
                    }

                    self.character_data.current_opponent =
                        self.find_suitable_opponent(character_info);
                }

                match self.character_data.combat_state {
                    CombatState::PreAttack => {
                        if self.character_data.attack_timer.is_done() {
                            self.character_data.character_values.attack_anim.update(dt);
                            if self
                                .character_data
                                .character_values
                                .attack_anim
                                .finished_playing
                            {
                                self.character_data.character_values.attack_anim.reset();
                                self.character_data.attack_timer.reset();
                                self.character_data.combat_state = CombatState::Attack;
                            }
                        }
                    }
                    CombatState::Attack => {
                        let self_idx = self.character_data.unique_char_id;
                        let opponent_idx = self.character_data.current_opponent.unwrap();

                        self.do_attack(
                            self_idx,
                            opponent_idx,
                            &mut game_context.character_action_manager,
                        );

                        // move to the wind down attack before evaluating state
                        self.character_data.combat_state = CombatState::PostAttack;
                    }
                    CombatState::PostAttack => {
                        let post_attack_anim =
                            &mut self.character_data.character_values.post_attack_anim;

                        post_attack_anim.update(dt);

                        if post_attack_anim.finished_playing {
                            post_attack_anim.reset();
                            self.character_data.combat_state = CombatState::PreAttack;
                        }
                    }
                }
            }
        }

        self.character_data.is_hovering = false;
        self.character_data.is_hovering_for_move = false;
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
        let mut spr = match self.character_data.state {
            CharacterState::None => match &self.character_kind {
                CharacterKind::Gatherer(gatherer) => gatherer.current_sprite(&self.character_data),
                CharacterKind::Enemy(enemy) => enemy.current_sprite(&self.character_data),
            },
            CharacterState::Moving { .. } => self
                .character_data
                .character_values
                .move_anim
                .current_sprite(),
            CharacterState::InCombat => match self.character_data.combat_state {
                CombatState::PreAttack => self
                    .character_data
                    .character_values
                    .attack_anim
                    .current_sprite(),
                CombatState::Attack => *self
                    .character_data
                    .character_values
                    .attack_anim
                    .sprite_animation
                    .frames
                    .last()
                    .unwrap(),
                CombatState::PostAttack => self
                    .character_data
                    .character_values
                    .post_attack_anim
                    .current_sprite(),
            },
        };

        if self.character_data.facing_direction == FacingDirection::Left {
            // if i dont -0.1 then the width will apparently be less than the width due
            // to floating point shenanigans, since its drawn to a low res grid its
            // rounded down, i have to add to the width essentially
            spr.src_rect.width = -spr.src_rect.width - 0.1;
        }

        return spr;
    }

    /// for now prioritizes lowest health opponent, but can be changed eventually
    pub fn find_suitable_opponent(
        &mut self,
        character_info: &HashMap<CharID, BasicCharacterInfo>,
    ) -> Option<CharID> {
        let mut lowest_id = None;
        let mut lowest_health = f32::MAX;

        for ch_id in &self.character_data.opponents {
            let hp = character_info.get(&ch_id).unwrap().health;

            if hp < lowest_health {
                lowest_id = Some(*ch_id);
                lowest_health = hp;
            }
        }

        return lowest_id;
    }

    pub fn clean_opponents_list(&mut self, character_info: &HashMap<CharID, BasicCharacterInfo>) {
        self.character_data.opponents.retain(|ch_id| {
            character_info
                .get(ch_id)
                .is_some_and(|info| info.health > 0.0)
        });
    }

    #[inline]
    pub fn get_draw_pos(&self) -> Vector2 {
        return self.character_data.pos + self.character_data.character_values.draw_offset;
    }

    #[inline]
    pub fn get_hover_rect(&self) -> Rectangle {
        let d_pos = self.get_draw_pos();
        return Rectangle::new(
            d_pos.x,
            d_pos.y,
            self.character_data.character_values.width,
            self.character_data.character_values.height,
        );
    }

    #[inline]
    pub fn get_tile_index(&self, map_dimensions: MapDimensions) -> usize {
        let pos = self.character_data.pos;
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
        if !camera_utils::is_in_update_area(object.object_data.pos, game_context) {
            return;
        }

        object.update(game_context, false);
    }

    pub fn is_idle(&self) -> bool {
        match &self.character_kind {
            CharacterKind::Gatherer(gatherer) => {
                return gatherer.gatherer_state == GathererState::Idle;
            }
            // enemy itself is a base struct, so dipatch is one level deeper
            CharacterKind::Enemy(enemy) => enemy.is_idle(),
        }
    }

    pub fn level_up(&mut self) {
        self.character_data.character_values.move_speed += 0.05;
        self.character_data.character_values.attack_power += 1.0;
        self.character_data.character_values.max_health += 5.0;
        self.character_data.character_values.time_between_attacks -= 0.01;
        self.character_data.attack_timer =
            Timer::new(self.character_data.character_values.time_between_attacks);
    }

    pub fn do_attack(
        &mut self,
        self_id: CharID,
        target_id: CharID,
        character_action_manager: &mut CharacterActionManager,
    ) {
        character_action_manager.push_action(CharacterAction::Attack {
            attacker_id: self_id,
            target_id,
        });
    }
}
