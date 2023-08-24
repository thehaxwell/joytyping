use stepper::StepperButtonTrait;
use std::time::{Duration, Instant};

use crate::{settings_data::KeyClickConfig, gamepad::CustomButton};

use self::{keys_config::KeysConfig, input_controller::{InputController, InputControllerTrait}};

#[cfg(test)]
mod tests;

// the steppers are gamepads r1 and l1
// buttons. They are called steppers because
// they allow the user to navigate between steps
pub mod stepper;

pub mod keys_config;

pub mod input_controller;

pub struct JoyKeyboard {
    input_controller: Box<dyn InputControllerTrait>,
    r1_stepper_button: Box<dyn StepperButtonTrait>,
    l1_stepper_button: Box<dyn StepperButtonTrait>,
    current_step: Step,
    current_layer: Layer,
    duration_threshold_to_count_move_to_layer_as_visit: Duration,

    key_mappings: KeysConfig,
}

impl JoyKeyboard {
    pub fn new(input_controller: Box<dyn InputControllerTrait>,
               r1_stepper_button: Box<dyn StepperButtonTrait>,
               l1_stepper_button: Box<dyn StepperButtonTrait>,
    key_mappings: KeysConfig,
               ) -> JoyKeyboard {
        JoyKeyboard {
            input_controller,
            r1_stepper_button,
            l1_stepper_button,
            current_step: Step::Step1,
            current_layer: Layer::First,
            duration_threshold_to_count_move_to_layer_as_visit: Duration::from_millis(500),
            key_mappings,
        }
    }
    
    pub fn get_current_step(&self)-> Step {
        self.current_step
    }

    pub fn get_current_layer(&self)-> Layer {
        self.current_layer
    }

    pub fn button_pressed(&mut self, button: CustomButton){
        let gamepad_key = match button{
            CustomButton::Base(gilrs::Button::North) => {
                self.key_mappings.north
            },
            CustomButton::Base(gilrs::Button::East) => {
                self.key_mappings.east
            },
            CustomButton::Base(gilrs::Button::South) => {
                self.key_mappings.south
            },
            CustomButton::Base(gilrs::Button::West) => {
                self.key_mappings.west
            },
            CustomButton::Base(gilrs::Button::DPadUp) => {
                self.key_mappings.d_pad_up
            },
            CustomButton::Base(gilrs::Button::DPadDown) => {
                self.key_mappings.d_pad_down
            },
            CustomButton::Base(gilrs::Button::DPadLeft) => {
                self.key_mappings.d_pad_left
            },
            CustomButton::Base(gilrs::Button::DPadRight) => {
                self.key_mappings.d_pad_right
            },
            CustomButton::LeftStickUp => {
                self.key_mappings.left_stick_up
            },
            CustomButton::LeftStickDown => {
                self.key_mappings.left_stick_down
            },
            CustomButton::LeftStickLeft => {
                self.key_mappings.left_stick_left
            },
            CustomButton::LeftStickRight => {
                self.key_mappings.left_stick_right
            },
            CustomButton::RightStickUp => {
                self.key_mappings.right_stick_up
            },
            CustomButton::RightStickDown => {
                self.key_mappings.right_stick_down
            },
            CustomButton::RightStickLeft => {
                self.key_mappings.right_stick_left
            },
            CustomButton::RightStickRight => {
                self.key_mappings.right_stick_right
            },
            _ => return (),

        };

        let key_to_click = match self.current_layer {
            Layer::First | Layer::VisitingFirst(_)
                => match self.current_step {
                Step::Step1 => {
                    gamepad_key.first_layer_step_1
                },
                Step::Step2 => {
                    gamepad_key.first_layer_step_2
                },
                Step::Step3 => {
                    gamepad_key.first_layer_step_3
                },
                Step::Step4 => {
                    gamepad_key.first_layer_step_4
                },
            },
            Layer::Second | Layer::VisitingSecond(_)
                => match self.current_step {
                Step::Step1 => {
                    gamepad_key.second_layer_step_1
                },
                Step::Step2 => {
                    gamepad_key.second_layer_step_2
                },
                Step::Step3 => {
                    gamepad_key.second_layer_step_3
                },
                Step::Step4 => {
                    gamepad_key.second_layer_step_4
                },
            },
        };

        self.input_controller.key_down(key_to_click);
    }

    pub fn button_released(&mut self) {
        self.input_controller.key_up();
    }

    pub fn trigger_input(&mut self) {
        self.input_controller.trigger_input();
    }

    pub fn set_r1_mod_is_down(&mut self, is_down: bool) {
        self.input_controller.key_up();
        let double_clicked = self.r1_stepper_button
            .set_is_down_and_return_is_double_click(is_down);
        self.update_current_layer(StepperButtonDirection::Right, is_down, double_clicked,);
        self.update_current_step();
    }

    pub fn set_l1_mod_is_down(&mut self, is_down: bool) {
        self.input_controller.key_up();
        let double_clicked = self.l1_stepper_button
            .set_is_down_and_return_is_double_click(is_down);
        self.update_current_layer(StepperButtonDirection::Left, is_down, double_clicked,);
        self.update_current_step();
    }

    fn time_since_visit_counts_as_layer_switch(&self,time_since_visit: Instant) -> bool {
        time_since_visit.elapsed()
            < self.duration_threshold_to_count_move_to_layer_as_visit
    }

    fn update_current_layer(&mut self, stepper_direction: StepperButtonDirection, is_down: bool, just_double_clicked: bool) {
        match &self.current_layer {
            Layer::First => if  just_double_clicked {
                println!("-----> double click and hold");
                self.current_layer = Layer::VisitingSecond(LayerVisitDetails::new(stepper_direction))
            },
            Layer::Second => if  just_double_clicked {
                println!("-----> double click and hold");
                self.current_layer = Layer::VisitingFirst(LayerVisitDetails::new(stepper_direction))
            },
            Layer::VisitingFirst(s)
                => if s.visit_through_stepper_at_direction==stepper_direction && !is_down {
                if self.time_since_visit_counts_as_layer_switch(
                    s.time_of_visit) {
                    println!("-----> switch layers");
                    self.current_layer = Layer::First;
                }
                else {
                    println!("-----> was just visiting...");
                    self.current_layer = Layer::Second;
                }
            },
            Layer::VisitingSecond(s)
                => if s.visit_through_stepper_at_direction==stepper_direction && !is_down {
                if self.time_since_visit_counts_as_layer_switch(
                    s.time_of_visit) {
                    println!("-----> switch layers");
                    self.current_layer = Layer::Second;
                }
                else {
                    println!("-----> was just visiting...");
                    self.current_layer = Layer::First;
                }
            },
        };
    }

    fn update_current_step(&mut self) {
        let r1_is_down = self.r1_stepper_button.get_is_down()
            // the rest of these conditions are to only register
            // r1-down if it isn't being used to visit a layer
            && if let Layer::VisitingFirst(details) | Layer::VisitingSecond(details)
             = &self.current_layer {
                 details.visit_through_stepper_at_direction !=StepperButtonDirection::Right
             } else {true};


        let l1_is_down = self.l1_stepper_button.get_is_down()
            // the rest of these conditions are to only register
            // l1-down if it isn't being used to visit a layer
            && if let Layer::VisitingFirst(details) | Layer::VisitingSecond(details)
             = &self.current_layer {
                 details.visit_through_stepper_at_direction !=StepperButtonDirection::Left
             } else {true};

        self.current_step = if r1_is_down
            && !l1_is_down {
            Step::Step2
        }
        else if !r1_is_down 
            && l1_is_down {
            Step::Step3
        }
        else if r1_is_down 
            && l1_is_down {
            Step::Step4
        }
        else {
            Step::Step1
        };
    }

}

pub struct GamepadKeyConfig {
    pub first_layer_step_1: KeyClickConfig,
    pub first_layer_step_2: KeyClickConfig,
    pub first_layer_step_3: KeyClickConfig,
    pub first_layer_step_4: KeyClickConfig,
    pub second_layer_step_1: KeyClickConfig,
    pub second_layer_step_2: KeyClickConfig,
    pub second_layer_step_3: KeyClickConfig,
    pub second_layer_step_4: KeyClickConfig,
}

#[derive(Debug,PartialEq,Clone,Copy)]
pub enum Step {
    Step1,
    Step2,
    Step3,
    Step4,
}

#[derive(Debug,PartialEq,Clone,Copy)]
pub enum StepperButtonDirection {
    Right,
    Left,
}

#[derive(Debug,PartialEq,Clone,Copy)]
pub struct LayerVisitDetails {
    time_of_visit: std::time::Instant,
    visit_through_stepper_at_direction: StepperButtonDirection,
}
impl LayerVisitDetails {
    pub fn new(stepper_button: StepperButtonDirection) -> Self {
        LayerVisitDetails {
            time_of_visit: std::time::Instant::now(),
            visit_through_stepper_at_direction: stepper_button,
        }
    }
}

#[derive(Debug,PartialEq,Clone,Copy)]
pub enum Layer {
    First,
    VisitingFirst(LayerVisitDetails),
    Second,
    VisitingSecond(LayerVisitDetails),
}



