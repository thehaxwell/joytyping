use enigo::*;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait EnigoTrait {
    fn key_click(&mut self, key: Key);
    fn key_down(&mut self, key: Key);
    fn key_up(&mut self, key: Key);
    fn mouse_move_relative(&mut self, x: i32, y: i32);
    fn mouse_down(&mut self, button: MouseButton);
    fn mouse_up(&mut self, button: MouseButton);
    fn mouse_scroll_x(&mut self, length: i32);
    fn mouse_scroll_y(&mut self, length: i32);
}

pub struct EnigoWrapper {
    enigo: Enigo,
}

impl EnigoWrapper {
    pub fn new() -> Self {
        Self {
            enigo: Enigo::new(),
        }
    }
}

impl EnigoTrait for EnigoWrapper {
    fn key_click(&mut self, key: Key) {
        self.enigo.key_click(key);
    }

    fn key_up(&mut self, key:Key) {
        self.enigo.key_up(key);
    }

    fn key_down(&mut self,key:Key) {
        self.enigo.key_down(key);
    }

    fn mouse_move_relative(&mut self, x: i32, y: i32){
        self.enigo.mouse_move_relative(x,y);
    }

    fn mouse_down(&mut self, button: MouseButton){
        self.enigo.mouse_down(button);
    }

    fn mouse_up(&mut self, button: MouseButton){
        self.enigo.mouse_up(button);
    }

    fn mouse_scroll_x(&mut self, length: i32){
        self.enigo.mouse_scroll_x(length);
    }

    fn mouse_scroll_y(&mut self, length: i32){
        self.enigo.mouse_scroll_y(length);
    }
}
