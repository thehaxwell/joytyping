use crate::LeftOrRight;
use super::StickSwitchEvent;
use super::StickSwitchButton;
use super::AxisClickThresholds;
use super::CardinalCustomButtons;
use super::StickSwitchInterpreter;
use super::StickSwitchInterpreterTrait;

#[test]
fn stick_swtich_interpreter_new_works() {
    let stick_swtich_interpreter = StickSwitchInterpreter::new(
        AxisClickThresholds {
            up: 0.3248,
            down: 0.243,
            left: 0.11211,
            right: 0.999,
            alignment: LeftOrRight::Left
        },
        CardinalCustomButtons {
            up: StickSwitchButton::RightStickUp,
            down: StickSwitchButton::RightStickDown,
            left: StickSwitchButton::RightStickLeft,
            right: StickSwitchButton::RightStickRight,
        },
    );

    assert_eq!(stick_swtich_interpreter.cardinal_custom_buttons,
        CardinalCustomButtons {
            up: StickSwitchButton::RightStickUp,
            down: StickSwitchButton::RightStickDown,
            left: StickSwitchButton::RightStickLeft,
            right: StickSwitchButton::RightStickRight,
        });
    assert_eq!(stick_swtich_interpreter.axis_click_thresholds,
        AxisClickThresholds {
            up: 0.3248,
            down: 0.243,
            left: 0.11211,
            right: 0.999,
            alignment: LeftOrRight::Left
        },);
}

#[test]
fn stick_swtich_interpreter_interpret_move_works() {
    let mut stick_swtich_interpreter = StickSwitchInterpreter {
        cardinal_custom_buttons: CardinalCustomButtons {
            up: StickSwitchButton::LeftStickUp,
            down: StickSwitchButton::LeftStickDown,
            left: StickSwitchButton::LeftStickLeft,
            right: StickSwitchButton::LeftStickRight,
        },
        axis_click_thresholds: AxisClickThresholds {
            up: 0.5,
            down: 0.8,
            left: 0.5,
            right: 0.3,
            alignment: LeftOrRight::Left
        },
        latest_button_pressed: None,
    };
    assert!(stick_swtich_interpreter.interpret_move(0.0, 0.0).is_none());
    assert!(stick_swtich_interpreter.interpret_move(0.29, 0.0).is_none());
    assert!(stick_swtich_interpreter.interpret_move(0.3, 0.0).is_none());
    // move is only recognized when it goes PAST the threshold
    assert_eq!(
        stick_swtich_interpreter.interpret_move(0.31, 0.0).unwrap(),
        StickSwitchButton::LeftStickRight,);
    assert_eq!(
        stick_swtich_interpreter.interpret_move(0.4, 0.0).unwrap(),
        StickSwitchButton::LeftStickRight,);
    assert_eq!(
        // y-axis has priority over x-axis
        stick_swtich_interpreter.interpret_move(0.4, -0.39).unwrap(),
        StickSwitchButton::LeftStickRight,);
    // left right active zone but isn't past down's threshold
    assert!(
        stick_swtich_interpreter.interpret_move(0.4, -0.41).is_none());

    assert!(
        stick_swtich_interpreter.interpret_move(0.4, -0.8).is_none());
    // finally get past down's threshold
    assert_eq!(
        stick_swtich_interpreter.interpret_move(0.4, -0.81).unwrap(),
        StickSwitchButton::LeftStickDown,);
    assert_eq!(
        stick_swtich_interpreter.interpret_move(0.4, -1.0).unwrap(),
        StickSwitchButton::LeftStickDown,);


    assert!(stick_swtich_interpreter.interpret_move(0.0, 0.0).is_none());
    assert!(stick_swtich_interpreter.interpret_move(-0.49, 0.0).is_none());
    assert!(stick_swtich_interpreter.interpret_move(-0.5, 0.0).is_none());
    // move is only recognized when it goes PAST the threshold
    assert_eq!(
        stick_swtich_interpreter.interpret_move(-0.51, 0.0).unwrap(),
        StickSwitchButton::LeftStickLeft,);
    assert_eq!(
        stick_swtich_interpreter.interpret_move(-0.51, 0.49).unwrap(),
        StickSwitchButton::LeftStickLeft,);
    // Remember that the y-axis get priority over x-axis when they're equal
    assert_eq!(
        stick_swtich_interpreter.interpret_move(-0.51, 0.51).unwrap(),
        StickSwitchButton::LeftStickUp,);
    assert_eq!(
        stick_swtich_interpreter.interpret_move(-0.51, 1.0).unwrap(),
        StickSwitchButton::LeftStickUp,);
}

#[test]
fn stick_switch_interpreter_interpret_stick_works() {
    let mut stick_swtich_interpreter = StickSwitchInterpreter::new(
        AxisClickThresholds {
            up: 0.55,
            down: 0.56,
            left: 0.57,
            right: 0.58,
            alignment: LeftOrRight::Right,
        },
        CardinalCustomButtons {
            up: StickSwitchButton::RightStickUp,
            right: StickSwitchButton::RightStickRight,
            down: StickSwitchButton::RightStickDown,
            left: StickSwitchButton::RightStickLeft
        },
     );

    assert!(stick_swtich_interpreter.interpret_stick_move(Some(0.0),Some(0.0),).is_none());
    assert!(stick_swtich_interpreter.interpret_stick_move(Some(0.0),Some(0.2),).is_none());
    assert!(stick_swtich_interpreter.interpret_stick_move(Some(0.0),Some(0.4),).is_none());
    assert_eq!(stick_swtich_interpreter.interpret_stick_move(Some(0.0),Some(0.6)),Some(
            StickSwitchEvent::ButtonPressed(StickSwitchButton::RightStickUp)));
    assert!(stick_swtich_interpreter.interpret_stick_move(Some(0.0),Some(0.8),).is_none());
    assert!(stick_swtich_interpreter.interpret_stick_move(Some(0.0),Some(0.6),).is_none());
    assert_eq!(stick_swtich_interpreter.interpret_stick_move(Some(0.0),Some(0.4)),Some(
            StickSwitchEvent::ButtonReleased(StickSwitchButton::RightStickUp)));
    assert!(stick_swtich_interpreter.interpret_stick_move(Some(0.0),Some(0.3),).is_none());
    assert_eq!(stick_swtich_interpreter.interpret_stick_move(Some(0.0),Some(0.56)),Some(
            StickSwitchEvent::ButtonPressed(StickSwitchButton::RightStickUp)));
    assert_eq!(stick_swtich_interpreter.interpret_stick_move(Some(0.0),Some(0.55)),Some(
            StickSwitchEvent::ButtonReleased(StickSwitchButton::RightStickUp)));
    assert_eq!(stick_swtich_interpreter.interpret_stick_move(Some(0.0),Some(0.56)),Some(
            StickSwitchEvent::ButtonPressed(StickSwitchButton::RightStickUp)));

    assert_eq!(stick_swtich_interpreter.interpret_stick_move(Some(0.0),Some(0.55)),Some(
            StickSwitchEvent::ButtonReleased(StickSwitchButton::RightStickUp)));
    assert_eq!(stick_swtich_interpreter.interpret_stick_move(Some(0.0),Some(0.56)),Some(
            StickSwitchEvent::ButtonPressed(StickSwitchButton::RightStickUp)));
    assert!(stick_swtich_interpreter.interpret_stick_move(Some(0.0),Some(0.59),).is_none());

    assert_eq!(stick_swtich_interpreter.interpret_stick_move(Some(0.0),Some(0.0)),Some(
            StickSwitchEvent::ButtonReleased(StickSwitchButton::RightStickUp)));
}
