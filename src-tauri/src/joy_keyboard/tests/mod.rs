use crate::gamepad::CustomButton;
use crate::joy_keyboard::{JoyKeyboard, Step, Layer};

use crate::joy_keyboard::enigo_wrapper::MockEnigoTrait;
use crate::joy_keyboard::stepper::MockStepperButtonTrait;
use crate::joy_keyboard::StepperButtonDirection;

use self::joy_keyboard_keys_config::joy_keyboard_keys_config;

use gilrs::Button;
use mockall::predicate::*;

#[cfg(test)]
mod joy_keyboard_keys_config;

fn assert_just_now(time: std::time::Instant){
    assert!(time.elapsed().as_secs() < 1);
}

//--------------
//  KEYS MATCH LAYER AND STEP
//--------------

fn setup_key_click_hard_coded_matching_test(
    layer: Layer, step: Step, gamepad_key_config: Option<enigo::Key>){
    let mut mock_enigo = MockEnigoTrait::new();

    if let Some(config) = gamepad_key_config {
        mock_enigo
            .expect_key_click()
            .with(eq(config))
            .return_const(());
    }
    else {
        mock_enigo
            .expect_key_click()
            .times(0);
    }

    let mock_right_stepper_button = MockStepperButtonTrait::new();
    let mock_left_stepper_button = MockStepperButtonTrait::new();
    let mut joy_keyboard = JoyKeyboard::new(
        Box::new(mock_enigo),
        Box::new(mock_right_stepper_button),
        Box::new(mock_left_stepper_button),
        joy_keyboard_keys_config()
    );

    joy_keyboard.current_step = step;
    joy_keyboard.current_layer = layer;

    joy_keyboard.button_pressed(CustomButton::Base(Button::South));
}

#[test]
fn key_click_hard_coded_layer_1_step_1() {
    setup_key_click_hard_coded_matching_test(
        Layer::First,
        Step::Step1,
        joy_keyboard_keys_config().south.first_layer_step_1.key);
}

#[test]
fn key_click_hard_coded_layer_1_step_2() {
    setup_key_click_hard_coded_matching_test(
        Layer::First,
        Step::Step2,
        joy_keyboard_keys_config().south.first_layer_step_2.key);
}

#[test]
fn key_click_hard_coded_layer_1_step_3() {
    setup_key_click_hard_coded_matching_test(
        Layer::First,
        Step::Step3,
        joy_keyboard_keys_config().south.first_layer_step_3.key);
}

#[test]
fn key_click_hard_coded_layer_2_step_1() {
    setup_key_click_hard_coded_matching_test(
        Layer::Second,
        Step::Step1,
        joy_keyboard_keys_config().south.second_layer_step_1.key);
}

#[test]
fn key_click_hard_coded_layer_2_step_2() {
    setup_key_click_hard_coded_matching_test(
        Layer::Second,
        Step::Step2,
        joy_keyboard_keys_config().south.second_layer_step_2.key);
}

#[test]
fn key_click_hard_coded_layer_2_step_3() {
    setup_key_click_hard_coded_matching_test(
        Layer::Second,
        Step::Step3,
        joy_keyboard_keys_config().south.second_layer_step_3.key);
}

//--------------
//  STEPS MOVEMENTS ON LAYER 1
//--------------

#[test]
fn set_r1_mod_is_down_layer_1_step_1() {
    const R1_IS_DOWN: bool = false;
    const L1_IS_DOWN: bool = false;

    let mock_enigo = MockEnigoTrait::new();

    let mut mock_right_stepper_button = MockStepperButtonTrait::new();
    mock_right_stepper_button
        .expect_set_is_down_and_return_is_double_click()
        .with(eq(R1_IS_DOWN))
        .return_const(false);
    mock_right_stepper_button
        .expect_get_is_down()
        .return_const(R1_IS_DOWN);

    let mut mock_left_stepper_button = MockStepperButtonTrait::new();
    mock_left_stepper_button
        .expect_get_is_down()
        .return_const(L1_IS_DOWN);

    let mut joy_keyboard = JoyKeyboard::new(
        Box::new(mock_enigo),
        Box::new(mock_right_stepper_button),
        Box::new(mock_left_stepper_button),
        joy_keyboard_keys_config()
    );

    joy_keyboard.set_r1_mod_is_down(R1_IS_DOWN);
    assert_eq!(joy_keyboard.current_step,Step::Step1);
    assert_eq!(joy_keyboard.current_layer,Layer::First);
}

#[test]
fn set_r1_mod_is_down_layer_1_step_2() {
    const R1_IS_DOWN: bool = true;
    const L1_IS_DOWN: bool = false;

    let mock_enigo = MockEnigoTrait::new();

    let mut mock_right_stepper_button = MockStepperButtonTrait::new();
    mock_right_stepper_button
        .expect_set_is_down_and_return_is_double_click()
        .with(eq(R1_IS_DOWN))
        .return_const(false);
    mock_right_stepper_button
        .expect_get_is_down()
        .return_const(R1_IS_DOWN);

    let mut mock_left_stepper_button = MockStepperButtonTrait::new();
    mock_left_stepper_button
        .expect_get_is_down()
        .return_const(L1_IS_DOWN);

    let mut joy_keyboard = JoyKeyboard::new(
        Box::new(mock_enigo),
        Box::new(mock_right_stepper_button),
        Box::new(mock_left_stepper_button),
        joy_keyboard_keys_config()
    );

    joy_keyboard.set_r1_mod_is_down(R1_IS_DOWN);
    assert_eq!(joy_keyboard.current_step,Step::Step2);
    assert_eq!(joy_keyboard.current_layer,Layer::First);
}

#[test]
fn set_l1_mod_is_down_layer_1_step_3() {
    const R1_IS_DOWN: bool = false;
    const L1_IS_DOWN: bool = true;

    let mock_enigo = MockEnigoTrait::new();

    let mut mock_right_stepper_button = MockStepperButtonTrait::new();
    mock_right_stepper_button
        .expect_get_is_down()
        .return_const(R1_IS_DOWN);

    let mut mock_left_stepper_button = MockStepperButtonTrait::new();
    mock_left_stepper_button
        .expect_set_is_down_and_return_is_double_click()
        .with(eq(L1_IS_DOWN))
        .return_const(false);
    mock_left_stepper_button
        .expect_get_is_down()
        .return_const(L1_IS_DOWN);

    let mut joy_keyboard = JoyKeyboard::new(
        Box::new(mock_enigo),
        Box::new(mock_right_stepper_button),
        Box::new(mock_left_stepper_button),
        joy_keyboard_keys_config()
    );

    joy_keyboard.set_l1_mod_is_down(L1_IS_DOWN);
    assert_eq!(joy_keyboard.current_step,Step::Step3);
    assert_eq!(joy_keyboard.current_layer,Layer::First);
}

#[test]
fn set_r1_and_l1_mod_is_down_layer_1_step_4() {
    const R1_IS_DOWN: bool = true;
    const L1_IS_DOWN: bool = true;

    let mock_enigo = MockEnigoTrait::new();

    let mut mock_right_stepper_button = MockStepperButtonTrait::new();
    mock_right_stepper_button
        .expect_set_is_down_and_return_is_double_click()
        .with(eq(R1_IS_DOWN))
        .return_const(false);
    mock_right_stepper_button
        .expect_get_is_down()
        .return_const(R1_IS_DOWN);

    let mut mock_left_stepper_button = MockStepperButtonTrait::new();
    mock_left_stepper_button
        .expect_set_is_down_and_return_is_double_click()
        .with(eq(L1_IS_DOWN))
        .return_const(false);
    mock_left_stepper_button
        .expect_get_is_down()
        .return_const(L1_IS_DOWN);

    let mut joy_keyboard = JoyKeyboard::new(
        Box::new(mock_enigo),
        Box::new(mock_right_stepper_button),
        Box::new(mock_left_stepper_button),
        joy_keyboard_keys_config()
    );

    joy_keyboard.set_r1_mod_is_down(R1_IS_DOWN);
    joy_keyboard.set_l1_mod_is_down(L1_IS_DOWN);
    assert_eq!(joy_keyboard.current_step,Step::Step4);
    assert_eq!(joy_keyboard.current_layer,Layer::First);
}

//--------------
//  STEPS MOVEMENTS ON LAYER 2
//--------------

#[test]
fn set_r1_mod_is_down_layer_2_step_1() {
    const R1_IS_DOWN: bool = false;
    const L1_IS_DOWN: bool = false;

    let mock_enigo = MockEnigoTrait::new();

    let mut mock_right_stepper_button = MockStepperButtonTrait::new();
    mock_right_stepper_button
        .expect_set_is_down_and_return_is_double_click()
        .with(eq(R1_IS_DOWN))
        .return_const(false);
    mock_right_stepper_button
        .expect_get_is_down()
        .return_const(R1_IS_DOWN);

    let mut mock_left_stepper_button = MockStepperButtonTrait::new();
    mock_left_stepper_button
        .expect_get_is_down()
        .return_const(L1_IS_DOWN);

    let mut joy_keyboard = JoyKeyboard::new(
        Box::new(mock_enigo),
        Box::new(mock_right_stepper_button),
        Box::new(mock_left_stepper_button),
        joy_keyboard_keys_config()
    );

    joy_keyboard.current_layer = Layer::Second;

    joy_keyboard.set_r1_mod_is_down(R1_IS_DOWN);
    assert_eq!(joy_keyboard.current_step,Step::Step1);
    assert_eq!(joy_keyboard.current_layer,Layer::Second);
}

#[test]
fn set_r1_mod_is_down_layer_2_step_2() {
    const R1_IS_DOWN: bool = true;
    const L1_IS_DOWN: bool = false;

    let mock_enigo = MockEnigoTrait::new();

    let mut mock_right_stepper_button = MockStepperButtonTrait::new();
    mock_right_stepper_button
        .expect_set_is_down_and_return_is_double_click()
        .with(eq(R1_IS_DOWN))
        .return_const(false);
    mock_right_stepper_button
        .expect_get_is_down()
        .return_const(R1_IS_DOWN);

    let mut mock_left_stepper_button = MockStepperButtonTrait::new();
    mock_left_stepper_button
        .expect_get_is_down()
        .return_const(L1_IS_DOWN);

    let mut joy_keyboard = JoyKeyboard::new(
        Box::new(mock_enigo),
        Box::new(mock_right_stepper_button),
        Box::new(mock_left_stepper_button),
        joy_keyboard_keys_config()
    );

    joy_keyboard.current_layer = Layer::Second;

    joy_keyboard.set_r1_mod_is_down(R1_IS_DOWN);
    assert_eq!(joy_keyboard.current_step,Step::Step2);
    assert_eq!(joy_keyboard.current_layer,Layer::Second);
}

#[test]
fn set_l1_mod_is_down_layer_2_step_3() {
    const R1_IS_DOWN: bool = false;
    const L1_IS_DOWN: bool = true;

    let mock_enigo = MockEnigoTrait::new();

    let mut mock_right_stepper_button = MockStepperButtonTrait::new();
    mock_right_stepper_button
        .expect_get_is_down()
        .return_const(R1_IS_DOWN);

    let mut mock_left_stepper_button = MockStepperButtonTrait::new();
    mock_left_stepper_button
        .expect_set_is_down_and_return_is_double_click()
        .with(eq(L1_IS_DOWN))
        .return_const(false);
    mock_left_stepper_button
        .expect_get_is_down()
        .return_const(L1_IS_DOWN);

    let mut joy_keyboard = JoyKeyboard::new(
        Box::new(mock_enigo),
        Box::new(mock_right_stepper_button),
        Box::new(mock_left_stepper_button),
        joy_keyboard_keys_config()
    );

    joy_keyboard.current_layer = Layer::Second;

    joy_keyboard.set_l1_mod_is_down(L1_IS_DOWN);
    assert_eq!(joy_keyboard.current_step,Step::Step3);
    assert_eq!(joy_keyboard.current_layer,Layer::Second);
}

#[test]
fn set_r1_and_l1_mod_is_down_layer_2_step_4() {
    const R1_IS_DOWN: bool = true;
    const L1_IS_DOWN: bool = true;

    let mock_enigo = MockEnigoTrait::new();

    let mut mock_right_stepper_button = MockStepperButtonTrait::new();
    mock_right_stepper_button
        .expect_set_is_down_and_return_is_double_click()
        .with(eq(R1_IS_DOWN))
        .return_const(false);
    mock_right_stepper_button
        .expect_get_is_down()
        .return_const(R1_IS_DOWN);

    let mut mock_left_stepper_button = MockStepperButtonTrait::new();
    mock_left_stepper_button
        .expect_set_is_down_and_return_is_double_click()
        .with(eq(L1_IS_DOWN))
        .return_const(false);
    mock_left_stepper_button
        .expect_get_is_down()
        .return_const(L1_IS_DOWN);

    let mut joy_keyboard = JoyKeyboard::new(
        Box::new(mock_enigo),
        Box::new(mock_right_stepper_button),
        Box::new(mock_left_stepper_button),
        joy_keyboard_keys_config()
    );

    joy_keyboard.current_layer = Layer::Second;

    joy_keyboard.set_r1_mod_is_down(R1_IS_DOWN);
    joy_keyboard.set_l1_mod_is_down(L1_IS_DOWN);
    assert_eq!(joy_keyboard.current_step,Step::Step4);
    assert_eq!(joy_keyboard.current_layer,Layer::Second);
}

//--------------
// STEPS MOVEMENT
// -------------

#[test]
fn use_r1_mod_to_visit_layer_2_step_1() {
    const R1_IS_DOWN: bool = true;
    const L1_IS_DOWN: bool = false;

    let mock_enigo = MockEnigoTrait::new();

    let mut mock_right_stepper_button = MockStepperButtonTrait::new();
    mock_right_stepper_button
        .expect_set_is_down_and_return_is_double_click()
        .with(eq(R1_IS_DOWN))
        .return_const(true);
    mock_right_stepper_button
        .expect_get_is_down()
        .return_const(R1_IS_DOWN);

    let mut mock_left_stepper_button = MockStepperButtonTrait::new();
    mock_left_stepper_button
        .expect_get_is_down()
        .return_const(L1_IS_DOWN);

    let mut joy_keyboard = JoyKeyboard::new(
        Box::new(mock_enigo),
        Box::new(mock_right_stepper_button),
        Box::new(mock_left_stepper_button),
        joy_keyboard_keys_config()
    );

    joy_keyboard.set_r1_mod_is_down(R1_IS_DOWN);

    assert_eq!(joy_keyboard.current_step,Step::Step1);

    if let Layer::VisitingSecond(details)
        = joy_keyboard.current_layer {
        assert_eq!(details.visit_through_stepper_at_direction,
                   StepperButtonDirection::Right);
        assert_just_now(details.time_of_visit);
    } else {
        panic!("Expected layer to be VisitingSecond");
    }
}

#[test]
fn use_l1_mod_to_visit_layer_2_step_1() {
    const R1_IS_DOWN: bool = false;
    const L1_IS_DOWN: bool = true;

    let mock_enigo = MockEnigoTrait::new();

    let mut mock_right_stepper_button = MockStepperButtonTrait::new();
    mock_right_stepper_button
        .expect_get_is_down()
        .return_const(R1_IS_DOWN);

    let mut mock_left_stepper_button = MockStepperButtonTrait::new();
    mock_left_stepper_button
        .expect_set_is_down_and_return_is_double_click()
        .with(eq(L1_IS_DOWN))
        .return_const(true);
    mock_left_stepper_button
        .expect_get_is_down()
        .return_const(L1_IS_DOWN);

    let mut joy_keyboard = JoyKeyboard::new(
        Box::new(mock_enigo),
        Box::new(mock_right_stepper_button),
        Box::new(mock_left_stepper_button),
        joy_keyboard_keys_config()
    );

    joy_keyboard.set_l1_mod_is_down(L1_IS_DOWN);
    
    assert_eq!(joy_keyboard.current_step,Step::Step1);

    if let Layer::VisitingSecond(details)
        = joy_keyboard.current_layer {
        assert_eq!(details.visit_through_stepper_at_direction,
                   StepperButtonDirection::Left);
        assert_just_now(details.time_of_visit);
    } else {
        panic!("Expected layer to be VisitingSecond");
    }
}

#[test]
fn use_l1_mod_to_visit_layer_2_and_r1_to_go_to_step_2() {
    const L1_IS_DOWN: bool = true;

    let mock_enigo = MockEnigoTrait::new();

    let mut mock_right_stepper_button = MockStepperButtonTrait::new();
    mock_right_stepper_button
        .expect_set_is_down_and_return_is_double_click()
        .with(eq(true))
        .return_const(false);

    let mut first_invocation = true;
    mock_right_stepper_button
        .expect_get_is_down()
        .times(2)
        .returning(move|| {
            if first_invocation {
                first_invocation = false;
                false
            } else {
                true
            }
        });

    let mut mock_left_stepper_button = MockStepperButtonTrait::new();
    mock_left_stepper_button
        .expect_set_is_down_and_return_is_double_click()
        .with(eq(L1_IS_DOWN))
        .return_const(true);
    mock_left_stepper_button
        .expect_get_is_down()
        .return_const(L1_IS_DOWN);

    let mut joy_keyboard = JoyKeyboard::new(
        Box::new(mock_enigo),
        Box::new(mock_right_stepper_button),
        Box::new(mock_left_stepper_button),
        joy_keyboard_keys_config()
    );

    joy_keyboard.set_l1_mod_is_down(L1_IS_DOWN);
    
    assert_eq!(joy_keyboard.current_step,Step::Step1);

    if let Layer::VisitingSecond(details)
        = &joy_keyboard.current_layer {
        assert_eq!(details.visit_through_stepper_at_direction,
                   StepperButtonDirection::Left);
        assert_just_now(details.time_of_visit);
    } else {
        panic!("Expected layer to be VisitingSecond");
    }

    // go to step 2 still on Visiting second layer
    joy_keyboard.set_r1_mod_is_down(true);
    assert_eq!(joy_keyboard.current_step,Step::Step2);
    if let Layer::VisitingSecond(details)
        = &joy_keyboard.current_layer {
        assert_eq!(details.visit_through_stepper_at_direction,
                   StepperButtonDirection::Left);
        assert_just_now(details.time_of_visit);
    } else {
        panic!("Expected layer to be VisitingSecond");
    }
}

#[test]
fn use_r1_mod_to_visit_layer_2_and_l1_to_go_to_step_3() {
    const R1_IS_DOWN: bool = true;

    let mock_enigo = MockEnigoTrait::new();

    let mut mock_right_stepper_button = MockStepperButtonTrait::new();
    mock_right_stepper_button
        .expect_set_is_down_and_return_is_double_click()
        .with(eq(R1_IS_DOWN))
        .return_const(true);
    mock_right_stepper_button
        .expect_get_is_down()
        .return_const(R1_IS_DOWN);

    let mut mock_left_stepper_button = MockStepperButtonTrait::new();
    mock_left_stepper_button
        .expect_set_is_down_and_return_is_double_click()
        .with(eq(true))
        .return_const(false);

    let mut first_invocation = true;
    mock_left_stepper_button
        .expect_get_is_down()
        .times(2)
        .returning(move|| {
            if first_invocation {
                first_invocation = false;
                false
            } else {
                true
            }
        });

    let mut joy_keyboard = JoyKeyboard::new(
        Box::new(mock_enigo),
        Box::new(mock_right_stepper_button),
        Box::new(mock_left_stepper_button),
        joy_keyboard_keys_config()
    );

    joy_keyboard.set_r1_mod_is_down(R1_IS_DOWN);

    assert_eq!(joy_keyboard.current_step,Step::Step1);

    if let Layer::VisitingSecond(details)
        = &joy_keyboard.current_layer {
        assert_eq!(details.visit_through_stepper_at_direction,
                   StepperButtonDirection::Right);
        assert_just_now(details.time_of_visit);
    } else {
        panic!("Expected layer to be VisitingSecond");
    }

    // then go to step 3 using l1
    joy_keyboard.set_l1_mod_is_down(true);

    assert_eq!(joy_keyboard.current_step,Step::Step3);
    if let Layer::VisitingSecond(details)
        = &joy_keyboard.current_layer {
        assert_eq!(details.visit_through_stepper_at_direction,
                   StepperButtonDirection::Right);
        assert_just_now(details.time_of_visit);
    } else {
        panic!("Expected layer to be VisitingSecond");
    }
}

//--------------
// SWITCHING LAYERS
//--------------

#[test]
fn set_r1_mod_is_down_visit_second_layer() {
    const R1_IS_DOWN: bool = true;
    const L1_IS_DOWN: bool = false;

    let mock_enigo = MockEnigoTrait::new();

    let mut mock_right_stepper_button = MockStepperButtonTrait::new();
    mock_right_stepper_button
        .expect_set_is_down_and_return_is_double_click()
        .with(eq(R1_IS_DOWN))
        .return_const(true);
    mock_right_stepper_button
        .expect_get_is_down()
        .return_const(R1_IS_DOWN);

    let mut mock_left_stepper_button = MockStepperButtonTrait::new();
    mock_left_stepper_button
        .expect_get_is_down()
        .return_const(L1_IS_DOWN);

    let mut joy_keyboard = JoyKeyboard::new(
        Box::new(mock_enigo),
        Box::new(mock_right_stepper_button),
        Box::new(mock_left_stepper_button),
        joy_keyboard_keys_config()
    );

    joy_keyboard.set_r1_mod_is_down(R1_IS_DOWN);
    assert_eq!(joy_keyboard.current_step,Step::Step1);

    if let Layer::VisitingSecond(details)
        = joy_keyboard.current_layer {
        assert_eq!(details.visit_through_stepper_at_direction,
                   StepperButtonDirection::Right);
        assert_just_now(details.time_of_visit);
    } else {
        panic!("Expected layer to be VisitingSecond");
    }
}

#[test]
fn set_l1_mod_is_down_visit_second_layer() {
    const R1_IS_DOWN: bool = false;
    const L1_IS_DOWN: bool = true;

    let mock_enigo = MockEnigoTrait::new();

    let mut mock_right_stepper_button = MockStepperButtonTrait::new();
    mock_right_stepper_button
        .expect_get_is_down()
        .return_const(R1_IS_DOWN);

    let mut mock_left_stepper_button = MockStepperButtonTrait::new();
    mock_left_stepper_button
        .expect_set_is_down_and_return_is_double_click()
        .with(eq(L1_IS_DOWN))
        .return_const(true);
    mock_left_stepper_button
        .expect_get_is_down()
        .return_const(L1_IS_DOWN);

    let mut joy_keyboard = JoyKeyboard::new(
        Box::new(mock_enigo),
        Box::new(mock_right_stepper_button),
        Box::new(mock_left_stepper_button),
        joy_keyboard_keys_config()
    );

    joy_keyboard.set_l1_mod_is_down(L1_IS_DOWN);
    assert_eq!(joy_keyboard.current_step,Step::Step1);

    if let Layer::VisitingSecond(details)
        = joy_keyboard.current_layer {
        assert_eq!(details.visit_through_stepper_at_direction,
                   StepperButtonDirection::Left);
        assert_just_now(details.time_of_visit);
    } else {
        panic!("Expected layer to be VisitingSecond");
    }
}

#[test]
fn set_r1_mod_is_down_go_to_second_layer() {
    let mock_enigo = MockEnigoTrait::new();

    let mut mock_right_stepper_button = MockStepperButtonTrait::new();
    // should apply after the first call to set_r1_mod_is_down
    // but this hasn't been strictly tested
    mock_right_stepper_button
        .expect_set_is_down_and_return_is_double_click()
        .with(eq(true))
        .return_const(true);

    // should apply after the second call to set_r1_mod_is_down
    // but this hasn't been strictly tested
    mock_right_stepper_button
        .expect_set_is_down_and_return_is_double_click()
        .with(eq(false))
        .return_const(true);

    // matches both after the first and second call of set_r1_mod_is_down
    // had to create a work around for this
    let mut first_invocation = true;
    mock_right_stepper_button
        .expect_get_is_down()
        .times(2)
        .returning(move|| {
            if first_invocation {
                first_invocation = false;
                true
            } else {
                false
            }
        });


    let mut mock_left_stepper_button = MockStepperButtonTrait::new();
    mock_left_stepper_button
        .expect_get_is_down()
        .return_const(false);

    let mut joy_keyboard = JoyKeyboard::new(
        Box::new(mock_enigo),
        Box::new(mock_right_stepper_button),
        Box::new(mock_left_stepper_button),
        joy_keyboard_keys_config()
    );

    joy_keyboard.set_r1_mod_is_down(true);
    assert_eq!(joy_keyboard.current_step,Step::Step1);
    if let Layer::VisitingSecond(details)
        = &joy_keyboard.current_layer {
        assert_eq!(details.visit_through_stepper_at_direction,
                   StepperButtonDirection::Right);
        assert_just_now(details.time_of_visit);
    } else {
        panic!("Expected layer to be VisitingSecond");
    }


    joy_keyboard.set_r1_mod_is_down(false);
    assert_eq!(joy_keyboard.current_step,Step::Step1);
    assert_eq!(joy_keyboard.current_layer,Layer::Second);
}

#[test]
fn set_l1_mod_is_down_go_to_second_layer() {
    let mock_enigo = MockEnigoTrait::new();

    let mut mock_right_stepper_button = MockStepperButtonTrait::new();
    mock_right_stepper_button
        .expect_get_is_down()
        .return_const(false);

    let mut mock_left_stepper_button = MockStepperButtonTrait::new();
    // should apply after the first call to set_r1_mod_is_down
    // but this hasn't been strictly tested
    mock_left_stepper_button
        .expect_set_is_down_and_return_is_double_click()
        .with(eq(true))
        .return_const(true);

    // should apply after the second call to set_r1_mod_is_down
    // but this hasn't been strictly tested
    mock_left_stepper_button
        .expect_set_is_down_and_return_is_double_click()
        .with(eq(false))
        .return_const(true);

    // matches both after the first and second call of set_r1_mod_is_down
    // had to create a work around for this
    let mut first_invocation = true;
    mock_left_stepper_button
        .expect_get_is_down()
        .times(2)
        .returning(move|| {
            if first_invocation {
                first_invocation = false;
                true
            } else {
                false
            }
        });


    let mut joy_keyboard = JoyKeyboard::new(
        Box::new(mock_enigo),
        Box::new(mock_right_stepper_button),
        Box::new(mock_left_stepper_button),
        joy_keyboard_keys_config()
    );

    joy_keyboard.set_l1_mod_is_down(true);
    assert_eq!(joy_keyboard.current_step,Step::Step1);
    if let Layer::VisitingSecond(details)
        = &joy_keyboard.current_layer {
        assert_eq!(details.visit_through_stepper_at_direction,
                   StepperButtonDirection::Left);
        assert_just_now(details.time_of_visit);
    } else {
        panic!("Expected layer to be VisitingSecond");
    }


    joy_keyboard.set_l1_mod_is_down(false);
    assert_eq!(joy_keyboard.current_step,Step::Step1);
    assert_eq!(joy_keyboard.current_layer,Layer::Second);
}

#[test]
fn set_r1_mod_is_down_go_back_to_first_layer() {
    let mock_enigo = MockEnigoTrait::new();

    let mut mock_right_stepper_button = MockStepperButtonTrait::new();
    // should apply after the first call to set_r1_mod_is_down
    // but this hasn't been strictly tested
    mock_right_stepper_button
        .expect_set_is_down_and_return_is_double_click()
        .with(eq(true))
        .return_const(true);

    // should apply after the second call to set_r1_mod_is_down
    // but this hasn't been strictly tested
    mock_right_stepper_button
        .expect_set_is_down_and_return_is_double_click()
        .with(eq(false))
        .return_const(true);

    // matches both after the first and second call of set_r1_mod_is_down
    // had to create a work around for this
    let mut first_invocation = true;
    mock_right_stepper_button
        .expect_get_is_down()
        .times(2)
        .returning(move|| {
            if first_invocation {
                first_invocation = false;
                true
            } else {
                false
            }
        });


    let mut mock_left_stepper_button = MockStepperButtonTrait::new();
    mock_left_stepper_button
        .expect_get_is_down()
        .return_const(false);

    let mut joy_keyboard = JoyKeyboard::new(
        Box::new(mock_enigo),
        Box::new(mock_right_stepper_button),
        Box::new(mock_left_stepper_button),
        joy_keyboard_keys_config()
    );

    joy_keyboard.current_layer = Layer::Second;

    joy_keyboard.set_r1_mod_is_down(true);
    assert_eq!(joy_keyboard.current_step,Step::Step1);
    if let Layer::VisitingFirst(details)
        = &joy_keyboard.current_layer {
        assert_eq!(details.visit_through_stepper_at_direction,
                   StepperButtonDirection::Right);
        assert_just_now(details.time_of_visit);
    } else {
        panic!("Expected layer to be VisitingFirst");
    }


    joy_keyboard.set_r1_mod_is_down(false);
    assert_eq!(joy_keyboard.current_step,Step::Step1);
    assert_eq!(joy_keyboard.current_layer,Layer::First);
}

#[test]
fn set_l1_mod_is_down_go_back_to_first_layer() {
    let mock_enigo = MockEnigoTrait::new();

    let mut mock_right_stepper_button = MockStepperButtonTrait::new();
    mock_right_stepper_button
        .expect_get_is_down()
        .return_const(false);

    let mut mock_left_stepper_button = MockStepperButtonTrait::new();
    // should apply after the first call to set_r1_mod_is_down
    // but this hasn't been strictly tested
    mock_left_stepper_button
        .expect_set_is_down_and_return_is_double_click()
        .with(eq(true))
        .return_const(true);

    // should apply after the second call to set_r1_mod_is_down
    // but this hasn't been strictly tested
    mock_left_stepper_button
        .expect_set_is_down_and_return_is_double_click()
        .with(eq(false))
        .return_const(true);

    // matches both after the first and second call of set_r1_mod_is_down
    // had to create a work around for this
    let mut first_invocation = true;
    mock_left_stepper_button
        .expect_get_is_down()
        .times(2)
        .returning(move|| {
            if first_invocation {
                first_invocation = false;
                true
            } else {
                false
            }
        });


    let mut joy_keyboard = JoyKeyboard::new(
        Box::new(mock_enigo),
        Box::new(mock_right_stepper_button),
        Box::new(mock_left_stepper_button),
        joy_keyboard_keys_config()
    );

    joy_keyboard.current_layer = Layer::Second;

    joy_keyboard.set_l1_mod_is_down(true);
    assert_eq!(joy_keyboard.current_step,Step::Step1);
    if let Layer::VisitingFirst(details)
        = &joy_keyboard.current_layer {
        assert_eq!(details.visit_through_stepper_at_direction,
                   StepperButtonDirection::Left);
        assert_just_now(details.time_of_visit);
    } else {
        panic!("Expected layer to be VisitingFirst");
    }


    joy_keyboard.set_l1_mod_is_down(false);
    assert_eq!(joy_keyboard.current_step,Step::Step1);
    assert_eq!(joy_keyboard.current_layer,Layer::First);
}

//--------------
//  KEY CLICKS LAYER 1
//--------------

#[test]
fn key_click_defaults_to_layer_1_step_1() {
    let mut mock_enigo = MockEnigoTrait::new();
    mock_enigo
        .expect_key_click()
        .with(eq(joy_keyboard_keys_config().south.first_layer_step_1.key.unwrap()))
        .return_const(());

    let mock_right_stepper_button = MockStepperButtonTrait::new();
    let mock_left_stepper_button = MockStepperButtonTrait::new();
    let mut joy_keyboard = JoyKeyboard::new(
        Box::new(mock_enigo),
        Box::new(mock_right_stepper_button),
        Box::new(mock_left_stepper_button),
        joy_keyboard_keys_config()
    );

    joy_keyboard.button_pressed(CustomButton::Base(Button::South));
}

#[test]
fn key_click_layer_1_step_2() {
    const R1_IS_DOWN: bool = true;
    const L1_IS_DOWN: bool = false;

    let mut mock_enigo = MockEnigoTrait::new();
    mock_enigo
        .expect_key_click()
        .with(eq(joy_keyboard_keys_config().south.first_layer_step_2.key.unwrap()))
        .return_const(());

    let mut mock_right_stepper_button = MockStepperButtonTrait::new();
    mock_right_stepper_button
        .expect_set_is_down_and_return_is_double_click()
        .with(eq(R1_IS_DOWN))
        .return_const(false);
    mock_right_stepper_button
        .expect_get_is_down()
        .return_const(R1_IS_DOWN);

    let mut mock_left_stepper_button = MockStepperButtonTrait::new();
    mock_left_stepper_button
        .expect_get_is_down()
        .return_const(L1_IS_DOWN);

    let mut joy_keyboard = JoyKeyboard::new(
        Box::new(mock_enigo),
        Box::new(mock_right_stepper_button),
        Box::new(mock_left_stepper_button),
        joy_keyboard_keys_config()
    );

    joy_keyboard.set_r1_mod_is_down(R1_IS_DOWN);
    joy_keyboard.button_pressed(CustomButton::Base(Button::South));
    assert_eq!(joy_keyboard.current_step,Step::Step2);
    assert_eq!(joy_keyboard.current_layer,Layer::First);
}

#[test]
fn key_click_layer_1_step_3() {
    const R1_IS_DOWN: bool = false;
    const L1_IS_DOWN: bool = true;

    let mut mock_enigo = MockEnigoTrait::new();
    mock_enigo
        .expect_key_click()
        .with(eq(joy_keyboard_keys_config().south.first_layer_step_3.key.unwrap()))
        .return_const(());

    let mut mock_right_stepper_button = MockStepperButtonTrait::new();
    mock_right_stepper_button
        .expect_get_is_down()
        .return_const(R1_IS_DOWN);

    let mut mock_left_stepper_button = MockStepperButtonTrait::new();
    mock_left_stepper_button
        .expect_set_is_down_and_return_is_double_click()
        .with(eq(L1_IS_DOWN))
        .return_const(false);
    mock_left_stepper_button
        .expect_get_is_down()
        .return_const(L1_IS_DOWN);

    let mut joy_keyboard = JoyKeyboard::new(
        Box::new(mock_enigo),
        Box::new(mock_right_stepper_button),
        Box::new(mock_left_stepper_button),
        joy_keyboard_keys_config()
    );

    joy_keyboard.set_l1_mod_is_down(L1_IS_DOWN);
    joy_keyboard.button_pressed(CustomButton::Base(Button::South));
    assert_eq!(joy_keyboard.current_step,Step::Step3);
    assert_eq!(joy_keyboard.current_layer,Layer::First);
}

#[test]
fn key_click_layer_1_step_4() {
    const R1_IS_DOWN: bool = true;
    const L1_IS_DOWN: bool = true;

    let mut mock_enigo = MockEnigoTrait::new();
    mock_enigo
        .expect_key_click()
        // .with(eq(joy_keyboard_keys_config().south.first_layer_step_4.key.unwrap()))
        .times(0); // because south.first_layer_step_4 is None

    let mut mock_right_stepper_button = MockStepperButtonTrait::new();
    mock_right_stepper_button
        .expect_set_is_down_and_return_is_double_click()
        .with(eq(R1_IS_DOWN))
        .return_const(false);
    mock_right_stepper_button
        .expect_get_is_down()
        .return_const(R1_IS_DOWN);

    let mut mock_left_stepper_button = MockStepperButtonTrait::new();
    mock_left_stepper_button
        .expect_set_is_down_and_return_is_double_click()
        .with(eq(L1_IS_DOWN))
        .return_const(false);
    mock_left_stepper_button
        .expect_get_is_down()
        .return_const(L1_IS_DOWN);

    let mut joy_keyboard = JoyKeyboard::new(
        Box::new(mock_enigo),
        Box::new(mock_right_stepper_button),
        Box::new(mock_left_stepper_button),
        joy_keyboard_keys_config()
    );

    joy_keyboard.set_r1_mod_is_down(R1_IS_DOWN);
    joy_keyboard.set_l1_mod_is_down(L1_IS_DOWN);
    joy_keyboard.button_pressed(CustomButton::Base(Button::South));
    assert_eq!(joy_keyboard.current_step,Step::Step4);
    assert_eq!(joy_keyboard.current_layer,Layer::First);
}

//--------------
//  KEY CLICKS LAYER 2
//--------------

#[test]
fn key_click_visit_layer_2_step_1_through_r1() {
    const R1_IS_DOWN: bool = true;
    const L1_IS_DOWN: bool = false;

    let mut mock_enigo = MockEnigoTrait::new();
    mock_enigo
        .expect_key_click()
        .with(eq(joy_keyboard_keys_config().south.second_layer_step_1.key.unwrap()))
        .return_const(());

    let mut mock_right_stepper_button = MockStepperButtonTrait::new();
    mock_right_stepper_button
        .expect_set_is_down_and_return_is_double_click()
        .with(eq(R1_IS_DOWN))
        .return_const(true);
    mock_right_stepper_button
        .expect_get_is_down()
        .return_const(R1_IS_DOWN);

    let mut mock_left_stepper_button = MockStepperButtonTrait::new();
    mock_left_stepper_button
        .expect_get_is_down()
        .return_const(L1_IS_DOWN);

    let mut joy_keyboard = JoyKeyboard::new(
        Box::new(mock_enigo),
        Box::new(mock_right_stepper_button),
        Box::new(mock_left_stepper_button),
        joy_keyboard_keys_config()
    );

    joy_keyboard.set_r1_mod_is_down(R1_IS_DOWN);
    joy_keyboard.button_pressed(CustomButton::Base(Button::South));
    assert_eq!(joy_keyboard.current_step,Step::Step1);

    if let Layer::VisitingSecond(details)
        = joy_keyboard.current_layer {
        assert_eq!(details.visit_through_stepper_at_direction,
                   StepperButtonDirection::Right);
        assert_just_now(details.time_of_visit);
    } else {
        panic!("Expected layer to be VisitingSecond");
    }
}


#[test]
fn key_click_visit_layer_2_step_1_through_l1() {
    const R1_IS_DOWN: bool = false;
    const L1_IS_DOWN: bool = true;

    let mut mock_enigo = MockEnigoTrait::new();
    mock_enigo
        .expect_key_click()
        .with(eq(joy_keyboard_keys_config().south.second_layer_step_1.key.unwrap()))
        .return_const(());

    let mut mock_right_stepper_button = MockStepperButtonTrait::new();
    mock_right_stepper_button
        .expect_get_is_down()
        .return_const(R1_IS_DOWN);

    let mut mock_left_stepper_button = MockStepperButtonTrait::new();
    mock_left_stepper_button
        .expect_set_is_down_and_return_is_double_click()
        .with(eq(L1_IS_DOWN))
        .return_const(true);
    mock_left_stepper_button
        .expect_get_is_down()
        .return_const(L1_IS_DOWN);

    let mut joy_keyboard = JoyKeyboard::new(
        Box::new(mock_enigo),
        Box::new(mock_right_stepper_button),
        Box::new(mock_left_stepper_button),
        joy_keyboard_keys_config()
    );

    joy_keyboard.set_l1_mod_is_down(L1_IS_DOWN);
    joy_keyboard.button_pressed(CustomButton::Base(Button::South));
    assert_eq!(joy_keyboard.current_step,Step::Step1);

    if let Layer::VisitingSecond(details)
        = joy_keyboard.current_layer {
        assert_eq!(details.visit_through_stepper_at_direction,
                   StepperButtonDirection::Left);
        assert_just_now(details.time_of_visit);
    } else {
        panic!("Expected layer to be VisitingSecond");
    }
}


