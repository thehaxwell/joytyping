use gilrs::Button;

use crate::{settings::{self,data::{Layer, SwitchOnClickReaction, KeyboardInput, SwitchEventAndReaction, Switches, CardinalLevers, SingleCardinalLever, MouseControl}}, quick_lookup_window::QuickLookupWindowTrait};

use self::{gilrs_events::{gilrs_wrapper::GilrsEventType, GilrsEventsTrait,stick_switch_interpreter::{StickSwitchButton,StickSwitchEvent}}, layers_navigator::{LayersNavigatorTrait, LayerVisitTrigger}, cardinal_levers_move_detector::CardinalLeversMoveDetectorTrait};

use self::switch_click_pattern_detector::{SwitchClickPatternDetectorTrait, SwitchClickPattern};

pub mod gilrs_events;
pub mod switch_click_pattern_detector;
pub mod layers_navigator;
pub mod cardinal_levers_move_detector;

// #[cfg(test)]
// mod tests;


pub struct Gamepad {
   gilrs_events: Box<dyn GilrsEventsTrait>,
   layers: Vec<Layer>,
   switch_click_pattern_detector: Box<dyn SwitchClickPatternDetectorTrait>,
   layers_navigator: Box<dyn LayersNavigatorTrait>,
   quick_lookup_window: Box<dyn QuickLookupWindowTrait>,
   mouse_cursor_move_detector: Box<dyn CardinalLeversMoveDetectorTrait>,
   mouse_scroll_detector: Box<dyn CardinalLeversMoveDetectorTrait>,
}

impl Gamepad {
    pub fn new(
       gilrs_events: Box<dyn GilrsEventsTrait>,
       layers_source: Vec<settings::data::Layer>,
       switch_click_pattern_detector: Box<dyn SwitchClickPatternDetectorTrait>,
       layers_navigator: Box<dyn LayersNavigatorTrait>,
       quick_lookup_window: Box<dyn QuickLookupWindowTrait>,
       mouse_cursor_move_detector: Box<dyn CardinalLeversMoveDetectorTrait>,
       mouse_scroll_detector: Box<dyn CardinalLeversMoveDetectorTrait>,
    ) -> Self {
        Gamepad{
            gilrs_events,
            layers: layers_source,
            switch_click_pattern_detector,

            layers_navigator,
            quick_lookup_window,
            mouse_cursor_move_detector,
            mouse_scroll_detector,
        }
    }


    pub fn tick(&mut self) -> Option<InputEvent> {

        let next_event_opt = self.switch_click_pattern_detector.tick();

        if let Some(next_event) = next_event_opt.clone() {
            self.layers_navigator
                .process_current_potential_visit(
                next_event);
        }

        match next_event_opt.clone() {
            Some(SwitchClickPattern::Click(switch)) => {
                match self.get_switch_event_and_reaction(switch.clone())
                            .and_then(|s_e_a_r| s_e_a_r.on_click) {
                    Some(SwitchOnClickReaction::Keyboard(keyboard_input)) 
                    => return Some(InputEvent::KeyClick(keyboard_input)),
                    Some(SwitchOnClickReaction::MoveToLayer(layer_specifier))
                    => self.layers_navigator.move_to_layer(layer_specifier),
                    Some(SwitchOnClickReaction::VisitLayer(layer_specifier))
                    => self.layers_navigator.visit_layer(LayerVisitTrigger::Click(switch),layer_specifier),
                    Some(SwitchOnClickReaction::MoveToOrVisitLayer(layer_specifier))
                    => self.layers_navigator.move_to_or_visit_layer(LayerVisitTrigger::Click(switch),layer_specifier),
                    Some(SwitchOnClickReaction::ShowQuickLookupWindow)
                    => {let _ = self.quick_lookup_window.show_or_open(switch);}
                    _ => ()
                }

            },
            Some(SwitchClickPattern::ClickAndHold(switch)) => {
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
                    => self.layers_navigator.visit_layer(LayerVisitTrigger::DoubleClick(switch),layer_specifier),
                    Some(SwitchOnClickReaction::MoveToOrVisitLayer(layer_specifier))
                    => self.layers_navigator.move_to_or_visit_layer(LayerVisitTrigger::DoubleClick(switch),layer_specifier),
                    Some(SwitchOnClickReaction::ShowQuickLookupWindow)
                    => {let _ = self.quick_lookup_window.show_or_open(switch);}
                    _ => ()
                }
            },
            Some(SwitchClickPattern::DoubleClickAndHold(switch)) => {
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
                self.layers_navigator.undo_last_layer_visit_with_switch(switch.clone());
                let _ = self.quick_lookup_window.hide(switch);
                return Some(InputEvent::KeyUp);
            }
            None => (),
        };

        if let Some(new_layer_index) 
            = self.layers_navigator.consumable_get_current_layer_index() {
            let _ = self
                .quick_lookup_window
                .update(new_layer_index);

            if let Some(CardinalLevers { left_stick, right_stick }) 
                = &self.layers[new_layer_index].cardinal_levers {
                self.mouse_cursor_move_detector.set_mouse_controls(
                    match left_stick {
                        Some(SingleCardinalLever::ControlMouseCursor(
                           mouse_control)) 
                        => Some(mouse_control.clone()),
                        _ => None,
                    },
                    match right_stick {
                        Some(SingleCardinalLever::ControlMouseCursor(
                           mouse_control)) 
                        => Some(mouse_control.clone()),
                        _ => None,
                    });


                self.mouse_scroll_detector.set_mouse_controls(
                    match left_stick {
                        Some(SingleCardinalLever::ControlMouseScrollwheel(
                           mouse_control)) 
                        => Some(mouse_control.clone()),
                        _ => None,
                    },
                    match right_stick {
                        Some(SingleCardinalLever::ControlMouseScrollwheel(
                           mouse_control)) 
                        => Some(mouse_control.clone()),
                        _ => None,
                    });
            }
            else {
                self.mouse_cursor_move_detector.set_mouse_controls(None,None);
                self.mouse_scroll_detector.set_mouse_controls(None,None);
            }

        }

        if let Some((x,y)) = self.mouse_cursor_move_detector.tick() {
            return Some(InputEvent::MoveMouseCursor(x,y))
        }
        if let Some((x,y)) = self.mouse_scroll_detector.tick() {
            return Some(InputEvent::MouseScroll(x,y))
        }

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
                    self.mouse_cursor_move_detector.axis_changed(axis,value);
                    self.mouse_scroll_detector.axis_changed(axis,value);
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
                Button::LeftTrigger2 => switches.left_trigger_2.clone(),
                Button::RightTrigger2 => switches.right_trigger_2.clone(),
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
    MoveMouseCursor(i32,i32),
    MouseScroll(i32,i32),
    KeyUp,
}

// #[derive(Debug, PartialEq)]
// pub struct LayerNodeRef{
//     pub id: String,
//     pub index: usize,
// }

#[derive(Debug,Clone,PartialEq)]
pub enum Switch {
    Button(Button),
    StickSwitchButton(StickSwitchButton),
}
