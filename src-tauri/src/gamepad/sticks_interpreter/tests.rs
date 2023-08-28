use crate::LeftOrRight;
use crate::gamepad::sticks_interpreter::SticksInterpreter;
use crate::gamepad::GamepadEvent;
use crate::gamepad::CustomButton;
// use crate::settings_data::AxisClickThresholds;

use super::AxisClickThresholds;
use super::CardinalCustomButtons;
use super::StickInterpreter;
use super::SticksInterpreterTrait;

#[test]
fn stick_interpreter_new_works() {
    let stick_interpreter = StickInterpreter::new(
        CardinalCustomButtons {
            up: CustomButton::Base(gilrs::Button::East),
            down: CustomButton::Base(gilrs::Button::C),
            left: CustomButton::Base(gilrs::Button::South),
            right: CustomButton::LeftStickUp,
        },
        AxisClickThresholds {
            up: 0.3248,
            down: 0.243,
            left: 0.11211,
            right: 0.999,
            alignment: LeftOrRight::Left
        },
    );

    assert_eq!(stick_interpreter.button,
        CardinalCustomButtons {
            up: CustomButton::Base(gilrs::Button::East),
            down: CustomButton::Base(gilrs::Button::C),
            left: CustomButton::Base(gilrs::Button::South),
            right: CustomButton::LeftStickUp,
        });
    assert_eq!(stick_interpreter.click_thresholds,
        AxisClickThresholds {
            up: 0.3248,
            down: 0.243,
            left: 0.11211,
            right: 0.999,
            alignment: LeftOrRight::Left
        },);
}

#[test]
fn stick_interpreter_interpret_move_works() {
    let mut stick_interpreter = StickInterpreter {
        button: CardinalCustomButtons {
            up: CustomButton::LeftStickUp,
            down: CustomButton::LeftStickDown,
            left: CustomButton::LeftStickLeft,
            right: CustomButton::LeftStickRight,
        },
        click_thresholds: AxisClickThresholds {
            up: 0.5,
            down: 0.8,
            left: 0.5,
            right: 0.3,
            alignment: LeftOrRight::Left
        },
    };
    assert!(stick_interpreter.interpret_move(0.0, 0.0).is_none());
    assert!(stick_interpreter.interpret_move(0.29, 0.0).is_none());
    assert!(stick_interpreter.interpret_move(0.3, 0.0).is_none());
    // move is only recognized when it goes PAST the threshold
    assert_eq!(
        stick_interpreter.interpret_move(0.31, 0.0).unwrap(),
        CustomButton::LeftStickRight,);
    assert_eq!(
        stick_interpreter.interpret_move(0.4, 0.0).unwrap(),
        CustomButton::LeftStickRight,);
    assert_eq!(
        // y-axis has priority over x-axis
        stick_interpreter.interpret_move(0.4, 0.39).unwrap(),
        CustomButton::LeftStickRight,);
    // left right active zone but isn't past down's threshold
    assert!(
        stick_interpreter.interpret_move(0.4, 0.41).is_none());

    assert!(
        stick_interpreter.interpret_move(0.4, 0.8).is_none());
    // finally get past down's threshold
    assert_eq!(
        stick_interpreter.interpret_move(0.4, 0.81).unwrap(),
        CustomButton::LeftStickDown,);
    assert_eq!(
        stick_interpreter.interpret_move(0.4, 1.0).unwrap(),
        CustomButton::LeftStickDown,);


    assert!(stick_interpreter.interpret_move(0.0, 0.0).is_none());
    assert!(stick_interpreter.interpret_move(-0.49, 0.0).is_none());
    assert!(stick_interpreter.interpret_move(-0.5, 0.0).is_none());
    // move is only recognized when it goes PAST the threshold
    assert_eq!(
        stick_interpreter.interpret_move(-0.51, 0.0).unwrap(),
        CustomButton::LeftStickLeft,);
    assert_eq!(
        stick_interpreter.interpret_move(-0.51, -0.49).unwrap(),
        CustomButton::LeftStickLeft,);
    // Remember that the y-axis get priority over x-axis when they're equal
    assert_eq!(
        stick_interpreter.interpret_move(-0.51, -0.51).unwrap(),
        CustomButton::LeftStickUp,);
    assert_eq!(
        stick_interpreter.interpret_move(-0.51, -1.0).unwrap(),
        CustomButton::LeftStickUp,);
}

#[test]
fn sticks_interpreter_interpret_new_works() {
    let sticks_interpreter = SticksInterpreter::new(
        AxisClickThresholds {
            up: 0.0988874,
            down: 0.238924,
            left: 0.21311,
            right: 0.0001,
            alignment: LeftOrRight::Left
        },
        AxisClickThresholds {
            up: 0.23049,
            down: 0.32890,
            left: 0.2112,
            right: 0.128,
            alignment: LeftOrRight::Right
        });
    assert!(sticks_interpreter.latest_left_button_pressed.is_none());
    assert!(sticks_interpreter.latest_right_button_pressed.is_none());

    assert_eq!(
        sticks_interpreter.right_stick_interpreter,
        StickInterpreter::new(
            CardinalCustomButtons {
                up: CustomButton::RightStickUp,
                right: CustomButton::RightStickRight,
                down: CustomButton::RightStickDown,
                left: CustomButton::RightStickLeft
            },
            AxisClickThresholds {
                up: 0.23049,
                down: 0.32890,
                left: 0.2112,
                right: 0.128,
                alignment: LeftOrRight::Right
            },),);

    assert_eq!(
        sticks_interpreter.left_stick_interpreter,
        StickInterpreter::new(
            CardinalCustomButtons {
                up: CustomButton::LeftStickUp,
                right: CustomButton::LeftStickRight,
                down: CustomButton::LeftStickDown,
                left: CustomButton::LeftStickLeft
            },
            AxisClickThresholds {
                up: 0.0988874,
                down: 0.238924,
                left: 0.21311,
                right: 0.0001,
                alignment: LeftOrRight::Left
            },),);
}

#[test]
fn sticks_interpreter_interpret_new_rejects_bad_arguments() {
    assert!(std::panic::catch_unwind(|| {
        SticksInterpreter::new(
            AxisClickThresholds {
                up: 0.0988874,
                down: 0.238924,
                left: 0.21311,
                right: 0.0001,
                alignment: LeftOrRight::Right
            },
            AxisClickThresholds {
                up: 0.23049,
                down: 0.32890,
                left: 0.2112,
                right: 0.128,
                alignment: LeftOrRight::Right
            });
    }).is_err());

    assert!(std::panic::catch_unwind(|| {
        SticksInterpreter::new(
            AxisClickThresholds {
                up: 0.0988874,
                down: 0.238924,
                left: 0.21311,
                right: 0.0001,
                alignment: LeftOrRight::Left
            },
            AxisClickThresholds {
                up: 0.23049,
                down: 0.32890,
                left: 0.2112,
                right: 0.128,
                alignment: LeftOrRight::Left
            });
    }).is_err());

}

#[test]
fn sticks_interpreter_interpret_stick_works() {
    let mut axes_interpreter = SticksInterpreter::new(
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
        });

    assert!(axes_interpreter.interpret_stick_move(LeftOrRight::Right,0.0,0.0,).is_none());
    assert!(axes_interpreter.interpret_stick_move(LeftOrRight::Right,0.0,-0.2,).is_none());
    assert!(axes_interpreter.interpret_stick_move(LeftOrRight::Right,0.0,-0.4,).is_none());
    assert_eq!(axes_interpreter.interpret_stick_move(LeftOrRight::Right,0.0,-0.6),Some(
            GamepadEvent::ButtonPressed(CustomButton::RightStickUp)));
    assert!(axes_interpreter.interpret_stick_move(LeftOrRight::Right,0.0,-0.8,).is_none());
    assert!(axes_interpreter.interpret_stick_move(LeftOrRight::Right,0.0,-0.6,).is_none());
    assert_eq!(axes_interpreter.interpret_stick_move(LeftOrRight::Right,0.0,-0.4),Some(
            GamepadEvent::ButtonReleased(CustomButton::RightStickUp)));
    assert!(axes_interpreter.interpret_stick_move(LeftOrRight::Right,0.0,-0.3,).is_none());
    assert_eq!(axes_interpreter.interpret_stick_move(LeftOrRight::Right,0.0,-0.56),Some(
            GamepadEvent::ButtonPressed(CustomButton::RightStickUp)));
    assert_eq!(axes_interpreter.interpret_stick_move(LeftOrRight::Right,0.0,-0.55),Some(
            GamepadEvent::ButtonReleased(CustomButton::RightStickUp)));
    assert_eq!(axes_interpreter.interpret_stick_move(LeftOrRight::Right,0.0,-0.56),Some(
            GamepadEvent::ButtonPressed(CustomButton::RightStickUp)));

    assert!(axes_interpreter.interpret_stick_move(LeftOrRight::Left,0.2,0.1,).is_none());
    assert!(axes_interpreter.interpret_stick_move(LeftOrRight::Left,0.3,0.1,).is_none());
    assert_eq!(axes_interpreter.interpret_stick_move(LeftOrRight::Left,0.6,0.1),Some(
            GamepadEvent::ButtonPressed(CustomButton::LeftStickRight)));
    assert!(axes_interpreter.interpret_stick_move(LeftOrRight::Left,0.542,0.1,).is_none());
    assert_eq!(axes_interpreter.interpret_stick_move(LeftOrRight::Left,0.541,0.1),Some(
            GamepadEvent::ButtonReleased(CustomButton::LeftStickRight)));
    assert!(axes_interpreter.interpret_stick_move(LeftOrRight::Left,0.5,0.1,).is_none());
    assert!(axes_interpreter.interpret_stick_move(LeftOrRight::Left,0.4,0.1,).is_none());
    assert!(axes_interpreter.interpret_stick_move(LeftOrRight::Left,0.4,0.52,).is_none());
    assert_eq!(axes_interpreter.interpret_stick_move(LeftOrRight::Left,0.4,0.521),Some(
            GamepadEvent::ButtonPressed(CustomButton::LeftStickDown)));

    // come back to the right stick
    // and be able to continue where we left off
    // becuase the sticks are independent
    assert_eq!(axes_interpreter.interpret_stick_move(LeftOrRight::Right,0.0,-0.55),Some(
            GamepadEvent::ButtonReleased(CustomButton::RightStickUp)));
    assert_eq!(axes_interpreter.interpret_stick_move(LeftOrRight::Right,0.0,-0.56),Some(
            GamepadEvent::ButtonPressed(CustomButton::RightStickUp)));
    assert!(axes_interpreter.interpret_stick_move(LeftOrRight::Right,0.0,-0.59,).is_none());

    assert_eq!(axes_interpreter.interpret_stick_move(LeftOrRight::Left,0.0,0.0),Some(
            GamepadEvent::ButtonReleased(CustomButton::LeftStickDown)));
    assert_eq!(axes_interpreter.interpret_stick_move(LeftOrRight::Right,0.0,0.0),Some(
            GamepadEvent::ButtonReleased(CustomButton::RightStickUp)));
}

#[test]
fn sticks_interpreter_interpret_stick_move_wrapper_works() {
let mut axes_interpreter = SticksInterpreter::new(
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
    });

    assert!(axes_interpreter.interpret_stick_move_wrapper(
        LeftOrRight::Right,None,None).is_none());
    assert!(axes_interpreter.interpret_stick_move_wrapper(
        LeftOrRight::Right,None,Some(-0.2)).is_none());
    assert!(axes_interpreter.interpret_stick_move_wrapper(
        LeftOrRight::Right,None,Some(-0.4)).is_none());
    assert_eq!(axes_interpreter.interpret_stick_move_wrapper(
        LeftOrRight::Right,None,Some(-0.6)),Some(
            GamepadEvent::ButtonPressed(CustomButton::RightStickUp)));
    assert!(axes_interpreter.interpret_stick_move_wrapper(
        LeftOrRight::Right,None,Some(-0.8)).is_none());
    assert!(axes_interpreter.interpret_stick_move_wrapper(
        LeftOrRight::Right,None,Some(-0.6)).is_none());
    assert_eq!(axes_interpreter.interpret_stick_move_wrapper(
        LeftOrRight::Right,None,Some(-0.4)),Some(
            GamepadEvent::ButtonReleased(CustomButton::RightStickUp)));
    assert!(axes_interpreter.interpret_stick_move_wrapper(
        LeftOrRight::Right,None,Some(-0.3)).is_none());
    assert_eq!(axes_interpreter.interpret_stick_move_wrapper(
        LeftOrRight::Right,None,Some(-0.56)),Some(
            GamepadEvent::ButtonPressed(CustomButton::RightStickUp)));
    assert_eq!(axes_interpreter.interpret_stick_move_wrapper(
        LeftOrRight::Right,None,Some(-0.55)),Some(
            GamepadEvent::ButtonReleased(CustomButton::RightStickUp)));
    assert_eq!(axes_interpreter.interpret_stick_move_wrapper(
        LeftOrRight::Right,None,Some(-0.56)),Some(
            GamepadEvent::ButtonPressed(CustomButton::RightStickUp)));

    assert!(axes_interpreter.interpret_stick_move_wrapper(
        LeftOrRight::Left,Some(0.2),Some(0.1)).is_none());
    assert!(axes_interpreter.interpret_stick_move_wrapper(
        LeftOrRight::Left,Some(0.3),Some(0.1)).is_none());
    assert_eq!(axes_interpreter.interpret_stick_move_wrapper(
        LeftOrRight::Left,Some(0.6),Some(0.1)),Some(
            GamepadEvent::ButtonPressed(CustomButton::LeftStickRight)));
    assert!(axes_interpreter.interpret_stick_move_wrapper(
        LeftOrRight::Left,Some(0.542),Some(0.1)).is_none());
    assert_eq!(axes_interpreter.interpret_stick_move_wrapper(
        LeftOrRight::Left,Some(0.541),Some(0.1)),Some(
            GamepadEvent::ButtonReleased(CustomButton::LeftStickRight)));
    assert!(axes_interpreter.interpret_stick_move_wrapper(
        LeftOrRight::Left,Some(0.5),Some(0.1)).is_none());
    assert!(axes_interpreter.interpret_stick_move_wrapper(
        LeftOrRight::Left,Some(0.4),Some(0.1)).is_none());
    assert!(axes_interpreter.interpret_stick_move_wrapper(
        LeftOrRight::Left,Some(0.4),Some(0.52)).is_none());
    assert_eq!(axes_interpreter.interpret_stick_move_wrapper(
        LeftOrRight::Left,Some(0.4),Some(0.521)),Some(
            GamepadEvent::ButtonPressed(CustomButton::LeftStickDown)));

    // come back to the right stick
    // and be able to continue where we left off
    // becuase the sticks are independent
    assert_eq!(axes_interpreter.interpret_stick_move_wrapper(
        LeftOrRight::Right,None,Some(-0.55)),Some(
            GamepadEvent::ButtonReleased(CustomButton::RightStickUp)));
    assert_eq!(axes_interpreter.interpret_stick_move_wrapper(
        LeftOrRight::Right,None,Some(-0.56)),Some(
            GamepadEvent::ButtonPressed(CustomButton::RightStickUp)));
    assert!(axes_interpreter.interpret_stick_move_wrapper(
        LeftOrRight::Right,None,Some(-0.59)).is_none());

    assert_eq!(axes_interpreter.interpret_stick_move_wrapper(
        LeftOrRight::Left,None,None),Some(
            GamepadEvent::ButtonReleased(CustomButton::LeftStickDown)));
    assert_eq!(axes_interpreter.interpret_stick_move_wrapper(
        LeftOrRight::Right,None,None),Some(
            GamepadEvent::ButtonReleased(CustomButton::RightStickUp)));
}

#[test]
fn sticks_interpreter_interpret_left_and_right_stick_works() {
let mut axes_interpreter = SticksInterpreter::new(
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
    });

    assert!(axes_interpreter.interpret_right_stick_move(None,None).is_none());
    assert!(axes_interpreter.interpret_right_stick_move(None,Some(-0.2)).is_none());
    assert!(axes_interpreter.interpret_right_stick_move(None,Some(-0.4)).is_none());
    assert_eq!(axes_interpreter.interpret_right_stick_move(None,Some(-0.6)),Some(
            GamepadEvent::ButtonPressed(CustomButton::RightStickUp)));
    assert!(axes_interpreter.interpret_right_stick_move(None,Some(-0.8)).is_none());
    assert!(axes_interpreter.interpret_right_stick_move(None,Some(-0.6)).is_none());
    assert_eq!(axes_interpreter.interpret_right_stick_move(None,Some(-0.4)),Some(
            GamepadEvent::ButtonReleased(CustomButton::RightStickUp)));
    assert!(axes_interpreter.interpret_right_stick_move(None,Some(-0.3)).is_none());
    assert_eq!(axes_interpreter.interpret_right_stick_move(None,Some(-0.56)),Some(
            GamepadEvent::ButtonPressed(CustomButton::RightStickUp)));
    assert_eq!(axes_interpreter.interpret_right_stick_move(None,Some(-0.55)),Some(
            GamepadEvent::ButtonReleased(CustomButton::RightStickUp)));
    assert_eq!(axes_interpreter.interpret_right_stick_move(None,Some(-0.56)),Some(
            GamepadEvent::ButtonPressed(CustomButton::RightStickUp)));

    assert!(axes_interpreter.interpret_left_stick_move(Some(0.2),Some(0.1)).is_none());
    assert!(axes_interpreter.interpret_left_stick_move(Some(0.3),Some(0.1)).is_none());
    assert_eq!(axes_interpreter.interpret_left_stick_move(Some(0.6),Some(0.1)),Some(
            GamepadEvent::ButtonPressed(CustomButton::LeftStickRight)));
    assert!(axes_interpreter.interpret_left_stick_move(Some(0.542),Some(0.1)).is_none());
    assert_eq!(axes_interpreter.interpret_left_stick_move(Some(0.541),Some(0.1)),Some(
            GamepadEvent::ButtonReleased(CustomButton::LeftStickRight)));
    assert!(axes_interpreter.interpret_left_stick_move(Some(0.5),Some(0.1)).is_none());
    assert!(axes_interpreter.interpret_left_stick_move(Some(0.4),Some(0.1)).is_none());
    assert!(axes_interpreter.interpret_left_stick_move(Some(0.4),Some(0.52)).is_none());
    assert_eq!(axes_interpreter.interpret_left_stick_move(Some(0.4),Some(0.521)),Some(
            GamepadEvent::ButtonPressed(CustomButton::LeftStickDown)));

    // come back to the right stick
    // and be able to continue where we left off
    // becuase the sticks are independent
    assert_eq!(axes_interpreter.interpret_right_stick_move(None,Some(-0.55)),Some(
            GamepadEvent::ButtonReleased(CustomButton::RightStickUp)));
    assert_eq!(axes_interpreter.interpret_right_stick_move(None,Some(-0.56)),Some(
            GamepadEvent::ButtonPressed(CustomButton::RightStickUp)));
    assert!(axes_interpreter.interpret_right_stick_move(None,Some(-0.59)).is_none());

    assert_eq!(axes_interpreter.interpret_left_stick_move(None,None),Some(
            GamepadEvent::ButtonReleased(CustomButton::LeftStickDown)));
    assert_eq!(axes_interpreter.interpret_right_stick_move(None,None),Some(
            GamepadEvent::ButtonReleased(CustomButton::RightStickUp)));
}
