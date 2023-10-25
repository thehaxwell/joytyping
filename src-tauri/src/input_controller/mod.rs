use crate::gamepad::InputEvent;

use self::keyboard_input_controller::KeyboardInputControllerTrait;

pub mod keyboard_input_controller;
pub mod mouse_input_controller;
pub mod enigo_wrapper;

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

    pub fn trigger_input(&mut self){
        self.keyboard_input_controller.trigger_input();
        self.mouse_cursor_input_controller.trigger_input();
        self.mouse_scroll_input_controller.trigger_input();
    }

    pub fn react_to_gamepad_event(&mut self, event: InputEvent) {
        match event {
            InputEvent::KeyClick(key) => {
                self.keyboard_input_controller.key_up();
                self.mouse_button_input_controller.key_up();
                self.keyboard_input_controller.key_click(key);
            }
            InputEvent::KeyDown(key) => {
                self.keyboard_input_controller.key_up();
                self.mouse_button_input_controller.key_up();
                self.keyboard_input_controller.key_down(key);
            }
            InputEvent::MouseDown(key) => {
                self.keyboard_input_controller.key_up();
                self.mouse_button_input_controller.key_up();
                self.mouse_button_input_controller.key_down(key);
            }
            InputEvent::KeyUp => {
                self.keyboard_input_controller.key_up();
                self.mouse_button_input_controller.key_up();
                self.mouse_cursor_input_controller.end_boost();
            }
            InputEvent::MoveMouseCursor(x,y) => {
                self.mouse_cursor_input_controller.set_x_and_y_axes(x,y);
            }
            InputEvent::BoostMouseCursor(multiplier) => {
                self.mouse_cursor_input_controller.start_boost(multiplier);
            }
            InputEvent::MouseScroll(x,y) => {
                self.mouse_scroll_input_controller.set_x_and_y_axes(x,y);
            }
        }    
    }
}
