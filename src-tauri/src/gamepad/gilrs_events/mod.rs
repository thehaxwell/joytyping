use self::gilrs_wrapper::{GilrsEvent, Gilrs, GilrsEventType};

use self::stick_switch_interpreter::StickSwitchInterpreterTrait;
use self::trigger_2_switch_interpreter::{Trigger2SwitchInterpreterTrait, Trigger2SwitchEvent};

use gilrs::Button;
#[cfg(test)]
use mockall::{automock, predicate::*};

pub mod gilrs_wrapper;
pub mod stick_switch_interpreter;
pub mod trigger_2_switch_interpreter;

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
   left_trigger_2_switch_interpreter: Box<dyn Trigger2SwitchInterpreterTrait>,
   right_trigger_2_switch_interpreter: Box<dyn Trigger2SwitchInterpreterTrait>,
}

impl GilrsEvents {
    pub fn new(
       gilrs: Box<dyn Gilrs>,
       left_stick_switch_interpreter: Box<dyn StickSwitchInterpreterTrait>,
       right_stick_switch_interpreter: Box<dyn StickSwitchInterpreterTrait>,
       left_trigger_2_switch_interpreter: Box<dyn Trigger2SwitchInterpreterTrait>,
       right_trigger_2_switch_interpreter: Box<dyn Trigger2SwitchInterpreterTrait>,
    ) -> Self {
        Self {
            gilrs,
            left_stick_switch_interpreter,
            right_stick_switch_interpreter,
            left_trigger_2_switch_interpreter,
            right_trigger_2_switch_interpreter,
        }
    }
}

impl GilrsEventsTrait for GilrsEvents {
    fn next(&mut self) -> Option<GilrsEvent> {
        match self.gilrs.next_event() {
            Some(gilrs_event) => {
                match gilrs_event.event {
                    GilrsEventType::ButtonPressed(button, ) => {
                        match button {
                            // the Button::Unknown button is useless to us,
                            // and it was causing issues
                            Button::Unknown => None,
                            Button::LeftTrigger2 | Button::RightTrigger2 => None,
                            _ => Some(GilrsEvent{
                                time: gilrs_event.time,
                                event: GilrsEventType::ButtonPressed(button), 
                            }),
                        }
                    }
                    GilrsEventType::ButtonReleased(button, ) => {
                        match button {
                            // the Button::Unknown button is useless to us,
                            // and it was causing issues
                            Button::Unknown => None,
                            Button::LeftTrigger2 | Button::RightTrigger2 => None,
                            _ => Some(GilrsEvent{
                                time: gilrs_event.time,
                                event: GilrsEventType::ButtonReleased(button), 
                            }),
                        }
                    }
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
                    GilrsEventType::ButtonChanged(button, value) => {
                        // the Button::Unknown button is useless to us,
                        // and it was causing issues
                        if let Button::Unknown = button {
                            return None;
                        }

                        let trigger_2_switch_event_option = match button {
                            gilrs::Button::LeftTrigger2 
                                => self.left_trigger_2_switch_interpreter.interpret_move(value),
                            gilrs::Button::RightTrigger2 
                                => self.right_trigger_2_switch_interpreter.interpret_move(value),
                            _other => None
                        };

                        let new_event
                            = if let Some(event) = trigger_2_switch_event_option {
                                match event {
                                    Trigger2SwitchEvent::ButtonPressed(btn)
                                        => GilrsEventType::ButtonPressed(btn),
                                    Trigger2SwitchEvent::ButtonReleased(btn)
                                        => GilrsEventType::ButtonReleased(btn),
                                }
                            }
                            else {
                                GilrsEventType::ButtonChanged(button, value)
                            };

                        Some(GilrsEvent{
                            time: gilrs_event.time,
                            event: new_event, 
                        })
                    },
                    ref _other => Some(gilrs_event),
                }
            },
            _other => None
        }
    }
}
