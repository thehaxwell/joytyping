use crate::settings::models::layout::KeyboardInput;

#[cfg(test)]
use mockall::{automock, predicate::*};

use super::enigo_wrapper::EnigoTrait;

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
        self.active_key = Some(key_to_click);
        self.trigger_input();
    }

    fn key_up(&mut self) {
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

