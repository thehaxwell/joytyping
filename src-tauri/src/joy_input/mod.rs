use stepper::StepperButtonTrait;
use std::time::{Duration, Instant};

use self::enigo_wrapper::EnigoTrait;

#[cfg(test)]
mod tests;

// the steppers are gamepads r1 and l1
// buttons. They are called steppers because
// they allow the user to navigate between steps
pub mod stepper;

pub mod enigo_wrapper;

pub struct JoyKeyboard {
    enigo: Box<dyn EnigoTrait>,
    r1_stepper_button: Box<dyn StepperButtonTrait>,
    l1_stepper_button: Box<dyn StepperButtonTrait>,
    current_step: Step,
    current_layer: Layer,
    duration_threshold_to_count_move_to_layer_as_visit: Duration,
}

impl JoyKeyboard {
    pub fn new(enigo: Box<dyn EnigoTrait>,
               r1_stepper_button: Box<dyn StepperButtonTrait>,
               l1_stepper_button: Box<dyn StepperButtonTrait>,
               ) -> JoyKeyboard {
        JoyKeyboard {
            enigo,
            r1_stepper_button,
            l1_stepper_button,
            current_step: Step::Step1,
            current_layer: Layer::First,
            duration_threshold_to_count_move_to_layer_as_visit: Duration::from_millis(500),
        }
    }

    pub fn key_click(&mut self, gamepad_key: GamepadKeyConfig,) {
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

        if let Some(key) = key_to_click {
            println!("-> {:?}",key);
            self.enigo.key_click(key);
        }
    }

    pub fn set_r1_mod_is_down(&mut self, is_down: bool) {
        let double_clicked = self.r1_stepper_button
            .set_is_down_and_return_is_double_click(is_down);
        self.update_current_layer(StepperButtonDirection::Right, is_down, double_clicked,);
        self.update_current_step();
    }

    pub fn set_l1_mod_is_down(&mut self, is_down: bool) {
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
    pub first_layer_step_1: Option<enigo::Key>,
    pub first_layer_step_2: Option<enigo::Key>,
    pub first_layer_step_3: Option<enigo::Key>,
    pub first_layer_step_4: Option<enigo::Key>,
    pub second_layer_step_1: Option<enigo::Key>,
    pub second_layer_step_2: Option<enigo::Key>,
    pub second_layer_step_3: Option<enigo::Key>,
    pub second_layer_step_4: Option<enigo::Key>,
}

#[derive(Debug,PartialEq)]
enum Step {
    Step1,
    Step2,
    Step3,
    Step4,
}

#[derive(Debug,PartialEq)]
enum StepperButtonDirection {
    Right,
    Left,
}

#[derive(Debug,PartialEq)]
struct LayerVisitDetails {
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

#[derive(Debug,PartialEq)]
enum Layer {
    First,
    VisitingFirst(LayerVisitDetails),
    Second,
    VisitingSecond(LayerVisitDetails),
}

