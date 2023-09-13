use std::time::Duration;
use crate::settings_data::KeyboardInput;

use self::enigo_wrapper::EnigoTrait;

pub mod enigo_wrapper;

#[cfg(test)]
use mockall::{automock, predicate::*};

// TODO: redefine these tests, the effect mentioned
// there is now achieved by the natural wait between
// click/double-click and click-and-hold/double-click-and-hold
// #[cfg(test)]
// mod tests;

#[cfg_attr(test, automock)]
pub trait KeyboardInputControllerTrait {
    fn key_down(&mut self, key_to_click: KeyboardInput);
    fn key_up(&mut self);
    fn key_click(&mut self, key_to_click: KeyboardInput);
    fn trigger_input(&mut self);
}

pub struct KeyboardInputController {
    enigo: Box<dyn EnigoTrait>,
    active_key: Option<KeyboardInput>,
}

impl KeyboardInputController {
    pub fn new(
        enigo: Box<dyn EnigoTrait>,
    ) -> Self {
        Self{
            enigo,
            active_key: None,
        }
    }

    fn click(&mut self, key: KeyboardInput) {
        for modifier in key.modifiers.as_slice() {
            self.enigo.key_down(*modifier);
        }

        self.enigo.key_click(key.key);

        for modifier in key.modifiers.as_slice() {
            self.enigo.key_up(*modifier);
        }
    }
}

impl KeyboardInputControllerTrait for KeyboardInputController {
    // The key is immediately triggered and then 
    // it will be allowed to be triggered many times by calling trigger_input()
    // until key_up() is called (or key_down() is called again).
    fn key_down(&mut self, key_to_click: KeyboardInput) {
        println!("KeyboardInputController::key_down({:?})",key_to_click);
        self.active_key = Some(key_to_click);
        self.trigger_input();
    }

    fn key_up(&mut self) {
        println!("KeyboardInputController::key_up");
        self.active_key = None;
    }

    fn trigger_input(&mut self) {
        if let Some(active_key) = &self.active_key {
            self.click(active_key.clone());
        }
    }

    fn key_click(&mut self, key_to_click: KeyboardInput) {
        self.click(key_to_click);
    }
}


#[cfg_attr(test, automock)]
pub trait MouseInputControllerTrait {
    fn key_down(&mut self, key_to_click: enigo::MouseButton);
    fn key_up(&mut self);
    fn trigger_input(&mut self);
    fn set_mouse_cursor_x_axis(&mut self, value: Option<i32>);
    fn set_mouse_cursor_y_axis(&mut self, value: Option<i32>);
}

pub struct MouseInputController {
    enigo: Box<dyn EnigoTrait>,
    active_key: Option<enigo::MouseButton>,
    mouse_cursor_x_move: Option<i32>,
    mouse_cursor_y_move: Option<i32>,
}

impl MouseInputController {
    pub fn new(
        enigo: Box<dyn EnigoTrait>,
    ) -> Self {
        Self{
            enigo,
            active_key: None,
            mouse_cursor_x_move: None,
            mouse_cursor_y_move: None,
        }
    }
}

impl MouseInputControllerTrait for MouseInputController {
    fn key_down(&mut self, key_to_click: enigo::MouseButton) {
        self.active_key = Some(key_to_click);
        self.enigo.mouse_down(key_to_click);
    }

    fn key_up(&mut self) {
        if let Some(active_key) = self.active_key {
            self.enigo.mouse_up(active_key);
        }
        self.active_key = None;
    }

    fn trigger_input(&mut self) {
        if self.mouse_cursor_x_move.is_some()
            || self.mouse_cursor_y_move.is_some(){
            self.enigo.mouse_move_relative(
                self.mouse_cursor_x_move.unwrap_or(0),
                self.mouse_cursor_y_move.unwrap_or(0)
            );
        }
    }

    fn set_mouse_cursor_x_axis(&mut self, value: Option<i32>){
        self.mouse_cursor_x_move = value;
    }
   
    fn set_mouse_cursor_y_axis(&mut self, value: Option<i32>){
        self.mouse_cursor_y_move = value;
    }
}
