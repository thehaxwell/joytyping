use crate::gamepad::{gilrs_wrapper::MockGilrs, Gamepad, };
use crate::gamepad::sticks_interpreter::MockSticksInterpreterTrait;
use crate::gamepad::{GamepadEvent, CustomButton};

use mockall::predicate::*;

use super::gilrs_wrapper::{GilrsEvent, GilrsEventType};

fn setup_next_event_when_gilrs_next_event_is(event: GilrsEventType) -> Option<GamepadEvent> {
    let mut mock_gilrs = MockGilrs::new();
    let mut mock_sticks_interpreter = MockSticksInterpreterTrait::new();
    mock_gilrs
        .expect_next_event()
        .times(1)
        .returning(move || Some(GilrsEvent {
            event,
            time: std::time::SystemTime::now(),
        }));
    mock_sticks_interpreter
        .expect_interpret_left_stick_move()
        .times(0);
    mock_sticks_interpreter
        .expect_interpret_right_stick_move()
        .times(0);

    let mut gamepad = Gamepad::new(Box::new(mock_gilrs),Box::new(mock_sticks_interpreter));
    gamepad.next_event()
}

#[test]
fn next_event_returns_none_when_appropriate() {
    let mut mock_gilrs = MockGilrs::new();
    let mut mock_sticks_interpreter = MockSticksInterpreterTrait::new();
    mock_gilrs
        .expect_next_event()
        .times(1)
        .returning(|| None);
    mock_sticks_interpreter
        .expect_interpret_left_stick_move()
        .times(0)
        .returning(|_,_| None);

    let mut gamepad = Gamepad::new(Box::new(mock_gilrs),Box::new(mock_sticks_interpreter));
    assert_eq!(gamepad.next_event(),None);
}

#[test]
fn next_event_returns_button_pressed_event() {
    let button = gilrs::ev::Button::West;
    let res = setup_next_event_when_gilrs_next_event_is(
        GilrsEventType::ButtonPressed(button));
    assert_eq!(res,Some(GamepadEvent::ButtonPressed(CustomButton::Base(button))));
}

#[test]
fn next_event_returns_button_released_event() {
    let button = gilrs::ev::Button::North;
    let res = setup_next_event_when_gilrs_next_event_is(
        GilrsEventType::ButtonReleased(button));
    assert_eq!(res,Some(GamepadEvent::ButtonReleased(CustomButton::Base(button))));
}

#[test]
fn next_event_ignores_button_repeated_event() {
    let button = gilrs::ev::Button::East;
    let res = setup_next_event_when_gilrs_next_event_is(
        GilrsEventType::ButtonRepeated(button));
    assert_eq!(res,None);
}

#[test]
fn next_event_ignores_button_changed_event() {
    let button = gilrs::ev::Button::DPadUp;
    let res = setup_next_event_when_gilrs_next_event_is(
        GilrsEventType::ButtonChanged(button,0.00));
    assert_eq!(res,None);
}

#[test]
fn next_event_ignores_connected_event() {
    let res = setup_next_event_when_gilrs_next_event_is(
        GilrsEventType::Connected);
    assert_eq!(res,None);
}

#[test]
fn next_event_ignores_disconnected_event() {
    let res = setup_next_event_when_gilrs_next_event_is(
        GilrsEventType::Disconnected);
    assert_eq!(res,None);
}

#[test]
fn next_event_ignores_dropped_event() {
    let res = setup_next_event_when_gilrs_next_event_is(
        GilrsEventType::Dropped);
    assert_eq!(res,None);
}

//
// TEST AXIS CHANGES
//

#[derive(Clone, Copy, Debug, PartialEq)]
struct AxisChangeTest {
    source_axis: gilrs::ev::Axis,
    axes_queried: (gilrs::ev::Axis,gilrs::ev::Axis),
    movement: (f32,f32),
    calls_left_stick_interpreter: bool,
    calls_right_stick_interpreter: bool,
    left_stick_interpretation: Option<GamepadEvent>,
    right_stick_interpretation: Option<GamepadEvent>,
}

fn setup_next_event_axis_change_test(config: AxisChangeTest) -> Option<GamepadEvent> {
    let mut mock_gilrs = MockGilrs::new();
    let mut mock_sticks_interpreter = MockSticksInterpreterTrait::new();
    mock_gilrs
        .expect_next_event()
        .times(1)
        .returning(move || Some(GilrsEvent {
            event: GilrsEventType::AxisChanged(config.source_axis, config.movement.0 ),
            time: std::time::SystemTime::now(),
        }));

    mock_gilrs
        .expect_get_gamepad_axis_data_value()
        .with(eq(config.axes_queried.0))
        .return_const(config.movement.0);
    mock_gilrs
        .expect_get_gamepad_axis_data_value()
        .with(eq(config.axes_queried.1))
        .return_const(config.movement.1);

    if config.calls_left_stick_interpreter {
        mock_sticks_interpreter
            .expect_interpret_left_stick_move()
            .with(eq(Some(config.movement.0),),eq(Some(config.movement.1)))
            .returning(move |_,_| config.left_stick_interpretation);
    } else {
        mock_sticks_interpreter
            .expect_interpret_left_stick_move()
            .times(0);
    }

    if config.calls_right_stick_interpreter {
        mock_sticks_interpreter
            .expect_interpret_right_stick_move()
            .with(eq(Some(config.movement.0),),eq(Some(config.movement.1)))
            .returning(move |_,_| config.right_stick_interpretation);
    } else {
        mock_sticks_interpreter
            .expect_interpret_right_stick_move()
            .times(0);
    }

    let mut gamepad = Gamepad::new(Box::new(mock_gilrs),Box::new(mock_sticks_interpreter));
    gamepad.next_event()
}

// Left stick

#[test]
fn next_event_reflects_inconsequential_left_stick_move() {
    assert_eq!(setup_next_event_axis_change_test(AxisChangeTest {
        source_axis: gilrs::ev::Axis::LeftStickX,
        axes_queried: (gilrs::ev::Axis::LeftStickX,gilrs::ev::Axis::LeftStickY),
        movement: (0.0,0.125),
        calls_left_stick_interpreter: true,
        left_stick_interpretation: None,
        calls_right_stick_interpreter: false,
        right_stick_interpretation: None,
    }),None);
}
#[test]
fn next_event_reflects_left_stick_move_to_left() {
    let expected_res = Some(
        GamepadEvent::ButtonPressed(CustomButton::LeftStickLeft));
    assert_eq!(setup_next_event_axis_change_test(AxisChangeTest {
        source_axis: gilrs::ev::Axis::LeftStickX,
        axes_queried: (gilrs::ev::Axis::LeftStickX,gilrs::ev::Axis::LeftStickY),
        movement: (-0.5,0.125),
        calls_left_stick_interpreter: true,
        left_stick_interpretation: expected_res,
        calls_right_stick_interpreter: false,
        right_stick_interpretation: None,
    }),expected_res);
}

#[test]
fn next_event_reflects_left_stick_move_to_right() {
    let expected_res = Some(
        GamepadEvent::ButtonPressed(CustomButton::LeftStickRight));
    assert_eq!(setup_next_event_axis_change_test(AxisChangeTest {
        source_axis: gilrs::ev::Axis::LeftStickX,
        axes_queried: (gilrs::ev::Axis::LeftStickX,gilrs::ev::Axis::LeftStickY),
        movement: (0.5,0.125),
        calls_left_stick_interpreter: true,
        left_stick_interpretation: expected_res,
        calls_right_stick_interpreter: false,
        right_stick_interpretation: None,
    }),expected_res);
}

#[test]
fn next_event_reflects_left_stick_move_to_up() {
    let expected_res = Some(
        GamepadEvent::ButtonPressed(CustomButton::LeftStickUp));
    assert_eq!(setup_next_event_axis_change_test(AxisChangeTest {
        source_axis: gilrs::ev::Axis::LeftStickX,
        axes_queried: (gilrs::ev::Axis::LeftStickX,gilrs::ev::Axis::LeftStickY),
        movement: (0.4,0.5),
        calls_left_stick_interpreter: true,
        left_stick_interpretation: expected_res,
        calls_right_stick_interpreter: false,
        right_stick_interpretation: None,
    }),expected_res);
}

#[test]
fn next_event_reflects_left_stick_move_to_down() {
    let expected_res = Some(
        GamepadEvent::ButtonPressed(CustomButton::LeftStickDown));
    assert_eq!(setup_next_event_axis_change_test(AxisChangeTest {
        source_axis: gilrs::ev::Axis::LeftStickX,
        axes_queried: (gilrs::ev::Axis::LeftStickX,gilrs::ev::Axis::LeftStickY),
        movement: (0.4,-0.5),
        calls_left_stick_interpreter: true,
        left_stick_interpretation: expected_res,
        calls_right_stick_interpreter: false,
        right_stick_interpretation: None,
    }),expected_res);
}

// Right stick

#[test]
fn next_event_reflects_inconsequential_right_stick_move() {
    assert_eq!(setup_next_event_axis_change_test(AxisChangeTest {
        source_axis: gilrs::ev::Axis::RightStickX,
        axes_queried: (gilrs::ev::Axis::RightStickX,gilrs::ev::Axis::RightStickY),
        movement: (0.4988,0.125),
        calls_left_stick_interpreter: false,
        left_stick_interpretation: None,
        calls_right_stick_interpreter: true,
        right_stick_interpretation: None,
    }),None);
}

#[test]
fn next_event_reflects_right_stick_move_to_left() {
    let expected_res = Some(
        GamepadEvent::ButtonPressed(CustomButton::RightStickRight));
    assert_eq!(setup_next_event_axis_change_test(AxisChangeTest {
        source_axis: gilrs::ev::Axis::RightStickX,
        axes_queried: (gilrs::ev::Axis::RightStickX,gilrs::ev::Axis::RightStickY),
        movement: (-0.5,0.125),
        calls_left_stick_interpreter: false,
        left_stick_interpretation: None,
        calls_right_stick_interpreter: true,
        right_stick_interpretation: expected_res,
    }),expected_res);
}

#[test]
fn next_event_reflects_right_stick_move_to_right() {
    let expected_res = Some(
        GamepadEvent::ButtonPressed(CustomButton::RightStickRight));
    assert_eq!(setup_next_event_axis_change_test(AxisChangeTest {
        source_axis: gilrs::ev::Axis::RightStickX,
        axes_queried: (gilrs::ev::Axis::RightStickX,gilrs::ev::Axis::RightStickY),
        movement: (0.5,0.125),
        calls_left_stick_interpreter: false,
        left_stick_interpretation: None,
        calls_right_stick_interpreter: true,
        right_stick_interpretation: expected_res,
    }),expected_res);
}

#[test]
fn next_event_reflects_right_stick_move_to_up() {
    let expected_res = Some(
        GamepadEvent::ButtonPressed(CustomButton::RightStickUp));
    assert_eq!(setup_next_event_axis_change_test(AxisChangeTest {
        source_axis: gilrs::ev::Axis::RightStickX,
        axes_queried: (gilrs::ev::Axis::RightStickX,gilrs::ev::Axis::RightStickY),
        movement: (0.4,0.5),
        calls_left_stick_interpreter: false,
        left_stick_interpretation: None,
        calls_right_stick_interpreter: true,
        right_stick_interpretation: expected_res,
    }),expected_res);
}

#[test]
fn next_event_reflects_right_stick_move_to_down() {
    let expected_res = Some(
        GamepadEvent::ButtonPressed(CustomButton::RightStickDown));
    assert_eq!(setup_next_event_axis_change_test(AxisChangeTest {
        source_axis: gilrs::ev::Axis::RightStickX,
        axes_queried: (gilrs::ev::Axis::RightStickX,gilrs::ev::Axis::RightStickY),
        movement: (0.4,-0.5),
        calls_left_stick_interpreter: false,
        left_stick_interpretation: None,
        calls_right_stick_interpreter: true,
        right_stick_interpretation: expected_res,
    }),expected_res);
}

