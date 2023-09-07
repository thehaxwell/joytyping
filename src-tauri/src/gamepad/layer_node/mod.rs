use crate::{settings_data::{KeyboardInput, self}, gamepad::stick_switch_interpreter::StickSwitchEvent};

use super::gilrs_wrapper::{GilrsEvent, GilrsEventType};


#[derive(Debug, Clone, PartialEq)]
pub struct LayerNode {
    source: settings_data::Layer,
}
impl LayerNode {
    pub fn new(
        source: settings_data::Layer,
        pointers: &Vec<LayerNodeRef>,
    ) -> Self{
        Self {
            source: source.clone_and_set_layer_pointers(pointers),
        }
    }

    pub fn get_id(&self) -> String {
        self.source.id.clone()
    }

    pub fn process_gamepad_event(&self, event: GilrsEvent) -> ProcessGamepadEventReturnValue {
        let input_event = match event.event {
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
        };

        ProcessGamepadEventReturnValue {
            input_event,
            next_node_index: None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum InputEvent {
    KeyDown(KeyboardInput),
    KeyUp,
}

#[derive(Debug, PartialEq)]
pub struct LayerNodeRef{
    pub id: String,
    pub index: u32,
}

pub struct ProcessGamepadEventReturnValue {
    pub input_event: Option<InputEvent>,
    pub next_node_index: Option<usize>,
}
