use enigo::MouseButton;

use crate::settings::models::layout::KeyboardInput;

use self::keyboard_input_controller::KeyboardInputControllerTrait;

pub mod keyboard_input_controller;
pub mod mouse_input_controller;
pub mod enigo_wrapper;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait InputControllerTrait {
    fn key_click(&mut self, key: KeyboardInput);
    fn key_down(&mut self, key: KeyboardInput);
    fn mouse_down(&mut self, key: MouseButton);
    fn key_up(&mut self);
    fn move_mouse_cursor(&mut self, x: i32, y: i32);
    fn boost_mouse_cursor(&mut self, multiplier: u32);
    fn mouse_scroll(&mut self, x: i32, y: i32);

    fn trigger_input(&mut self);
}

pub struct InputController {
    keyboard_input_controller: Box<dyn KeyboardInputControllerTrait>,
    mouse_cursor_input_controller: Box<dyn mouse_input_controller::MouseCardinalLeverInputControllerTrait>,
    mouse_scroll_input_controller: Box<dyn mouse_input_controller::MouseCardinalLeverInputControllerTrait>,
    mouse_button_input_controller: Box<dyn mouse_input_controller::button::ButtonTrait>,
}

impl InputController {
    pub fn new(
        keyboard_input_controller: Box<dyn KeyboardInputControllerTrait>,
        mouse_cursor_input_controller: Box<dyn mouse_input_controller::MouseCardinalLeverInputControllerTrait>,
        mouse_scroll_input_controller: Box<dyn mouse_input_controller::MouseCardinalLeverInputControllerTrait>,
        mouse_button_input_controller: Box<dyn mouse_input_controller::button::ButtonTrait>,
    ) -> Self {
        Self {
            keyboard_input_controller,
            mouse_cursor_input_controller,
            mouse_scroll_input_controller,
            mouse_button_input_controller,
        }
    }
}

impl InputControllerTrait for InputController {
    fn key_click(&mut self, key: KeyboardInput){
        self.keyboard_input_controller.key_up();
        self.mouse_button_input_controller.key_up();
        self.keyboard_input_controller.key_click(key);
	}

    fn key_down(&mut self, key: KeyboardInput){
        self.keyboard_input_controller.key_up();
        self.mouse_button_input_controller.key_up();
        self.keyboard_input_controller.key_down(key);
	}

    fn mouse_down(&mut self, key: MouseButton){
        self.keyboard_input_controller.key_up();
        self.mouse_button_input_controller.key_up();
        self.mouse_button_input_controller.key_down(key);
	}
    fn key_up(&mut self){
        self.keyboard_input_controller.key_up();
        self.mouse_button_input_controller.key_up();
        self.mouse_cursor_input_controller.end_boost();
	}
    fn move_mouse_cursor(&mut self, x: i32, y: i32){
        self.mouse_cursor_input_controller.set_x_and_y_axes(x,y);
	}
    fn boost_mouse_cursor(&mut self, multiplier: u32){
        self.mouse_cursor_input_controller.start_boost(multiplier);
	}
    fn mouse_scroll(&mut self, x: i32, y: i32){
        self.mouse_scroll_input_controller.set_x_and_y_axes(x,y);
	}

    fn trigger_input(&mut self){
        self.keyboard_input_controller.trigger_input();
        self.mouse_cursor_input_controller.trigger_input();
        self.mouse_scroll_input_controller.trigger_input();
    }
}
