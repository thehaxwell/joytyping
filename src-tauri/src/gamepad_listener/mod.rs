use gilrs::Button;

use crate::{settings::models::layout::SwitchOnClickReaction, quick_lookup_window, input_controller::InputControllerTrait};

use self::{gilrs_events::{gilrs_wrapper::GilrsEventType, GilrsEventsTrait,stick_switch_interpreter::{StickSwitchButton,StickSwitchEvent}}, layers_navigator::{LayersNavigatorTrait, LayerVisitTrigger}, cardinal_levers_move_detector::mouse::MouseEvent};

use self::switch_click_pattern_detector::{SwitchClickPatternDetectorTrait, SwitchClickPattern};

pub mod gilrs_events;
pub mod switch_click_pattern_detector;
pub mod layers_navigator;
pub mod cardinal_levers_move_detector;

#[cfg(test)]
mod tests;

pub struct GamepadListener {
   gilrs_events: Box<dyn GilrsEventsTrait>,
   switch_click_pattern_detector: Box<dyn SwitchClickPatternDetectorTrait>,
   layers_navigator: Box<dyn layers_navigator::controller::ControllerTrait>,
   mouse_cardinal_levers_move_detector: Box<dyn cardinal_levers_move_detector::mouse::MouseTrait>,

   // controllers
   quick_lookup_window: Box<dyn quick_lookup_window::controller::ControllerTrait>,
   input: Box<dyn InputControllerTrait>,
}

impl GamepadListener {
    pub fn new(
       gilrs_events: Box<dyn GilrsEventsTrait>,
       switch_click_pattern_detector: Box<dyn SwitchClickPatternDetectorTrait>,
       layers_navigator: Box<dyn layers_navigator::controller::ControllerTrait>,
       mouse_cardinal_levers_move_detector: Box<dyn cardinal_levers_move_detector::mouse::MouseTrait>,

       // controllers
       quick_lookup_window: Box<dyn quick_lookup_window::controller::ControllerTrait>,
       input: Box<dyn InputControllerTrait>,
    ) -> Self {
        GamepadListener{
            gilrs_events,
            switch_click_pattern_detector,

            layers_navigator,
            mouse_cardinal_levers_move_detector,

            quick_lookup_window,
            input,
        }
    }


    pub fn next_command(&mut self) {
        let next_event_opt = self.switch_click_pattern_detector.tick();

        let reaction = self.layers_navigator.process_switch_event(next_event_opt.clone());

        match next_event_opt.clone() {
            Some(SwitchClickPattern::Click(switch)) 
                | Some(SwitchClickPattern::DoubleClick(switch)) => {
                match reaction {
                    Some(SwitchOnClickReaction::Keyboard(keyboard_input)) 
                    => self.input.key_click(keyboard_input),
                    Some(SwitchOnClickReaction::Mouse(mouse_input)) 
                    => self.input.mouse_down(mouse_input.button),
                    Some(SwitchOnClickReaction::ShowQuickLookupWindowOnHold)
                    => {let _ = self.quick_lookup_window.show_until_switch_keyup(switch);},
                    Some(SwitchOnClickReaction::ToggleQuickLookupWindow)
                    => {let _ = self.quick_lookup_window.toggle_by_switch(switch);},
                    Some(SwitchOnClickReaction::BoostMouseCursorByMultiplier(mul))
                    => self.input.boost_mouse_cursor(mul),
                    _ => ()
                }

            }
            Some(SwitchClickPattern::ClickAndHold(_switch)) 
                | Some(SwitchClickPattern::DoubleClickAndHold(_switch)) => {
                match reaction {
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
                    Some(SwitchOnClickReaction::Keyboard(keyboard_input) )
                       // => return Some(Command::InputEvent(InputEvent::KeyDown(keyboard_input))),
                       => self.input.key_down(keyboard_input),
                    _other => (),
                }
            },
            Some(SwitchClickPattern::ClickEnd(switch)) => {
                let _ = self.quick_lookup_window.react_to_keyup(switch.clone());
                self.input.key_up();
            }
            None => (),

        };

        if let Some(new_layer_index) 
            = self.layers_navigator.consumable_get_current_layer_index() {

            self.mouse_cardinal_levers_move_detector
                .set_mouse_controls(self.layers_navigator.get_cardinal_levers());

            let _ = self.quick_lookup_window.emit_current_layer_notification(new_layer_index);
        }

        match self.mouse_cardinal_levers_move_detector.tick() {
            Some(MouseEvent::MoveCursor(x,y)) => self.input.move_mouse_cursor(x,y),
            Some(MouseEvent::Scroll(x,y)) => self.input.mouse_scroll(x,y),
            None => (),
        }

        self.input.trigger_input();
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

// #[derive(Debug, Clone, PartialEq)]
// pub enum Command {
//     InputEvent(InputEvent),
//     KeyUp(Switch),
// }
//
// #[derive(Debug, Clone, PartialEq)]
// pub enum InputEvent {
//     KeyClick(KeyboardInput),
//     KeyDown(KeyboardInput),
//     MouseDown(MouseButton),
//     MoveMouseCursor(i32,i32),
//     MouseScroll(i32,i32),
//     KeyUp,
//     BoostMouseCursor(u32),
// }

#[derive(Debug,Clone,PartialEq)]
pub enum Switch {
    Button(Button),
    StickSwitchButton(StickSwitchButton),
}
