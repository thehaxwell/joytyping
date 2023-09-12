use std::time::{Duration, Instant};

use gilrs::Button;

use crate::gamepad::stick_switch_interpreter::StickSwitchButton;

#[cfg(test)]
use mockall::{automock, predicate::*};

use super::Switch;

#[cfg(test)]
mod tests;

const CLICK_HOLD_INTERVAL_THRESHOLD: Duration = Duration::from_millis(500);
const DOUBLE_CLICK_INTERVAL_THRESHOLD: Duration = Duration::from_millis(500);

#[cfg_attr(test, automock)]
pub trait SwitchClickPatternDetectorTrait {
    fn tick(&mut self) -> Option<SwitchClickPattern>;
    fn button_pressed(&mut self, button: Button);
    fn button_released(&mut self, button: Button);
    fn axis_button_pressed(&mut self, button: StickSwitchButton);
    fn axis_button_released(&mut self, button: StickSwitchButton);
}

// READ ME to understand this struct
//
// on_double_click is a sub-event of on_double_click_and_hold.
// on_click is a sub-event of all the rest (on_click_and_hold, on_double_click,
//   and on_double_click_and_hold)
//
// Here's a graphic representation of this
//
//           on_click_and_hold
//          /
// on_click
//          \
//           on_double_click
//                           \
//                            on_double_click_and_hold
//
// where time is flowing from left to right
// and the user can go from on_click and "build up"
// while moving to the left. Here's how this
// "build-up" is done:
//
// -> on_click: 
// key_down (fires immediately)
// -> on_click_and_hold: 
// {on_click} > no key_up for CLICK_HOLD_INTERVAL_THRESHOLD
// -> on_double_click: 
// {on_click} > key_up within 5ms > key_down within DOUBLE_CLICK_INTERVAL_THRESHOLD
// -> on_double_click_and_hold:
// {on_double_click} > no key_up for CLICK_HOLD_INTERVAL_THRESHOLD
//
// The event is stored in a one-space queue where
// the event will be returned by calling tick.
// If another event is sent while another one was
// waiting to be read then the former event will be
// replaced by the latter and lost.
//
// Note that the build-up is reset when
// - a different key is clicked
// - it becomes impossible to get to the next step 
//      (eg. waiting too long between clicks cannot lead
//      to a double click)
// - a leaf in the tree is reached, there is no next step
//
#[derive(Debug,PartialEq)]
pub struct SwitchClickPatternDetector {
    latest_switch_click_pattern: Option<SwitchClickPattern>,
    latest_switch_event: Option<LatestSwitchEvent>,
}

impl SwitchClickPatternDetector {
    pub fn new() -> Self {
        Self {
            latest_switch_click_pattern: None,
            latest_switch_event: None,
        }
    }

    fn switch_pressed(
        &mut self, switch: Switch){
        // if the latest button press qualifies for double_click
        // then let it be so; otherwise register as a click
        // The latter case is coded first and the former second
        let mut new_step = LatestSwitchEvent {
            switch: switch.clone(),
            instant: Instant::now(),
            event_type: SwitchEventType::KeyDownIntoClick,
        };

        if let Some(LatestSwitchEvent { switch: sw, instant, event_type: _ })
            = &self.latest_switch_event {
                if switch == sw.clone() && instant.elapsed() < DOUBLE_CLICK_INTERVAL_THRESHOLD {
                    new_step = LatestSwitchEvent {
                        switch: switch.clone(),
                        instant: Instant::now(),
                        event_type: SwitchEventType::KeyDownIntoDoubleClick,
                    }
                }
        }

        self.latest_switch_event = Some(new_step.clone());

        // the rest is to update self.latest_switch_click_pattern
        // which can only either take on Click or DoubleClick here (fn button_pressed())
        match new_step.event_type {
            SwitchEventType::KeyDownIntoClick
              => self.latest_switch_click_pattern
                 = Some(SwitchClickPattern::Click(switch.clone())),
            SwitchEventType::KeyDownIntoDoubleClick
              => self.latest_switch_click_pattern 
                 = Some(SwitchClickPattern::DoubleClick(switch.clone())),
            _ => (),
        }
    }

    fn switch_released(
        &mut self, switch: Switch){
        if let Some(_event)
            = &self.latest_switch_event {
                self.latest_switch_event = Some(LatestSwitchEvent {
                    switch: switch.clone(),
                    instant: Instant::now(),
                    event_type: SwitchEventType::KeyUp,
                });
        }

        self.latest_switch_click_pattern 
            = Some(SwitchClickPattern::ClickEnd(switch.clone()));
    }
}

impl SwitchClickPatternDetectorTrait for SwitchClickPatternDetector {
    fn tick(&mut self) -> Option<SwitchClickPattern>{
        if let Some(LatestSwitchEvent { switch, instant, event_type }) 
            = &self.latest_switch_event {
            match event_type {
                SwitchEventType::KeyDownIntoClick
                    => {
                        if instant.elapsed() > CLICK_HOLD_INTERVAL_THRESHOLD {
                            self.latest_switch_click_pattern 
                                = Some(SwitchClickPattern::ClickAndHold(
                                        switch.clone()));
                        }
                }
                SwitchEventType::KeyDownIntoDoubleClick
                    => {
                        if instant.elapsed() > CLICK_HOLD_INTERVAL_THRESHOLD {
                            self.latest_switch_click_pattern 
                                = Some(SwitchClickPattern::DoubleClickAndHold(
                                        switch.clone()));
                        }
                }
                SwitchEventType::KeyUp => ()
            }
        }
        let pattern = self.latest_switch_click_pattern.clone();
        self.latest_switch_click_pattern = None;
        pattern
    }

    fn button_pressed(
        &mut self, button: Button){
        self.switch_pressed(Switch::Button(button));
    }

    fn button_released(
        &mut self, button: Button){
        self.switch_released(Switch::Button(button));
    }

    fn axis_button_pressed(
        &mut self, button: StickSwitchButton){
        self.switch_pressed(Switch::StickSwitchButton(button));
    }

    fn axis_button_released(
        &mut self, button: StickSwitchButton){
        self.switch_released(Switch::StickSwitchButton(button));
    }
}

#[derive(Debug,Clone,PartialEq)]
pub enum SwitchClickPattern {
    Click(Switch),
    ClickAndHold(Switch),
    DoubleClick(Switch),
    DoubleClickAndHold(Switch),
    ClickEnd(Switch),
}

#[derive(Debug,Clone,PartialEq)]
struct LatestSwitchEvent {
    switch: Switch,
    instant: Instant,
    event_type: SwitchEventType,
}

#[derive(Debug,Clone,PartialEq)]
enum SwitchEventType {
    KeyDownIntoClick,
    KeyDownIntoDoubleClick,
    KeyUp,
}