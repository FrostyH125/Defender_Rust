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
        object_index: Option<usize>,
        gather_target: Option<GatherTarget>,
    },
    Gathering {
        object_index: usize,
        gather_target: GatherTarget,
    },
}

pub struct Gatherer {
    pub data: CharacterData,
    pub state: GathererState,
    gathering_power: f32,
    gather_timer: Timer
}

impl Gatherer {
    pub fn new(pos: Vector2) -> Character {
        let gatherer = Gatherer {
            data: CharacterData::new(pos, Vector2::zero(), 8.0, 8.0, 30.0),
            state: GathererState::Idle,
            gathering_power: 20.0,
            gather_timer: Timer::new(2.0)
        };

        return Character::GathererChar(gatherer);
    }

    pub fn update(&mut self, game_context: &mut GameContext, map: &mut TileMap) {
        match self.state {
            GathererState::Idle => (),
            GathererState::LookingForObject(gather_target) => {
                // first look within the same block as you
                let check_cells = map.get_3_x_3_cell_grid(v2_to_cord(self.data.pos));

                let closest_obj: Option<ObjectEntry> =
                    self.find_closest_target(&map.map_object_grid, check_cells, gather_target);

                match closest_obj {
                    Some(t) => {
                        map.map_object_grid[t.idx].get_mut_data().is_occupied = true;
                        self.state = MovingToObject {
                            target_pos: t.pos,
                            object_index: Some(t.idx),
                            gather_target: Some(gather_target),
                        }
                    }
                    None => self.state = GathererState::Idle,
                }
            }
            GathererState::MovingToObject {
                target_pos,
                object_index,
                gather_target,
            } => match self.data.move_to(target_pos, game_context, map) {
                CharacterMovementResult::Success => {
                    // gather the object if it exists, otherwise, it was just a normal move
                    self.state = match gather_target {
                        Some(g_t) => GathererState::Gathering { object_index: object_index.unwrap(), gather_target: g_t },
                        None => GathererState::Idle,
                    };     
                }
                CharacterMovementResult::NotArrivedYet => (),
                CharacterMovementResult::NoRoute | CharacterMovementResult::TooLong => {
                    self.state = GathererState::Idle
                }
            },
            GathererState::Gathering {
                object_index,
                gather_target,
            } => {
                self.gather_timer.track(game_context.dt);

                if self.gather_timer.is_done() {
                    self.gather_timer.reset();
                    
                    if self.gather(&mut map.map_object_grid[object_index]) {
                        self.state = GathererState::LookingForObject(gather_target);
                    }
                }
            }
        }
    }

    fn gather(&self, obj: &mut Object) -> bool {
        obj.take_hit(self.gathering_power);

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

                match &closest_obj {
                    Some(obj_entry) => {
                        let dist = obj_data.pos.distance_to(self.data.pos);

                        // only add if its closer
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
            GatherTarget::Grass => todo!(),
        }

        return false;
    }

    pub fn sprite(&self) -> &Sprite {
        return &GATHERER_SPRITE;
    }
}
