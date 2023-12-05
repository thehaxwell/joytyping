use crate::{gamepad::gilrs_events::{GilrsEvents,GilrsEventsTrait, stick_switch_interpreter::{StickSwitchButton, StickSwitchEvent}, gilrs_wrapper::GamepadInfo}, LeftOrRight};

use super::{gilrs_wrapper::{MockGilrs, GilrsEventType, GilrsEvent}, stick_switch_interpreter::MockStickSwitchInterpreterTrait, trigger_2_switch_interpreter::{MockTrigger2SwitchInterpreterTrait, Trigger2SwitchEvent}};

use mockall::predicate::*;

fn assert_just_now(time: std::time::SystemTime){
    assert!(time.elapsed().unwrap().as_millis() < 200);
}

fn setup_next_when_gilrs_next_event_is(event: GilrsEventType) -> Option<GilrsEvent> {
    let mut mock_gilrs = MockGilrs::new();
    let mut mock_left_stick_switch_interpreter = MockStickSwitchInterpreterTrait::new();
    let mut mock_right_stick_switch_interpreter = MockStickSwitchInterpreterTrait::new();
    let mut mock_left_trigger_2_switch_interpreter = MockTrigger2SwitchInterpreterTrait::new();
    let mut mock_right_trigger_2_switch_interpreter = MockTrigger2SwitchInterpreterTrait::new();
    mock_gilrs
        .expect_next_event()
        .times(1)
        .returning(move || Some(GilrsEvent {
            event: event.clone(),
            time: std::time::SystemTime::now(),
        }));
    mock_left_stick_switch_interpreter
        .expect_interpret_stick_move()
        .times(0);
    mock_right_stick_switch_interpreter
        .expect_interpret_stick_move()
        .times(0);
    mock_left_trigger_2_switch_interpreter
        .expect_interpret_move()
        .times(0);
    mock_right_trigger_2_switch_interpreter
        .expect_interpret_move()
        .times(0);

    let mut gamepad = GilrsEvents::new(
        Box::new(mock_gilrs),
        Box::new(mock_left_stick_switch_interpreter),
        Box::new(mock_right_stick_switch_interpreter),
        Box::new(mock_left_trigger_2_switch_interpreter),
        Box::new(mock_right_trigger_2_switch_interpreter),
    );
    gamepad.next()
}

#[test]
fn next_passes_through_all_non_intercepted_events(){
    let event = GilrsEventType::ButtonPressed(gilrs::ev::Button::North);
    let res = setup_next_when_gilrs_next_event_is(event.clone()).unwrap();
    assert_eq!(res.event,event.clone());
    assert_just_now(res.time);

    let event = GilrsEventType::ButtonRepeated(gilrs::ev::Button::LeftThumb);
    let res = setup_next_when_gilrs_next_event_is(event.clone()).unwrap();
    assert_eq!(res.event,event.clone());
    assert_just_now(res.time);

    let event = GilrsEventType::ButtonChanged(gilrs::ev::Button::Start,0.1452);
    let res = setup_next_when_gilrs_next_event_is(event.clone()).unwrap();
    assert_eq!(res.event,event.clone());
    assert_just_now(res.time);

    let event = GilrsEventType::ButtonReleased(gilrs::ev::Button::West);
    let res = setup_next_when_gilrs_next_event_is(event.clone()).unwrap();
    assert_eq!(res.event,event.clone());
    assert_just_now(res.time);

    let event = GilrsEventType::Connected(Some(GamepadInfo{name: "PS3 Controller".to_string()}));
    let res = setup_next_when_gilrs_next_event_is(event.clone()).unwrap();
    assert_eq!(res.event,event.clone());
    assert_just_now(res.time);

    let event = GilrsEventType::Disconnected;
    let res = setup_next_when_gilrs_next_event_is(event.clone()).unwrap();
    assert_eq!(res.event,event.clone());
    assert_just_now(res.time);

    let event = GilrsEventType::Dropped;
    let res = setup_next_when_gilrs_next_event_is(event.clone()).unwrap();
    assert_eq!(res.event,event.clone());
    assert_just_now(res.time);
}

#[test]
fn next_returns_none_when_appropriate() {
    let mut mock_gilrs = MockGilrs::new();
    let mut mock_left_stick_switch_interpreter = MockStickSwitchInterpreterTrait::new();
    let mut mock_right_stick_switch_interpreter = MockStickSwitchInterpreterTrait::new();
    let mock_left_trigger_2_switch_interpreter = MockTrigger2SwitchInterpreterTrait::new();
    let mock_right_trigger_2_switch_interpreter = MockTrigger2SwitchInterpreterTrait::new();
    mock_gilrs
        .expect_next_event()
        .times(1)
        .returning(|| None);
    mock_left_stick_switch_interpreter
        .expect_interpret_stick_move()
        .times(0)
        .returning(|_,_| None);
    mock_right_stick_switch_interpreter
        .expect_interpret_stick_move()
        .times(0)
        .returning(|_,_| None);

    let mut gamepad = GilrsEvents::new(
        Box::new(mock_gilrs),
        Box::new(mock_left_stick_switch_interpreter),
        Box::new(mock_right_stick_switch_interpreter),
        Box::new(mock_left_trigger_2_switch_interpreter),
        Box::new(mock_right_trigger_2_switch_interpreter),
    );
    assert_eq!(gamepad.next(),None);
}


fn setup_next_when_gilrs_next_event_is_axis_change(
    left_or_right: LeftOrRight, axis: gilrs::Axis, value: f32, switch_stick_event: StickSwitchEvent
    ) -> Option<GilrsEvent> {
    let mut mock_gilrs = MockGilrs::new();
    let mut mock_left_stick_switch_interpreter = MockStickSwitchInterpreterTrait::new();
    let mut mock_right_stick_switch_interpreter = MockStickSwitchInterpreterTrait::new();
    let mock_left_trigger_2_switch_interpreter = MockTrigger2SwitchInterpreterTrait::new();
    let mock_right_trigger_2_switch_interpreter = MockTrigger2SwitchInterpreterTrait::new();

    mock_gilrs
        .expect_next_event()
        .times(1)
        .returning(move || Some(GilrsEvent {
            event: GilrsEventType::AxisChanged(axis, value, None),
            time: std::time::SystemTime::now(),
        }));

    // create random values for x- and y-axes
    let (x_axis_value, y_axis_value) = (value * 2.5, value * 1.7);
    mock_gilrs
        .expect_get_gamepad_axis_data_value()
        .with(eq(
            if let LeftOrRight::Left = left_or_right {
                gilrs::Axis::LeftStickX} else {
                    gilrs::Axis::RightStickX}))
        .times(1)
        .return_const(x_axis_value);
    mock_gilrs
        .expect_get_gamepad_axis_data_value()
        .with(eq(
            if let LeftOrRight::Left = left_or_right {
                gilrs::Axis::LeftStickY} else {
                    gilrs::Axis::RightStickY}))
        .times(1)
        .return_const(y_axis_value);


    if let LeftOrRight::Left = left_or_right {
        mock_left_stick_switch_interpreter
            .expect_interpret_stick_move()
            .with(eq(Some(x_axis_value)), eq(Some(y_axis_value)))
            .times(1)
            .returning(move |_,_| Some(switch_stick_event));
        mock_right_stick_switch_interpreter
            .expect_interpret_stick_move()
            .times(0);
    }
    else {
        mock_left_stick_switch_interpreter
            .expect_interpret_stick_move()
            .times(0);
        mock_right_stick_switch_interpreter
            .expect_interpret_stick_move()
            .with(eq(Some(x_axis_value)), eq(Some(y_axis_value)))
            .times(1)
            .returning(move |_,_| Some(switch_stick_event));
    }

    let mut gamepad = GilrsEvents::new(
        Box::new(mock_gilrs),
        Box::new(mock_left_stick_switch_interpreter),
        Box::new(mock_right_stick_switch_interpreter),
        Box::new(mock_left_trigger_2_switch_interpreter),
        Box::new(mock_right_trigger_2_switch_interpreter),
    );
    gamepad.next()
}

#[test]
fn next_correctly_adds_to_axis_changed_event() {
    let axis = gilrs::Axis::LeftStickX;
    let value = 0.1434;
    let switch_stick_event = StickSwitchEvent::ButtonPressed(
                    StickSwitchButton::LeftStickRight);

    let res = setup_next_when_gilrs_next_event_is_axis_change(
        LeftOrRight::Left,axis,value,switch_stick_event).unwrap();
    assert_eq!(res.event,
        GilrsEventType::AxisChanged(
            axis, value, Some(switch_stick_event)));
    assert_just_now(res.time);


    let axis = gilrs::Axis::LeftStickY;
    let value = 2.313;
    let switch_stick_event = StickSwitchEvent::ButtonPressed(
                    StickSwitchButton::LeftStickUp);

    let res = setup_next_when_gilrs_next_event_is_axis_change(
        LeftOrRight::Left,axis,value,switch_stick_event).unwrap();
    assert_eq!(res.event,
        GilrsEventType::AxisChanged(
            axis, value, Some(switch_stick_event)));
    assert_just_now(res.time);

    // Right side

    let axis = gilrs::Axis::RightStickX;
    let value = 0.1434;
    let switch_stick_event = StickSwitchEvent::ButtonPressed(
                    StickSwitchButton::RightStickRight);

    let res = setup_next_when_gilrs_next_event_is_axis_change(
        LeftOrRight::Right,axis,value,switch_stick_event).unwrap();
    assert_eq!(res.event,
        GilrsEventType::AxisChanged(
            axis, value, Some(switch_stick_event)));
    assert_just_now(res.time);


    let axis = gilrs::Axis::RightStickY;
    let value = 2.313;
    let switch_stick_event = StickSwitchEvent::ButtonPressed(
                    StickSwitchButton::RightStickUp);

    let res = setup_next_when_gilrs_next_event_is_axis_change(
        LeftOrRight::Right,axis,value,switch_stick_event).unwrap();
    assert_eq!(res.event,
        GilrsEventType::AxisChanged(
            axis, value, Some(switch_stick_event)));
    assert_just_now(res.time);
}

fn setup_next_trigger_button_pressed_event(
    button: gilrs::ev::Button, value: f32) -> Option<GilrsEvent>{
    let mut mock_gilrs = MockGilrs::new();
    let mut mock_left_stick_switch_interpreter = MockStickSwitchInterpreterTrait::new();
    let mut mock_right_stick_switch_interpreter = MockStickSwitchInterpreterTrait::new();
    let mut mock_left_trigger_2_switch_interpreter = MockTrigger2SwitchInterpreterTrait::new();
    let mut mock_right_trigger_2_switch_interpreter = MockTrigger2SwitchInterpreterTrait::new();

    mock_gilrs
        .expect_next_event()
        .times(1)
        .returning(move || Some(GilrsEvent {
            event: GilrsEventType::ButtonChanged(button,value),
            time: std::time::SystemTime::now(),
        }));
    mock_left_stick_switch_interpreter
        .expect_interpret_stick_move()
        .times(0);
    mock_right_stick_switch_interpreter
        .expect_interpret_stick_move()
        .times(0);

    if let gilrs::ev::Button::LeftTrigger2 = button {
        mock_left_trigger_2_switch_interpreter
            .expect_interpret_move()
            .times(1)
            .with(eq(value))
            .return_const(Trigger2SwitchEvent::ButtonPressed(button));
    }
    else {
        mock_left_trigger_2_switch_interpreter
            .expect_interpret_move()
            .times(0);
    }

    if let gilrs::ev::Button::RightTrigger2 = button {
        mock_right_trigger_2_switch_interpreter
            .expect_interpret_move()
            .times(1)
            .with(eq(value))
            .return_const(Trigger2SwitchEvent::ButtonPressed(button));
    }
    else {
        mock_right_trigger_2_switch_interpreter
            .expect_interpret_move()
            .times(0);
    }

    let mut gamepad = GilrsEvents::new(
        Box::new(mock_gilrs),
        Box::new(mock_left_stick_switch_interpreter),
        Box::new(mock_right_stick_switch_interpreter),
        Box::new(mock_left_trigger_2_switch_interpreter),
        Box::new(mock_right_trigger_2_switch_interpreter),
    );
    gamepad.next()
}

#[test]
fn next_correctly_changes_trigger_2_button_changed_events_to_button_pressed() {
    let res = setup_next_trigger_button_pressed_event(
        gilrs::ev::Button::LeftTrigger2,0.1452);
    assert_eq!(res.clone().unwrap().event,
        GilrsEventType::ButtonPressed(gilrs::ev::Button::LeftTrigger2));
    assert_just_now(res.clone().unwrap().time);

    let res = setup_next_trigger_button_pressed_event(
        gilrs::ev::Button::LeftTrigger2,0.321);
    assert_eq!(res.clone().unwrap().event,
        GilrsEventType::ButtonPressed(gilrs::ev::Button::LeftTrigger2));
    assert_just_now(res.clone().unwrap().time);

    let res = setup_next_trigger_button_pressed_event(
        gilrs::ev::Button::RightTrigger2,0.32);
    assert_eq!(res.clone().unwrap().event,
        GilrsEventType::ButtonPressed(gilrs::ev::Button::RightTrigger2));
    assert_just_now(res.clone().unwrap().time);

    let res = setup_next_trigger_button_pressed_event(
        gilrs::ev::Button::RightTrigger2,0.0001);
    assert_eq!(res.clone().unwrap().event,
        GilrsEventType::ButtonPressed(gilrs::ev::Button::RightTrigger2));
    assert_just_now(res.clone().unwrap().time);
}
