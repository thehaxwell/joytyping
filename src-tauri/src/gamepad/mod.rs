use crate::{gamepad::{gilrs_wrapper::Gilrs, stick_switch_interpreter::StickSwitchEvent}, input_controller::KeyInputShape};

use self::{gilrs_wrapper::{GilrsEvent, GilrsEventType}, stick_switch_interpreter::StickSwitchInterpreterTrait};

pub mod gilrs_wrapper;
// #[cfg(test)]
// mod tests;

pub mod stick_switch_interpreter;

pub struct Gamepad {
   gilrs: Box<dyn Gilrs>,
   left_stick_switch_interpreter: Box<dyn StickSwitchInterpreterTrait>,
   right_stick_switch_interpreter: Box<dyn StickSwitchInterpreterTrait>,
}

impl Gamepad {
    pub fn new(
        gilrs: Box<dyn Gilrs>,
        left_stick_switch_interpreter: Box<dyn StickSwitchInterpreterTrait>,
        right_stick_switch_interpreter: Box<dyn StickSwitchInterpreterTrait>,
    ) -> Self {
        Gamepad{
            gilrs,
            left_stick_switch_interpreter,
            right_stick_switch_interpreter,
        }
    }

    pub fn next_event(&mut self) -> Option<InputEvent> {
        match self.gilrs.next_event() {
            Some(GilrsEvent { event, time: _}) => {
            match event {
                GilrsEventType::ButtonPressed(button, )
                    => Some(InputEvent::KeyDown(KeyInputShape {
                            key: enigo::Key::Layout('a'),
                            modifiers: vec![],
                        })),
                GilrsEventType::ButtonRepeated(button, ) => {
                    print!("ButtonRepeated: {:?}\n",button);
                    None
                },
                GilrsEventType::ButtonReleased(button, )
                    => Some(InputEvent::KeyUp),
                GilrsEventType::ButtonChanged(button, _value, ) => {
                    print!("ButtonChanged: {:?}\n",button);
                    None
                },
                GilrsEventType::AxisChanged(axis, value, ) => {
                    print!("AxisChanged: {:?}: {:?}\n",axis,value);
                    if true {
                        match axis {
                            gilrs::ev::Axis::LeftStickX | gilrs::ev::Axis::LeftStickY=> {
                                match self.left_stick_switch_interpreter.interpret_stick_move(
                                    self.gilrs.get_gamepad_axis_data_value(gilrs::Axis::LeftStickX),
                                    self.gilrs.get_gamepad_axis_data_value(gilrs::Axis::LeftStickY),
                                ){
                                    Some(StickSwitchEvent::ButtonPressed(_))
                                        => Some(InputEvent::KeyDown(KeyInputShape {
                                            key: enigo::Key::Layout('a'),
                                            modifiers: vec![],
                                        })),
                                    Some(StickSwitchEvent::ButtonReleased(_))
                                        => Some(InputEvent::KeyUp),
                                    None => None
                                }

                            },
                            gilrs::ev::Axis::RightStickX | gilrs::ev::Axis::RightStickY=> {
                                match self.right_stick_switch_interpreter.interpret_stick_move(
                                    self.gilrs.get_gamepad_axis_data_value(gilrs::Axis::RightStickX),
                                    self.gilrs.get_gamepad_axis_data_value(gilrs::Axis::RightStickY),
                                ){
                                    Some(StickSwitchEvent::ButtonPressed(_))
                                        => Some(InputEvent::KeyDown(KeyInputShape {
                                            key: enigo::Key::Layout('a'),
                                            modifiers: vec![],
                                        })),
                                    Some(StickSwitchEvent::ButtonReleased(_))
                                        => Some(InputEvent::KeyUp),
                                    None => None
                                }
                            }
                            _other => return None // return with no event
                        }
                    }
                    else {
                        // Some(GamepadEvent::AxisChanged(axis,value))
                        None
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

#[derive(Debug, PartialEq)]
pub enum InputEvent {
    KeyDown(KeyInputShape),
    KeyUp,
}
