use gilrs::Button;

use crate::{settings_data::{KeyboardInput, self, SwitchEventAndReaction, SwitchOnClickReaction}, gamepad::stick_switch_interpreter::StickSwitchEvent};

use self::switch_click_pattern_detector::{SwitchClickPattern, SwitchClickPatternDetectorTrait};

use super::{gilrs_wrapper::{GilrsEvent, GilrsEventType}, stick_switch_interpreter::StickSwitchButton};

pub mod switch_click_pattern_detector;

pub struct LayerNode {
    source: settings_data::Layer,
    // this struct will also need access to source, but it looks like
    // it'll be injected by the initializer of this (LayerNode) struct
    switch_click_pattern_detector: Option<Box<dyn SwitchClickPatternDetectorTrait>>,
}
impl LayerNode {
    pub fn new(
        source: settings_data::Layer,
        pointers: &Vec<LayerNodeRef>,
        switch_click_pattern_detector: Option<Box<dyn SwitchClickPatternDetectorTrait>>,
    ) -> Self{
        Self {
            source: source.clone_and_set_layer_pointers(pointers),
            switch_click_pattern_detector,
        }
    }

    pub fn get_id(&self) -> String {
        self.source.id.clone()
    }

    // the return value is similar to ProcessGamepadEventReturnValue
    pub fn tick(&mut self) -> GamepadEventReaction {
        if let Some(detector) = &mut self.switch_click_pattern_detector {
            match detector.tick() {
                Some(SwitchClickPattern::Click(switch)) => {
                    if let Some(s_e_a_r)
                    = self.get_switch_event_and_reaction_from_switch(switch) {
                        return LayerNode::translate_reaction(s_e_a_r.on_click);
                    };
                },
                Some(SwitchClickPattern::ClickAndHold(switch)) => {
                    if let Some(s_e_a_r)
                    = self.get_switch_event_and_reaction_from_switch(switch) {
                        return LayerNode::translate_reaction(s_e_a_r.on_click_and_hold);
                    };
                },
                Some(SwitchClickPattern::DoubleClick(switch)) => {
                    if let Some(s_e_a_r)
                    = self.get_switch_event_and_reaction_from_switch(switch) {
                        return LayerNode::translate_reaction(s_e_a_r.on_double_click);
                    };
                },
                Some(SwitchClickPattern::DoubleClickAndHold(switch)) => {
                    if let Some(s_e_a_r)
                    = self.get_switch_event_and_reaction_from_switch(switch) {
                        return LayerNode::translate_reaction(s_e_a_r.on_double_click_and_hold);
                    };
                },
                None => (),
                _other => { // the "...End" variants
                    // TODO: probably a good place to go back on layer visits
                    return GamepadEventReaction {
                        input_event: Some(InputEvent::KeyUp),
                        next_node_index: None,
                    }
                }
            }
        };

        return GamepadEventReaction {
            input_event: None,
            next_node_index: None,
        }
    }

    fn translate_reaction(reaction: Option<SwitchOnClickReaction>)-> GamepadEventReaction{
        match reaction {
            Some(SwitchOnClickReaction::Keyboard(KeyboardInput { key, modifiers })) 
            => GamepadEventReaction {
                input_event: Some(InputEvent::KeyClick(KeyboardInput {
                    key,
                    modifiers,
                })),
                next_node_index: None,
            },
            _ => GamepadEventReaction {
                input_event: None,
                next_node_index: None,
            }
        }
    }
    
    pub fn process_gamepad_event(&mut self, event: GilrsEvent){
        match event.event {
            GilrsEventType::ButtonPressed(button, ) => {
                print!("ButtonPressed: {:?}\n",button);
                if let Some(detector)
                    = &mut self.switch_click_pattern_detector {
                    detector.button_pressed(button);
                }
            }
            GilrsEventType::ButtonRepeated(button, ) => {
                print!("ButtonRepeated: {:?}\n",button);
                //     detector.button_repeated(button);
            },
            GilrsEventType::ButtonReleased(button, ) => {
                print!("ButtonReleased: {:?}\n",button);
                if let Some(detector)
                    = &mut self.switch_click_pattern_detector {
                    detector.button_released(button);
                }
            },
            GilrsEventType::ButtonChanged(button, _value, ) => {
                print!("ButtonChanged: {:?}\n",button);
                //     detector.button_changed(button);
            },
            GilrsEventType::AxisChanged(axis, value, switch_stick_event) => {
                print!("AxisChanged: {:?}: {:?}\n",axis,value);
                if let Some(event) = switch_stick_event {
                    match event {
                        StickSwitchEvent::ButtonPressed(button)
                            => if let Some(detector)
                                = &mut self.switch_click_pattern_detector {
                                detector.axis_button_pressed(button);
                            }
                        StickSwitchEvent::ButtonReleased(button)
                            => if let Some(detector)
                                = &mut self.switch_click_pattern_detector {
                                detector.axis_button_released(button);
                            }
                    };
                }
            },
            GilrsEventType::Connected => {
                print!("Connected!\n");
            },
            GilrsEventType::Disconnected => {
                print!("Disconnected!\n");
            },
            GilrsEventType::Dropped => {
                print!("Droppedn!\n");
            }
        };
    }

    fn get_switch_event_and_reaction_from_switch(
        &self,switch: Switch) -> Option<SwitchEventAndReaction> {
        if let Some(switches) = &self.source.switches {
            match switch {
                Switch::Button(button) => match button {
                    Button::North => switches.north.clone(),
                    Button::South => switches.south.clone(),
                    Button::East => switches.east.clone(),
                    Button::West => switches.west.clone(),
                    Button::DPadUp => switches.d_pad_up.clone(),
                    Button::DPadDown => switches.d_pad_down.clone(),
                    Button::DPadRight => switches.d_pad_right.clone(),
                    Button::DPadLeft => switches.d_pad_left.clone(),
                    Button::LeftTrigger => switches.left_trigger.clone(),
                    Button::RightTrigger => switches.right_trigger.clone(),
                    _ => None
                },
                Switch::StickSwitchButton(button) => match button {
                    StickSwitchButton::LeftStickUp => switches.left_stick_up.clone(),
                    StickSwitchButton::LeftStickDown => switches.left_stick_down.clone(),
                    StickSwitchButton::LeftStickRight => switches.left_stick_right.clone(),
                    StickSwitchButton::LeftStickLeft => switches.left_stick_left.clone(),
                    StickSwitchButton::RightStickUp => switches.right_stick_up.clone(),
                    StickSwitchButton::RightStickDown => switches.right_stick_down.clone(),
                    StickSwitchButton::RightStickRight => switches.right_stick_right.clone(),
                    StickSwitchButton::RightStickLeft => switches.right_stick_left.clone(),
                }
            }
        }
        else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum InputEvent {
    KeyClick(KeyboardInput),
    KeyDown(KeyboardInput),
    KeyUp,
}

#[derive(Debug, PartialEq)]
pub struct LayerNodeRef{
    pub id: String,
    pub index: u32,
}

pub struct GamepadEventReaction {
    pub input_event: Option<InputEvent>,
    pub next_node_index: Option<usize>,
}

#[derive(Debug,Clone,PartialEq)]
pub enum Switch {
    Button(Button),
    StickSwitchButton(StickSwitchButton),
}
