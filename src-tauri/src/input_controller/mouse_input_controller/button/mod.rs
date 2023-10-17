use enigo::MouseButton;

use crate::input_controller::enigo_wrapper::EnigoTrait;

#[cfg(test)]
mod tests;

pub struct Button {
    enigo: Box<dyn EnigoTrait>,
    active_button: Option<MouseButton>,
}

impl Button {
    pub fn new(
        enigo: Box<dyn EnigoTrait>,
    ) -> Self {
        Self{
            enigo,
            active_button: None,
        }
    }
}

impl Button {
    pub fn key_down(&mut self, key: MouseButton) {
        self.active_button = Some(key);
        self.enigo.mouse_down(key);
    }

    pub fn key_up(&mut self) {
        if let Some(button) = self.active_button {
            self.enigo.mouse_up(button);
        }
        self.active_button = None;
    }
}
