use crate::LeftOrRight;
use crate::models;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg(test)]
mod tests;

#[cfg_attr(test, automock)]
pub trait StickSwitchInterpreterTrait {
    fn interpret_stick_move (
        &mut self,
        x_axis_value: Option<f32>,
        y_axis_value: Option<f32>) -> Option<StickSwitchEvent>;
}

pub struct StickSwitchInterpreter {
    latest_button_pressed: Option<StickSwitchButton>,
    cardinal_custom_buttons: CardinalCustomButtons,
    axis_click_thresholds: AxisClickThresholds,
}

#[cfg_attr(test, automock)]
impl StickSwitchInterpreter {
    pub fn new(
        axis_click_thresholds: AxisClickThresholds,
        cardinal_custom_buttons: CardinalCustomButtons,
        ) -> StickSwitchInterpreter{

        StickSwitchInterpreter {
                cardinal_custom_buttons,
                axis_click_thresholds,
            latest_button_pressed: None,
        }
    }

    pub fn interpret_move(&mut self, x_axis: f32, y_axis: f32)-> Option<StickSwitchButton> {
        if x_axis.abs() > y_axis.abs() {
            if x_axis > self.axis_click_thresholds.right {
                return Some(self.cardinal_custom_buttons.right);
            }
            else if x_axis < self.axis_click_thresholds.left * -1.0 {
                return Some(self.cardinal_custom_buttons.left);
            }
        }
        else {
            if y_axis < self.axis_click_thresholds.down * -1.0{
                return Some(self.cardinal_custom_buttons.down);
            }
            else if y_axis > self.axis_click_thresholds.up {
                return Some(self.cardinal_custom_buttons.up);
            }

        }

        return None;
    }
}

impl StickSwitchInterpreterTrait for StickSwitchInterpreter {
    fn interpret_stick_move (
        &mut self,
        x_axis_value: Option<f32>,
        y_axis_value: Option<f32>) -> Option<StickSwitchEvent>{
        let x_value =
            if let Some(data) = x_axis_value {data} else {0.0};
        let y_value =
            if let Some(data) = y_axis_value {data} else {0.0};

        let button_pressed = 
            self.interpret_move(x_value,y_value);

        if let Some(key) = button_pressed {
            if let Some(latest) = self.latest_button_pressed {
                if latest == key {
                    // look at previous inputted button to avoid
                    // spamming a key while the stick is in the range
                    return None;

                }
            }
            self.latest_button_pressed = Some(key);
            Some(StickSwitchEvent::ButtonPressed(key))
        }
        else if let Some(latest) = self.latest_button_pressed {
            // If no key is down now, but there was
            // a key down before, then register its release
            self.latest_button_pressed = None;
            Some(StickSwitchEvent::ButtonReleased(latest))
        }
        else {
            None
        }
    }
}

#[derive(Debug,PartialEq)]
pub struct CardinalCustomButtons {
    pub up: StickSwitchButton,
    pub right: StickSwitchButton,
    pub down: StickSwitchButton,
    pub left: StickSwitchButton
}

#[derive(Debug,PartialEq)]
pub struct AxisClickThresholds {
    pub up: f32,
    pub right: f32,
    pub down: f32,
    pub left: f32,
    pub alignment: LeftOrRight,
}

impl AxisClickThresholds {
    pub fn get_from_setting(
        thresholds: models::main_config::StickSwitchesClickThresholds, alignment: LeftOrRight
        ) -> Self {
        if alignment == LeftOrRight::Left {
            Self {
                up: thresholds.left_stick_up,
                down: thresholds.left_stick_down,
                left: thresholds.left_stick_left,
                right: thresholds.left_stick_right,
                alignment,
            }
        }
        else {
            Self {
                up: thresholds.right_stick_up,
                down: thresholds.right_stick_down,
                left: thresholds.right_stick_left,
                right: thresholds.right_stick_right,
                alignment,
            }
        }
    }
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StickSwitchButton {
    LeftStickUp,
    LeftStickDown,
    LeftStickLeft,
    LeftStickRight,
    RightStickUp,
    RightStickDown,
    RightStickLeft,
    RightStickRight,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StickSwitchEvent {
    ButtonPressed(StickSwitchButton),
    ButtonReleased(StickSwitchButton),
}
