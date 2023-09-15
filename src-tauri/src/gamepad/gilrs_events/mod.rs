use self::gilrs_wrapper::{GilrsEvent, Gilrs, GilrsEventType};

use self::stick_switch_interpreter::StickSwitchInterpreterTrait;

#[cfg(test)]
use mockall::{automock, predicate::*};

pub mod gilrs_wrapper;
pub mod stick_switch_interpreter;

#[cfg(test)]
mod tests;

#[cfg_attr(test, automock)]
pub trait GilrsEventsTrait {
    fn next(&mut self) -> Option<GilrsEvent>;
}

pub struct GilrsEvents {
   gilrs: Box<dyn Gilrs>,
   left_stick_switch_interpreter: Box<dyn StickSwitchInterpreterTrait>,
   right_stick_switch_interpreter: Box<dyn StickSwitchInterpreterTrait>,
}

impl GilrsEvents {
    pub fn new(
       gilrs: Box<dyn Gilrs>,
       left_stick_switch_interpreter: Box<dyn StickSwitchInterpreterTrait>,
       right_stick_switch_interpreter: Box<dyn StickSwitchInterpreterTrait>,
    ) -> Self {
        Self {
            gilrs,
            left_stick_switch_interpreter,
            right_stick_switch_interpreter,
        }
    }
}

impl GilrsEventsTrait for GilrsEvents {
    fn next(&mut self) -> Option<GilrsEvent> {
        match self.gilrs.next_event() {
            Some(gilrs_event) => {
                match gilrs_event.event {
                    GilrsEventType::AxisChanged(axis, value, _) => {
                         let stick_switch_event_option = match axis {
                            gilrs::ev::Axis::LeftStickX | gilrs::ev::Axis::LeftStickY=> {
                                self.left_stick_switch_interpreter.interpret_stick_move(
                                    self.gilrs.get_gamepad_axis_data_value(gilrs::Axis::LeftStickX),
                                    self.gilrs.get_gamepad_axis_data_value(gilrs::Axis::LeftStickY),
                                )
                            },
                            gilrs::ev::Axis::RightStickX | gilrs::ev::Axis::RightStickY=> {
                                self.right_stick_switch_interpreter.interpret_stick_move(
                                    self.gilrs.get_gamepad_axis_data_value(gilrs::Axis::RightStickX),
                                    self.gilrs.get_gamepad_axis_data_value(gilrs::Axis::RightStickY),
                                ) 
                            }
                            _other => None
                        };

                        Some(GilrsEvent{
                            time: gilrs_event.time,
                            event: GilrsEventType::AxisChanged(axis, value, stick_switch_event_option), 
                        })
                    },
                    _other => Some(gilrs_event),
                }
            },
            _other => None
        }
    }
}
