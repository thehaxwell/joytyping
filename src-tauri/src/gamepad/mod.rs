use std::rc::Rc;

use crate::{gamepad::{gilrs_wrapper::Gilrs, stick_switch_interpreter::StickSwitchEvent}, settings_data::{KeyboardInput, self}};

use self::{gilrs_wrapper::{GilrsEvent, GilrsEventType}, stick_switch_interpreter::StickSwitchInterpreterTrait, layer_node::{LayerNode, InputEvent, LayerNodeRef}};

pub mod gilrs_wrapper;
// #[cfg(test)]
// mod tests;

pub mod stick_switch_interpreter;
pub mod layer_node;

pub struct Gamepad {
   gilrs: Box<dyn Gilrs>,
   left_stick_switch_interpreter: Box<dyn StickSwitchInterpreterTrait>,
   right_stick_switch_interpreter: Box<dyn StickSwitchInterpreterTrait>,
   layer_nodes: Vec<LayerNode>,
   current_layer_node_index: usize,
}

impl Gamepad {
    pub fn new(
        gilrs: Box<dyn Gilrs>,
        left_stick_switch_interpreter: Box<dyn StickSwitchInterpreterTrait>,
        right_stick_switch_interpreter: Box<dyn StickSwitchInterpreterTrait>,
        layers_source: Vec<settings_data::Layer>,
    ) -> Self {
        Gamepad{
            gilrs,
            left_stick_switch_interpreter,
            right_stick_switch_interpreter,
            layer_nodes: Gamepad::initialize_layer_nodes_vec(layers_source),
            current_layer_node_index: 0, // we always start with the first node
        }
    }

    fn initialize_layer_nodes_vec(source: Vec<settings_data::Layer>,) -> Vec<LayerNode> {
        let mut idx: u32= 0;
        let pointers: Vec<LayerNodeRef> = source
            .iter()
            .map(|layer|{
                let res = LayerNodeRef{id: layer.id.to_string(), index: idx};
                idx += 1;
                res
            })
            .collect();

        source
            .iter()
            .map(|layer|
                 LayerNode::new(layer.clone(), &pointers)
            )
            .collect()
    }

    pub fn next_event(&mut self) -> Option<InputEvent> {
        match self.next_gilrs_event() {
            Some(event) => {
                let return_val = self.layer_nodes[self.current_layer_node_index].process_gamepad_event(event);
                if let Some(new_index) = return_val.next_node_index {
                    self.current_layer_node_index = new_index;
                }
                return_val.input_event
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

