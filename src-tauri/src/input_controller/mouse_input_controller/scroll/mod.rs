use crate::input_controller::enigo_wrapper::EnigoTrait;

use super::MouseCardinalLeverInputControllerTrait;

#[cfg(test)]
mod tests;

pub struct Scroll {
    enigo: Box<dyn EnigoTrait>,
    mouse_scroll_x_move: i32,
    mouse_scroll_y_move: i32,
    boost_multiplier: Option<u32>,
}

impl Scroll {
    pub fn new(
        enigo: Box<dyn EnigoTrait>,
    ) -> Self {
        Self{
            enigo,
            mouse_scroll_x_move: 0,
            mouse_scroll_y_move: 0,
            boost_multiplier: None,
        }
    }
}

impl MouseCardinalLeverInputControllerTrait for Scroll {
    fn trigger_input(&mut self) {
        if self.mouse_scroll_x_move != 0 {
            self.enigo.mouse_scroll_x(
                self.mouse_scroll_x_move 
                    * self.boost_multiplier.unwrap_or(1) as i32
            );
        }
        if self.mouse_scroll_y_move != 0 {
            self.enigo.mouse_scroll_y(
                self.mouse_scroll_y_move
                    * self.boost_multiplier.unwrap_or(1) as i32
            );
        }
    }

    fn set_x_and_y_axes(&mut self, x_value: i32, y_value: i32){
        self.mouse_scroll_x_move = x_value;
        self.mouse_scroll_y_move = y_value;
    }

    fn start_boost(&mut self,multiplier: u32) {
        self.boost_multiplier = Some(multiplier);
    }

    fn end_boost(&mut self){
        self.boost_multiplier = None;
    }
}
