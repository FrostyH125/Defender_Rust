use std::collections::HashSet;

use raylib::{drawing::RaylibDrawHandle, math::Rectangle, texture::Texture2D};

use crate::{
    GameContext, entities::{character::Character, entity_manager::CharacterEntry, object::Object}, map::tile_map::MapObjectGrid, systems::action_buttons::action_button::{ActionButton, ActionButtonKind}, utils::mouse_utils::mouse_world_coords,
};

// struct with list action buttons
// ABM::update()
// ABM::draw()

pub struct ActionButtonManager {
    action_buttons: Vec<ActionButton>,
}

impl ActionButtonManager {

    pub fn trigger_match(&mut self, object_grid: &MapObjectGrid ,obj_ids: &[usize], chars: &[&mut CharacterEntry]) {

        self.action_buttons.clear();
        
        let successful_buttons = check_for_matches(object_grid, obj_ids, chars);
        
        if successful_buttons.len() > 0 {

            todo!("set button positions and sin stuff");
            
            for b in successful_buttons {
                self.action_buttons.push(b);
            }
        }
    }
    
    pub fn update(&mut self, dt: f32) {
        for b in &mut self.action_buttons {
            b.update(dt);
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, game_context: &GameContext) {    
        for b in &self.action_buttons {
            let hover = b.hover_rect.check_collision_point_rec(mouse_world_coords(game_context));
            
            b.draw(d, &game_context.texture, hover);
        }
    }
}

pub fn check_for_matches(
    object_grid: &MapObjectGrid,
    obj_ids: &[usize],
    chars: &[&mut CharacterEntry],
) -> Vec<ActionButton> {
    #[derive(Hash, Eq, PartialEq, Clone, Copy)]
    enum ObjectKind {
        Tree,
        Grass,
    }

    #[derive(Hash, Eq, PartialEq, Clone, Copy)]
    enum CharacterKind {
        Gatherer,
    }

    let mut successful_matches = Vec::new();
    let mut obj_types: HashSet<ObjectKind> = HashSet::new();
    let mut char_types: HashSet<CharacterKind> = HashSet::new();

    for id in obj_ids {
        let obj = &object_grid[*id];

        if let Object::NoObject = obj {
            continue;
        }

        let value = match obj {
            Object::NoObject => panic!("there should be no reason for you to be here"),
            Object::TreeObj(_) => ObjectKind::Tree,
            Object::GrassObj(_) => ObjectKind::Grass,
        };

        obj_types.insert(value);
    }

    for c in chars {
        let value = match c.character {
            Character::GathererChar(_) => CharacterKind::Gatherer,
        };
        char_types.insert(value);
    }

    for o in &obj_types {
        for c in &char_types {
            let button: Option<ActionButton> = match (*o, *c) {
                (ObjectKind::Tree, CharacterKind::Gatherer) => Some(ActionButton::new(ActionButtonKind::ChopButton)),
                _ => None,
            };

            if let Some(action_button) = button {
                successful_matches.push(action_button);
            }
        }
    }

    return successful_matches;
}
