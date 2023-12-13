use mockall::predicate::eq;

use crate::{quick_lookup_window::{controller::{Controller, QuickLookupWindowState}, MockQuickLookupWindowTrait}, gamepad_listener::{Switch, QuickLookupWindowEvent}, tauri_app_handle_wrapper::WindowOperationOutcome};

#[test]
fn works_for_show_until_switch_keyup() {
    let mut mock_quick_lookup_window = MockQuickLookupWindowTrait::new();

    mock_quick_lookup_window
        .expect_show()
        .times(1)
        .returning(|| Ok(WindowOperationOutcome::Success));

    let mut controller = Controller {
        quick_lookup_window: Box::new(mock_quick_lookup_window),
        current_state: QuickLookupWindowState::Hidden,
    };

    assert_eq!(controller.react_to_command(
       QuickLookupWindowEvent::ShowUntilSwitchKeyup(
            Switch::Button(gilrs::Button::East))).unwrap(),());

    assert_eq!(controller.current_state,
       QuickLookupWindowState::Showing{
           switch: Switch::Button(gilrs::Button::East), hide_on_keyup:true});
}

#[test]
fn handles_show_errors_for_show_until_switch_keyup() {
    let mut mock_quick_lookup_window = MockQuickLookupWindowTrait::new();
    
    mock_quick_lookup_window
        .expect_show()
        .times(1)
        .returning(|| Err(tauri::Error::InvalidWindowHandle));
    
    let mut controller = Controller {
        quick_lookup_window: Box::new(mock_quick_lookup_window),
        current_state: QuickLookupWindowState::Hidden,
    };
    
    assert_eq!(controller.react_to_command(
       QuickLookupWindowEvent::ShowUntilSwitchKeyup(
            Switch::Button(gilrs::Button::East)))
        .unwrap_err()
        .to_string(),
        "Unexpected `raw_window_handle` for the current platform".to_string());
    
    assert_eq!(controller.current_state,QuickLookupWindowState::Hidden);
}

#[test]
fn works_for_emit_current_layer_notification() {
    let mut mock_quick_lookup_window = MockQuickLookupWindowTrait::new();

    mock_quick_lookup_window
        .expect_update()
        .with(eq(3))
        .times(1)
        .returning(|_| Ok(()));

    let mut controller = Controller {
        quick_lookup_window: Box::new(mock_quick_lookup_window),
        current_state: QuickLookupWindowState::Hidden,
    };

    assert_eq!(controller.react_to_command(
       QuickLookupWindowEvent::EmitCurrentLayerNotification(3)).unwrap(),());

    assert_eq!(controller.current_state,QuickLookupWindowState::Hidden);
}

#[test]
fn handles_update_errors_emit_current_layer_notification() {
    let mut mock_quick_lookup_window = MockQuickLookupWindowTrait::new();
    
    mock_quick_lookup_window
        .expect_update()
        .with(eq(11))
        .times(1)
        .returning(|_| Err(tauri::Error::InvalidWindowHandle));
    
    let mut controller = Controller {
        quick_lookup_window: Box::new(mock_quick_lookup_window),
        current_state: QuickLookupWindowState::Hidden,
    };
    
    assert_eq!(controller.react_to_command(
       QuickLookupWindowEvent::EmitCurrentLayerNotification(11))
        .unwrap_err()
        .to_string(),
        "Unexpected `raw_window_handle` for the current platform".to_string());
    
    assert_eq!(controller.current_state,QuickLookupWindowState::Hidden);
}

#[test]
fn works_for_toggle_by_switch_showing_state() {
    let mut mock_quick_lookup_window = MockQuickLookupWindowTrait::new();

    mock_quick_lookup_window
        .expect_hide()
        .times(1)
        .returning(|| Ok(WindowOperationOutcome::Success));

    let mut controller = Controller {
        quick_lookup_window: Box::new(mock_quick_lookup_window),
        current_state: QuickLookupWindowState::Showing{
           switch: Switch::Button(gilrs::Button::East), hide_on_keyup:true},
    };

    assert_eq!(controller.react_to_command(
       QuickLookupWindowEvent::ToggleBySwitch(
            Switch::Button(gilrs::Button::East))).unwrap(),());

    assert_eq!(controller.current_state,QuickLookupWindowState::Hidden);


    // switches don't match so nothing happens
    let mock_quick_lookup_window = MockQuickLookupWindowTrait::new();

    let mut controller = Controller {
        quick_lookup_window: Box::new(mock_quick_lookup_window),
        current_state: QuickLookupWindowState::Showing{
           switch: Switch::Button(gilrs::Button::South), hide_on_keyup:true},
    };

    assert_eq!(controller.react_to_command(
       QuickLookupWindowEvent::ToggleBySwitch(
            Switch::Button(gilrs::Button::East))).unwrap(),());

    assert_eq!(controller.current_state,QuickLookupWindowState::Showing{
           switch: Switch::Button(gilrs::Button::South), hide_on_keyup:true});
}

#[test]
fn handles_hide_errors_for_toggle_by_switch_showing_state() {
    let mut mock_quick_lookup_window = MockQuickLookupWindowTrait::new();
   
    mock_quick_lookup_window
        .expect_hide()
        .times(1)
        .returning(|| Err(tauri::Error::InvalidWindowHandle));
   
    let mut controller = Controller {
        quick_lookup_window: Box::new(mock_quick_lookup_window),
        current_state: QuickLookupWindowState::Showing{
           switch: Switch::Button(gilrs::Button::East), hide_on_keyup:true},
    };
   
    assert_eq!(controller.react_to_command(
       QuickLookupWindowEvent::ToggleBySwitch(
            Switch::Button(gilrs::Button::East)))
        .unwrap_err()
        .to_string(),
        "Unexpected `raw_window_handle` for the current platform".to_string());
   
    assert_eq!(controller.current_state,QuickLookupWindowState::Showing{
           switch: Switch::Button(gilrs::Button::East), hide_on_keyup:true});
}

#[test]
fn works_for_toggle_by_switch_hidden_state() {
    let mut mock_quick_lookup_window = MockQuickLookupWindowTrait::new();

    mock_quick_lookup_window
        .expect_show()
        .times(1)
        .returning(|| Ok(WindowOperationOutcome::Success));

    let mut controller = Controller {
        quick_lookup_window: Box::new(mock_quick_lookup_window),
        current_state: QuickLookupWindowState::Hidden,
    };

    assert_eq!(controller.react_to_command(
       QuickLookupWindowEvent::ToggleBySwitch(
            Switch::Button(gilrs::Button::East))).unwrap(),());

    assert_eq!(controller.current_state,QuickLookupWindowState::Showing{
        switch: Switch::Button(gilrs::Button::East), hide_on_keyup: false});
}

#[test]
fn handles_hide_errors_for_toggle_by_switch_hidden_state() {
    let mut mock_quick_lookup_window = MockQuickLookupWindowTrait::new();
   
    mock_quick_lookup_window
        .expect_show()
        .times(1)
        .returning(|| Err(tauri::Error::InvalidWindowHandle));
   
    let mut controller = Controller {
        quick_lookup_window: Box::new(mock_quick_lookup_window),
        current_state: QuickLookupWindowState::Hidden,
    };
   
    assert_eq!(controller.react_to_command(
       QuickLookupWindowEvent::ToggleBySwitch(
            Switch::Button(gilrs::Button::East)))
        .unwrap_err()
        .to_string(),
        "Unexpected `raw_window_handle` for the current platform".to_string());
   
    assert_eq!(controller.current_state,QuickLookupWindowState::Hidden);
}
