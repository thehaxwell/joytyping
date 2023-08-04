use crate::gamepad::gilrs_wrapper::Gilrs;
use crate::gamepad::sticks_interpreter::SticksInterpreterTrait;

use self::gilrs_wrapper::{GilrsEvent, GilrsEventType};

pub mod gilrs_wrapper;
#[cfg(test)]
mod tests;

pub mod sticks_interpreter;

pub struct Gamepad {
   gilrs: Box<dyn Gilrs>,
   axes_interpreter: Box<dyn SticksInterpreterTrait>,
}

impl Gamepad {
    pub fn new(gilrs: Box<dyn Gilrs>, axes_interpreter: Box<dyn SticksInterpreterTrait>) -> Self {
        Gamepad{
            gilrs,
            axes_interpreter,
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
                    match axis {
                        gilrs::ev::Axis::LeftStickX | gilrs::ev::Axis::LeftStickY=> {
                            self.axes_interpreter.interpret_left_stick_move(
                                self.gilrs.get_gamepad_axis_data_value(gilrs::Axis::LeftStickX),
                                self.gilrs.get_gamepad_axis_data_value(gilrs::Axis::LeftStickY),
                            )

                        },
                        gilrs::ev::Axis::RightStickX | gilrs::ev::Axis::RightStickY=> {
                            self.axes_interpreter.interpret_right_stick_move(
                                self.gilrs.get_gamepad_axis_data_value(gilrs::Axis::RightStickX),
                                self.gilrs.get_gamepad_axis_data_value(gilrs::Axis::RightStickY),
                            )
                        }
                        _other => return None // return with no event
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
}

