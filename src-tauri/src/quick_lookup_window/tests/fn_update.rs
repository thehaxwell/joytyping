use mockall::predicate::eq;

use crate::{settings::models::{self, main_config::Theme}, quick_lookup_window::{files::MockFilesTrait, QuickLookupWindow, QuickLookupWindowTrait, QuickLookupWindowState, UpdateKeyboardEventPayload}, tauri_app_handle_wrapper::{MockTauriAppHandleTrait, WindowOperationOutcome, EmitWindowEventPayload}, gamepad::Switch};


const WINDOW_LABEL: &str = "quick-lookup";

fn setup_quick_lookup_window_settings_example() -> models::QuickLookupWindow {
    models::QuickLookupWindow{
        inner_size: models::HeightAndWidth{
            height: 100.0,
            width: 100.0,
        },
        source_code: models::BrowserSourceCode{
            js_iife_bundle_file_path: "path/to/file/bundle.js".to_string(),
            css_file_path: None,
        }
    }
}

#[test]
fn works_when_no_window_is_open() {
    let mut mock_tauri_app_handle = MockTauriAppHandleTrait::new();
    let mock_files = MockFilesTrait::new();

    mock_tauri_app_handle
        .expect_emit_window_event()
        .with(eq(WINDOW_LABEL),
            eq("update-keyboard"),
            eq(EmitWindowEventPayload::UpdateKeyboardEventPayload(
                UpdateKeyboardEventPayload{
                layer: 2,
            })))
        .returning(|_,_,_| Ok(WindowOperationOutcome::WindowNotFound));

    let mut quick_lookup_window = QuickLookupWindow { 
        tauri_app_handle: Box::new(mock_tauri_app_handle),
        current_state: QuickLookupWindowState::Hidden,
        initialization_script: None,
        current_layer: 0,
        files: Box::new(mock_files),
        quick_lookup_window_settings: 
            setup_quick_lookup_window_settings_example(),
        restart_on_change_file_path: 
            Some("other/file/path/bundle.js".to_string()),
        theme: Theme::Light,
    };
    assert!(quick_lookup_window.update(2).is_ok());
    assert_eq!(quick_lookup_window.current_layer,2);
}


#[test]
fn works_when_a_window_is_open() {
    let mut mock_tauri_app_handle = MockTauriAppHandleTrait::new();
    let mock_files = MockFilesTrait::new();

    mock_tauri_app_handle
        .expect_emit_window_event()
        .with(eq(WINDOW_LABEL),
            eq("update-keyboard"),
            eq(EmitWindowEventPayload::UpdateKeyboardEventPayload(
                UpdateKeyboardEventPayload{
                layer: 2,
            })))
        .returning(|_,_,_| Ok(WindowOperationOutcome::Success));

    let mut quick_lookup_window = QuickLookupWindow { 
        tauri_app_handle: Box::new(mock_tauri_app_handle),
        current_state: QuickLookupWindowState::Showing(Switch::Button(gilrs::Button::DPadUp)),
        initialization_script: None,
        current_layer: 0,
        files: Box::new(mock_files),
        quick_lookup_window_settings: 
            setup_quick_lookup_window_settings_example(),
        restart_on_change_file_path: 
            Some("other/file/path/bundle.js".to_string()),
        theme: Theme::Light,
    };
    assert!(quick_lookup_window.update(2).is_ok());
    assert_eq!(quick_lookup_window.current_layer,2);
}

#[test]
fn handles_emit_window_event_errors() {
    let mut mock_tauri_app_handle = MockTauriAppHandleTrait::new();
    let mock_files = MockFilesTrait::new();

    mock_tauri_app_handle
        .expect_emit_window_event()
        .with(eq(WINDOW_LABEL),
            eq("update-keyboard"),
            eq(EmitWindowEventPayload::UpdateKeyboardEventPayload(
                UpdateKeyboardEventPayload{
                layer: 2,
            })))
        .returning(|_,_,_| Err(tauri::Error::FailedToSendMessage));

    let mut quick_lookup_window = QuickLookupWindow { 
        tauri_app_handle: Box::new(mock_tauri_app_handle),
        current_state: QuickLookupWindowState::Hidden,
        initialization_script: None,
        current_layer: 0,
        files: Box::new(mock_files),
        quick_lookup_window_settings: 
            setup_quick_lookup_window_settings_example(),
        restart_on_change_file_path: 
            Some("other/file/path/bundle.js".to_string()),
        theme: Theme::Light,
    };
    assert_eq!(quick_lookup_window.update(2)
            .unwrap_err()
            .to_string(),
            "failed to send message to the webview".to_string());
    assert_eq!(quick_lookup_window.current_layer,2);
}

