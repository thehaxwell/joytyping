use gilrs::Button;

use crate::settings_data::{self, Layer, SwitchOnClickReaction, KeyboardInput, SwitchEventAndReaction, Switches, LayerSpecifier};

use self::{gilrs_events::{gilrs_wrapper::GilrsEventType, GilrsEventsTrait,stick_switch_interpreter::{StickSwitchButton,StickSwitchEvent}}, layers_navigator::LayersNavigatorTrait};

use self::switch_click_pattern_detector::{SwitchClickPatternDetectorTrait, SwitchClickPattern};

pub mod gilrs_events;
pub mod switch_click_pattern_detector;
pub mod layers_navigator;

// #[cfg(test)]
// mod tests;


pub struct Gamepad {
   gilrs_events: Box<dyn GilrsEventsTrait>,
   layers: Vec<Layer>,
   switch_click_pattern_detector: Box<dyn SwitchClickPatternDetectorTrait>,
   layers_navigator: Box<dyn LayersNavigatorTrait>,
}

impl Gamepad {
    pub fn new(
       gilrs_events: Box<dyn GilrsEventsTrait>,
       layers_source: Vec<settings_data::Layer>,
       switch_click_pattern_detector: Box<dyn SwitchClickPatternDetectorTrait>,
       layers_navigator: Box<dyn LayersNavigatorTrait>,
    ) -> Self {
        Gamepad{
            gilrs_events,
            layers: layers_source,
            switch_click_pattern_detector,

            layers_navigator,
        }
    }


    pub fn tick(&mut self) -> Option<InputEvent> {
        match self.switch_click_pattern_detector.tick() {
            Some(SwitchClickPattern::Click(switch)) => {
                // self.commit_potential_visit_conditionally_and_unset(
                //     |potential_visit_trigger_switch| switch.clone() != potential_visit_trigger_switch );
                self.layers_navigator.commit_potential_visit_if_trigger_switch_not_equals_and_unset(switch.clone());

                match self.get_switch_event_and_reaction(switch.clone())
                            .and_then(|s_e_a_r| s_e_a_r.on_click) {
                    Some(SwitchOnClickReaction::Keyboard(keyboard_input)) 
                    => return Some(InputEvent::KeyClick(keyboard_input)),
                    Some(SwitchOnClickReaction::MoveToLayer(layer_specifier))
                    => self.layers_navigator.move_to_layer(layer_specifier),
                    Some(SwitchOnClickReaction::VisitLayer(layer_specifier))
                    => self.layers_navigator.visit_layer(switch,layer_specifier),
                    Some(SwitchOnClickReaction::MoveToOrVisitLayer(layer_specifier))
                    => self.layers_navigator.move_to_or_visit_layer(switch,layer_specifier),
                    _ => ()
                }

            },
            Some(SwitchClickPattern::ClickAndHold(switch)) => {
            // self.commit_potential_visit_conditionally_and_unset(
            //     |potential_visit_trigger_switch| switch.clone() == potential_visit_trigger_switch );
                self.layers_navigator.commit_potential_visit_if_trigger_switch_equals_and_unset(switch.clone());
                    if let Some(s_e_a_r)
                    = self.get_switch_event_and_reaction(switch.clone()) {
                        // if on_click is set to
                        // type out some key, then hold down that key
                        if let Some(SwitchOnClickReaction::Keyboard(keyboard_input)) 
                            = s_e_a_r.on_click {
                            return Some(InputEvent::KeyDown(keyboard_input))
                        }
                    };
            },
            Some(SwitchClickPattern::DoubleClick(switch)) => {
                // self.commit_potential_visit_conditionally_and_unset(
                //     |potential_visit_trigger_switch| switch.clone() != potential_visit_trigger_switch );
                self.layers_navigator.commit_potential_visit_if_trigger_switch_not_equals_and_unset(switch.clone());

                match self.get_switch_event_and_reaction(switch.clone())
                            .and_then(
                                // use on_double_click or on_click if it isn't set
                                |s_e_a_r| s_e_a_r.on_double_click
                                                 .or_else(|| s_e_a_r.on_click )) {
                    Some(SwitchOnClickReaction::Keyboard(keyboard_input)) 
                    => return Some(InputEvent::KeyClick(keyboard_input)),
                    Some(SwitchOnClickReaction::MoveToLayer(layer_specifier))
                    => self.layers_navigator.move_to_layer(layer_specifier),
                    Some(SwitchOnClickReaction::VisitLayer(layer_specifier))
                    => self.layers_navigator.visit_layer(switch,layer_specifier),
                    Some(SwitchOnClickReaction::MoveToOrVisitLayer(layer_specifier))
                    => self.layers_navigator.move_to_or_visit_layer(switch,layer_specifier),
                    _ => ()
                }
            },
            Some(SwitchClickPattern::DoubleClickAndHold(switch)) => {
            // self.commit_potential_visit_conditionally_and_unset(
            //     |potential_visit_trigger_switch| switch.clone() == potential_visit_trigger_switch );
                self.layers_navigator.commit_potential_visit_if_trigger_switch_equals_and_unset(switch.clone());
                    if let Some(s_e_a_r)
                    = self.get_switch_event_and_reaction(switch.clone()) {
                        // if on_double_click (or fallback to on_click) is set to
                        // type out some key, then hold down that key
                        if let Some(SwitchOnClickReaction::Keyboard(keyboard_input)) 
                            = s_e_a_r.on_double_click {
                            return Some(InputEvent::KeyDown(keyboard_input))
                        }
                        else if let Some(SwitchOnClickReaction::Keyboard(keyboard_input)) 
                            = s_e_a_r.on_click {
                            return Some(InputEvent::KeyDown(keyboard_input))
                        }

                    };

            },
            Some(SwitchClickPattern::ClickEnd(switch)) => {
                self.layers_navigator.unset_potential_visit();

                self.layers_navigator.undo_last_layer_visit_with_switch(switch);

                return Some(InputEvent::KeyUp);
            }
            None => (),
        };

        None
    }

    // returns true if there is yet another event
    pub fn next_event(&mut self) -> bool{
        if let Some(event) = self.gilrs_events.next() {
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

    // fn next_gilrs_event(&mut self) -> Option<GilrsEvent> {
    //     match self.gilrs.next_event() {
    //         Some(gilrs_event) => {
    //             match gilrs_event.event {
    //                 GilrsEventType::AxisChanged(axis, value, _) => {
    //                      let stick_switch_event_option = match axis {
    //                         gilrs::ev::Axis::LeftStickX | gilrs::ev::Axis::LeftStickY=> {
    //                             self.left_stick_switch_interpreter.interpret_stick_move(
    //                                 self.gilrs.get_gamepad_axis_data_value(gilrs::Axis::LeftStickX),
    //                                 self.gilrs.get_gamepad_axis_data_value(gilrs::Axis::LeftStickY),
    //                             )
    //                         },
    //                         gilrs::ev::Axis::RightStickX | gilrs::ev::Axis::RightStickY=> {
    //                             self.right_stick_switch_interpreter.interpret_stick_move(
    //                                 self.gilrs.get_gamepad_axis_data_value(gilrs::Axis::RightStickX),
    //                                 self.gilrs.get_gamepad_axis_data_value(gilrs::Axis::RightStickY),
    //                             ) 
    //                         }
    //                         _other => None
    //                     };
    //
    //                     Some(GilrsEvent{
    //                         time: gilrs_event.time,
    //                         event: GilrsEventType::AxisChanged(axis, value, stick_switch_event_option), 
    //                     })
    //                 },
    //                 _other => Some(gilrs_event),
    //             }
    //         },
    //         _other => None
    //     }
    // }

    fn get_switch_event_and_reaction_from_switches(
        switch: Switch, switches: &Switches) -> Option<SwitchEventAndReaction> {
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

    fn get_switch_event_and_reaction(
        &self,switch: Switch) -> Option<SwitchEventAndReaction> {
        if let Some(switches) = &self.layers[self.layers_navigator.get_current_layer_index()].switches {
            Gamepad::get_switch_event_and_reaction_from_switches(
                switch, switches)
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
    pub index: usize,
}

#[derive(Debug,Clone,PartialEq)]
pub enum Switch {
    Button(Button),
    StickSwitchButton(StickSwitchButton),
}

#[derive(Debug,Clone,PartialEq)]
pub enum Handle {
    Click,
    ClickAndHold,
    DoubleClick,
    DoubleClickAndHold,
    ClickEnd,
}

#[derive(Debug,Clone,PartialEq)]
pub enum DynamicSwitchEventReaction {
    DoubleClick(EventReactionOrNesting),
    DoubleClickAndHold(EventReactionOrNesting),
    ClickEnd(EventReactionOrNesting),
    Click(EventReactionOrNesting),
    ClickAndHold(EventReactionOrNesting),
}

#[derive(Debug,Clone,PartialEq)]
pub enum EventReactionOrNesting {
    Reaction(SwitchOnClickReaction),
    Nesting(Box<DynamicSwitchEventReaction>),
}
