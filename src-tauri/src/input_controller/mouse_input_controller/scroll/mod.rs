use crate::input_controller::enigo_wrapper::EnigoTrait;

use super::MouseInputControllerTrait;

#[cfg(test)]
mod tests;

pub struct Scroll {
    enigo: Box<dyn EnigoTrait>,
    mouse_scroll_x_move: i32,
    mouse_scroll_y_move: i32,
}

impl Scroll {
    pub fn new(
        enigo: Box<dyn EnigoTrait>,
    ) -> Self {
        Self{
            enigo,
            mouse_scroll_x_move: 0,
            mouse_scroll_y_move: 0,
        }
    }
}

impl MouseInputControllerTrait for Scroll {
    fn trigger_input(&mut self) {
        if self.mouse_scroll_x_move != 0 {
            self.enigo.mouse_scroll_x(
                self.mouse_scroll_x_move,
            );
        }
        if self.mouse_scroll_y_move != 0 {
            self.enigo.mouse_scroll_y(
                self.mouse_scroll_y_move,
            );
        }
    }

    fn set_x_and_y_axes(&mut self, x_value: i32, y_value: i32){
        self.mouse_scroll_x_move = x_value;
        self.mouse_scroll_y_move = y_value;
    }
}
