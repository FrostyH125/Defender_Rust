use basic_raylib_core::system::input_handler::InputState;
use raylib::{color::Color, drawing::{RaylibDraw, RaylibDrawHandle}, math::{Rectangle, Vector2}};

use crate::{GameContext, utils::mouse_utils::{self, mouse_world_coords}};

pub struct SelectRect {
    origin_pos: Vector2,
    pub rectangle: Rectangle,
    origin_set: bool,
    pub is_dragging: bool,
    pub is_selecting_this_frame: bool,
    pub select_range_active: bool,
}


impl SelectRect {
    pub fn new() -> Self {
        SelectRect { origin_pos: Vector2::zero(), rectangle: Rectangle::default(), origin_set: false, is_selecting_this_frame: false, is_dragging: false, select_range_active: false }
    }
    
    pub fn update(&mut self, game_context: &GameContext) {
        self.is_dragging = game_context.input_state.left_dragging;
        

        self.is_selecting_this_frame = false;
        
        if self.is_dragging {
            let mouse_pos = mouse_world_coords(game_context);
            
            if !self.origin_set {
                self.origin_pos = mouse_pos;
                self.origin_set = true;
            }

            let mut rect_width = mouse_pos.x - self.origin_pos.x;
            let mut rect_height = mouse_pos.y - self.origin_pos.y;

            let rect_x = if rect_width < 0.0 {
                rect_width = rect_width.abs();
                mouse_pos.x
            } else {
                self.origin_pos.x
            };

            let rect_y = if rect_height < 0.0 {
                rect_height = rect_height.abs();
                mouse_pos.y
            } else {
                self.origin_pos.y
            };

            self.rectangle.x = rect_x;
            self.rectangle.y = rect_y;
            self.rectangle.width = rect_width;
            self.rectangle.height = rect_height;
        }

        // wont trigger if self.is_selecting_this_frame does
        if game_context.input_state.left_stopped_dragging_this_frame {
            self.is_selecting_this_frame = true;
            self.origin_set = false;
        }
        
        self.select_range_active = self.is_dragging || self.is_selecting_this_frame;
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        if !self.is_dragging {
            return;
        }
        d.draw_rectangle_lines_ex(self.rectangle, 2.0, Color::WHITE);
    }
}