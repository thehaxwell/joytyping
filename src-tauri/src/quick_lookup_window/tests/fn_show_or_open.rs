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
fn works_when_the_window_is_closed() {
    let mut mock_tauri_app_handle = MockTauriAppHandleTrait::new();
    let mock_files = MockFilesTrait::new();

    mock_tauri_app_handle
        .expect_show_window()
        .with(eq(WINDOW_LABEL))
        .returning(|_| Ok(WindowOperationOutcome::Success));

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
    assert!(quick_lookup_window.show_or_open(Switch::Button(gilrs::Button::East)).is_ok());
    assert_eq!(
       quick_lookup_window.current_state,
       QuickLookupWindowState::Showing(Switch::Button(gilrs::Button::East)));
}

#[test]
fn works_when_the_window_didnt_exist_build() {
    let mut mock_tauri_app_handle = MockTauriAppHandleTrait::new();
    let mock_files = MockFilesTrait::new();

    mock_tauri_app_handle
        .expect_show_window()
        .with(eq(WINDOW_LABEL))
        .returning(|_| Ok(WindowOperationOutcome::WindowNotFound));

    let initialization_script = "initialization_script placeholder".to_string();
    let quick_lookup_window_settings = models::QuickLookupWindow {
        inner_size: models::HeightAndWidth{
            height: 100.0,
            width: 100.0,
        },
        source_code: models::BrowserSourceCode{
            js_iife_bundle_file_path: "path/to/file/bundle.js".to_string(),
            css_file_path: None,
        }
    };

    mock_tauri_app_handle
        .expect_create_window()
        .with(eq(tauri_app_handle_wrapper::CreateWindowArgs{
                    label: WINDOW_LABEL.to_string(),
                    url: tauri::WindowUrl::App("quick-lookup.html".into()),
                    initialization_script: 
                        Some(format!(
                           "window.__START_LAYER__= 0;{}", 
                           initialization_script.clone())),
                    title: Some("Joytyping Quick Lookup".to_string()),
                    inner_size: Some(quick_lookup_window_settings.inner_size.clone()),
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
        quick_lookup_window_settings,
        restart_on_change_file_path: 
            Some("other/file/path/bundle.js".to_string()),
        theme: Theme::Light,
    };
    assert!(quick_lookup_window.show_or_open(Switch::Button(gilrs::Button::East)).is_ok());
    assert_eq!(
       quick_lookup_window.current_state,
       QuickLookupWindowState::Showing(Switch::Button(gilrs::Button::East)));


    // let mut quick_lookup_window 
    //     = setup_works_when_the_window_didnt_exist_build(Some(Theme::Light),"light");
    // assert!(quick_lookup_window.show_or_open(Switch::Button(gilrs::Button::East)).is_ok());
    // assert_eq!(
    //    quick_lookup_window.current_state,
    //    QuickLookupWindowState::Showing(Switch::Button(gilrs::Button::East)));
    //
    //
    // let mut quick_lookup_window 
    //     = setup_works_when_the_window_didnt_exist_build(None,"light");
    // assert!(quick_lookup_window.show_or_open(Switch::Button(gilrs::Button::East)).is_ok());
    // assert_eq!(
    //    quick_lookup_window.current_state,
    //    QuickLookupWindowState::Showing(Switch::Button(gilrs::Button::East)));
}

#[test]
fn handle_show_window_error() {
    let mut mock_tauri_app_handle = MockTauriAppHandleTrait::new();
    let mock_files = MockFilesTrait::new();

    mock_tauri_app_handle
        .expect_show_window()
        .with(eq(WINDOW_LABEL))
        .returning(|_| Err(tauri::Error::WebviewNotFound));


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
    assert_eq!(
        quick_lookup_window
            .show_or_open(Switch::Button(gilrs::Button::East))
            .unwrap_err()
            .to_string(),
            "webview not found: invalid label or it was closed".to_string());
    assert_eq!(
       quick_lookup_window.current_state,
       QuickLookupWindowState::Hidden);
}

#[test]
fn handle_create_window_error() {
    let mut mock_tauri_app_handle = MockTauriAppHandleTrait::new();
    let mock_files = MockFilesTrait::new();

    mock_tauri_app_handle
        .expect_show_window()
        .with(eq(WINDOW_LABEL))
        .returning(|_| Ok(WindowOperationOutcome::WindowNotFound));

    let initialization_script = "initialization_script placeholder".to_string();
    let quick_lookup_window_settings = models::QuickLookupWindow {
        inner_size: models::HeightAndWidth{
            height: 100.0,
            width: 100.0,
        },
        source_code: models::BrowserSourceCode{
            js_iife_bundle_file_path: "path/to/file/bundle.js".to_string(),
            css_file_path: None,
        }
    };

    mock_tauri_app_handle
        .expect_create_window()
        .with(eq(tauri_app_handle_wrapper::CreateWindowArgs{
                    label: WINDOW_LABEL.to_string(),
                    url: tauri::WindowUrl::App("quick-lookup.html".into()),
                    initialization_script: 
                        Some(format!(
                           "window.__START_LAYER__= 0;{}", 
                           initialization_script.clone())),
                    title: Some("Joytyping Quick Lookup".to_string()),
                    inner_size: Some(quick_lookup_window_settings.inner_size.clone()),
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
        quick_lookup_window_settings,
        restart_on_change_file_path: 
            Some("other/file/path/bundle.js".to_string()),
        theme: Theme::Light,
    };

    assert_eq!(
        quick_lookup_window
            .show_or_open(Switch::Button(gilrs::Button::East))
            .unwrap_err()
            .to_string(),
            "webview not found: invalid label or it was closed".to_string());
    assert_eq!(
       quick_lookup_window.current_state,
       QuickLookupWindowState::Hidden);
}
