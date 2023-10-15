#[cfg(test)]
use mockall::{automock, predicate::*};

pub mod scroll;
pub mod cursor;

#[cfg_attr(test, automock)]
pub trait MouseInputControllerTrait {
    fn trigger_input(&mut self);
    fn set_x_and_y_axes(&mut self, x_value: i32, y_value: i32);
}


