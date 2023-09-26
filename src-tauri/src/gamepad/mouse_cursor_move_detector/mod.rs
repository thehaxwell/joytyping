use crate::settings::data::{CardinalLevers, ControlMouseCursorFunction, SingleCardinalLever};

pub trait MouseCursorMoveDetectorTrait {
    fn load_cardinal_levers(&mut self, cardinal_levers: Option<CardinalLevers>);
    fn tick(&mut self) -> Option<(i32,i32)>;
    fn axis_changed(&mut self, axis: gilrs::ev::Axis, value: f32);
}

pub struct MouseCursorMoveDetector {
    left_deadzone_upper_limit: Option<f32>,
    right_deadzone_upper_limit: Option<f32>,

    current_left_stick_x: Option<i32>,
    current_left_stick_y: Option<i32>,
    current_right_stick_x: Option<i32>,
    current_right_stick_y: Option<i32>,
}

impl MouseCursorMoveDetector {
    pub fn new(
    ) -> Self {
        Self {
            left_deadzone_upper_limit: None,
            right_deadzone_upper_limit: None,
            current_left_stick_x: None,
            current_left_stick_y: None,
            current_right_stick_x: None,
            current_right_stick_y: None,
        }
    }
}

const SCALE_FACTOR: f32 = 10.0;
impl MouseCursorMoveDetectorTrait for MouseCursorMoveDetector {
    fn load_cardinal_levers(&mut self, cardinal_levers: Option<CardinalLevers>) {
        if let Some(CardinalLevers { left_stick, right_stick }) 
            = cardinal_levers {
            self.left_deadzone_upper_limit = match left_stick {
                Some(SingleCardinalLever::ControlMouseCursor(
                   ControlMouseCursorFunction{deadzone_upper_limit})) 
                => Some(deadzone_upper_limit),
                _ => None,
            };

            self.right_deadzone_upper_limit = match right_stick {
                Some(SingleCardinalLever::ControlMouseCursor(
                   ControlMouseCursorFunction{deadzone_upper_limit})) 
                => Some(deadzone_upper_limit),
                _ => None,
            };
        }
    }

    fn axis_changed(&mut self, axis: gilrs::ev::Axis, value: f32){
        let new_move_abs_opt = match axis {
            gilrs::ev::Axis::LeftStickX | gilrs::ev::Axis::LeftStickY 
               => if let Some(limit) = self.left_deadzone_upper_limit {
                       Some(if value.abs() >= limit.abs() {
                          (value * SCALE_FACTOR).round() as i32
                       } else { 0 })
                   }
                   else { None },
            gilrs::ev::Axis::RightStickX | gilrs::ev::Axis::RightStickY 
               => if let Some(limit) = self.right_deadzone_upper_limit {
                       Some(if value.abs() >= limit.abs() {
                          (value * SCALE_FACTOR).round() as i32
                       } else { 0 })
                   }
                   else { None },
            _ => None,
        };

        if let Some(new_move_abs) = new_move_abs_opt {
            match axis {
                gilrs::ev::Axis::LeftStickX 
                => self.current_left_stick_x = Some(new_move_abs),
                gilrs::ev::Axis::LeftStickY 
                => self.current_left_stick_y = Some(-new_move_abs),
                gilrs::ev::Axis::RightStickX 
                => self.current_right_stick_x = Some(new_move_abs),
                gilrs::ev::Axis::RightStickY 
                => self.current_right_stick_y = Some(-new_move_abs),
                _ => (),
            }
        };
    }

    fn tick(&mut self) -> Option<(i32,i32)> {
        if let Some(x) = self.current_left_stick_x {
            if let Some(y) = self.current_left_stick_y {
                return Some((x,y))
            }
        }

        if let Some(x) = self.current_right_stick_x {
            if let Some(y) = self.current_right_stick_y {
                return Some((x,y))
            }
        }

        None
    }
}
