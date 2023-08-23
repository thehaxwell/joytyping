use crate::LeftOrRight;
use crate::gamepad::GamepadEvent;
use crate::gamepad::CustomButton;
use crate::settings_data;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg(test)]
mod tests;

#[cfg_attr(test, automock)]
pub trait SticksInterpreterTrait {
    fn interpret_left_stick_move (
        &mut self,
        x_axis_value: Option<f32>,
        y_axis_value: Option<f32>) -> Option<GamepadEvent>;

    fn interpret_right_stick_move (
        &mut self,
        x_axis_value: Option<f32>,
        y_axis_value: Option<f32>) -> Option<GamepadEvent>;
}

pub struct SticksInterpreter {
    right_stick_interpreter: StickInterpreter,
    left_stick_interpreter: StickInterpreter,
    latest_left_button_pressed: Option<CustomButton>,
    latest_right_button_pressed: Option<CustomButton>,
}

#[cfg_attr(test, automock)]
impl SticksInterpreter {
    pub fn new(first_axis_click_thresholds: AxisClickThresholds,
               second_axis_click_thresholds: AxisClickThresholds,
               ) -> SticksInterpreter{
        // Sorting out the parameters as the left and right
       let (right_axis_click_thresholds, left_axis_click_thresholds)
           = if first_axis_click_thresholds.alignment == LeftOrRight::Left
               && second_axis_click_thresholds.alignment == LeftOrRight::Right {
                (second_axis_click_thresholds, first_axis_click_thresholds)
           }
           else if first_axis_click_thresholds.alignment == LeftOrRight::Right
               && second_axis_click_thresholds.alignment == LeftOrRight::Left {
                (first_axis_click_thresholds, second_axis_click_thresholds)
           }
           else {
                panic!("Invalid alignment combination: expected left-right or right-left")
           };

        SticksInterpreter {
            right_stick_interpreter: StickInterpreter::new(
                CardinalCustomButtons {
                    up: CustomButton::RightStickUp,
                    right: CustomButton::RightStickRight,
                    down: CustomButton::RightStickDown,
                    left: CustomButton::RightStickLeft
                },
                right_axis_click_thresholds,
            ),
            left_stick_interpreter: StickInterpreter::new(
                CardinalCustomButtons {
                    up: CustomButton::LeftStickUp,
                    right: CustomButton::LeftStickRight,
                    down: CustomButton::LeftStickDown,
                    left: CustomButton::LeftStickLeft
                },
                left_axis_click_thresholds,
            ),
            latest_left_button_pressed: None,
            latest_right_button_pressed: None,
        }
    }

    fn interpret_stick_move_wrapper(
        &mut self, is_left_stick: bool,
        x_axis_value: Option<f32>,
        y_axis_value: Option<f32>)-> Option<GamepadEvent> {
        let x_value =
            if let Some(data) = x_axis_value {data} else {0.0};
        let y_value =
            if let Some(data) = y_axis_value {data} else {0.0};
        self.interpret_stick_move(is_left_stick,x_value,y_value)
    }

    // The difference with this one is that is
    // processes x and y coordinates together to
    // eliminate possible race conditions
    fn interpret_stick_move(
        &mut self, is_left_stick: bool,
        x_value: f32,
        y_value: f32)-> Option<GamepadEvent> {
        if is_left_stick {
                let button_pressed = 
                    self.left_stick_interpreter
                    .interpret_move(x_value,y_value);

                if let Some(key) = button_pressed {
                    if let Some(latest) = self.latest_left_button_pressed {
                        if latest == key {
                            // look at previous inputted button to avoid
                            // spamming a key while the stick is in the range
                            return None;

                        }
                    }
                    self.latest_left_button_pressed = Some(key);
                    Some(GamepadEvent::ButtonPressed(key))
                }
                else if let Some(latest) = self.latest_left_button_pressed {
                    // If no key is down now, but there was
                    // a key down before, then register its release
                    self.latest_left_button_pressed = None;
                    Some(GamepadEvent::ButtonReleased(latest))
                }
                else {
                    None
                }
            }
            else {
                let button_pressed = 
                    self.right_stick_interpreter
                    .interpret_move(x_value,y_value);

                if let Some(key) = button_pressed {
                    if let Some(latest) = self.latest_right_button_pressed {
                        if latest == key {
                            // look at previous inputted button to avoid
                            // spamming a key while the stick is in the range
                            return None;

                        }
                    }
                    self.latest_right_button_pressed = Some(key);
                    Some(GamepadEvent::ButtonPressed(key))
                }
                else if let Some(latest) = self.latest_right_button_pressed {
                    // If no key is down now, but there was
                    // a key down before, then register its release
                    self.latest_right_button_pressed = None;
                    Some(GamepadEvent::ButtonReleased(latest))
                }
                else {
                    None
                }
            }

    }

}

impl SticksInterpreterTrait for SticksInterpreter {
    fn interpret_left_stick_move (
        &mut self,
        x_axis_value: Option<f32>,
        y_axis_value: Option<f32>) -> Option<GamepadEvent>{
        self.interpret_stick_move_wrapper(true,x_axis_value,y_axis_value)
    }
    
    fn interpret_right_stick_move (
        &mut self,
        x_axis_value: Option<f32>,
        y_axis_value: Option<f32>) -> Option<GamepadEvent>{
        self.interpret_stick_move_wrapper(false,x_axis_value,y_axis_value)
    }
}

struct CardinalCustomButtons {
    up: CustomButton,
    right: CustomButton,
    down: CustomButton,
    left: CustomButton
}

struct StickInterpreter {
    button: CardinalCustomButtons,
    click_thresholds: AxisClickThresholds,
}

impl StickInterpreter {
    pub fn new(button: CardinalCustomButtons, click_thresholds: AxisClickThresholds) -> StickInterpreter{
        StickInterpreter {
            button,
            click_thresholds,
        }
    }

    pub fn interpret_move(&mut self, x_axis: f32, y_axis: f32)-> Option<CustomButton> {
        if x_axis.abs() > y_axis.abs() {
            if x_axis > self.click_thresholds.right {
                return Some(self.button.right);
            }
            else if x_axis < self.click_thresholds.left * -1.0 {
                return Some(self.button.left);
            }
        }
        else {
            if y_axis > self.click_thresholds.up {
                return Some(self.button.up);
            }
            else if y_axis < self.click_thresholds.down * -1.0 {
                return Some(self.button.down);
            }

        }

        return None;
    }
}

pub struct AxisClickThresholds {
    pub up: f32,
    pub right: f32,
    pub down: f32,
    pub left: f32,
    pub alignment: LeftOrRight,
}

impl AxisClickThresholds {
    pub fn get_from_setting(
        thresholds: settings_data::AxisClickThresholds, alignment: LeftOrRight
        ) -> Self {
        Self {
            up: thresholds.up,
            right: thresholds.right,
            down: thresholds.down,
            left: thresholds.left,
            alignment
        }
    }
}
