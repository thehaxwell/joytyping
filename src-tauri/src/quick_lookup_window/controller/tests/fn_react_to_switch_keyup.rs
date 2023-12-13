use crate::{quick_lookup_window::{controller::{Controller, QuickLookupWindowState}, MockQuickLookupWindowTrait}, gamepad::Switch, tauri_app_handle_wrapper::WindowOperationOutcome};


#[test]
fn works() {
    let mut mock_quick_lookup_window = MockQuickLookupWindowTrait::new();

    mock_quick_lookup_window
        .expect_hide()
        .times(1)
        .returning(|| Ok(WindowOperationOutcome::Success));

    let mut controller = Controller {
        quick_lookup_window: Box::new(mock_quick_lookup_window),
        current_state: QuickLookupWindowState::Showing {
            switch: Switch::Button(gilrs::Button::East),
            hide_on_keyup: true },
    };

    assert_eq!(controller.react_to_switch_keyup(Switch::Button(gilrs::Button::East)).unwrap(),());
    assert_eq!(controller.current_state,QuickLookupWindowState::Hidden);
}

#[test]
fn works_when_window_not_found() {
    let mut mock_quick_lookup_window = MockQuickLookupWindowTrait::new();

    mock_quick_lookup_window
        .expect_hide()
        .times(1)
        .returning(|| Ok(WindowOperationOutcome::WindowNotFound));

    let mut controller = Controller {
        quick_lookup_window: Box::new(mock_quick_lookup_window),
        current_state: QuickLookupWindowState::Showing {
            switch: Switch::Button(gilrs::Button::East),
            hide_on_keyup: true },
    };

    assert_eq!(controller.react_to_switch_keyup(Switch::Button(gilrs::Button::East)).unwrap(),());
    assert_ne!(controller.current_state,QuickLookupWindowState::Hidden);
}

#[test]
fn handles_hide_error() {
    let mut mock_quick_lookup_window = MockQuickLookupWindowTrait::new();

    mock_quick_lookup_window
        .expect_hide()
        .times(1)
        .returning(|| Err(tauri::Error::WebviewNotFound));

    let mut controller = Controller {
        quick_lookup_window: Box::new(mock_quick_lookup_window),
        current_state: QuickLookupWindowState::Showing {
            switch: Switch::Button(gilrs::Button::East),
            hide_on_keyup: true },
    };

    assert_eq!(controller.react_to_switch_keyup(Switch::Button(gilrs::Button::East))
        .unwrap_err()
        .to_string(),
        "webview not found: invalid label or it was closed".to_string());
    assert_ne!(controller.current_state,QuickLookupWindowState::Hidden);
}

#[test]
fn does_nothing_when_hide_on_keyup_isnt_set() {
    let mock_quick_lookup_window = MockQuickLookupWindowTrait::new();

    let mut controller = Controller {
        quick_lookup_window: Box::new(mock_quick_lookup_window),
        current_state: QuickLookupWindowState::Showing { 
            switch: Switch::Button(gilrs::Button::South),
            hide_on_keyup: false },
    };

    assert_eq!(controller.react_to_switch_keyup(Switch::Button(gilrs::Button::East)).unwrap(),());


    let mock_quick_lookup_window = MockQuickLookupWindowTrait::new();

    let mut controller = Controller {
        quick_lookup_window: Box::new(mock_quick_lookup_window),
        current_state: QuickLookupWindowState::Showing {
            switch: Switch::Button(gilrs::Button::East),
            hide_on_keyup: false },
    };

    assert_eq!(controller.react_to_switch_keyup(Switch::Button(gilrs::Button::East)).unwrap(),());
}
#[test]
fn does_nothing_when_state_is_hidden() {
    let mock_quick_lookup_window = MockQuickLookupWindowTrait::new();

    let mut controller = Controller {
        quick_lookup_window: Box::new(mock_quick_lookup_window),
        current_state: QuickLookupWindowState::Hidden,
    };

    assert_eq!(controller.react_to_switch_keyup(Switch::Button(gilrs::Button::East)).unwrap(),());


}
