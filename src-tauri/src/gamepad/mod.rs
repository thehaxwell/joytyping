use crate::gamepad::gilrs_wrapper::Gilrs;

use self::{gilrs_wrapper::{GilrsEvent, GilrsEventType}, stick_switch_interpreter::StickSwitchInterpreterTrait};

pub mod gilrs_wrapper;
#[cfg(test)]
mod tests;

pub mod stick_switch_interpreter;

pub struct Gamepad {
   gilrs: Box<dyn Gilrs>,
   left_stick_switch_interpreter: Box<dyn StickSwitchInterpreterTrait>,
   right_stick_switch_interpreter: Box<dyn StickSwitchInterpreterTrait>,
   should_interpret_stick_change_as_click: bool,
}

impl Gamepad {
    pub fn new(
        gilrs: Box<dyn Gilrs>,
        left_stick_switch_interpreter: Box<dyn StickSwitchInterpreterTrait>,
        right_stick_switch_interpreter: Box<dyn StickSwitchInterpreterTrait>,
        should_interpret_stick_change_as_click: bool
    ) -> Self {
        Gamepad{
            gilrs,
            left_stick_switch_interpreter,
            right_stick_switch_interpreter,
            should_interpret_stick_change_as_click,
        }
    }

    pub fn next_event(&mut self) -> Option<GamepadEvent> {
        match self.gilrs.next_event() {
            Some(GilrsEvent { event, time: _}) => {
            match event {
                GilrsEventType::ButtonPressed(button, )=> Some(GamepadEvent::ButtonPressed(CustomButton::Base(button))),
                GilrsEventType::ButtonRepeated(button, ) => {
                    print!("ButtonRepeated: {:?}\n",button);
                    None
                },
                GilrsEventType::ButtonReleased(button, ) => Some(GamepadEvent::ButtonReleased(CustomButton::Base(button))),
                GilrsEventType::ButtonChanged(button, _value, ) => {
                    print!("ButtonChanged: {:?}\n",button);
                    None
                },
                GilrsEventType::AxisChanged(axis, value, ) => {
                    print!("AxisChanged: {:?}: {:?}\n",axis,value);
                    if self.should_interpret_stick_change_as_click {
                        match axis {
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
                            _other => return None // return with no event
                        }
                    }
                    else {
                        Some(GamepadEvent::AxisChanged(axis,value))
                    }

                },
                GilrsEventType::Connected => {
                    print!("Connected!\n");
                    None
                },
                GilrsEventType::Disconnected => {
                    print!("Disconnected!\n");
                    None
                },
                GilrsEventType::Dropped => {
                    print!("Droppedn!\n");
                None
                }
            }
            },
            _other => None
        }
    }

    pub fn set_should_interpret_stick_change_as_click(&mut self, should: bool) {
        self.should_interpret_stick_change_as_click = should;
    }
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CustomButton {
    Base(gilrs::Button),
    LeftStickUp,
    LeftStickDown,
    LeftStickLeft,
    LeftStickRight,
    RightStickUp,
    RightStickDown,
    RightStickLeft,
    RightStickRight,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GamepadEvent {
    ButtonPressed(CustomButton),
    ButtonReleased(CustomButton),
    AxisChanged(gilrs::Axis,f32),
}

