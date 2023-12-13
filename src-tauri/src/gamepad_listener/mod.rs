use enigo::MouseButton;
use gilrs::Button;

use crate::{settings::models::layout::{SwitchOnClickReaction, KeyboardInput}, quick_lookup_window::QuickLookupWindowTrait};

use self::{gilrs_events::{gilrs_wrapper::GilrsEventType, GilrsEventsTrait,stick_switch_interpreter::{StickSwitchButton,StickSwitchEvent}}, layers_navigator::{LayersNavigatorTrait, LayerVisitTrigger}, layers_wrapper::LayersWrapperTrait};

use self::switch_click_pattern_detector::{SwitchClickPatternDetectorTrait, SwitchClickPattern};

pub mod gilrs_events;
pub mod switch_click_pattern_detector;
pub mod layers_navigator;
pub mod cardinal_levers_move_detector;
pub mod layers_wrapper;

#[cfg(test)]
mod tests;


pub struct GamepadListener {
   gilrs_events: Box<dyn GilrsEventsTrait>,
   layers: Box<dyn LayersWrapperTrait>,
   switch_click_pattern_detector: Box<dyn SwitchClickPatternDetectorTrait>,
   layers_navigator: Box<dyn LayersNavigatorTrait>,
   mouse_cardinal_levers_move_detector: Box<dyn cardinal_levers_move_detector::mouse::MouseTrait>,
}

impl GamepadListener {
    pub fn new(
       gilrs_events: Box<dyn GilrsEventsTrait>,
       layers: Box<dyn LayersWrapperTrait>,
       switch_click_pattern_detector: Box<dyn SwitchClickPatternDetectorTrait>,
       layers_navigator: Box<dyn LayersNavigatorTrait>,
       mouse_cardinal_levers_move_detector: Box<dyn cardinal_levers_move_detector::mouse::MouseTrait>,
    ) -> Self {
        GamepadListener{
            gilrs_events,
            layers,
            switch_click_pattern_detector,

            layers_navigator,
            mouse_cardinal_levers_move_detector,
        }
    }


    pub fn next_command(&mut self) -> Option<Command> {
        let next_event_opt = self.switch_click_pattern_detector.tick();

        if let Some(next_event) = next_event_opt.clone() {
            self.layers_navigator
                .process_current_potential_visit(
                next_event);
        }

        match next_event_opt.clone() {
            Some(SwitchClickPattern::Click(switch)) => {
                match self.layers.get_switch_event_and_reaction(
                         self.layers_navigator.get_current_layer_index(),
                         switch.clone())
                       .and_then(|s_e_a_r| s_e_a_r.on_click) {
                    Some(SwitchOnClickReaction::Keyboard(keyboard_input)) 
                    => return Some(Command::InputEvent(InputEvent::KeyClick(keyboard_input))),
                    Some(SwitchOnClickReaction::Mouse(mouse_input)) 
                    => return Some(Command::InputEvent(InputEvent::MouseDown(mouse_input.button))),
                    Some(SwitchOnClickReaction::MoveToLayer(layer_specifier))
                    => self.layers_navigator.move_to_layer(layer_specifier),
                    Some(SwitchOnClickReaction::VisitLayer(layer_specifier))
                    => self.layers_navigator.visit_layer(LayerVisitTrigger::Click(switch),layer_specifier),
                    Some(SwitchOnClickReaction::MoveToOrVisitLayer(layer_specifier))
                    => self.layers_navigator.move_to_or_visit_layer(LayerVisitTrigger::Click(switch),layer_specifier),
                    Some(SwitchOnClickReaction::ShowQuickLookupWindowOnHold)
                    => return Some(Command::QuickLookupWindowEvent(QuickLookupWindowEvent::ShowUntilSwitchKeyup(switch))),
                    Some(SwitchOnClickReaction::ToggleQuickLookupWindow)
                    => return Some(Command::QuickLookupWindowEvent(QuickLookupWindowEvent::ToggleBySwitch(switch))),
                    Some(SwitchOnClickReaction::BoostMouseCursorByMultiplier(mul))
                    => return Some(Command::InputEvent(InputEvent::BoostMouseCursor(mul))),
                    _ => ()
                }

            },
            Some(SwitchClickPattern::ClickAndHold(switch)) => {
                if let Some(s_e_a_r)
                    = self.layers.get_switch_event_and_reaction(
                         self.layers_navigator.get_current_layer_index(),
                         switch.clone()) {
                    // if on_click is set to
                    // type out some key, then hold down that key
                    //
                    // Interesting application: 
                    //
                    // Since SwitchClickPattern::ClickAndHold is fired a moment
                    // after SwitchClickPattern::Click, if the switch has
                    // been set to be a keyboard input on_click
                    // GamepadListener will fire InputEvent::KeyClick
                    // event once, take a break, and InputEvent::KeyDown
                    // (which tell KeyboardInputController to fire the key in
                    // rapid-fire style). 
                    //
                    // This gives the effect for key clicks
                    // similar to how the system keyboard works when the user
                    // clicks and holds a alpha-numeric key.
                    // Here's a visualisation of the effect
                    //
                    //  *Key press* |.............|..|..|..|..|..| *Key Release*
                    //
                    if let Some(SwitchOnClickReaction::Keyboard(keyboard_input)) 
                        = s_e_a_r.on_click {
                        return Some(Command::InputEvent(InputEvent::KeyDown(keyboard_input)))
                    }
                };
            },
            Some(SwitchClickPattern::DoubleClick(switch)) => {
                match self.layers.get_switch_event_and_reaction(
                         self.layers_navigator.get_current_layer_index(),
                         switch.clone())
                       .and_then(
                           // this is useful to allow typing a letter twice fast,
                           // like "oo" in "look","book","too" etc.
                           // the first click will be Click and the second DoubleClick
                           |s_e_a_r| s_e_a_r.on_double_click
                                            .or_else(|| s_e_a_r.on_click )) {
                    Some(SwitchOnClickReaction::Keyboard(keyboard_input)) 
                    => return Some(Command::InputEvent(InputEvent::KeyClick(keyboard_input))),
                    Some(SwitchOnClickReaction::Mouse(mouse_input)) 
                    => return Some(Command::InputEvent(InputEvent::MouseDown(mouse_input.button))),
                    Some(SwitchOnClickReaction::MoveToLayer(layer_specifier))
                    => self.layers_navigator.move_to_layer(layer_specifier),
                    Some(SwitchOnClickReaction::VisitLayer(layer_specifier))
                    => self.layers_navigator.visit_layer(LayerVisitTrigger::DoubleClick(switch),layer_specifier),
                    Some(SwitchOnClickReaction::MoveToOrVisitLayer(layer_specifier))
                    => self.layers_navigator.move_to_or_visit_layer(LayerVisitTrigger::DoubleClick(switch),layer_specifier),
                    Some(SwitchOnClickReaction::ShowQuickLookupWindowOnHold)
                    => return Some(Command::QuickLookupWindowEvent(QuickLookupWindowEvent::ShowUntilSwitchKeyup(switch))),
                    Some(SwitchOnClickReaction::ToggleQuickLookupWindow)
                    => return Some(Command::QuickLookupWindowEvent(QuickLookupWindowEvent::ToggleBySwitch(switch))),
                    Some(SwitchOnClickReaction::BoostMouseCursorByMultiplier(mul))
                    => return Some(Command::InputEvent(InputEvent::BoostMouseCursor(mul))),
                    _ => ()
                }
            },
            Some(SwitchClickPattern::DoubleClickAndHold(switch)) => {
                match self.layers.get_switch_event_and_reaction(
                     self.layers_navigator.get_current_layer_index(),
                     switch.clone())
                    .and_then(
                        // if on_double_click (or fallback to on_click) is set to
                        // type out some key, then hold down that key
                           |s_e_a_r| s_e_a_r.on_double_click
                                            .or_else(|| s_e_a_r.on_click )
                        ) {
                    Some(SwitchOnClickReaction::Keyboard(keyboard_input)) 
                        => return Some(Command::InputEvent(InputEvent::KeyDown(keyboard_input))),
                    _ => (),

                };
            },
            Some(SwitchClickPattern::ClickEnd(switch)) => {
                self.layers_navigator.undo_last_layer_visit_with_switch(switch.clone());
                // return Some(Command::QuickLookupWindowEvent(QuickLookupWindowEvent::HideIfMatchesSwitch(switch)));
                // return Some(Command::InputEvent(InputEvent::KeyUp));
                return Some(Command::KeyUp(switch));
            }
            None => (),
        };

        if let Some(new_layer_index) 
            = self.layers_navigator.consumable_get_current_layer_index() {

            self.mouse_cardinal_levers_move_detector
                .set_mouse_controls(self.layers.get_cardinal_levers(new_layer_index));

            return Some(Command::QuickLookupWindowEvent(QuickLookupWindowEvent::EmitCurrentLayerNotification(new_layer_index)));
        }

        self.mouse_cardinal_levers_move_detector
            .tick()
            .map(|input_event| Command::InputEvent(input_event))
    }

    // returns true if there is yet another event
    pub fn next_event(&mut self) -> Option<GilrsEventType> {
        if let Some(event) = self.gilrs_events.next() {
            match event.event {
                GilrsEventType::ButtonPressed(button, ) => {
                    print!("ButtonPressed: {:?}\n",button);
                    self.switch_click_pattern_detector.button_pressed(button);
                }
                GilrsEventType::ButtonRepeated(button, ) => {
                    print!("ButtonRepeated: {:?}\n",button);
                },
                GilrsEventType::ButtonReleased(button, ) => {
                    print!("ButtonReleased: {:?}\n",button);
                    self.switch_click_pattern_detector.button_released(button);
                },
                GilrsEventType::ButtonChanged(button, _value) => {
                    print!("ButtonChanged: {:?}\n",button);
                },
                GilrsEventType::AxisChanged(axis, value, stick_switch_event_opt) => {
                    print!("AxisChanged: {:?}: {:?}\n",axis,value);
                    self.mouse_cardinal_levers_move_detector.axis_changed(axis,value);
                    if let Some(event) = stick_switch_event_opt {
                        match event {
                            StickSwitchEvent::ButtonPressed(button)
                                => self.switch_click_pattern_detector.axis_button_pressed(button),
                            StickSwitchEvent::ButtonReleased(button)
                                => self.switch_click_pattern_detector.axis_button_released(button),
                        };
                    }
                },
                ref _other => (),
            }
            Some(event.event)
        }
        else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    InputEvent(InputEvent),
    QuickLookupWindowEvent(QuickLookupWindowEvent),
    KeyUp(Switch),
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputEvent {
    KeyClick(KeyboardInput),
    KeyDown(KeyboardInput),
    MouseDown(MouseButton),
    MoveMouseCursor(i32,i32),
    MouseScroll(i32,i32),
    KeyUp,
    BoostMouseCursor(u32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuickLookupWindowEvent {
    ShowUntilSwitchKeyup(Switch),
    EmitCurrentLayerNotification(usize),
    ToggleBySwitch(Switch),
}

#[derive(Debug,Clone,PartialEq)]
pub enum Switch {
    Button(Button),
    StickSwitchButton(StickSwitchButton),
}
