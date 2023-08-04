use std::time::Duration;

#[cfg(test)]
mod tests;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait StepperButtonTrait {
    fn set_is_down_and_return_is_double_click(&mut self, is_down: bool) -> bool;
    fn get_is_down(&self) -> bool;
}

pub struct StepperButton {
    is_down: bool,
    latest_click_time: Option<std::time::Instant>,
    double_click_max_duration: Duration,
}

impl Drop for StepperButton {
    fn drop(&mut self) {
        if self.is_down {
            self.is_down = false;
        }
    }
}

impl StepperButton {
    pub fn new() -> StepperButton {
        StepperButton {
            is_down: false,
            latest_click_time: None,
            double_click_max_duration: Duration::from_millis(500),
        }
    }
}


impl StepperButtonTrait for StepperButton {
    fn set_is_down_and_return_is_double_click(
        &mut self, is_down: bool) -> bool {
        self.is_down = is_down;
        if is_down {
            if let Some(instant) = self.latest_click_time {
                if instant.elapsed() <= self.double_click_max_duration {
                    return true;
                }
            }
            self.latest_click_time = Some(std::time::Instant::now());
        }
        false
    }

    fn get_is_down(&self) -> bool {
        self.is_down
    }
}
