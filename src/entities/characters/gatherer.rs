use raylib::math::Vector2;
use zander_game_core_rs::{raylib::{animation_data::SpriteAnimationData, sprite::Sprite, sprite_animation::SpriteAnimationInstance}, system::timer::Timer};

use crate::{
    GameContext, entities::{
        character::{Affiliation, Character, CharacterData, CharacterMovementResult, CharacterSpecificValues}, characters::gatherer::GathererState::MovingToObject, object::Object,
    }, map::tile_map::{MapObjectGrid, TileMap}, utils::pathfinding::PathResult::NoPath,
};

pub static GATHERER_IDLE_ANIM: SpriteAnimationData = SpriteAnimationData {
    frames: &[
        Sprite::new(16, 176, 8, 8),
        Sprite::new(24, 176, 8, 8),
        Sprite::new(32, 176, 8, 8),
        Sprite::new(40, 176, 8, 8),
    ],
    frame_duration: 0.5,
    should_loop: true,
};

pub static GATHERER_MOVE_ANIM: SpriteAnimationData = SpriteAnimationData {
    frames: &[
        Sprite::new(16, 184, 8, 8),
        Sprite::new(24, 184, 8, 8),
        Sprite::new(32, 184, 8, 8),
    ],
    frame_duration: 0.25,
    should_loop: true,
};

pub static GATHERER_ATTACK_ANIM: SpriteAnimationData = SpriteAnimationData {
    frames: &[
        Sprite::new(16, 192, 8, 8),
        Sprite::new(24, 192, 8, 8),
        Sprite::new(32, 192, 8, 8),
        Sprite::new(40, 192, 8, 8),
        Sprite::new(48, 192, 8, 8),
    ],
    frame_duration: 0.25,
    should_loop: false
};

struct ObjectEntry {
    idx: usize,
    pos: Vector2,
    dist: f32,
}

#[derive(Clone, Copy, Eq, PartialEq)]
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
    pub data: CharacterData,
    pub state: GathererState,
    gathering_power: f32,
    gather_timer: Timer,
    pub object_indices: Vec<usize>,
    current_index: Option<usize>,
    pub should_unoccupy_current_obj: bool,
}

impl Gatherer {
    pub fn new(pos: Vector2) -> Character {

        let character_values = CharacterSpecificValues {
            idle_anim: SpriteAnimationInstance::new(&GATHERER_IDLE_ANIM),
            move_anim: SpriteAnimationInstance::new(&GATHERER_MOVE_ANIM),
            attack_anim: SpriteAnimationInstance::new(&GATHERER_ATTACK_ANIM),
            post_attack_anim: None,
            affiliation: Affiliation::Good,
            draw_offset: Vector2::zero(),
            time_between_attacks: 1.0,
            attack_power: 5.0,
            width: 8.0,
            height: 8.0,
            move_speed: 30.0,
            max_health: 100.0,
        };
        
        let gatherer = Gatherer {
            data: CharacterData::new(pos, character_values),
            state: GathererState::Idle,
            gathering_power: 20.0,
            gather_timer: Timer::new(2.0),
            object_indices: Vec::new(),
            current_index: None,
            should_unoccupy_current_obj: false,
        };

        return Character::GathererChar(gatherer);
    }

    pub fn update(&mut self, game_context: &mut GameContext, map: &mut TileMap) {
        if self.should_unoccupy_current_obj {
            self.should_unoccupy_current_obj = false;

            if let Some(o_idx) = self.current_index {
                map.map_object_grid[o_idx].get_mut_data().is_occupied = false;
                self.current_index = None;
            }
        }

        match self.state {
            GathererState::Idle => (),
            GathererState::LookingForObject { gather_target } => {
                self.looking_for_object(map, gather_target);
            }
            GathererState::MovingToObject {
                target_pos,
                gather_target,
            } => {
                self.moving_to_object(game_context, map, target_pos, gather_target);
            }
            GathererState::GatheringObject { gather_target } => {
                self.gathering_object(game_context, map, gather_target);
            }
        }
    }

    fn gathering_object(
        &mut self,
        game_context: &mut GameContext,
        map: &mut TileMap,
        gather_target: GatherTarget,
    ) {
        self.gather_timer.track(game_context.dt);

        if self.gather_timer.is_done() {
            self.gather_timer.reset();

            if self.gather(
                &mut map.map_object_grid[self.current_index.unwrap()],
                game_context,
            ) {
                self.state = GathererState::LookingForObject { gather_target };
            }
        }
    }

    fn moving_to_object(
        &mut self,
        game_context: &mut GameContext,
        map: &mut TileMap,
        target_pos: Vector2,
        gather_target: GatherTarget,
    ) {
        match self.data.move_to(target_pos, game_context, map) {
            CharacterMovementResult::Success => {
                self.state = GathererState::GatheringObject {
                    gather_target: gather_target,
                };
            }
            CharacterMovementResult::NotArrivedYet => (),
            CharacterMovementResult::NoRoute | CharacterMovementResult::TooLong => {
                self.object_indices.clear();
                map.map_object_grid[self.current_index.unwrap()]
                    .get_mut_data()
                    .is_occupied = false;
                self.current_index = None;
                self.state = GathererState::Idle;
            }
        }
    }

    fn looking_for_object(&mut self, map: &mut TileMap, gather_target: GatherTarget) {
        // reset this here because if an object that is currently being gathered is reselected, then
        // i need it to reset the timer so it doesnt just continue off from where it stopped.
        // this also just acts as a nice safeguard
        self.gather_timer.reset();
        self.data.path = NoPath;
        self.data.target_pos = None;

        let closest_obj: Option<ObjectEntry> =
            self.find_closest_target(&map.map_object_grid, gather_target);

        match closest_obj {
            Some(o) => {
                map.map_object_grid[o.idx].get_mut_data().is_occupied = true;
                self.current_index = Some(o.idx);
                self.state = MovingToObject {
                    target_pos: o.pos,
                    gather_target: gather_target,
                };
            }
            None => {
                self.current_index = None;
                self.object_indices.clear();
                self.state = GathererState::Idle
            }
        }
    }

    fn gather(&self, obj: &mut Object, game_context: &mut GameContext) -> bool {
        obj.take_hit(
            self.gathering_power,
            game_context,
            self.data.facing_direction,
        );

        if obj.should_not_be_used_again_by_anything() {
            obj.get_mut_data().is_marked_for_gathering = false;
            return true;
        }

        return false;
    }

    fn find_closest_target(
        &self,
        object_grid: &MapObjectGrid,
        target_obj: GatherTarget,
    ) -> Option<ObjectEntry> {
        let mut closest_obj: Option<ObjectEntry> = None;

        for idx in &self.object_indices {
            let obj = &object_grid[*idx];

            if let Object::NoObject = obj {
                continue;
            }

            if obj.should_not_be_used_again_by_anything() {
                continue;
            }

            if !Gatherer::obj_matches_target(obj, target_obj) {
                continue;
            }

            let obj_data = obj.get_data();

            match &closest_obj {
                Some(obj_entry) => {
                    let dist = obj_data.pos.distance_to(self.data.pos);

                    if dist < obj_entry.dist {
                        closest_obj = Some(ObjectEntry {
                            idx: *idx,
                            pos: obj_data.pos,
                            dist,
                        })
                    }
                }
                None => {
                    closest_obj = Some(ObjectEntry {
                        idx: *idx,
                        pos: obj_data.pos,
                        dist: obj_data.pos.distance_to(self.data.pos),
                    })
                }
            }
        }

        return closest_obj;
    }

    fn obj_matches_target(obj: &Object, target_obj: GatherTarget) -> bool {
        match target_obj {
            GatherTarget::Tree => {
                if let Object::TreeObj(tree) = obj {
                    if tree.data.is_marked_for_gathering && !tree.data.is_occupied {
                        return true;
                    }
                }
            }
            GatherTarget::Grass => {
                if let Object::GrassObj(grass) = obj {
                    if grass.data.is_marked_for_gathering && !grass.data.is_occupied {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    pub fn current_sprite(&self) -> Sprite {
        return GATHERER_IDLE_ANIM.frames[0];
    }
}
