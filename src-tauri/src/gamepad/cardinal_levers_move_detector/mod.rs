use crate::models::main_config::DeadzoneUpperLimits;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg(test)]
mod tests;

pub mod mouse;

#[cfg_attr(test, automock)]
pub trait CardinalLeversMoveDetectorTrait {
    // fn set_mouse_controls(&mut self,
    //     left_mouse_control: Option<MouseControl>,
    //     right_mouse_control: Option<MouseControl>, );
    fn activate_levers(&mut self,
        left_lever: bool,
        right_lever: bool,);
    fn tick(&mut self) -> Option<(i32,i32)>;
    fn axis_changed(&mut self, axis: gilrs::ev::Axis, value: f32);
}

pub struct CardinalLeversMoveDetector {
    // left_mouse_control: Option<MouseControl>,
    // right_mouse_control: Option<MouseControl>,

    left_lever_is_active: bool,
    right_lever_is_active: bool,

    deadzone_upper_limits: DeadzoneUpperLimits,
    scale_factor: f32,

    current_left_stick_x: Option<i32>,
    current_left_stick_y: Option<i32>,
    current_right_stick_x: Option<i32>,
    current_right_stick_y: Option<i32>,

    left_stick_at_rest_counter: u8,
    right_stick_at_rest_counter: u8,
}

impl CardinalLeversMoveDetector {
    pub fn new(
        deadzone_upper_limits: DeadzoneUpperLimits,
        scale_factor: f32,
    ) -> Self {
        Self {
            deadzone_upper_limits,
            scale_factor,
            // left_mouse_control: None,
            // right_mouse_control: None,
            left_lever_is_active: false,
            right_lever_is_active: false,
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
    // fn set_mouse_controls(&mut self,
    //     left_mouse_control: Option<MouseControl>,
    //     right_mouse_control: Option<MouseControl>, ) {
    //     self.left_mouse_control = left_mouse_control;
    //     self.current_left_stick_x = None;
    //     self.current_left_stick_y = None;
    //
    //     self.right_mouse_control = right_mouse_control;
    //     self.current_right_stick_x = None;
    //     self.current_right_stick_y = None;
    // }

    fn activate_levers(&mut self,
        left_lever: bool,
        right_lever: bool,) {
        self.left_lever_is_active = left_lever;
        self.current_left_stick_x = None;
        self.current_left_stick_y = None;

        self.right_lever_is_active = right_lever;
        self.current_right_stick_x = None;
        self.current_right_stick_y = None;
    }

    fn axis_changed(&mut self, axis: gilrs::ev::Axis, value: f32){
        match axis {
            gilrs::ev::Axis::LeftStickX
               => if self.left_lever_is_active {
                        self.current_left_stick_x 
                            = Some(calc(self.scale_factor, self.deadzone_upper_limits.left_stick, value));
                        if self.current_left_stick_y.is_none() {
                            self.current_left_stick_y = Some(0);
                        }
                   }
            gilrs::ev::Axis::LeftStickY 
               => if self.left_lever_is_active {
                        self.current_left_stick_y 
                            = Some(-calc(self.scale_factor, self.deadzone_upper_limits.left_stick, value));
                        if self.current_left_stick_x.is_none() {
                            self.current_left_stick_x = Some(0);
                        }
                   }
            gilrs::ev::Axis::RightStickX
               => if self.right_lever_is_active {
                        self.current_right_stick_x 
                            = Some(calc(self.scale_factor, self.deadzone_upper_limits.right_stick, value));
                        if self.current_right_stick_y.is_none() {
                            self.current_right_stick_y = Some(0);
                        }
                   }
            gilrs::ev::Axis::RightStickY 
               => if self.right_lever_is_active {
                        self.current_right_stick_y 
                            = Some(-calc(self.scale_factor, self.deadzone_upper_limits.right_stick, value));
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

fn calc(scale_factor: f32,
        deadzone_upper_limit: f32,
        value: f32) -> i32 {
    if value.abs() >= deadzone_upper_limit.abs() {
       (value * scale_factor).round() as i32
    } else { 0 }
}

