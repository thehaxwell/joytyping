#[cfg(test)]
use mockall::{automock, predicate::*};

pub mod scroll;
pub mod cursor;
pub mod button;

#[cfg_attr(test, automock)]
pub trait MouseCardinalLeverInputControllerTrait {
    fn trigger_input(&mut self);
    fn set_x_and_y_axes(&mut self, x_value: i32, y_value: i32);
    fn start_boost(&mut self,multiplier: u32);
    fn end_boost(&mut self);
}


