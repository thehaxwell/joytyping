use mockall::predicate::eq;

use crate::{settings::{error_display_window::ErrorDisplayWindow, data}, tauri_app_handle_wrapper::{MockTauriAppHandleTrait, self}};

struct ExampleError {
    message: String,
}
impl ToString for ExampleError {
    fn to_string(&self) -> String {
        self.message.clone()
    }
}

const WINDOW_LABEL: &str = "settings-error";

#[test]
fn fn_open_and_show_works() {
    let mut mock_tauri_app_handle = MockTauriAppHandleTrait::new();

    mock_tauri_app_handle
        .expect_create_window()
        .with(eq(tauri_app_handle_wrapper::CreateWindowArgs{
                    label: WINDOW_LABEL.to_string(),
                    url: tauri::WindowUrl::App("#/settings-load-error".into()),
                    initialization_script: 
                        Some(r#"window.__ERROR_MESSAGE__ = `Something went \`wrong\``;"#.to_string()),
                    title: Some("Failed to load Joytyping settings".to_string()),
                    inner_size: Some(data::HeightAndWidth{height: 500.0, width: 500.0}),
                    center: Some(()),
                    decorations: None,
                    always_on_top: None,
                    skip_taskbar: None,
                    focused: None
                }))
        .returning(|_| Ok(()));

    let mut quick_lookup_window = ErrorDisplayWindow { 
        tauri_app_handle: Box::new(mock_tauri_app_handle),
    };

    assert!(
        quick_lookup_window
        .open_and_show(ExampleError{message: "Something went `wrong`".to_string()})
        .is_ok());
}

#[test]
fn fn_open_and_show_handles_create_window_error() {
    let mut mock_tauri_app_handle = MockTauriAppHandleTrait::new();

    mock_tauri_app_handle
        .expect_create_window()
        .with(eq(tauri_app_handle_wrapper::CreateWindowArgs{
                    label: WINDOW_LABEL.to_string(),
                    url: tauri::WindowUrl::App("#/settings-load-error".into()),
                    initialization_script: 
                        Some(r#"window.__ERROR_MESSAGE__ = `Something went \`wrong\``;"#.to_string()),
                    title: Some("Failed to load Joytyping settings".to_string()),
                    inner_size: Some(data::HeightAndWidth{height: 500.0, width: 500.0}),
                    center: Some(()),
                    decorations: None,
                    always_on_top: None,
                    skip_taskbar: None,
                    focused: None
                }))
        .returning(|_| Err(tauri::Error::WebviewNotFound));

    let mut quick_lookup_window = ErrorDisplayWindow { 
        tauri_app_handle: Box::new(mock_tauri_app_handle),
    };

    assert_eq!(
        quick_lookup_window
        .open_and_show(ExampleError{message: "Something went `wrong`".to_string()})
        .unwrap_err()
        .to_string(),
        "webview not found: invalid label or it was closed".to_string());
}
