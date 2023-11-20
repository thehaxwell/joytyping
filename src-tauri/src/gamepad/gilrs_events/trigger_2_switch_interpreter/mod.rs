use crate::{models::main_config::Trigger2SwitchesClickThresholds, LeftOrRight};

use gilrs::Button;
#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg(test)]
mod tests;

#[cfg_attr(test, automock)]
pub trait Trigger2SwitchInterpreterTrait {
    fn interpret_move (
        &mut self,
        value: f32,
    ) -> Option<Trigger2SwitchEvent>;
}

pub struct Trigger2SwitchInterpreter {
    trigger_2_click_thresholds: Trigger2SwitchesClickThresholds,
    alignment: LeftOrRight,
    is_pressed: bool,
}

#[cfg_attr(test, automock)]
impl Trigger2SwitchInterpreter {
    pub fn new(
            trigger_2_click_thresholds: Trigger2SwitchesClickThresholds,
            alignment: LeftOrRight,
        ) -> Trigger2SwitchInterpreter{

        Trigger2SwitchInterpreter {
            trigger_2_click_thresholds,
            alignment,
            is_pressed: false,
        }
    }

}

impl Trigger2SwitchInterpreterTrait for Trigger2SwitchInterpreter {
    fn interpret_move(&mut self, value: f32)-> Option<Trigger2SwitchEvent> {
        let (threshold,button) = match self.alignment {
            LeftOrRight::Left
                => (self.trigger_2_click_thresholds.left_trigger_2,Button::LeftTrigger2),
            LeftOrRight::Right
                => (self.trigger_2_click_thresholds.right_trigger_2,Button::RightTrigger2),
        };

        if value >= threshold && !self.is_pressed {
            self.is_pressed = true;
            return Some(Trigger2SwitchEvent::ButtonPressed(button));
        }
        else if value < threshold && self.is_pressed {
            self.is_pressed = false;
            return Some(Trigger2SwitchEvent::ButtonReleased(button));
        }
        return None;
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Trigger2SwitchEvent {
    ButtonPressed(Button),
    ButtonReleased(Button),
}
