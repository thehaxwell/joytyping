use crate::settings::data::{CardinalLevers, ControlMouseCursorFunction, SingleCardinalLever};

pub trait MouseCursorMoveDetectorTrait {
    fn set_cardinal_levers(&mut self, cardinal_levers: Option<CardinalLevers>);
    fn tick(&mut self) -> Option<(i32,i32)>;
    fn axis_changed(&mut self, axis: gilrs::ev::Axis, value: f32);
}

pub struct MouseCursorMoveDetector {
    cardinal_levers_settings: Option<CardinalLevers>,
    // latest_event: Option<(i32,i32)>,

    current_left_stick_x: Option<i32>,
    current_left_stick_y: Option<i32>,
    current_right_stick_x: Option<i32>,
    current_right_stick_y: Option<i32>,
}

impl MouseCursorMoveDetector {
    pub fn new(
    ) -> Self {
        Self {
            cardinal_levers_settings: None,
            // latest_event: None,
            current_left_stick_x: None,
            current_left_stick_y: None,
            current_right_stick_x: None,
            current_right_stick_y: None,
        }
    }
}

const SCALE_FACTOR: f32 = 10.0;
impl MouseCursorMoveDetectorTrait for MouseCursorMoveDetector {
    fn set_cardinal_levers(&mut self, cardinal_levers: Option<CardinalLevers>) {
        self.cardinal_levers_settings = cardinal_levers;
    }

    fn axis_changed(&mut self, axis: gilrs::ev::Axis, value: f32){
        if let Some(CardinalLevers { left_stick, right_stick }) 
            = &self.cardinal_levers_settings {
            match axis {
                gilrs::ev::Axis::LeftStickX 
                    => self.current_left_stick_x 
                        = match left_stick.clone() {
                           Some(SingleCardinalLever::ControlMouseCursor(
                               ControlMouseCursorFunction{deadzone_upper_limit}))
                            // => Some((value - deadzone_upper_limit.x).round() as i32 + SCALE_FACTOR),
                            // => Some(value.round() as i32),
                            => Some((value * SCALE_FACTOR).round() as i32),
                            _ => None
                        },
                gilrs::ev::Axis::LeftStickY
                    => self.current_left_stick_y 
                        = match left_stick.clone() {
                           Some(SingleCardinalLever::ControlMouseCursor(
                               ControlMouseCursorFunction{deadzone_upper_limit}))
                            // => Some((value - deadzone_upper_limit.y).round() as i32 + SCALE_FACTOR),
                            // => Some(-value.round() as i32),
                            => Some(-(value * SCALE_FACTOR).round() as i32),
                            _ => None
                        },
                gilrs::ev::Axis::RightStickX 
                    => self.current_right_stick_x 
                        = match right_stick.clone() {
                           Some(SingleCardinalLever::ControlMouseCursor(
                               ControlMouseCursorFunction{deadzone_upper_limit}))
                            // => Some((value - deadzone_upper_limit.x).round() as i32 + SCALE_FACTOR),
                            // => Some(value.round() as i32),
                            => Some((value * SCALE_FACTOR).round() as i32),
                            _ => None
                        },
                gilrs::ev::Axis::RightStickY
                    => self.current_right_stick_y 
                        = match right_stick.clone() {
                           Some(SingleCardinalLever::ControlMouseCursor(
                               ControlMouseCursorFunction{deadzone_upper_limit}))
                            // => Some((value - center_at.y).round() as i32 + SCALE_FACTOR),
                            // => Some(-value.round() as i32),
                            => Some(-(value * SCALE_FACTOR).round() as i32),
                            _ => None
                        },
                _ => (),
            }
        }
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
