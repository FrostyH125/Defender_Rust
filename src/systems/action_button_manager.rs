use std::collections::HashSet;

use raylib::{
    drawing::RaylibDrawHandle,
    math::Vector2,

};

use crate::{
    GameContext,
    entities::{character::Character, entity_manager::CharacterEntry, object::Object},
    map::tile_map::MapObjectGrid,
    systems::{
        action_buttons::action_button::{ActionButton, ActionButtonKind},
        entity_selecting_manager::EntitySelectingManager,
    },
    utils::{entity_utils::get_char_by_index},
};

pub struct ActionButtonManager {
    action_buttons: Vec<ActionButton>,
}

impl ActionButtonManager {
    pub fn new() -> Self {
        return Self {
            action_buttons: Vec::new(),
        };
    }

    pub fn clear_buttons(&mut self) {
        self.action_buttons.clear();
    }

    pub fn try_trigger_match(
        &mut self,
        game_context: &mut GameContext,
        button_tray_base_pos: Vector2,
        selector: &EntitySelectingManager,
        object_grid: &MapObjectGrid,
        chars: &mut [CharacterEntry],
    ) {
        self.clear_buttons();

        let mut successful_buttons = check_for_matches(
            object_grid,
            &selector.selected_objects,
            &selector.selected_characters,
            chars,
        );

        if successful_buttons.len() > 0 {
            const MARGIN_SIZE: f32 = 4.0;
            const BUTTON_SIZE: f32 = 16.0;
            const MAX_HEIGHT_OFFSET: f32 = 5.0;
            const START_HEIGHT_FROM_BASE: f32 = 8.0;

            // handle base case of there only being one button (in which case the game would otherwise crash
            // because when it adjusts the number of buttons, it divides by 0)
            if successful_buttons.len() == 1 {
                let b = &mut successful_buttons[0];
                let x = button_tray_base_pos.x - (BUTTON_SIZE / 2.0);
                let y = button_tray_base_pos.y
                    - (START_HEIGHT_FROM_BASE + MAX_HEIGHT_OFFSET + BUTTON_SIZE);
                b.rect.x = x;
                b.spawn_y_pos = y;
                b.rect.y = y;
                b.make_spawn_particles(game_context);
                self.action_buttons.push(successful_buttons.remove(0));
                return;
            }

            // subtract 1 because i want this to be accurate with current button
            // i dont want it to be len because if there were 3 buttons, len would be 3, but current button
            // starting at 1 would mean the first button is being counted as 33% of the progress, where
            // i would want button 1 (which would be 0) to be counted as 0%, 2 (which would be 1) to be
            // 50%, and 2 as 100%
            let num_of_buttons_adjusted = successful_buttons.len() - 1;
            let mut current_button = 0;

            let button_qty_f32 = successful_buttons.len() as f32;

            // total size of buttons + total size of margins
            let tray_total_width =
                button_qty_f32 * BUTTON_SIZE + MARGIN_SIZE * (button_qty_f32 - 1.0);
            let mut x = button_tray_base_pos.x - (tray_total_width / 2.0);
            let mut sin_offset = 0.0;

            for mut b in successful_buttons {
                let mut current_progress = current_button as f32 / num_of_buttons_adjusted as f32;

                // making it so that after the halfway point it comes back down again
                if current_progress >= 0.5 {
                    current_progress = 1.0 - current_progress;
                }

                // since otherwise it would only be between 0.0..=0.5
                current_progress *= 2.0;

                let y = button_tray_base_pos.y
                    - (START_HEIGHT_FROM_BASE
                        + (current_progress * MAX_HEIGHT_OFFSET)
                        + BUTTON_SIZE);
                b.rect.x = x;
                b.spawn_y_pos = y;
                b.rect.y = y;
                b.sin_offset = sin_offset;

                b.make_spawn_particles(game_context);

                self.action_buttons.push(b);

                current_button += 1;
                sin_offset += 0.5;
                x += BUTTON_SIZE + MARGIN_SIZE;
            }
        }
    }

    pub fn update(&mut self, game_context: &mut GameContext, selector: &EntitySelectingManager, object_grid: &mut MapObjectGrid, chars: &mut [CharacterEntry]) {
        for b in &mut self.action_buttons {
            b.update(game_context);
            if b.is_hovering && game_context.input_state.left_clicked_once {
                b.on_click(&selector.selected_objects, &selector.selected_characters, object_grid, chars);
                b.make_pop_particles(&mut game_context.particle_system);
            }
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, game_context: &GameContext) {
        for b in &self.action_buttons {
            b.draw(d, &game_context.texture);
        }
    }

    pub fn check_for_buttons_being_hovered(&self) -> bool {
        let mut button_hovering = false;

        for b in &self.action_buttons {
            if b.is_hovering {
                button_hovering = true;
            }
        }

        return button_hovering;
    }
}

fn check_for_matches(
    object_grid: &MapObjectGrid,
    obj_ids: &[usize],
    char_ids: &[usize],
    character_entries: &mut [CharacterEntry],
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

    for c_id in char_ids {
        let character = get_char_by_index(character_entries, *c_id);

        let value = match character.character {
            Character::GathererChar(_) => CharacterKind::Gatherer,
        };

        char_types.insert(value);
    }

    for o in &obj_types {
        for c in &char_types {
            let button: Option<ActionButton> = match (*o, *c) {
                (ObjectKind::Tree, CharacterKind::Gatherer) => {
                    Some(ActionButton::new(ActionButtonKind::ChopButton))
                }
                _ => None,
            };

            if let Some(action_button) = button {
                successful_matches.push(action_button);
            }
        }
    }

    return successful_matches;
}
