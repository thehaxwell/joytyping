use std::{path::Path, io::ErrorKind};

use crate::{settings::data, quick_lookup_window::{files::MockFilesTrait, QuickLookupWindow, QuickLookupWindowState}, tauri_app_handle_wrapper::MockTauriAppHandleTrait};


fn setup_quick_lookup_window_settings_example(js_path:String) -> data::QuickLookupWindow {
    data::QuickLookupWindow{
        theme: Some(data::QuickLookupWindowTheme::Light),
        inner_size: data::HeightAndWidth{
            height: 100.0,
            width: 100.0,
        },
        source_code: data::BrowserSourceCode{
            js_iife_bundle_file_path: js_path,
            css_file_path: None,
        }
    }
}

#[test]
fn works() {
    let quick_lookup_window = QuickLookupWindow { 
        tauri_app_handle: Box::new(MockTauriAppHandleTrait::new()),
        current_state: QuickLookupWindowState::Hidden,
        initialization_script: None,
        current_layer: 0,
        files: Box::new(MockFilesTrait::new()),
        quick_lookup_window_settings: 
            setup_quick_lookup_window_settings_example(
                "path/to/dev/file/bundle.js".to_string()),
        restart_on_change_file_path: None,
    };

    assert_eq!(quick_lookup_window
        .conditionally_call_watcher(|path|{
            assert_eq!(path, Path::new("path/to/dev/file/bundle.js"));
            Result::<(),notify::Error>::Ok(())
        }).unwrap(),());


    let quick_lookup_window = QuickLookupWindow { 
        tauri_app_handle: Box::new(MockTauriAppHandleTrait::new()),
        current_state: QuickLookupWindowState::Hidden,
        initialization_script: None,
        current_layer: 0,
        files: Box::new(MockFilesTrait::new()),
        quick_lookup_window_settings: 
            setup_quick_lookup_window_settings_example(
                "other/file/path/bundle.js".to_string()),
        restart_on_change_file_path: Some("other/file/path/bundle.js".to_string()),
    };

    assert_eq!(quick_lookup_window
        .conditionally_call_watcher(|path|{
            assert_eq!(path, Path::new("other/file/path/bundle.js"));
            Result::<(),notify::Error>::Ok(())
        }).unwrap(),());
}

fn setup_error_message(msg: String) -> String {
    format!("{}\n{}\n{}\n{}\n{}\n{}",
        "Error in",
        "> development",
        "   > quick_lookup_window",
        "      > source_code",
        "         > js_iife_bundle_file_path",
        msg,)
}

#[test]
fn handles_errors_correctly() {
    let mock_tauri_app_handle = MockTauriAppHandleTrait::new();
    let mock_files = MockFilesTrait::new();

    let quick_lookup_window = QuickLookupWindow { 
        tauri_app_handle: Box::new(mock_tauri_app_handle),
        current_state: QuickLookupWindowState::Hidden,
        initialization_script: None,
        current_layer: 0,
        files: Box::new(mock_files),
        quick_lookup_window_settings:
            setup_quick_lookup_window_settings_example(
                "path/to/dev/file/bundle.js".to_string()),
        restart_on_change_file_path: Some("path/to/dev/file/bundle.js".to_string()),
    };

    let res = quick_lookup_window
        .conditionally_call_watcher(|path|{
           assert_eq!(path, Path::new("path/to/dev/file/bundle.js"));
           Result::<(),notify::Error>::Err(
               notify::Error::new(
                   notify::ErrorKind::Generic(
                       "this is the error message".to_string())))});
    assert_eq!(
        res.unwrap_err(),
        setup_error_message("this is the error message".to_string()));


    let res = quick_lookup_window
        .conditionally_call_watcher(|path|{
           assert_eq!(path, Path::new("path/to/dev/file/bundle.js"));
           Result::<(),notify::Error>::Err(
               notify::Error::new(
                   notify::ErrorKind::Generic(
                       "this is the error message".to_string())))});
    assert_eq!(
        res.unwrap_err(),
        setup_error_message("this is the error message".to_string()));


    let res = quick_lookup_window
        .conditionally_call_watcher(|path|{
           assert_eq!(path, Path::new("path/to/dev/file/bundle.js"));
           Result::<(),notify::Error>::Err(
               notify::Error::new(
                   notify::ErrorKind::PathNotFound))});
    assert_eq!(
        res.unwrap_err(),
        setup_error_message("No path was found.".to_string()));


    let res = quick_lookup_window
        .conditionally_call_watcher(|path|{
           assert_eq!(path, Path::new("path/to/dev/file/bundle.js"));
           Result::<(),notify::Error>::Err(
               notify::Error::new(
                   notify::ErrorKind::Io(
                       std::io::Error::from(ErrorKind::UnexpectedEof))))});
    assert_eq!(
        res.unwrap_err(),
        setup_error_message("unexpected end of file".to_string()));
}

