use std::time::Duration;
use crate::settings_data::KeyboardInput;

use self::enigo_wrapper::EnigoTrait;

pub mod enigo_wrapper;

#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg(test)]
mod tests;

const DELAY_DURATION: Duration = Duration::from_millis(500);

#[cfg_attr(test, automock)]
pub trait KeyboardInputControllerTrait {
    fn key_down(&mut self, key_to_click: KeyboardInput);
    fn key_up(&mut self);
    fn trigger_input(&mut self);
}

pub struct KeyboardInputController {
    enigo: Box<dyn EnigoTrait>,
    active_key: Option<KeyboardInput>,
    instant_to_start_delay_from: Option<std::time::Instant>,
}

impl KeyboardInputController {
    pub fn new(
        enigo: Box<dyn EnigoTrait>,
    ) -> Self {
        Self{
            enigo,
            active_key: None,
            instant_to_start_delay_from: None,
        }
    }

    fn allow_input(&self) -> bool {
        if let Some(instant) = self.instant_to_start_delay_from {
            if instant.elapsed() <= DELAY_DURATION {
                return false;
            }
        }
        true
    }

}

impl KeyboardInputControllerTrait for KeyboardInputController {
    // The key is immediately triggered and then after DELAY_DURATION
    // it will be allowed to be triggered many times by calling trigger_input()
    // until key_up() is called (or key_down() is called again).
    fn key_down(&mut self, key_to_click: KeyboardInput) {
        self.active_key = Some(key_to_click);
        self.instant_to_start_delay_from = None;
        self.trigger_input();
        self.instant_to_start_delay_from = Some(std::time::Instant::now());
    }

    fn key_up(&mut self) {
        self.active_key = None;
        self.instant_to_start_delay_from = None;
    }

    fn trigger_input(&mut self) {
        if let Some(active_key) = &self.active_key {
            if !self.allow_input() {
                return;
            }

            for modifier in active_key.modifiers.as_slice() {
                    println!("-> {:?}",modifier);
                    self.enigo.key_down(*modifier);
            }

            println!("-> {:?}",active_key.key);
            self.enigo.key_click(active_key.key);

            for modifier in active_key.modifiers.as_slice() {
                    println!("-> {:?}",modifier);
                    self.enigo.key_up(*modifier);
            }
            self.instant_to_start_delay_from = None;
        }
    }
}

//
// #[cfg_attr(test, automock)]
// pub trait MouseInputControllerTrait {
//     fn key_down(&mut self, key_to_click: MouseInput);
//     fn key_up(&mut self);
//     fn trigger_input(&mut self);
//     fn update_mouse_cursor_x_axis(&mut self, value: Option<i32>);
//     fn update_mouse_cursor_y_axis(&mut self, value: Option<i32>);
// }
//
// pub struct MouseInputController {
//     enigo: Box<dyn EnigoTrait>,
//     active_key: Option<MouseInput>,
//     mouse_cursor_x_move: Option<i32>,
//     mouse_cursor_y_move: Option<i32>,
// }
//
// impl MouseInputController {
//     pub fn new(
//         enigo: Box<dyn EnigoTrait>,
//     ) -> Self {
//         Self{
//             enigo,
//             active_key: None,
//             mouse_cursor_x_move: None,
//             mouse_cursor_y_move: None,
//         }
//     }
// }
//
// impl MouseInputControllerTrait for MouseInputController {
//     fn key_down(&mut self, key_to_click: MouseInput) {
//         self.active_key = Some(key_to_click);
//         self.enigo.mouse_down(self.active_key);
//     }
//
//     fn key_up(&mut self) {
//         self.enigo.mouse_up(self.active_key);
//         self.active_key = None;
//     }
//
//     fn trigger_input(&mut self) {
//         if self.mouse_cursor_x_move.is_some()
//             || self.mouse_cursor_y_move.is_some(){
//             self.enigo.mouse_move_relative(
//                 self.mouse_cursor_x_move.unwrap_or(0),
//                 self.mouse_cursor_y_move.unwrap_or(0)
//             );
//         }
//     }
//
//     fn update_mouse_cursor_x_axis(&mut self, value: Option<i32>){
//         self.mouse_cursor_x_move = value;
//     }
//    
//     fn update_mouse_cursor_y_axis(&mut self, value: Option<i32>){
//         self.mouse_cursor_y_move = value;
//     }
// }