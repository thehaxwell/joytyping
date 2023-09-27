use crate::settings::data::MouseControl;

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
        }
    }
}

impl CardinalLeversMoveDetectorTrait for CardinalLeversMoveDetector {
    fn set_mouse_controls(&mut self,
        left_mouse_control: Option<MouseControl>,
        right_mouse_control: Option<MouseControl>, ) {
        self.left_mouse_control = left_mouse_control;
        self.right_mouse_control = right_mouse_control;
    }

    fn axis_changed(&mut self, axis: gilrs::ev::Axis, value: f32){
        match axis {
            gilrs::ev::Axis::LeftStickX
               => self.current_left_stick_x
                  =if let Some(MouseControl { deadzone_upper_limit, scale_factor }) 
                      = self.left_mouse_control {
                       Some(if value.abs() >= deadzone_upper_limit.abs() {
                          (value * scale_factor).round() as i32
                       } else { 0 })
                   }
                   else { None },
            gilrs::ev::Axis::LeftStickY 
               => self.current_left_stick_y
                  =if let Some(MouseControl { deadzone_upper_limit, scale_factor }) 
                      = self.left_mouse_control {
                       Some(if value.abs() >= deadzone_upper_limit.abs() {
                          -(value * scale_factor).round() as i32
                       } else { 0 })
                   }
                   else { None },
            gilrs::ev::Axis::RightStickX
               => self.current_right_stick_x
                  =if let Some(MouseControl { deadzone_upper_limit, scale_factor }) 
                      = self.left_mouse_control {
                       Some(if value.abs() >= deadzone_upper_limit.abs() {
                          (value * scale_factor).round() as i32
                       } else { 0 })
                   }
                   else { None },
            gilrs::ev::Axis::RightStickY 
               => self.current_right_stick_y
                  =if let Some(MouseControl { deadzone_upper_limit, scale_factor }) 
                      = self.left_mouse_control {
                       Some(if value.abs() >= deadzone_upper_limit.abs() {
                          -(value * scale_factor).round() as i32
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
