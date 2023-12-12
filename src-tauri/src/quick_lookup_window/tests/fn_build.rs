use mockall::predicate::eq;

use crate::{settings::models::{self, main_config::Theme}, quick_lookup_window::{files::MockFilesTrait, QuickLookupWindow, QuickLookupWindowTrait, QuickLookupWindowState}, tauri_app_handle_wrapper::{MockTauriAppHandleTrait, WindowOperationOutcome, self}, gamepad::Switch};

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
fn works() {
    let mut mock_tauri_app_handle = MockTauriAppHandleTrait::new();
    let mock_files = MockFilesTrait::new();

    let initialization_script = "initialization script placeholder".to_string();

    mock_tauri_app_handle
        .expect_create_window()
        .with(eq(tauri_app_handle_wrapper::CreateWindowArgs{
                label: WINDOW_LABEL.to_string(),
                url: tauri::WindowUrl::App("/quick-lookup".into()),
                initialization_script: 
                        Some(format!(
                           "window.__START_LAYER__= 0;{}", 
                           initialization_script.clone())),
                title: Some("Joytyping Quick Lookup".to_string()),
                inner_size: Some(setup_quick_lookup_window_settings_example().inner_size.clone()),
                center: Some(()),
                decorations: Some(false),
                always_on_top: Some(true),
                skip_taskbar: Some(true),
                focused: Some(false)
            }))
        .returning(|_| Ok(()));

    let mut quick_lookup_window = QuickLookupWindow { 
        tauri_app_handle: Box::new(mock_tauri_app_handle),
        current_state: QuickLookupWindowState::Hidden,
        initialization_script: Some(initialization_script),
        current_layer: 0,
        files: Box::new(mock_files),
        quick_lookup_window_settings: 
            setup_quick_lookup_window_settings_example(),
        restart_on_change_file_path: 
            Some("other/file/path/bundle.js".to_string()),
        theme: Theme::Light,
    };
    assert!(quick_lookup_window.build().is_ok());
    assert_eq!(
       quick_lookup_window.current_state,
       QuickLookupWindowState::Hidden);
}

#[test]
fn handles_create_window_error_correctly() {
    let mut mock_tauri_app_handle = MockTauriAppHandleTrait::new();
    let mock_files = MockFilesTrait::new();

    let initialization_script = "initialization script placeholder".to_string();

    mock_tauri_app_handle
        .expect_create_window()
        .with(eq(tauri_app_handle_wrapper::CreateWindowArgs{
                label: WINDOW_LABEL.to_string(),
                url: tauri::WindowUrl::App("/quick-lookup".into()),
                initialization_script: 
                        Some(format!(
                           "window.__START_LAYER__= 0;{}", 
                           initialization_script.clone())),
                title: Some("Joytyping Quick Lookup".to_string()),
                inner_size: Some(setup_quick_lookup_window_settings_example().inner_size.clone()),
                center: Some(()),
                decorations: Some(false),
                always_on_top: Some(true),
                skip_taskbar: Some(true),
                focused: Some(false)
            }))
        .returning(|_| Err(tauri::Error::WebviewNotFound));

    let mut quick_lookup_window = QuickLookupWindow { 
        tauri_app_handle: Box::new(mock_tauri_app_handle),
        current_state: QuickLookupWindowState::Hidden,
        initialization_script: Some(initialization_script),
        current_layer: 0,
        files: Box::new(mock_files),
        quick_lookup_window_settings: 
            setup_quick_lookup_window_settings_example(),
        restart_on_change_file_path: 
            Some("other/file/path/bundle.js".to_string()),
        theme: Theme::Light,
    };
    assert_eq!(quick_lookup_window.build()
            .unwrap_err()
            .to_string(),
            "webview not found: invalid label or it was closed".to_string());
    assert_eq!(
       quick_lookup_window.current_state,
       QuickLookupWindowState::Hidden);
}
