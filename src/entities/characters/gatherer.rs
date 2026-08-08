use basic_raylib_core::{graphics::sprite::Sprite, system::timer::Timer};
use raylib::math::Vector2;

use crate::{
    GameContext,
    entities::{
        character::{Character, CharacterData, CharacterMovementResult},
        characters::gatherer::GathererState::MovingToObject,
        object::{Object, ObjectState},
    },
    map::tile_map::{MapObjectGrid, TileMap},
    utils::vector2_utils::v2_to_cord,
};

pub static GATHERER_SPRITE: Sprite = Sprite::new(16, 72, 8, 8);

struct ObjectEntry {
    idx: usize,
    pos: Vector2,
    dist: f32,
}

#[derive(Clone, Copy)]
pub enum GatherTarget {
    Tree,
    Grass,
}

pub enum GathererState {
    Idle,
    LookingForObject(GatherTarget),
    MovingToObject {
        target_pos: Vector2,
        object_index: usize,
        gather_target: GatherTarget,
    },
    GatheringObject {
        object_index: usize,
        gather_target: GatherTarget,
    },
}

impl std::fmt::Debug for GathererState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Idle => write!(f, "Idle"),
            Self::LookingForObject(_) => write!(f, "Looking for Object"),
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
}

impl Gatherer {
    pub fn new(pos: Vector2) -> Character {
        let gatherer = Gatherer {
            data: CharacterData::new(pos, Vector2::zero(), 8.0, 8.0, 30.0),
            state: GathererState::Idle,
            gathering_power: 20.0,
            gather_timer: Timer::new(2.0),
        };

        return Character::GathererChar(gatherer);
    }

    pub fn update(&mut self, game_context: &mut GameContext, map: &mut TileMap) {
        match self.state {
            GathererState::Idle => (),
            GathererState::LookingForObject(gather_target) => {
                self.looking_for_object(map, gather_target)
            }

            GathererState::MovingToObject {
                target_pos,
                object_index,
                gather_target,
            } => {
                self.moving_to_object(game_context, map, target_pos, object_index, gather_target);
            }
            GathererState::GatheringObject {
                object_index,
                gather_target,
            } => {
                self.gathering_object(game_context, map, object_index, gather_target);
            }
        }
    }

    fn gathering_object(
        &mut self,
        game_context: &mut GameContext,
        map: &mut TileMap,
        object_index: usize,
        gather_target: GatherTarget,
    ) {
        self.gather_timer.track(game_context.dt);

        if self.gather_timer.is_done() {
            self.gather_timer.reset();

            if self.gather(&mut map.map_object_grid[object_index]) {
                self.state = GathererState::LookingForObject(gather_target);
            }
        }
    }

    fn moving_to_object(
        &mut self,
        game_context: &mut GameContext,
        map: &mut TileMap,
        target_pos: Vector2,
        object_index: usize,
        gather_target: GatherTarget,
    ) {
        map.map_object_grid[object_index].get_mut_data().is_occupied = true;

        match self.data.move_to(target_pos, game_context, map) {
            CharacterMovementResult::Success => {
                self.state = GathererState::GatheringObject {
                        object_index: object_index,
                        gather_target: gather_target,
                };
            }
            CharacterMovementResult::NotArrivedYet => (),
            CharacterMovementResult::NoRoute | CharacterMovementResult::TooLong => {
                self.state = GathererState::Idle;
            }
        }
    }

    fn looking_for_object(&mut self, map: &mut TileMap, gather_target: GatherTarget) {
        // reset this here because if an object that is currently being gathered is reselected, then
        // i need it to reset the timer so it doesnt just continue off from where it stopped.
        // this also just acts as a nice safeguard
        self.gather_timer.reset();

        let check_cells = map.get_3_x_3_cell_grid(v2_to_cord(self.data.pos));

        let closest_obj: Option<ObjectEntry> =
            self.find_closest_target(&map.map_object_grid, check_cells, gather_target);

        match closest_obj {
            Some(t) => {
                map.map_object_grid[t.idx].get_mut_data().is_occupied = true;
                self.state = MovingToObject {
                    target_pos: t.pos,
                    object_index: t.idx,
                    gather_target: gather_target,
                }
            }
            None => self.state = GathererState::Idle,
        }
    }

    fn gather(&self, obj: &mut Object) -> bool {
        obj.take_hit(self.gathering_power);
        obj.get_mut_data().is_occupied = true;
        return obj.get_data().state == ObjectState::Breaking;
    }

    fn find_closest_target(
        &self,
        object_grid: &MapObjectGrid,
        check_cells: Vec<&crate::map::map_cell::MapCell>,
        target_obj: GatherTarget,
    ) -> Option<ObjectEntry> {
        let mut closest_obj: Option<ObjectEntry> = None;

        for cell in check_cells {
            // check each object in that cell
            for idx in &cell.objects_in_cell {
                let obj = &object_grid[*idx];
                if !Gatherer::obj_matches_target(obj, target_obj) {
                    continue;
                }
                
                let obj_data = obj.get_data();

                // since objects reset their is_occupied variable each frame
                // the object its looking at could potentially fit the
                // requirements even though its breaking and would turn to NoObject
                // soon
                // this would crash the game since the character would then
                // try to get the data from the NoObject
                if let ObjectState::Breaking = obj_data.state {
                    continue;
                }

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
            GatherTarget::Grass => todo!("grass not yet implemented for collecting"),
        }

        return false;
    }

    pub fn sprite(&self) -> Sprite {
        return GATHERER_SPRITE;
    }
}
