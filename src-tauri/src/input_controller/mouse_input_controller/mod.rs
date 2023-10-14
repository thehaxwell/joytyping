#[cfg(test)]
use mockall::{automock, predicate::*};

// TODO: redefine these tests, the effect mentioned
// there is now achieved by the natural wait between
// click/double-click and click-and-hold/double-click-and-hold
// #[cfg(test)]
// mod tests;

pub mod scroll;
pub mod cursor;

#[cfg_attr(test, automock)]
pub trait MouseInputControllerTrait {
    fn trigger_input(&mut self);
    fn set_x_and_y_axes(&mut self, x_value: i32, y_value: i32);
}


