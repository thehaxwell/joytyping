use std::time::SystemTime;

use gilrs::Button;
use mockall::predicate::eq;

use crate::{gamepad::{switch_click_pattern_detector::MockSwitchClickPatternDetectorTrait, layers_wrapper::MockLayersWrapperTrait, gilrs_events::{MockGilrsEventsTrait, gilrs_wrapper::{GilrsEvent, GilrsEventType}, stick_switch_interpreter::{StickSwitchEvent, StickSwitchButton}}, layers_navigator::MockLayersNavigatorTrait, cardinal_levers_move_detector, Gamepad}, quick_lookup_window::MockQuickLookupWindowTrait};

struct SetupNextEventTestArgs {
    event: GilrsEventType,
    // button: Button,
}

fn setup_next_event_test(
    args: SetupNextEventTestArgs) -> bool {
    let mut mock_switch_click_pattern_detector = MockSwitchClickPatternDetectorTrait::new();
    let mock_layers_wrapper = MockLayersWrapperTrait::new();
    let mut mock_gilrs_events = MockGilrsEventsTrait::new();
    let mock_layers_navigator = MockLayersNavigatorTrait::new();
    let mock_quick_lookup_window = MockQuickLookupWindowTrait::new();
    let mut mock_mouse_cardinal_levers_move_detector 
        = cardinal_levers_move_detector::mouse::MockMouseTrait::new();

    mock_gilrs_events
        .expect_next()
        .return_const(
            GilrsEvent{
                event: args.event,
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

    let mut gamepad = Gamepad {
       gilrs_events: Box::new(mock_gilrs_events),
       layers: Box::new(mock_layers_wrapper),
       switch_click_pattern_detector: Box::new(mock_switch_click_pattern_detector),
       layers_navigator: Box::new(mock_layers_navigator),
       quick_lookup_window: Box::new(mock_quick_lookup_window),
       mouse_cardinal_levers_move_detector: 
           Box::new(mock_mouse_cardinal_levers_move_detector),
    };

    gamepad.next_event()
}

#[test]
fn button_pressed_events() {
    assert!(setup_next_event_test(
        SetupNextEventTestArgs {
            event: GilrsEventType::ButtonPressed(Button::East, ),
        }));

    assert!(setup_next_event_test(
        SetupNextEventTestArgs {
            event: GilrsEventType::ButtonPressed(Button::RightTrigger, ),
        }));

    assert!(setup_next_event_test(
        SetupNextEventTestArgs {
            event: GilrsEventType::ButtonPressed(Button::Mode, ),
        }));
}

#[test]
fn button_released_events() {
    assert!(setup_next_event_test(
        SetupNextEventTestArgs {
            event: GilrsEventType::ButtonReleased(Button::East, ),
        }));

    assert!(setup_next_event_test(
        SetupNextEventTestArgs {
            event: GilrsEventType::ButtonReleased(Button::RightTrigger, ),
        }));

    assert!(setup_next_event_test(
        SetupNextEventTestArgs {
            event: GilrsEventType::ButtonReleased(Button::Mode, ),
        }));
}

#[test]
fn axis_changed_events() {
    assert!(setup_next_event_test(
        SetupNextEventTestArgs {
            event: GilrsEventType::AxisChanged(
               gilrs::Axis::LeftStickX, 0.34, 
               Some(StickSwitchEvent::ButtonPressed(StickSwitchButton::RightStickUp))),
        }));

    assert!(setup_next_event_test(
        SetupNextEventTestArgs {
            event: GilrsEventType::AxisChanged(
               gilrs::Axis::LeftStickY, 0.0, 
               Some(StickSwitchEvent::ButtonPressed(StickSwitchButton::RightStickUp))),
        }));

    assert!(setup_next_event_test(
        SetupNextEventTestArgs {
            event: GilrsEventType::AxisChanged(
               gilrs::Axis::RightStickX, 0.34, 
               Some(StickSwitchEvent::ButtonPressed(StickSwitchButton::LeftStickRight))),
        }));

}
