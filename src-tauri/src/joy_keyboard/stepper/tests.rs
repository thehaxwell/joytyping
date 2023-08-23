use crate::joy_keyboard::stepper::{StepperButton, StepperButtonTrait};

#[test]
fn double_click_works() {
    let mut stepper = StepperButton::new();
    assert_eq!(stepper.set_is_down_and_return_is_double_click(true),false);
    assert_eq!(stepper.is_down,true);
    assert_eq!(stepper.set_is_down_and_return_is_double_click(false),false);
    assert_eq!(stepper.is_down,false);
    assert_eq!(stepper.set_is_down_and_return_is_double_click(true),true);
    assert_eq!(stepper.is_down,true);
}
