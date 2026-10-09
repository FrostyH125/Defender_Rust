use std::iter::Map;

use raylib::math::Vector2;
use zander_game_core_rs::{
    raylib::{
        animation_data::SpriteAnimationData, sprite::Sprite,
        sprite_animation::SpriteAnimationInstance,
    },
    system::timer::Timer,
};

use crate::{
    GameContext,
    entities::{
        character::{
            Affiliation, Character, CharacterData, CharacterKind, CharacterMovementResult,
            CharacterSpecificData, SimpleCharacterKind,
        },
        characters::gatherer::GathererState::MovingToObject,
        object::Object,
    },
    map::tile_map::{MapObjectGrid, TileMap},
    utils::entity_utils::object_matches_gathering_target,
};

static GATHERER_IDLE_ANIM: SpriteAnimationData = SpriteAnimationData {
    // frames: &[
    //     Sprite::new(16, 176, 8, 8),
    //     Sprite::new(24, 176, 8, 8),
    //     Sprite::new(32, 176, 8, 8),
    //     Sprite::new(40, 176, 8, 8),
    // ],
    frames: &Sprite::create_sequence_of_sprites::<4>(16, 176, 8, 8),
    frame_duration: 0.5,
    should_loop: true,
};

static GATHERER_MOVE_ANIM: SpriteAnimationData = SpriteAnimationData {
    frames: &Sprite::create_sequence_of_sprites::<3>(16, 184, 8, 8),
    frame_duration: 0.25,
    should_loop: true,
};

static GATHERER_ATTACK_ANIM: SpriteAnimationData = SpriteAnimationData {
    frames: &Sprite::create_sequence_of_sprites::<5>(16, 192, 8, 8),
    frame_duration: 0.25,
    should_loop: false,
};

static GATHERER_GATHER_ANIM: SpriteAnimationData = SpriteAnimationData {
    frames: &Sprite::create_sequence_of_sprites::<9>(16, 200, 8, 8),
    frame_duration: 0.05,
    should_loop: false,
};

static GATHERER_POST_ATTACK_ANIM: SpriteAnimationData = SpriteAnimationData {
    frames: &Sprite::create_sequence_of_sprites::<5>(64, 192, 8, 8),
    frame_duration: 0.1,
    should_loop: false,
};

struct ObjectEntry {
    idx: usize,
    pos: Vector2,
    dist: f32,
}

#[derive(Clone, Copy, Eq, PartialEq)]
// when adding a new gather target, you need to add the match to entity_utils::object_matches_gather_target()
pub enum GatherTarget {
    Tree,
    Grass,
}

#[derive(PartialEq)]
pub enum GathererState {
    Idle,
    LookingForObject {
        gather_target: GatherTarget,
    },
    MovingToObject {
        target_pos: Vector2,
        gather_target: GatherTarget,
    },
    GatheringObject {
        gather_target: GatherTarget,
    },
}

impl std::fmt::Debug for GathererState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Idle => write!(f, "Idle"),
            Self::LookingForObject { .. } => write!(f, "Looking for Object"),
            Self::MovingToObject { .. } => write!(f, "Moving to Object"),
            Self::GatheringObject { .. } => write!(f, "Gathering"),
        }
    }
}

pub struct Gatherer {
    object_indices: Vec<usize>,
    current_index: Option<usize>,
    gather_anim: SpriteAnimationInstance,
    pub gatherer_state: GathererState,
    gather_timer: Timer,
    gathering_power: f32,
}

impl Gatherer {
    pub fn new(pos: Vector2) -> Character {
        let character_values = CharacterSpecificData {
            idle_anim: SpriteAnimationInstance::new(&GATHERER_IDLE_ANIM),
            move_anim: SpriteAnimationInstance::new(&GATHERER_MOVE_ANIM),
            attack_anim: SpriteAnimationInstance::new(&GATHERER_ATTACK_ANIM),
            post_attack_anim: SpriteAnimationInstance::new(&GATHERER_POST_ATTACK_ANIM),
            affiliation: Affiliation::Good,
            draw_offset: Vector2::zero(),
            time_between_attacks: 1.0,
            attack_power: 5.0,
            width: 8.0,
            height: 8.0,
            move_speed: 30.0,
            max_health: 100.0,
            character_kind: SimpleCharacterKind::Gatherer,
        };

        let data = CharacterData::new(pos, character_values);

        let gatherer = Gatherer {
            gatherer_state: GathererState::Idle,
            gather_anim: SpriteAnimationInstance::new(&GATHERER_GATHER_ANIM),
            gathering_power: 20.0,
            gather_timer: Timer::new(2.0),
            object_indices: Vec::new(),
            current_index: None,
        };

        return Character {
            character_data: data,
            character_kind: CharacterKind::Gatherer(gatherer),
        };
    }

    pub fn update(
        &mut self,
        character_data: &mut CharacterData,
        game_context: &mut GameContext,
        map: &mut TileMap,
    ) {
        match self.gatherer_state {
            GathererState::Idle => (),
            GathererState::LookingForObject { gather_target } => {
                self.looking_for_object(character_data, map, gather_target);
            }
            GathererState::MovingToObject {
                target_pos,
                gather_target,
            } => {
                self.moving_to_object(character_data, game_context, map, target_pos, gather_target);
            }
            GathererState::GatheringObject { gather_target } => {
                self.gathering_object(character_data, game_context, map, gather_target);
            }
        }
    }

    fn gathering_object(
        &mut self,
        character_data: &mut CharacterData,
        game_context: &mut GameContext,
        map: &mut TileMap,
        gather_target: GatherTarget,
    ) {
        self.gather_timer.track(game_context.dt);

        if !self.gather_timer.is_done() {
            return;
        }

        self.gather_anim.update(game_context.dt);

        if !self.gather_anim.finished_playing {
            return;
        }

        self.gather_timer.reset();
        self.gather_anim.reset();

        if self.gather(
            character_data,
            &mut map.map_object_grid[self.current_index.unwrap()]
                .as_mut()
                .unwrap(),
            game_context,
        ) {
            self.gatherer_state = GathererState::LookingForObject { gather_target };
        }
    }

    fn moving_to_object(
        &mut self,
        character_data: &mut CharacterData,
        game_context: &mut GameContext,
        map: &mut TileMap,
        target_pos: Vector2,
        gather_target: GatherTarget,
    ) {
        character_data
            .character_values
            .move_anim
            .update(game_context.dt);

        match character_data.move_to(target_pos, game_context, map) {
            CharacterMovementResult::Success => {
                character_data.character_values.move_anim.reset();

                self.gatherer_state = GathererState::GatheringObject {
                    gather_target: gather_target,
                };
            }
            CharacterMovementResult::NotArrivedYet => (),
            CharacterMovementResult::NoRoute | CharacterMovementResult::TooLong => {
                character_data.character_values.move_anim.reset();
                self.object_indices.clear();
                map.map_object_grid[self.current_index.unwrap()]
                    .as_mut()
                    .unwrap()
                    .set_unoccupied();
                self.current_index = None;
                self.gatherer_state = GathererState::Idle;
            }
        }
    }

    fn looking_for_object(
        &mut self,
        character_data: &mut CharacterData,
        map: &mut TileMap,
        gather_target: GatherTarget,
    ) {
        // reset this here because if an object that is currently being gathered is reselected, then
        // i need it to reset the timer so it doesnt just continue off from where it stopped.
        self.gather_timer.reset();

        Self::remove_dead_objects(&mut self.object_indices, &mut map.map_object_grid);

        let closest_obj: Option<ObjectEntry> =
            self.find_closest_target(character_data, &map.map_object_grid, gather_target);

        match closest_obj {
            Some(o) => {
                map.map_object_grid[o.idx].as_mut().unwrap().set_occupied();
                self.current_index = Some(o.idx);
                self.gatherer_state = MovingToObject {
                    target_pos: o.pos,
                    gather_target: gather_target,
                };
            }
            None => {
                self.current_index = None;
                self.object_indices.clear();
                self.gatherer_state = GathererState::Idle
            }
        }
    }

    fn gather(
        &self,
        character_data: &mut CharacterData,
        obj: &mut Object,
        game_context: &mut GameContext,
    ) -> bool {
        obj.take_hit(
            self.gathering_power,
            game_context,
            character_data.facing_direction,
        );

        game_context
            .visual_effects_manager
            .add_damage_number(obj.get_center_pos(), self.gathering_power);

        if obj.is_breaking() {
            obj.unmark_for_gathering();
            return true;
        }

        return false;
    }

    fn find_closest_target(
        &self,
        character_data: &mut CharacterData,
        object_grid: &MapObjectGrid,
        target_obj: GatherTarget,
    ) -> Option<ObjectEntry> {
        let mut closest_obj: Option<ObjectEntry> = None;

        for idx in &self.object_indices {
            let obj = &object_grid[*idx];

            if let None = obj {
                continue;
            }

            let obj = obj.as_ref().unwrap();

            if obj.is_breaking() {
                continue;
            }

            if !Gatherer::obj_matches_target_and_is_available(obj, target_obj) {
                continue;
            }

            match &closest_obj {
                Some(obj_entry) => {
                    let dist = obj.object_data.pos.distance_to(character_data.pos);

                    if dist < obj_entry.dist {
                        closest_obj = Some(ObjectEntry {
                            idx: *idx,
                            pos: obj.object_data.pos,
                            dist,
                        })
                    }
                }
                None => {
                    closest_obj = Some(ObjectEntry {
                        idx: *idx,
                        pos: obj.object_data.pos,
                        dist: obj.object_data.pos.distance_to(character_data.pos),
                    })
                }
            }
        }

        return closest_obj;
    }

    fn obj_matches_target_and_is_available(obj: &Object, target_obj: GatherTarget) -> bool {
        if obj.is_occupied() {
            return false;
        };

        if !obj.is_marked_for_gathering() {
            return false;
        }

        return object_matches_gathering_target(target_obj, &obj.object_kind);
    }

    pub fn current_sprite(&self, character_data: &CharacterData) -> Sprite {
        match self.gatherer_state {
            GathererState::Idle => character_data.character_values.idle_anim.current_sprite(),
            GathererState::LookingForObject { .. } => {
                character_data.character_values.idle_anim.current_sprite()
            }
            MovingToObject { .. } => character_data.character_values.move_anim.current_sprite(),
            GathererState::GatheringObject { .. } => {
                if self.gather_anim.is_playing {
                    self.gather_anim.current_sprite()
                } else {
                    GATHERER_IDLE_ANIM.frames[0]
                }
            }
        }
    }

    pub fn set_new_target(
        &mut self,
        character_data: &mut CharacterData,
        gather_target: GatherTarget,
        object_grid: &mut MapObjectGrid,
        obj_ids_of_type: &[usize],
    ) {
        // set to idle so that the character doesnt keep fighting or walking
        character_data.set_idle_character_state();

        // reset current objects in queue
        // also set no current object if there was one
        self.object_indices.clear();
        if let Some(o_idx) = self.current_index {
            object_grid[o_idx].as_mut().unwrap().set_unoccupied();
            self.current_index = None;
        }

        // set new object ids for gathering
        self.object_indices = obj_ids_of_type.to_vec();

        // set new state with the proper gather target
        self.gatherer_state = GathererState::LookingForObject { gather_target };
    }

    pub fn remove_dead_objects(object_indices: &mut Vec<usize>, object_grid: &MapObjectGrid) {
        for i in (0..object_indices.len()).rev() {
            let obj = &object_grid[object_indices[i]];

            match obj {
                Some(o) => {
                    // even if the object exists, remove it if its unusable, meaning its already been broken
                    if o.is_breaking() {
                        object_indices.swap_remove(i);
                    }
                }
                None => {
                    // object no longer exists
                    object_indices.swap_remove(i);
                }
            }
        }
    }
}
