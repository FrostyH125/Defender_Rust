use std::collections::HashMap;

use raylib::{color::Color, math::{Vector2, Vector3}};

pub struct Light {
    pub position: Vector3,
    pub color: Vector3,
    pub intensity: f32,
    pub radius: f32
}

pub struct Lights {
    pub all_lights: HashMap<usize, Light>,
    next_id: usize
}

impl Lights {
    pub fn new() -> Self {
        return Self {
            next_id: 0,
            all_lights: HashMap::new()
        }
    }

    

    /// returns an id for the light created, the only way to
    /// get rid of this light is to remove it with this id.
    /// note: the color parameter only reads the rgb values
    /// note: the id cannot be ignored, it must be assigned to a variable
    #[must_use = "The only way that a light can be removed is with the original ID"]
    pub fn add_light(&mut self, position: Vector3, color: Color, intensity: f32, radius: f32) -> usize {

        let normalized_color = color.color_normalize();
        let final_color = Vector3::new(normalized_color.x, normalized_color.y, normalized_color.z);
        
        let light = Light {
            position,
            color: final_color,
            intensity,
            radius,
        };

        let id = self.next_id;

        self.all_lights.insert(id, light);
        
        self.next_id += 1;
        return id;
    }

    pub fn set_light_pos(&mut self, id: usize, new_pos: Vector3) {
        self.all_lights.entry(id).and_modify(|l| l.position = new_pos);
    }

    pub fn set_light_pos_no_z(&mut self, id: usize, new_pos_xy: Vector2) {
        self.all_lights.entry(id).and_modify(|l| {
            l.position.x = new_pos_xy.x;
            l.position.y = new_pos_xy.y;
        });
    }

    pub fn remove_light(&mut self, id: usize) {
        self.all_lights.remove(&id);
    }
}