use basic_raylib_core::graphics::sprite::Sprite;
use raylib::math::Vector2;

use crate::{
    GameContext, entities::{
        character::{Character, CharacterData, CharacterMovementResult}, characters::gatherer::{self, GathererState::MovingToObject}, object::Object,
    }, map::tile_map::{MapObjectGrid, TileMap}, utils::{map_utils::get_cell_at_cord, vector2_utils::v2_to_cord},
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
    },
    Gathering {
        object_index: usize,
    },
}

pub struct Gatherer {
    pub data: CharacterData,
    pub state: GathererState,
}

impl Gatherer {
    pub fn new(pos: Vector2) -> Character {
        let gatherer = Gatherer {
            data: CharacterData::new(pos, Vector2::zero(), 8.0, 8.0, 30.0),
            state: GathererState::Idle,
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
                        }
                    }
                    None => self.state = GathererState::Idle,
                }
            }
            GathererState::MovingToObject {
                target_pos,
                object_index,
            } => match self.data.move_to(target_pos, game_context, map) {
                CharacterMovementResult::Success => {
                    self.state = GathererState::Gathering {
                        object_index: object_index.unwrap(),
                    }
                }
                CharacterMovementResult::NotArrivedYet => (),
                CharacterMovementResult::NoRoute | CharacterMovementResult::TooLong => {
                    self.state = GathererState::Idle
                }
            },
            GathererState::Gathering { object_index } => {
                // if obj.gather( // obj.take_hit() ) {
                //      self.state = GathererState::LookingForTree
                // }
            }
        }
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
