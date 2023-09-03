use mockall::{predicate::eq, Sequence};

use crate::input_controller::{enigo_wrapper::MockEnigoTrait, InputController, DELAY_DURATION};

use super::{InputControllerTrait, InputShape};

fn assert_just_now(time: std::time::Instant){
    assert!(time.elapsed().as_secs() < 1);
}

//TODO: write unit tests to test mouse input functionality

// Here is how this class is intended to be used:
//
// InputController exists to enable the user to hold a key,
// and have it fired many times so long as the key is held "down".
// To match the way the system keyboard tends to do it,
// InputController (and its caller) will fire the input
// event once, take a break, and then many times consistantly
// until key_up() is called (or key_down() is called again).
// Here's a visualisation of the calls:
//
//  *key_down()* |.............|..|..|..|..|..| *key_up()*
//
//  where "|" is the moment when the key is input on-screen and "." is a
//  tiny unit of time spent waiting.
//
//  Here's a more technical explanation of this process
// 1. key_down() is called and given an active_key.
//    The key is immediately triggered, meaning the input-event is fired.
// 2. InputController::trigger_input() is constantly being called by
//    by the main loop by the main app controller.
//    But from the moment key_down() is called until DELAY_DURATION has
//    elapsed, trigger_input() will not trigger any input events.
// 3. After DELAY_DURATION has elapsed, trigger_input() will, each time it
//    is called, trigger the input event that types the active_key.
// 4. Once key_up() is called (meaning the button has been released by
//    the user), active_key will be unassigned, and so there will be no
//    input events triggered by trigger_input() anymore - that is until
//    key_down() is called again, which will take us back to 1.
// 4.2. If key_down() is called again before key_up() is called
//     the active_key will be updated, and InputController will be reset
//     to step 1.
//
//     *the following 2 tests generally test that this process works*
    
#[test]
fn input_controller_works_as_intended_1() {
    let mut mock_enigo = MockEnigoTrait::new();

    let mut seq = Sequence::new();

        //1. key_down() is called and given an active_key.
    mock_enigo
        .expect_key_click() 
        .times(1)
        .with(eq(enigo::Key::Layout('*')))
        .return_const(())
        .in_sequence(&mut seq);

        // 3. After DELAY_DURATION has elapsed, trigger_input() fires events
    mock_enigo 
        .expect_key_click()
        .times(5)
        .with(eq(enigo::Key::Layout('*')))
        .return_const(())
        .in_sequence(&mut seq);

    // 1. key_down() is called and given an active_key.
    let mut input_controller = InputController::new(Box::new(mock_enigo));
    assert!(input_controller.active_key.is_none());
    assert!(input_controller.instant_to_start_delay_from.is_none());

    input_controller.key_down(
        InputShape::Key(super::KeyInputShape {
            key: enigo::Key::Layout('*'),
            modifiers: [].to_vec(),
        }));
    assert_just_now(input_controller.instant_to_start_delay_from.unwrap());
    assert_eq!(input_controller.active_key.as_ref().unwrap(),
        &InputShape::Key(super::KeyInputShape {
            key: enigo::Key::Layout('*'),
            modifiers: [].to_vec(),
        }));

    // 2. InputController::trigger_input() will not fire
    input_controller.trigger_input();
    // this is to prove that the no events were fired
    assert!(!input_controller.allow_input());
    assert!(input_controller.instant_to_start_delay_from.is_some());


    // 3. After DELAY_DURATION has elapsed, trigger_input() fires events
    std::thread::sleep(DELAY_DURATION);
    input_controller.trigger_input();
    input_controller.trigger_input();
    input_controller.trigger_input();
    input_controller.trigger_input();
    input_controller.trigger_input();


    // 4. Once key_up() is called there will be no
    //    input events triggered by trigger_input() anymore
    input_controller.key_up();
    assert!(input_controller.active_key.is_none());
    assert!(input_controller.instant_to_start_delay_from.is_none());

    // these calls should do nothing
    input_controller.trigger_input();
    input_controller.trigger_input();

}

#[test]
fn input_controller_works_as_intended_2() {
    let mut mock_enigo = MockEnigoTrait::new();

    let mut seq = Sequence::new();

        //1. key_down() is called and given an active_key.
    mock_enigo
        .expect_key_click() 
        .times(1)
        .with(eq(enigo::Key::Layout('*')))
        .return_const(())
        .in_sequence(&mut seq);

        // 3. After DELAY_DURATION has elapsed, trigger_input() fires events
    mock_enigo 
        .expect_key_click()
        .times(5)
        .with(eq(enigo::Key::Layout('*')))
        .return_const(())
        .in_sequence(&mut seq);

        // 4.2. If key_down() is called again before key_up() is called
        //      then the active_key will be updated
    mock_enigo
        .expect_key_click() 
        .times(1)
        .with(eq(enigo::Key::Space))
        .return_const(())
        .in_sequence(&mut seq);

    // 1. key_down() is called and given an active_key.
    let mut input_controller = InputController::new(Box::new(mock_enigo));
    assert!(input_controller.active_key.is_none());
    assert!(input_controller.instant_to_start_delay_from.is_none());

    input_controller.key_down(
        InputShape::Key(super::KeyInputShape {
            key: enigo::Key::Layout('*'),
            modifiers: [].to_vec(),
        }));
    assert_just_now(input_controller.instant_to_start_delay_from.unwrap());
    assert_eq!(input_controller.active_key.as_ref().unwrap(),
        &InputShape::Key(super::KeyInputShape {
            key: enigo::Key::Layout('*'),
            modifiers: [].to_vec(),
        }));

    // 2. InputController::trigger_input() will not fire
    input_controller.trigger_input();
    // this is to prove that the no events were fired
    assert!(!input_controller.allow_input());
    assert!(input_controller.instant_to_start_delay_from.is_some());


    // 3. After DELAY_DURATION has elapsed, trigger_input() fires events
    std::thread::sleep(DELAY_DURATION);
    input_controller.trigger_input();
    input_controller.trigger_input();
    input_controller.trigger_input();
    input_controller.trigger_input();
    input_controller.trigger_input();

    // 4.2. If key_down() is called again before key_up() is called
    //      then the active_key will be updated
    input_controller.key_down(
        InputShape::Key(super::KeyInputShape {
            key: enigo::Key::Space,
            modifiers: [].to_vec(),
        }));
    assert_just_now(input_controller.instant_to_start_delay_from.unwrap());
    assert_eq!(input_controller.active_key.unwrap(),
        InputShape::Key(super::KeyInputShape {
            key: enigo::Key::Space,
            modifiers: [].to_vec(),
        }));

}

// ----------------
// the rest is non-exhaustive tests of the different parts
// ----------------
#[test]
fn allow_input_works() {
    assert!(true);
    let mock_enigo = MockEnigoTrait::new();
    assert!(InputController::new(Box::new(mock_enigo)).allow_input());

    let mock_enigo = MockEnigoTrait::new();
    let mut input_controller = InputController::new(Box::new(mock_enigo));
    input_controller.instant_to_start_delay_from = Some(std::time::Instant::now());
    assert!(!input_controller.allow_input());

    std::thread::sleep(DELAY_DURATION);
    assert!(input_controller.allow_input());
}

#[test]
fn key_down_works() {
    let mut mock_enigo = MockEnigoTrait::new();
    mock_enigo
        .expect_key_click()
        .with(eq(enigo::Key::Layout('a')))
        .return_const(());

    let mut input_controller = InputController::new(Box::new(mock_enigo));
    input_controller.key_down(
        InputShape::Key(super::KeyInputShape {
            key: enigo::Key::Layout('a'),
            modifiers: [].to_vec(),
        }));
    assert_just_now(input_controller.instant_to_start_delay_from.unwrap());
}

#[test]
fn key_down_for_key_with_modifiers_works() {
    let mut mock_enigo = MockEnigoTrait::new();
    mock_enigo
        .expect_key_click()
        .with(eq(enigo::Key::Layout('1')))
        .return_const(());
    mock_enigo
        .expect_key_down()
        .withf(|arg| *arg == enigo::Key::Shift || *arg == enigo::Key::Alt)
        .times(2)
        .return_const(());
    mock_enigo
        .expect_key_up()
        .times(2)
        .return_const(());

    let mut input_controller = InputController::new(Box::new(mock_enigo));
    input_controller.key_down(
        InputShape::Key(super::KeyInputShape {
            key: enigo::Key::Layout('1'),
            modifiers: [enigo::Key::Shift,enigo::Key::Alt].to_vec(),
        }));
    assert_just_now(input_controller.instant_to_start_delay_from.unwrap());
}

#[test]
fn trigger_input_works() {
    let mut mock_enigo = MockEnigoTrait::new();
    mock_enigo
        .expect_key_click()
        .with(eq(enigo::Key::Tab))
        .return_const(());
    let mut input_controller = InputController::new(Box::new(mock_enigo));
    input_controller.active_key = Some(
        InputShape::Key(super::KeyInputShape {
            key: enigo::Key::Tab,
            modifiers: [].to_vec(),
        }));
    input_controller.trigger_input();
}

#[test]
fn trigger_input_calls_enigo_function_in_the_right_order() {
    let mut mock_enigo = MockEnigoTrait::new();
    let mut seq = Sequence::new();
    mock_enigo
        .expect_key_down()
        .times(1)
        .with(eq(enigo::Key::Shift))
        .return_const(())
        .in_sequence(&mut seq);
    mock_enigo
        .expect_key_down()
        .times(1)
        .with(eq(enigo::Key::Alt))
        .return_const(())
        .in_sequence(&mut seq);

    mock_enigo
        .expect_key_click()
        .times(1)
        .with(eq(enigo::Key::Layout('1')))
        .return_const(())
        .in_sequence(&mut seq);

    mock_enigo
        .expect_key_up()
        .times(1)
        .with(eq(enigo::Key::Shift))
        .return_const(())
        .in_sequence(&mut seq);
    mock_enigo
        .expect_key_up()
        .times(1)
        .with(eq(enigo::Key::Alt))
        .return_const(())
        .in_sequence(&mut seq);

    let mut input_controller = InputController::new(Box::new(mock_enigo));
    input_controller.active_key =
        Some(InputShape::Key(super::KeyInputShape {
            key: enigo::Key::Layout('1'),
            modifiers: [enigo::Key::Shift,enigo::Key::Alt].to_vec(),
        }));
    input_controller.trigger_input();
    assert!(input_controller.instant_to_start_delay_from.is_none());
}
