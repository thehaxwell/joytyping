use crate::joy_input::{JoyKeyboard, Step, Layer};

use crate::joy_input::enigo_wrapper::MockEnigoTrait;
use crate::joy_input::stepper::MockStepperButtonTrait;
use crate::joy_input::StepperButtonDirection;

use super::GamepadKeyConfig;
use mockall::predicate::*;

const GAMEPAD_KEY_CONFIG: GamepadKeyConfig = GamepadKeyConfig {
    first_layer_step_1: Some(enigo::Key::Layout('I')),
    first_layer_step_2: Some(enigo::Key::Layout('U')),
    first_layer_step_3: Some(enigo::Key::Layout('v')),
    first_layer_step_4: Some(enigo::Key::Layout('A')),
    second_layer_step_1: Some(enigo::Key::Layout('&')),
    second_layer_step_2: Some(enigo::Key::Layout('}')),
    second_layer_step_3: Some(enigo::Key::Layout('A')),
    second_layer_step_4: Some(enigo::Key::Layout('z')),
};


fn assert_just_now(time: std::time::Instant){
    assert!(time.elapsed().as_secs() < 1);
}

//--------------
//  KEYS MATCH LAYER AND STEP
//--------------

fn setup_key_click_hard_coded_matching_test(
    layer: Layer, step: Step, gamepad_key_config: enigo::Key){
    let mut mock_enigo = MockEnigoTrait::new();
    mock_enigo
        .expect_key_click()
        .with(eq(gamepad_key_config))
        .return_const(());

    let mock_right_stepper_button = MockStepperButtonTrait::new();
    let mock_left_stepper_button = MockStepperButtonTrait::new();
    let mut joy_keyboard = JoyKeyboard::new(
        Box::new(mock_enigo),
        Box::new(mock_right_stepper_button),
        Box::new(mock_left_stepper_button),
    );

    joy_keyboard.current_step = step;
    joy_keyboard.current_layer = layer;

    joy_keyboard.key_click(GAMEPAD_KEY_CONFIG);
}

#[test]
fn key_click_hard_coded_layer_1_step_1() {
    setup_key_click_hard_coded_matching_test(
        Layer::First,
        Step::Step1,
        GAMEPAD_KEY_CONFIG.first_layer_step_1.unwrap());
}

#[test]
fn key_click_hard_coded_layer_1_step_2() {
    setup_key_click_hard_coded_matching_test(
        Layer::First,
        Step::Step2,
        GAMEPAD_KEY_CONFIG.first_layer_step_2.unwrap());
}

#[test]
fn key_click_hard_coded_layer_1_step_3() {
    setup_key_click_hard_coded_matching_test(
        Layer::First,
        Step::Step3,
        GAMEPAD_KEY_CONFIG.first_layer_step_3.unwrap());
}

#[test]
fn key_click_hard_coded_layer_2_step_1() {
    setup_key_click_hard_coded_matching_test(
        Layer::Second,
        Step::Step1,
        GAMEPAD_KEY_CONFIG.second_layer_step_1.unwrap());
}

#[test]
fn key_click_hard_coded_layer_2_step_2() {
    setup_key_click_hard_coded_matching_test(
        Layer::Second,
        Step::Step2,
        GAMEPAD_KEY_CONFIG.second_layer_step_2.unwrap());
}

#[test]
fn key_click_hard_coded_layer_2_step_3() {
    setup_key_click_hard_coded_matching_test(
        Layer::Second,
        Step::Step3,
        GAMEPAD_KEY_CONFIG.second_layer_step_3.unwrap());
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
        .with(eq(GAMEPAD_KEY_CONFIG.first_layer_step_1.unwrap()))
        .return_const(());

    let mock_right_stepper_button = MockStepperButtonTrait::new();
    let mock_left_stepper_button = MockStepperButtonTrait::new();
    let mut joy_keyboard = JoyKeyboard::new(
        Box::new(mock_enigo),
        Box::new(mock_right_stepper_button),
        Box::new(mock_left_stepper_button),
    );

    joy_keyboard.key_click(GAMEPAD_KEY_CONFIG);
}

#[test]
fn key_click_layer_1_step_2() {
    const R1_IS_DOWN: bool = true;
    const L1_IS_DOWN: bool = false;

    let mut mock_enigo = MockEnigoTrait::new();
    mock_enigo
        .expect_key_click()
        .with(eq(GAMEPAD_KEY_CONFIG.first_layer_step_2.unwrap()))
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
    );

    joy_keyboard.set_r1_mod_is_down(R1_IS_DOWN);
    joy_keyboard.key_click(GAMEPAD_KEY_CONFIG);
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
        .with(eq(GAMEPAD_KEY_CONFIG.first_layer_step_3.unwrap()))
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
    );

    joy_keyboard.set_l1_mod_is_down(L1_IS_DOWN);
    joy_keyboard.key_click(GAMEPAD_KEY_CONFIG);
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
        .with(eq(GAMEPAD_KEY_CONFIG.first_layer_step_4.unwrap()))
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
    );

    joy_keyboard.set_r1_mod_is_down(R1_IS_DOWN);
    joy_keyboard.set_l1_mod_is_down(L1_IS_DOWN);
    joy_keyboard.key_click(GAMEPAD_KEY_CONFIG);
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
        .with(eq(GAMEPAD_KEY_CONFIG.second_layer_step_1.unwrap()))
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
    );

    joy_keyboard.set_r1_mod_is_down(R1_IS_DOWN);
    joy_keyboard.key_click(GAMEPAD_KEY_CONFIG);
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
        .with(eq(GAMEPAD_KEY_CONFIG.second_layer_step_1.unwrap()))
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
    );

    joy_keyboard.set_l1_mod_is_down(L1_IS_DOWN);
    joy_keyboard.key_click(GAMEPAD_KEY_CONFIG);
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
