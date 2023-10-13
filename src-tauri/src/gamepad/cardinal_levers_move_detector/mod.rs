use crate::settings::data::MouseControl;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg(test)]
mod tests;

pub mod mouse;

#[cfg_attr(test, automock)]
pub trait CardinalLeversMoveDetectorTrait {
    fn set_mouse_controls(&mut self,
        left_mouse_control: Option<MouseControl>,
        right_mouse_control: Option<MouseControl>, );
    fn tick(&mut self) -> Option<(i32,i32)>;
    fn axis_changed(&mut self, axis: gilrs::ev::Axis, value: f32);
}

pub struct CardinalLeversMoveDetector {
    left_mouse_control: Option<MouseControl>,
    right_mouse_control: Option<MouseControl>,

    current_left_stick_x: Option<i32>,
    current_left_stick_y: Option<i32>,
    current_right_stick_x: Option<i32>,
    current_right_stick_y: Option<i32>,

    left_stick_at_rest_counter: u8,
    right_stick_at_rest_counter: u8,
}

impl CardinalLeversMoveDetector {
    pub fn new(
    ) -> Self {
        Self {
            left_mouse_control: None,
            right_mouse_control: None,
            current_left_stick_x: None,
            current_left_stick_y: None,
            current_right_stick_x: None,
            current_right_stick_y: None,
            left_stick_at_rest_counter: 0,
            right_stick_at_rest_counter: 0,
        }
    }
}

const EMIT_RESTING_POSITION_EVENT_LIMIT: u8 = 5;
impl CardinalLeversMoveDetectorTrait for CardinalLeversMoveDetector {
    fn set_mouse_controls(&mut self,
        left_mouse_control: Option<MouseControl>,
        right_mouse_control: Option<MouseControl>, ) {
        self.left_mouse_control = left_mouse_control;
        self.current_left_stick_x = None;
        self.current_left_stick_y = None;

        self.right_mouse_control = right_mouse_control;
        self.current_right_stick_x = None;
        self.current_right_stick_y = None;
    }

    fn axis_changed(&mut self, axis: gilrs::ev::Axis, value: f32){
        match axis {
            gilrs::ev::Axis::LeftStickX
               => if let Some(mouse_control) 
                      = &self.left_mouse_control {
                        self.current_left_stick_x 
                            = Some(calc(&mouse_control, value));
                        if self.current_left_stick_y.is_none() {
                            self.current_left_stick_y = Some(0);
                        }
                   }
            gilrs::ev::Axis::LeftStickY 
               => if let Some(mouse_control) 
                      = &self.left_mouse_control {
                        self.current_left_stick_y 
                            = Some(-calc(&mouse_control, value));
                        if self.current_left_stick_x.is_none() {
                            self.current_left_stick_x = Some(0);
                        }
                   }
            gilrs::ev::Axis::RightStickX
               => if let Some(mouse_control) 
                      = &self.right_mouse_control {
                        self.current_right_stick_x 
                            = Some(calc(&mouse_control, value));
                        if self.current_right_stick_y.is_none() {
                            self.current_right_stick_y = Some(0);
                        }
                   }
            gilrs::ev::Axis::RightStickY 
               => if let Some(mouse_control) 
                      = &self.right_mouse_control {
                        self.current_right_stick_y 
                            = Some(-calc(&mouse_control, value));
                        if self.current_right_stick_x.is_none() {
                            self.current_right_stick_x = Some(0);
                        }
                   }
            _ => (),
        }
    }

    fn tick(&mut self) -> Option<(i32,i32)> {
        if let Some(x) = self.current_left_stick_x {
            if let Some(y) = self.current_left_stick_y {
                if x == 0 && y == 0 {
                    if self.left_stick_at_rest_counter 
                        < EMIT_RESTING_POSITION_EVENT_LIMIT {
                        self.left_stick_at_rest_counter +=  1;
                    }
                }
                else {
                    self.left_stick_at_rest_counter = 0;
                }

                if self.left_stick_at_rest_counter
                    < EMIT_RESTING_POSITION_EVENT_LIMIT {
                    return Some((x,y))
                }
            }
        }

        if let Some(x) = self.current_right_stick_x {
            if let Some(y) = self.current_right_stick_y {
                if x == 0 && y == 0 {
                    if self.right_stick_at_rest_counter 
                        < EMIT_RESTING_POSITION_EVENT_LIMIT {
                        self.right_stick_at_rest_counter += 1;
                    }
                }
                else {
                    self.right_stick_at_rest_counter = 0;
                }

                if self.right_stick_at_rest_counter
                    < EMIT_RESTING_POSITION_EVENT_LIMIT {
                    return Some((x,y))
                }
            }
        }

        None
    }
}

fn calc(mouse_control: &MouseControl, value: f32) -> i32 {
    if value.abs() >= mouse_control.deadzone_upper_limit.abs() {
       (value * mouse_control.scale_factor).round() as i32
    } else { 0 }
}

