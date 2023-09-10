use crate::{gamepad::gilrs_wrapper::Gilrs, settings_data};

use self::{gilrs_wrapper::{GilrsEvent, GilrsEventType}, stick_switch_interpreter::StickSwitchInterpreterTrait, layer_node::{LayerNode, InputEvent, LayerNodeRef, switch_click_pattern_detector::SwitchClickPatternDetectorTrait}};

pub mod gilrs_wrapper;
// #[cfg(test)]
// mod tests;

pub mod stick_switch_interpreter;
pub mod layer_node;

type NewLayerNodeFunction = fn(
        source: settings_data::Layer,
        pointers: &Vec<LayerNodeRef>,
        switch_click_pattern_detector: Option<Box<dyn SwitchClickPatternDetectorTrait>>
    ) -> LayerNode;

type NewSwitchClickPatternDetectorFunction = fn(
    ) -> Box<dyn SwitchClickPatternDetectorTrait>;

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
        create_new_layer_node_fn: NewLayerNodeFunction,
        create_new_switch_click_pattern_detector: NewSwitchClickPatternDetectorFunction,
    ) -> Self {

        let mut idx: u32= 0;
        let pointers: Vec<LayerNodeRef> = layers_source
            .iter()
            .map(|layer|{
                let res = LayerNodeRef{id: layer.id.to_string(), index: idx};
                idx += 1;
                res
            })
            .collect();

        let layer_nodes: Vec<LayerNode> = layers_source
            .iter()
            .map(|layer|
                create_new_layer_node_fn(
                    layer.clone(),
                    &pointers,
                    if layer.switches.is_some() {
                        Some(create_new_switch_click_pattern_detector())
                    } else {
                        None 
                    }
                )
            )
            .collect();

        Gamepad{
            gilrs,
            left_stick_switch_interpreter,
            right_stick_switch_interpreter,
            layer_nodes,
            current_layer_node_index: 0, // we always start with the first node
        }
    }

    pub fn tick(&mut self) -> Option<InputEvent> {
        let return_val = self.layer_nodes[self.current_layer_node_index].tick();
        if let Some(new_index) = return_val.next_node_index {
            self.current_layer_node_index = new_index;
        }
        return_val.input_event
    }

    // returns true if there is yet another event
    pub fn next_event(&mut self) -> bool{
        if let Some(event) = self.next_gilrs_event() {
            self.layer_nodes[self.current_layer_node_index].process_gamepad_event(event);
            true
        }
        else {
            false
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

