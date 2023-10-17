use crate::input_controller::enigo_wrapper::EnigoTrait;

use super::MouseCardinalLeverInputControllerTrait;

#[cfg(test)]
mod tests;

pub struct Cursor {
    enigo: Box<dyn EnigoTrait>,
    mouse_cursor_x_move: i32,
    mouse_cursor_y_move: i32,
}

impl Cursor {
    pub fn new(
        enigo: Box<dyn EnigoTrait>,
    ) -> Self {
        Self{
            enigo,
            mouse_cursor_x_move: 0,
            mouse_cursor_y_move: 0,
        }
    }
}

impl MouseCardinalLeverInputControllerTrait for Cursor {
    fn trigger_input(&mut self) {
        if self.mouse_cursor_x_move != 0
            || self.mouse_cursor_y_move != 0 {
            self.enigo.mouse_move_relative(
                self.mouse_cursor_x_move,
                self.mouse_cursor_y_move,
            );
        }
    }

    fn set_x_and_y_axes(&mut self, x_value: i32, y_value: i32){
        self.mouse_cursor_x_move = x_value;
        self.mouse_cursor_y_move = y_value;
    }
}
