use gilrs::Button;

use crate::{gamepad::{gilrs_wrapper::Gilrs, stick_switch_interpreter::StickSwitchEvent}, settings_data::{self, Layer, SwitchOnClickReaction, KeyboardInput, SwitchEventAndReaction}};

use self::{gilrs_wrapper::{GilrsEvent, GilrsEventType}, stick_switch_interpreter::{StickSwitchInterpreterTrait, StickSwitchButton}};

use self::switch_click_pattern_detector::{SwitchClickPatternDetectorTrait, SwitchClickPattern};
use self::on_release_reactions::OnReleaseReactionsTrait;

pub mod gilrs_wrapper;
// #[cfg(test)]
// mod tests;

pub mod stick_switch_interpreter;
pub mod on_release_reactions;
pub mod switch_click_pattern_detector;

pub struct Gamepad {
   gilrs: Box<dyn Gilrs>,
   left_stick_switch_interpreter: Box<dyn StickSwitchInterpreterTrait>,
   right_stick_switch_interpreter: Box<dyn StickSwitchInterpreterTrait>,

   layers: Vec<Layer>,
   current_layer_index: usize,
   switch_click_pattern_detector: Box<dyn SwitchClickPatternDetectorTrait>,
   on_release_reactions: Box<dyn OnReleaseReactionsTrait>,
}

impl Gamepad {
    pub fn new(
        gilrs: Box<dyn Gilrs>,
        left_stick_switch_interpreter: Box<dyn StickSwitchInterpreterTrait>,
        right_stick_switch_interpreter: Box<dyn StickSwitchInterpreterTrait>,
        layers_source: Vec<settings_data::Layer>,
       switch_click_pattern_detector: Box<dyn SwitchClickPatternDetectorTrait>,
       on_release_reactions: Box<dyn OnReleaseReactionsTrait>,
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

        let layers: Vec<Layer> = layers_source
            .iter()
            .map(|layer|
                layer.clone_and_set_layer_pointers(&pointers)
            )
            .collect();

        Gamepad{
            gilrs,
            left_stick_switch_interpreter,
            right_stick_switch_interpreter,

            layers,
            current_layer_index: 0,
            switch_click_pattern_detector,
            on_release_reactions,
        }
    }

    pub fn tick(&mut self) -> Option<InputEvent> {
        match self.switch_click_pattern_detector.tick() {
            Some(SwitchClickPattern::Click(switch)) => {
                if let Some(s_e_a_r)
                = self.get_switch_event_and_reaction(switch.clone()) {
                    match s_e_a_r.on_click {
                        Some(SwitchOnClickReaction::Keyboard(keyboard_input)) 
                        => return Some(InputEvent::KeyClick(keyboard_input)),
                        Some(SwitchOnClickReaction::MoveToLayer(layer_specifier))
                        => self.current_layer_index = layer_specifier.index_in_gamepad.unwrap().try_into().unwrap(),
                        Some(SwitchOnClickReaction::VisitLayer(layer_specifier))
                        => {
                            self.on_release_reactions
                                .add_consumable(
                                    switch,
                                    SwitchOnReleaseReaction::MoveToLayerAtIndex(
                                        self.current_layer_index));
                            self.current_layer_index = layer_specifier.index_in_gamepad.unwrap().try_into().unwrap();
                        },
                        _ => ()
                    }
                };
            },
            Some(SwitchClickPattern::ClickAndHold(switch)) => {
                if let Some(s_e_a_r)
                = self.get_switch_event_and_reaction(switch.clone()) {
                    match s_e_a_r.on_click_and_hold {
                        Some(SwitchOnClickReaction::Keyboard(keyboard_input)) 
                        => return Some(InputEvent::KeyClick(keyboard_input)),
                        Some(SwitchOnClickReaction::MoveToLayer(layer_specifier))
                        => self.current_layer_index = layer_specifier.index_in_gamepad.unwrap().try_into().unwrap(),
                        Some(SwitchOnClickReaction::VisitLayer(layer_specifier))
                        => {
                            self.on_release_reactions
                                .add_consumable(
                                    switch,
                                    SwitchOnReleaseReaction::MoveToLayerAtIndex(
                                        self.current_layer_index));
                            self.current_layer_index = layer_specifier.index_in_gamepad.unwrap().try_into().unwrap();
                        },
                        _ => ()
                    }
                };
            },
            Some(SwitchClickPattern::DoubleClick(switch)) => {
                if let Some(s_e_a_r)
                = self.get_switch_event_and_reaction(switch.clone()) {
                    match s_e_a_r.on_double_click {
                        Some(SwitchOnClickReaction::Keyboard(keyboard_input)) 
                        => return Some(InputEvent::KeyClick(keyboard_input)),
                        Some(SwitchOnClickReaction::MoveToLayer(layer_specifier))
                        => self.current_layer_index = layer_specifier.index_in_gamepad.unwrap().try_into().unwrap(),
                        Some(SwitchOnClickReaction::VisitLayer(layer_specifier))
                        => {
                            self.on_release_reactions
                                .add_consumable(
                                    switch,
                                    SwitchOnReleaseReaction::MoveToLayerAtIndex(
                                        self.current_layer_index));
                            self.current_layer_index = layer_specifier.index_in_gamepad.unwrap().try_into().unwrap();
                        },
                        _ => ()
                    }
                };
            },
            Some(SwitchClickPattern::DoubleClickAndHold(switch)) => {
                if let Some(s_e_a_r)
                = self.get_switch_event_and_reaction(switch.clone()) {
                    match s_e_a_r.on_double_click_and_hold {
                        Some(SwitchOnClickReaction::Keyboard(keyboard_input)) 
                        => return Some(InputEvent::KeyClick(keyboard_input)),
                        Some(SwitchOnClickReaction::MoveToLayer(layer_specifier))
                        => self.current_layer_index = layer_specifier.index_in_gamepad.unwrap().try_into().unwrap(),
                        Some(SwitchOnClickReaction::VisitLayer(layer_specifier))
                        => {
                            self.on_release_reactions
                                .add_consumable(
                                    switch,
                                    SwitchOnReleaseReaction::MoveToLayerAtIndex(
                                        self.current_layer_index));
                            self.current_layer_index = layer_specifier.index_in_gamepad.unwrap().try_into().unwrap();
                        },
                        _ => ()
                    }
                };
            },
            Some(SwitchClickPattern::ClickEnd(switch)) => {
                // TODO: probably a good place to go back on layer visits
                // self.current_layer_index = new_index;

                match self.on_release_reactions.get(switch.clone()) {
                    Some(SwitchOnReleaseReaction::MoveToLayerAtIndex(index))
                    => {
                        self.current_layer_index = index;
                    }
                    None => (),
                };

                return Some(InputEvent::KeyUp);
            }
            None => (),
        };

        None
    }

    // returns true if there is yet another event
    pub fn next_event(&mut self) -> bool{
        if let Some(event) = self.next_gilrs_event() {
            match event.event {
                GilrsEventType::ButtonPressed(button, ) => {
                    print!("ButtonPressed: {:?}\n",button);
                        self.switch_click_pattern_detector.button_pressed(button);
                }
                GilrsEventType::ButtonRepeated(button, ) => {
                    print!("ButtonRepeated: {:?}\n",button);
                    //     self.switch_click_pattern_detector.button_repeated(button);
                },
                GilrsEventType::ButtonReleased(button, ) => {
                    print!("ButtonReleased: {:?}\n",button);
                        self.switch_click_pattern_detector.button_released(button);
                },
                GilrsEventType::ButtonChanged(button, _value, ) => {
                    print!("ButtonChanged: {:?}\n",button);
                    //     self.switch_click_pattern_detector.button_changed(button);
                },
                GilrsEventType::AxisChanged(axis, value, switch_stick_event) => {
                    print!("AxisChanged: {:?}: {:?}\n",axis,value);
                    if let Some(event) = switch_stick_event {
                        match event {
                            StickSwitchEvent::ButtonPressed(button)
                                => self.switch_click_pattern_detector.axis_button_pressed(button),
                            StickSwitchEvent::ButtonReleased(button)
                                => self.switch_click_pattern_detector.axis_button_released(button),
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
            }
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

    fn get_switch_event_and_reaction(
        &self,switch: Switch) -> Option<SwitchEventAndReaction> {
        if let Some(switches) = &self.layers[self.current_layer_index].switches {
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

#[derive(Debug,Clone,PartialEq)]
pub enum SwitchOnReleaseReaction {
    MoveToLayerAtIndex(usize),
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

#[derive(Debug,Clone,PartialEq)]
pub enum Switch {
    Button(Button),
    StickSwitchButton(StickSwitchButton),
}
