use crate::{gamepad::{gilrs_wrapper::Gilrs, stick_switch_interpreter::StickSwitchEvent}, settings_data::KeyboardInput};

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
        match self.next_gilrs_event() {
            Some(GilrsEvent { event, time: _}) => {
            match event {
                GilrsEventType::ButtonPressed(button, )
                    => Some(InputEvent::KeyDown(KeyboardInput {
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
                GilrsEventType::AxisChanged(axis, value, switch_stick_event) => {
                    print!("AxisChanged: {:?}: {:?}\n",axis,value);
                    if let Some(event) = switch_stick_event {
                        match event {
                            StickSwitchEvent::ButtonPressed(_)
                                => Some(InputEvent::KeyDown(KeyboardInput {
                                    key: enigo::Key::Layout('a'),
                                    modifiers: vec![],
                                })),
                            StickSwitchEvent::ButtonReleased(_)
                                => Some(InputEvent::KeyUp),
                        }
                    }
                    else {
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

    fn next_gilrs_event(&mut self) -> Option<GilrsEvent> {
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

#[derive(Debug, PartialEq)]
pub enum InputEvent {
    KeyDown(KeyboardInput),
    KeyUp,
}
