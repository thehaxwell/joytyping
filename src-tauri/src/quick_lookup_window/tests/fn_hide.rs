use mockall::predicate::eq;

use crate::{settings::models::{self, main_config::Theme}, quick_lookup_window::{files::MockFilesTrait, QuickLookupWindow, QuickLookupWindowTrait}, tauri_app_handle_wrapper::{MockTauriAppHandleTrait, WindowOperationOutcome}};

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
        .expect_hide_window()
        .with(eq(WINDOW_LABEL))
        .returning(|_| Ok(WindowOperationOutcome::WindowNotFound));

    let mut quick_lookup_window = QuickLookupWindow { 
        tauri_app_handle: Box::new(mock_tauri_app_handle),
        initialization_script: None,
        current_layer: 0,
        files: Box::new(mock_files),
        quick_lookup_window_settings: 
            setup_quick_lookup_window_settings_example(),
        restart_on_change_file_path: 
            Some("other/file/path/bundle.js".to_string()),
        theme: Theme::Light,
    };
    assert_eq!(quick_lookup_window.hide().unwrap(),
        WindowOperationOutcome::WindowNotFound);
}

#[test]
fn works_when_a_window_is_opened() {
    let mut mock_tauri_app_handle = MockTauriAppHandleTrait::new();
    let mock_files = MockFilesTrait::new();

    mock_tauri_app_handle
        .expect_hide_window()
        .with(eq(WINDOW_LABEL))
        .returning(|_| Ok(WindowOperationOutcome::Success));

    let mut quick_lookup_window = QuickLookupWindow { 
        tauri_app_handle: Box::new(mock_tauri_app_handle),
        initialization_script: None,
        current_layer: 0,
        files: Box::new(mock_files),
        quick_lookup_window_settings: 
            setup_quick_lookup_window_settings_example(),
        restart_on_change_file_path: 
            Some("other/file/path/bundle.js".to_string()),
        theme: Theme::Light,
    };
    assert_eq!(quick_lookup_window.hide().unwrap(),
        WindowOperationOutcome::Success);
}

#[test]
fn handles_hide_window_error() {
    let mut mock_tauri_app_handle = MockTauriAppHandleTrait::new();
    let mock_files = MockFilesTrait::new();

    mock_tauri_app_handle
        .expect_hide_window()
        .with(eq(WINDOW_LABEL))
        .returning(|_| Err(tauri::Error::InvalidWindowHandle));

    let mut quick_lookup_window = QuickLookupWindow { 
        tauri_app_handle: Box::new(mock_tauri_app_handle),
        initialization_script: None,
        current_layer: 0,
        files: Box::new(mock_files),
        quick_lookup_window_settings: 
            setup_quick_lookup_window_settings_example(),
        restart_on_change_file_path: 
            Some("other/file/path/bundle.js".to_string()),
        theme: Theme::Light,
    };
    assert_eq!(
        quick_lookup_window.hide()
        .unwrap_err()
        .to_string(),
        "Unexpected `raw_window_handle` for the current platform".to_string());
}
