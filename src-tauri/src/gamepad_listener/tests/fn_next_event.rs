use std::time::SystemTime;

use gilrs::Button;
use mockall::predicate::eq;

use crate::gamepad_listener::{switch_click_pattern_detector::MockSwitchClickPatternDetectorTrait, gilrs_events::{MockGilrsEventsTrait, gilrs_wrapper::{GilrsEvent, GilrsEventType}, stick_switch_interpreter::{StickSwitchEvent, StickSwitchButton}}, layers_navigator::MockLayersNavigatorTrait, cardinal_levers_move_detector, GamepadListener};

struct SetupNextEventTestArgs {
    event: GilrsEventType,
    // button: Button,
}

fn setup_next_event_test(
    args: SetupNextEventTestArgs) -> Option<GilrsEventType> {
    let mut mock_switch_click_pattern_detector = MockSwitchClickPatternDetectorTrait::new();
    let mut mock_gilrs_events = MockGilrsEventsTrait::new();
    let mock_layers_navigator = MockLayersNavigatorTrait::new();
    let mut mock_mouse_cardinal_levers_move_detector 
        = cardinal_levers_move_detector::mouse::MockMouseTrait::new();

    mock_gilrs_events
        .expect_next()
        .return_const(
            GilrsEvent{
                event: args.event.clone(),
                time: SystemTime::now(),
            });

    match args.event {
        GilrsEventType::ButtonPressed(button, ) => {
            mock_switch_click_pattern_detector
                .expect_button_pressed()
                .with(eq(button))
                .return_const(());
        },
        GilrsEventType::ButtonReleased(button, ) => {
            mock_switch_click_pattern_detector
                .expect_button_released()
                .with(eq(button))
                .return_const(());
        },
        GilrsEventType::AxisChanged(axis, value, stick_switch_event_opt) => {
            mock_mouse_cardinal_levers_move_detector
                .expect_axis_changed()
                .with(eq(axis), eq(value))
                .return_const(());

            if let Some(stick_switch_event) 
                = stick_switch_event_opt {

                match stick_switch_event {
                    StickSwitchEvent::ButtonPressed(button) => {
                        mock_switch_click_pattern_detector
                            .expect_axis_button_pressed()
                            .with(eq(button))
                            .return_const(());
                    }
                    StickSwitchEvent::ButtonReleased(button) => {
                        mock_switch_click_pattern_detector
                            .expect_axis_button_released()
                            .with(eq(button))
                            .return_const(());
                    },
                };
            }
        },
        _ => (),
    }

    let mut gamepad_listener = GamepadListener {
       gilrs_events: Box::new(mock_gilrs_events),
       switch_click_pattern_detector: Box::new(mock_switch_click_pattern_detector),
       layers_navigator: Box::new(mock_layers_navigator),
       mouse_cardinal_levers_move_detector: 
           Box::new(mock_mouse_cardinal_levers_move_detector),
    };

    gamepad_listener.next_event()
}

#[test]
fn button_pressed_events() {
    assert_eq!(setup_next_event_test(
        SetupNextEventTestArgs {
            event: GilrsEventType::ButtonPressed(Button::East, ),
        }),Some(GilrsEventType::ButtonPressed(Button::East, )));
    
    assert_eq!(setup_next_event_test(
        SetupNextEventTestArgs {
            event: GilrsEventType::ButtonPressed(Button::RightTrigger, ),
        }),Some(GilrsEventType::ButtonPressed(Button::RightTrigger, )));
    
    assert_eq!(setup_next_event_test(
        SetupNextEventTestArgs {
            event: GilrsEventType::ButtonPressed(Button::Mode, ),
        }),Some(GilrsEventType::ButtonPressed(Button::Mode, )));
}

#[test]
fn button_released_events() {
    assert_eq!(setup_next_event_test(
        SetupNextEventTestArgs {
            event: GilrsEventType::ButtonReleased(Button::East, ),
        }),Some(GilrsEventType::ButtonReleased(Button::East, )));

    assert_eq!(setup_next_event_test(
        SetupNextEventTestArgs {
            event: GilrsEventType::ButtonReleased(Button::RightTrigger, ),
        }),Some(GilrsEventType::ButtonReleased(Button::RightTrigger, )));

    assert_eq!(setup_next_event_test(
        SetupNextEventTestArgs {
            event: GilrsEventType::ButtonReleased(Button::Mode, ),
        }),Some(GilrsEventType::ButtonReleased(Button::Mode, )));
}

#[test]
fn axis_changed_events() {
    assert_eq!(setup_next_event_test(
        SetupNextEventTestArgs {
            event: GilrsEventType::AxisChanged(
               gilrs::Axis::LeftStickX, 0.34, 
               Some(StickSwitchEvent::ButtonPressed(StickSwitchButton::RightStickUp))),
        }),Some(GilrsEventType::AxisChanged(
               gilrs::Axis::LeftStickX, 0.34, 
               Some(StickSwitchEvent::ButtonPressed(StickSwitchButton::RightStickUp)))));

    assert_eq!(setup_next_event_test(
        SetupNextEventTestArgs {
            event: GilrsEventType::AxisChanged(
               gilrs::Axis::LeftStickY, 0.0, 
               Some(StickSwitchEvent::ButtonPressed(StickSwitchButton::RightStickUp))),
        }),Some(GilrsEventType::AxisChanged(
               gilrs::Axis::LeftStickY, 0.0, 
               Some(StickSwitchEvent::ButtonPressed(StickSwitchButton::RightStickUp)))));

    assert_eq!(setup_next_event_test(
        SetupNextEventTestArgs {
            event: GilrsEventType::AxisChanged(
               gilrs::Axis::RightStickX, 0.34, 
               Some(StickSwitchEvent::ButtonPressed(StickSwitchButton::LeftStickRight))),
        }),Some(GilrsEventType::AxisChanged(
               gilrs::Axis::RightStickX, 0.34, 
               Some(StickSwitchEvent::ButtonPressed(StickSwitchButton::LeftStickRight)))));

}
