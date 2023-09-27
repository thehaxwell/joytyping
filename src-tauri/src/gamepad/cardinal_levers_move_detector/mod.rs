pub trait CardinalLeversMoveDetectorTrait {
    fn set_deadzone_upper_limits(&mut self,
        left_deadzone_upper_limit: Option<f32>,
        right_deadzone_upper_limit: Option<f32>, );
    fn tick(&mut self) -> Option<(i32,i32)>;
    fn axis_changed(&mut self, axis: gilrs::ev::Axis, value: f32);
}

pub struct CardinalLeversMoveDetector {
    left_deadzone_upper_limit: Option<f32>,
    right_deadzone_upper_limit: Option<f32>,

    current_left_stick_x: Option<i32>,
    current_left_stick_y: Option<i32>,
    current_right_stick_x: Option<i32>,
    current_right_stick_y: Option<i32>,
}

impl CardinalLeversMoveDetector {
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
impl CardinalLeversMoveDetectorTrait for CardinalLeversMoveDetector {
    fn set_deadzone_upper_limits(&mut self,
        left_deadzone_upper_limit: Option<f32>,
        right_deadzone_upper_limit: Option<f32>, ) {
        self.left_deadzone_upper_limit = left_deadzone_upper_limit;
        self.right_deadzone_upper_limit = right_deadzone_upper_limit;
    }

    fn axis_changed(&mut self, axis: gilrs::ev::Axis, value: f32){
        match axis {
            gilrs::ev::Axis::LeftStickX
               => self.current_left_stick_x
                  =if let Some(limit) = self.left_deadzone_upper_limit {
                       Some(if value.abs() >= limit.abs() {
                          (value * SCALE_FACTOR).round() as i32
                       } else { 0 })
                   }
                   else { None },
            gilrs::ev::Axis::LeftStickY 
               => self.current_left_stick_y
                  =if let Some(limit) = self.left_deadzone_upper_limit {
                       Some(if value.abs() >= limit.abs() {
                          -(value * SCALE_FACTOR).round() as i32
                       } else { 0 })
                   }
                   else { None },
            gilrs::ev::Axis::RightStickX
               => self.current_right_stick_x
                  =if let Some(limit) = self.right_deadzone_upper_limit {
                       Some(if value.abs() >= limit.abs() {
                          (value * SCALE_FACTOR).round() as i32
                       } else { 0 })
                   }
                   else { None },
            gilrs::ev::Axis::RightStickY 
               => self.current_right_stick_y
                  =if let Some(limit) = self.right_deadzone_upper_limit {
                       Some(if value.abs() >= limit.abs() {
                          -(value * SCALE_FACTOR).round() as i32
                       } else { 0 })
                   }
                   else { None },
            _ => (),
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
