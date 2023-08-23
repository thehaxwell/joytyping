use crate::LeftOrRight;
use crate::gamepad::sticks_interpreter::SticksInterpreter;
use crate::gamepad::GamepadEvent;
use crate::gamepad::CustomButton;

use super::AxisClickThresholds;

fn create_axes_and_move_stick_to(
    first_thresholds: AxisClickThresholds,
    second_thresholds: AxisClickThresholds,
    raw_events: Vec<(LeftOrRight,f32,f32)>
    ) -> Vec<Option<GamepadEvent>>{
    let mut axes_interpreter = SticksInterpreter::new(
        first_thresholds,
        second_thresholds,
        );
    raw_events
        .iter()
        .map(|m| axes_interpreter.interpret_stick_move(if let LeftOrRight::Left = m.0 { true } else {false},m.1,m.2))
        .collect()
}

//TODO: randomize the AxisClickThresholds values
// to make sure that the tests are not biased

#[test]
fn resting_position_fires_no_events() {
    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Left,0.0,0.0),
    ]);

    assert_eq!(res[0],None);

    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Right,0.0,0.0),
    ]);

    assert_eq!(res[0],None);
}

//
// SIMPLE EVENTS REGISTER
//

#[test]
fn tilt_left_stick_to_the_left() {
    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Left,-1.0,0.0),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickLeft)));
}

#[test]
fn tilt_right_stick_to_the_left() {
    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Right,-1.0,0.0),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickLeft)));
}

#[test]
fn tilt_left_stick_to_the_right() {
    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Left,1.0,0.0),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickRight)));
}


#[test]
fn tilt_right_stick_to_the_right() {
    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Right,1.0,0.0),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickRight)));
}

#[test]
fn tilt_left_stick_up() {
    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Left,0.0,1.0),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickUp)));
}

#[test]
fn tilt_right_stick_up() {
    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Right,0.0,1.0),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickUp)));
}

#[test]
fn tilt_left_stick_down() {
    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Left,0.0,-1.0),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickDown)));
}

#[test]
fn tilt_right_stick_down() {
    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Right,0.0,-1.0),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickDown)));
}

//
// EXPRESS LOWER THRESHOLD FOR EVENT FIRE
//

#[test]
fn inadequately_tilt_left_stick_to_the_left_wont_register() {
    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.1,
            down: 0.2,
            left: 0.3,
            right: 0.4,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.6,
            left: 0.7,
            right: 0.8,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Left,-0.3,0.0),
    ]);

    assert_eq!(res[0],None);
}


#[test]
fn inadequately_tilt_right_stick_to_the_left_wont_register() {
    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.51,
            down: 0.52,
            left: 0.53,
            right: 0.541,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.55,
            down: 0.56,
            left: 0.57,
            right: 0.58,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Right,-0.57,0.0),
    ]);

    assert_eq!(res[0],None);
}

#[test]
fn inadequately_tilt_left_stick_to_the_right_wont_register() {
    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.105,
            down: 0.205,
            left: 0.305,
            right: 0.405,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.505,
            down: 0.605,
            left: 0.705,
            right: 0.805,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Left,0.305,0.0),
    ]);

    assert_eq!(res[0],None);
}

#[test]
fn inadequately_tilt_right_stick_to_the_right_wont_register() {
    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Right,0.5,0.0),
    ]);

    assert_eq!(res[0],None);
}

#[test]
fn inadequately_tilt_left_stick_up_wont_register() {
    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Left,0.0,0.5),
    ]);

    assert_eq!(res[0],None);
}

#[test]
fn inadequately_tilt_right_stick_up_wont_register() {
    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Right,0.0,0.5),
    ]);

    assert_eq!(res[0],None);
}

#[test]
fn inadequately_tilt_left_stick_down_wont_register() {
    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Left,0.0,-0.5),
    ]);

    assert_eq!(res[0],None);
}

#[test]
fn inadequately_tilt_right_stick_down_wont_register() {
    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Right,0.0,-0.5),
    ]);

    assert_eq!(res[0],None);
}


//
// MINIMAL TILT TO REGISTER BUTTON PRESS
//

#[test]
fn minimally_tilt_left_stick_to_the_left() {
    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5216,
            down: 0.9835,
            left: 0.37852,
            right: 0.8329,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.8392,
            down: 0.2389,
            left: 0.82934,
            right: 0.32948,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Left,-0.3785201,0.0),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickLeft)));
}

#[test]
fn minimally_tilt_right_stick_to_the_left() {
    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Right,-0.51,0.0),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickLeft)));
}

#[test]
fn minimally_tilt_left_stick_to_the_right() {
    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Left,0.51,0.0),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickRight)));
}

#[test]
fn minimally_tilt_right_stick_to_the_right() {
    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Right,0.51,0.0),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickRight)));
}

#[test]
fn minimally_tilt_left_stick_up() {
    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Left,0.0,0.51),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickUp)));
}

#[test]
fn minimally_tilt_right_stick_up() {
    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Right,0.0,0.51),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickUp)));
}

#[test]
fn minimally_tilt_left_stick_down() {
    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Left,0.0,-0.51),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickDown)));
}

#[test]
fn minimally_tilt_right_stick_down() {
    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Right,0.0,-0.51),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickDown)));
}

//
// BUTTON RELEASE WORKS
//


#[test]
fn tilt_and_untilt_left_stick_to_the_left() {
    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Left,-1.0,0.0),
        (LeftOrRight::Left,-0.1,0.0),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickLeft)));
    assert_eq!(res[1],Some(GamepadEvent::ButtonReleased(CustomButton::LeftStickLeft)));
}

#[test]
fn tilt_and_untilt_right_stick_to_the_left() {
    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Right,-1.0,0.0),
        (LeftOrRight::Right,-0.1,0.0),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickLeft)));
    assert_eq!(res[1],Some(GamepadEvent::ButtonReleased(CustomButton::RightStickLeft)));
}

#[test]
fn tilt_and_untilt_left_stick_to_the_right() {
    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Left,1.0,0.0),
        (LeftOrRight::Left,0.1,0.0),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickRight)));
    assert_eq!(res[1],Some(GamepadEvent::ButtonReleased(CustomButton::LeftStickRight)));
}

#[test]
fn tilt_and_untilt_right_stick_to_the_right() {
    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Right,1.0,0.0),
        (LeftOrRight::Right,0.1,0.0),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickRight)));
    assert_eq!(res[1],Some(GamepadEvent::ButtonReleased(CustomButton::RightStickRight)));
}

#[test]
fn tilt_and_untilt_left_stick_up() {
    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Left,0.0,1.0),
        (LeftOrRight::Left,0.0,0.1),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickUp)));
    assert_eq!(res[1],Some(GamepadEvent::ButtonReleased(CustomButton::LeftStickUp)));
}

#[test]
fn tilt_and_untilt_right_stick_up() {
    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Right,0.0,1.0),
        (LeftOrRight::Right,0.0,0.1),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickUp)));
    assert_eq!(res[1],Some(GamepadEvent::ButtonReleased(CustomButton::RightStickUp)));
}

#[test]
fn tilt_and_untilt_left_stick_down() {
    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Left,0.0,-1.0),
        (LeftOrRight::Left,0.0,-0.1),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickDown)));
    assert_eq!(res[1],Some(GamepadEvent::ButtonReleased(CustomButton::LeftStickDown)));
}

#[test]
fn tilt_and_untilt_right_stick_down() {
    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Right,0.0,-1.0),
        (LeftOrRight::Right,0.0,-0.1),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickDown)));
    assert_eq!(res[1],Some(GamepadEvent::ButtonReleased(CustomButton::RightStickDown)));
}

//
// DIAGONAL MOVEMENTS REGISTER
//

#[test]
fn tilt_left_stick_to_the_leftish () {
    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Left,-1.0,0.1),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickLeft)));

    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Left,-0.8,0.7),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickLeft)));

    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Left,-0.7,-0.6),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickLeft)));
}

#[test]
fn tilt_right_stick_to_the_leftish () {
    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Right,-1.0,0.1),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickLeft)));

    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Right,-0.8,0.7),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickLeft)));

    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Right,-0.7,-0.6),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickLeft)));
}

#[test]
fn tilt_left_stick_to_the_rightish () {
    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Left,1.0,0.1),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickRight)));

    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Left,0.8,0.7),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickRight)));

    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Left,0.7,-0.6),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickRight)));
}

#[test]
fn tilt_right_stick_to_the_rightish () {
    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Right,1.0,0.1),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickRight)));

    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Right,0.8,0.7),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickRight)));

    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Right,0.7,-0.6),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickRight)));
}

#[test]
fn tilt_left_stick_upish () {
    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Left,0.1,1.0),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickUp)));

    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Left,0.7,0.8),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickUp)));

    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Left,-0.6,0.7),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickUp)));
}

#[test]
fn tilt_right_stick_upish () {
    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Right,0.1,1.0),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickUp)));

    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Right,0.7,0.8),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickUp)));

    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Right,-0.6,0.7),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickUp)));
}

#[test]
fn tilt_left_stick_downish () {
    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Left,0.1,-1.0),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickDown)));

    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Left,0.7,-0.8),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickDown)));

    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Left,-0.6,-0.7),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::LeftStickDown)));
}

#[test]
fn tilt_right_stick_downish () {
    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Right,0.1,-1.0),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickDown)));

    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Right,0.7,-0.8),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickDown)));

    let res = create_axes_and_move_stick_to(
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Left,
        },
        AxisClickThresholds {
            up: 0.5,
            down: 0.5,
            left: 0.5,
            right: 0.5,
            alignment: LeftOrRight::Right,
        },
        vec![
        (LeftOrRight::Right,-0.6,-0.7),
    ]);

    assert_eq!(res[0],Some(GamepadEvent::ButtonPressed(CustomButton::RightStickDown)));
}
