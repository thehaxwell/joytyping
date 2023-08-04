use crate::gamepad::sticks_interpreter::SticksInterpreter;
use crate::gamepad::GamepadEvent;
use crate::gamepad::CustomButton;

const NOT_LEFT_ENOUGH_1: f32 = -0.1;
const NOT_LEFT_ENOUGH_2: f32 = -0.2;
const NOT_LEFT_ENOUGH_3: f32 = -0.3;
const NOT_LEFT_ENOUGH_4: f32 = -0.4;
const LEFT_ALMOST_ENOUGH: f32 = -0.5;
const LEFT_JUST_ENOUGH: f32 = -0.51;
const LEFT_ENOUGH_1: f32 = -0.6;
const LEFT_ENOUGH_2: f32 = -0.7;
const LEFT_ENOUGH_3: f32 = -0.8;
const LEFT_ENOUGH_4: f32 = -0.9;
const LEFTMOST_EXTREME: f32 = -1.0;

const NOT_RIGHT_ENOUGH_1: f32 = 0.1;
const NOT_RIGHT_ENOUGH_2: f32 = 0.2;
const NOT_RIGHT_ENOUGH_3: f32 = 0.3;
const NOT_RIGHT_ENOUGH_4: f32 = 0.4;
const RIGHT_ALMOST_ENOUGH: f32 = 0.5;
const RIGHT_JUST_ENOUGH: f32 = 0.51;
const RIGHT_ENOUGH_1: f32 = 0.6;
const RIGHT_ENOUGH_2: f32 = 0.7;
const RIGHT_ENOUGH_3: f32 = 0.8;
const RIGHT_ENOUGH_4: f32 = 0.9;
const RIGHTMOST_EXTREME: f32 = 1.0;

const NOT_UPWARD_ENOUGH_1: f32 = 0.1;
const NOT_UPWARD_ENOUGH_2: f32 = 0.2;
const NOT_UPWARD_ENOUGH_3: f32 = 0.3;
const NOT_UPWARD_ENOUGH_4: f32 = 0.4;
const UPWARD_ALMOST_ENOUGH: f32 = 0.5;
const UPWARD_JUST_ENOUGH: f32 = 0.51;
const UPWARD_ENOUGH_1: f32 = 0.6;
const UPWARD_ENOUGH_2: f32 = 0.7;
const UPWARD_ENOUGH_3: f32 = 0.8;
const UPWARD_ENOUGH_4: f32 = 0.9;
const UPWARD_EXTREME: f32 = 1.0;

const NOT_DOWNWARD_ENOUGH_1: f32 = -0.1;
const NOT_DOWNWARD_ENOUGH_2: f32 = -0.2;
const NOT_DOWNWARD_ENOUGH_3: f32 = -0.3;
const NOT_DOWNWARD_ENOUGH_4: f32 = -0.4;
const DOWNWARD_ALMOST_ENOUGH: f32 = -0.5;
const DOWNWARD_JUST_ENOUGH: f32 = -0.51;
const DOWNWARD_ENOUGH_1: f32 = -0.6;
const DOWNWARD_ENOUGH_2: f32 = -0.7;
const DOWNWARD_ENOUGH_3: f32 = -0.8;
const DOWNWARD_ENOUGH_4: f32 = -0.9;
const DOWNWARD_EXTREME: f32 = -1.0;

fn create_axes_and_move_stick_to(is_left_stick: bool, raw_events: Vec<(f32,f32)>) -> Vec<Option<GamepadEvent>>{
    let mut axes_interpreter = SticksInterpreter::new();
    raw_events
        .iter()
        .map(|m| axes_interpreter.interpret_stick_move(is_left_stick,m.0,m.1))
        .collect()
}

#[test]
fn resting_position_fires_no_events() {
    let res = create_axes_and_move_stick_to(true,vec![
        (0.0,0.0),
    ]);

    assert_eq!(res[0],None);

    let res = create_axes_and_move_stick_to(false,vec![
        (0.0,0.0),
    ]);

    assert_eq!(res[0],None);
}

//
// SIMPLE EVENTS REGISTER
//

#[test]
fn tilt_left_stick_to_the_left() {
    let res = create_axes_and_move_stick_to(true,vec![
        (LEFTMOST_EXTREME,0.0),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickLeft)));
}

#[test]
fn tilt_right_stick_to_the_left() {
    let res = create_axes_and_move_stick_to(false,vec![
        (LEFTMOST_EXTREME,0.0),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickLeft)));
}

#[test]
fn tilt_left_stick_to_the_right() {
    let res = create_axes_and_move_stick_to(true,vec![
        (RIGHTMOST_EXTREME,0.0),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickRight)));
}


#[test]
fn tilt_right_stick_to_the_right() {
    let res = create_axes_and_move_stick_to(false,vec![
        (RIGHTMOST_EXTREME,0.0),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickRight)));
}

#[test]
fn tilt_left_stick_up() {
    let res = create_axes_and_move_stick_to(true,vec![
        (0.0,UPWARD_EXTREME),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickUp)));
}

#[test]
fn tilt_right_stick_up() {
    let res = create_axes_and_move_stick_to(false,vec![
        (0.0,UPWARD_EXTREME),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickUp)));
}

#[test]
fn tilt_left_stick_down() {
    let res = create_axes_and_move_stick_to(true,vec![
        (0.0,DOWNWARD_EXTREME),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickDown)));
}

#[test]
fn tilt_right_stick_down() {
    let res = create_axes_and_move_stick_to(false,vec![
        (0.0,DOWNWARD_EXTREME),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickDown)));
}

//
// EXPRESS LOWER THRESHOLD FOR EVENT FIRE
//

#[test]
fn inadequately_tilt_left_stick_to_the_left_wont_register() {
    let res = create_axes_and_move_stick_to(true,vec![
        (LEFT_ALMOST_ENOUGH,0.0),
    ]);

    assert_eq!(res[0],None);
}


#[test]
fn inadequately_tilt_right_stick_to_the_left_wont_register() {
    let res = create_axes_and_move_stick_to(false,vec![
        (LEFT_ALMOST_ENOUGH,0.0),
    ]);

    assert_eq!(res[0],None);
}

#[test]
fn inadequately_tilt_left_stick_to_the_right_wont_register() {
    let res = create_axes_and_move_stick_to(true,vec![
        (RIGHT_ALMOST_ENOUGH,0.0),
    ]);

    assert_eq!(res[0],None);
}

#[test]
fn inadequately_tilt_right_stick_to_the_right_wont_register() {
    let res = create_axes_and_move_stick_to(false,vec![
        (RIGHT_ALMOST_ENOUGH,0.0),
    ]);

    assert_eq!(res[0],None);
}

#[test]
fn inadequately_tilt_left_stick_up_wont_register() {
    let res = create_axes_and_move_stick_to(true,vec![
        (0.0,UPWARD_ALMOST_ENOUGH),
    ]);

    assert_eq!(res[0],None);
}

#[test]
fn inadequately_tilt_right_stick_up_wont_register() {
    let res = create_axes_and_move_stick_to(false,vec![
        (0.0,UPWARD_ALMOST_ENOUGH),
    ]);

    assert_eq!(res[0],None);
}

#[test]
fn inadequately_tilt_left_stick_down_wont_register() {
    let res = create_axes_and_move_stick_to(true,vec![
        (0.0,DOWNWARD_ALMOST_ENOUGH),
    ]);

    assert_eq!(res[0],None);
}

#[test]
fn inadequately_tilt_right_stick_down_wont_register() {
    let res = create_axes_and_move_stick_to(false,vec![
        (0.0,DOWNWARD_ALMOST_ENOUGH),
    ]);

    assert_eq!(res[0],None);
}


//
// MINIMAL TILT TO REGISTER BUTTON PRESS
//

#[test]
fn minimally_tilt_left_stick_to_the_left() {
    let res = create_axes_and_move_stick_to(true,vec![
        (LEFT_JUST_ENOUGH,0.0),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickLeft)));
}

#[test]
fn minimally_tilt_right_stick_to_the_left() {
    let res = create_axes_and_move_stick_to(false,vec![
        (LEFT_JUST_ENOUGH,0.0),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickLeft)));
}

#[test]
fn minimally_tilt_left_stick_to_the_right() {
    let res = create_axes_and_move_stick_to(true,vec![
        (RIGHT_JUST_ENOUGH,0.0),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickRight)));
}

#[test]
fn minimally_tilt_right_stick_to_the_right() {
    let res = create_axes_and_move_stick_to(false,vec![
        (RIGHT_JUST_ENOUGH,0.0),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickRight)));
}

#[test]
fn minimally_tilt_left_stick_up() {
    let res = create_axes_and_move_stick_to(true,vec![
        (0.0,UPWARD_JUST_ENOUGH),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickUp)));
}

#[test]
fn minimally_tilt_right_stick_up() {
    let res = create_axes_and_move_stick_to(false,vec![
        (0.0,UPWARD_JUST_ENOUGH),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickUp)));
}

#[test]
fn minimally_tilt_left_stick_down() {
    let res = create_axes_and_move_stick_to(true,vec![
        (0.0,DOWNWARD_JUST_ENOUGH),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickDown)));
}

#[test]
fn minimally_tilt_right_stick_down() {
    let res = create_axes_and_move_stick_to(false,vec![
        (0.0,DOWNWARD_JUST_ENOUGH),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickDown)));
}

//
// BUTTON RELEASE WORKS
//


#[test]
fn tilt_and_untilt_left_stick_to_the_left() {
    let res = create_axes_and_move_stick_to(true,vec![
        (LEFTMOST_EXTREME,0.0),
        (NOT_LEFT_ENOUGH_1,0.0),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickLeft)));
    assert_eq!(res[1],Some(GamepadEvent::ButtonReleased(CustomButton::LeftStickLeft)));
}

#[test]
fn tilt_and_untilt_right_stick_to_the_left() {
    let res = create_axes_and_move_stick_to(false,vec![
        (LEFTMOST_EXTREME,0.0),
        (NOT_LEFT_ENOUGH_1,0.0),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickLeft)));
    assert_eq!(res[1],Some(GamepadEvent::ButtonReleased(CustomButton::RightStickLeft)));
}

#[test]
fn tilt_and_untilt_left_stick_to_the_right() {
    let res = create_axes_and_move_stick_to(true,vec![
        (RIGHTMOST_EXTREME,0.0),
        (NOT_RIGHT_ENOUGH_1,0.0),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickRight)));
    assert_eq!(res[1],Some(GamepadEvent::ButtonReleased(CustomButton::LeftStickRight)));
}

#[test]
fn tilt_and_untilt_right_stick_to_the_right() {
    let res = create_axes_and_move_stick_to(false,vec![
        (RIGHTMOST_EXTREME,0.0),
        (NOT_RIGHT_ENOUGH_1,0.0),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickRight)));
    assert_eq!(res[1],Some(GamepadEvent::ButtonReleased(CustomButton::RightStickRight)));
}

#[test]
fn tilt_and_untilt_left_stick_up() {
    let res = create_axes_and_move_stick_to(true,vec![
        (0.0,UPWARD_EXTREME),
        (0.0,NOT_UPWARD_ENOUGH_1),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickUp)));
    assert_eq!(res[1],Some(GamepadEvent::ButtonReleased(CustomButton::LeftStickUp)));
}

#[test]
fn tilt_and_untilt_right_stick_up() {
    let res = create_axes_and_move_stick_to(false,vec![
        (0.0,UPWARD_EXTREME),
        (0.0,NOT_UPWARD_ENOUGH_1),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickUp)));
    assert_eq!(res[1],Some(GamepadEvent::ButtonReleased(CustomButton::RightStickUp)));
}

#[test]
fn tilt_and_untilt_left_stick_down() {
    let res = create_axes_and_move_stick_to(true,vec![
        (0.0,DOWNWARD_EXTREME),
        (0.0,NOT_DOWNWARD_ENOUGH_1),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickDown)));
    assert_eq!(res[1],Some(GamepadEvent::ButtonReleased(CustomButton::LeftStickDown)));
}

#[test]
fn tilt_and_untilt_right_stick_down() {
    let res = create_axes_and_move_stick_to(false,vec![
        (0.0,DOWNWARD_EXTREME),
        (0.0,NOT_DOWNWARD_ENOUGH_1),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickDown)));
    assert_eq!(res[1],Some(GamepadEvent::ButtonReleased(CustomButton::RightStickDown)));
}

//
// DIAGONAL MOVEMENTS REGISTER
//

#[test]
fn tilt_left_stick_to_the_leftish () {
    let res = create_axes_and_move_stick_to(true,vec![
        (LEFTMOST_EXTREME,NOT_UPWARD_ENOUGH_1),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickLeft)));

    let res = create_axes_and_move_stick_to(true,vec![
        (LEFT_ENOUGH_3,UPWARD_ENOUGH_2),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickLeft)));

    let res = create_axes_and_move_stick_to(true,vec![
        (LEFT_ENOUGH_2,DOWNWARD_ENOUGH_1),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickLeft)));
}

#[test]
fn tilt_right_stick_to_the_leftish () {
    let res = create_axes_and_move_stick_to(false,vec![
        (LEFTMOST_EXTREME,NOT_UPWARD_ENOUGH_1),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickLeft)));

    let res = create_axes_and_move_stick_to(false,vec![
        (LEFT_ENOUGH_3,UPWARD_ENOUGH_2),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickLeft)));

    let res = create_axes_and_move_stick_to(false,vec![
        (LEFT_ENOUGH_2,DOWNWARD_ENOUGH_1),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickLeft)));
}

#[test]
fn tilt_left_stick_to_the_rightish () {
    let res = create_axes_and_move_stick_to(true,vec![
        (RIGHTMOST_EXTREME,NOT_UPWARD_ENOUGH_1),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickRight)));

    let res = create_axes_and_move_stick_to(true,vec![
        (RIGHT_ENOUGH_3,UPWARD_ENOUGH_2),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickRight)));

    let res = create_axes_and_move_stick_to(true,vec![
        (RIGHT_ENOUGH_2,DOWNWARD_ENOUGH_1),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickRight)));
}

#[test]
fn tilt_right_stick_to_the_rightish () {
    let res = create_axes_and_move_stick_to(false,vec![
        (RIGHTMOST_EXTREME,NOT_UPWARD_ENOUGH_1),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickRight)));

    let res = create_axes_and_move_stick_to(false,vec![
        (RIGHT_ENOUGH_3,UPWARD_ENOUGH_2),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickRight)));

    let res = create_axes_and_move_stick_to(false,vec![
        (RIGHT_ENOUGH_2,DOWNWARD_ENOUGH_1),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickRight)));
}

#[test]
fn tilt_left_stick_upish () {
    let res = create_axes_and_move_stick_to(true,vec![
        (NOT_RIGHT_ENOUGH_1,UPWARD_EXTREME),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickUp)));

    let res = create_axes_and_move_stick_to(true,vec![
        (RIGHT_ENOUGH_2,UPWARD_ENOUGH_3),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickUp)));

    let res = create_axes_and_move_stick_to(true,vec![
        (LEFT_ENOUGH_1,UPWARD_ENOUGH_2),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickUp)));
}

#[test]
fn tilt_right_stick_upish () {
    let res = create_axes_and_move_stick_to(false,vec![
        (NOT_RIGHT_ENOUGH_1,UPWARD_EXTREME),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickUp)));

    let res = create_axes_and_move_stick_to(false,vec![
        (RIGHT_ENOUGH_2,UPWARD_ENOUGH_3),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickUp)));

    let res = create_axes_and_move_stick_to(false,vec![
        (LEFT_ENOUGH_1,UPWARD_ENOUGH_2),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickUp)));
}

#[test]
fn tilt_left_stick_downish () {
    let res = create_axes_and_move_stick_to(true,vec![
        (NOT_RIGHT_ENOUGH_1,DOWNWARD_EXTREME),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickDown)));

    let res = create_axes_and_move_stick_to(true,vec![
        (RIGHT_ENOUGH_2,DOWNWARD_ENOUGH_3),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickDown)));

    let res = create_axes_and_move_stick_to(true,vec![
        (LEFT_ENOUGH_1,DOWNWARD_ENOUGH_2),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickDown)));
}

#[test]
fn tilt_right_stick_downish () {
    let res = create_axes_and_move_stick_to(false,vec![
        (NOT_RIGHT_ENOUGH_1,DOWNWARD_EXTREME),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickDown)));

    let res = create_axes_and_move_stick_to(false,vec![
        (RIGHT_ENOUGH_2,DOWNWARD_ENOUGH_3),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickDown)));

    let res = create_axes_and_move_stick_to(false,vec![
        (LEFT_ENOUGH_1,DOWNWARD_ENOUGH_2),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickDown)));
}
