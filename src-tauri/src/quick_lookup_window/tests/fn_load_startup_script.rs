use mockall::predicate::eq;

use crate::{tauri_app_handle_wrapper::MockTauriAppHandleTrait, quick_lookup_window::{files::{MockFilesTrait, StartupScriptLoadError}, QuickLookupWindow}, settings::models::{self, main_config::Theme}};

fn setup_quick_lookup_window_settings_example(
    source_code: models::BrowserSourceCode) -> models::QuickLookupWindow {
    models::QuickLookupWindow{
        inner_size: models::HeightAndWidth{
            height: 100.0,
            width: 100.0,
        },
        source_code,
    }
}

#[test]
fn works_when_no_window_is_open() {
    let mock_tauri_app_handle = MockTauriAppHandleTrait::new();
    let mut mock_files = MockFilesTrait::new();

    mock_files
        .expect_load_css()
        .with(eq(None))
        .returning(|_| Ok(None));

    mock_files
        .expect_load_js()
        .with(eq("path/to/file/bundle.js".to_string()))
        .returning(|_| Ok("console.log('this is the js code');".to_string()));

    let mut quick_lookup_window = QuickLookupWindow { 
        tauri_app_handle: Box::new(mock_tauri_app_handle),
        initialization_script: None,
        current_layer: 0,
        files: Box::new(mock_files),
        quick_lookup_window_settings: setup_quick_lookup_window_settings_example(
            models::BrowserSourceCode{
                js_iife_bundle_file_path: "path/to/file/bundle.js".to_string(),
                css_file_path: None,
            }),
        restart_on_change_file_path: 
            Some("other/file/path/bundle.js".to_string()),
        theme: Theme::Dark,
    };

    assert!(quick_lookup_window.load_startup_script().is_ok());

    assert_eq!(quick_lookup_window.initialization_script.clone().unwrap(),
       format!("{}{}{}{}",
               "window.addEventListener(\"load\", (event) => {",
               "document.documentElement.setAttribute('data-theme','dark');",
               "console.log('this is the js code');",
               "});",
       ));

}

#[test]
fn works_when_a_window_is_open() {
    let mock_tauri_app_handle = MockTauriAppHandleTrait::new();
    let mut mock_files = MockFilesTrait::new();

    mock_files
        .expect_load_css()
        .with(eq(None))
        .returning(|_| Ok(None));

    mock_files
        .expect_load_js()
        .with(eq("path/to/file/bundle.js".to_string()))
        .returning(|_| Ok("console.log('this is the js code');".to_string()));

    let mut quick_lookup_window = QuickLookupWindow { 
        tauri_app_handle: Box::new(mock_tauri_app_handle),
        initialization_script: None,
        current_layer: 0,
        files: Box::new(mock_files),
        quick_lookup_window_settings: setup_quick_lookup_window_settings_example(
            models::BrowserSourceCode{
                js_iife_bundle_file_path: "path/to/file/bundle.js".to_string(),
                css_file_path: None,
            }),
        restart_on_change_file_path: 
            Some("other/file/path/bundle.js".to_string()),
        theme: Theme::Light,
    };

    assert!(quick_lookup_window.load_startup_script().is_ok());

    assert_eq!(quick_lookup_window.initialization_script.clone().unwrap(),
       format!("{}{}{}{}",
               "window.addEventListener(\"load\", (event) => {",
               "document.documentElement.setAttribute('data-theme','light');",
               "console.log('this is the js code');",
               "});",
       ));

}

#[test]
fn works_when_css_is_expected_and_no_window_is_open() {
    let mock_tauri_app_handle = MockTauriAppHandleTrait::new();
    let mut mock_files = MockFilesTrait::new();

    let styles_code = "\
                var styleSheet = document.createElement(\"style\");\n\
                styleSheet.innerText = `.container{color:red;}`;\n\
                document.head.appendChild(styleSheet);".to_string();
    mock_files
        .expect_load_css()
        .with(eq(Some("styles.css".to_string())))
        // below is supposed to be a copy of styles_code
        .returning(move |_| Ok(Some("\
                var styleSheet = document.createElement(\"style\");\n\
                styleSheet.innerText = `.container{color:red;}`;\n\
                document.head.appendChild(styleSheet);".to_string())));

    mock_files
        .expect_load_js()
        .with(eq("path/to/file/bundle.js".to_string()))
        .returning(|_| Ok("console.log('this is the js code');".to_string()));

    let mut quick_lookup_window = QuickLookupWindow { 
        tauri_app_handle: Box::new(mock_tauri_app_handle),
        initialization_script: None,
        current_layer: 0,
        files: Box::new(mock_files),
        quick_lookup_window_settings: setup_quick_lookup_window_settings_example(
            models::BrowserSourceCode{
                js_iife_bundle_file_path: "path/to/file/bundle.js".to_string(),
                css_file_path: Some("styles.css".to_string()),
            }),
        restart_on_change_file_path: 
            Some("other/file/path/bundle.js".to_string()),
        theme: Theme::Dark,
    };

    assert!(quick_lookup_window.load_startup_script().is_ok());

    assert_eq!(quick_lookup_window.initialization_script.clone().unwrap(),
       format!("{}{}{}{}{}",
               "window.addEventListener(\"load\", (event) => {",
               "document.documentElement.setAttribute('data-theme','dark');",
               styles_code,
               "console.log('this is the js code');",
               "});",));

}

#[test]
fn handles_load_css_error() {
    let mock_tauri_app_handle = MockTauriAppHandleTrait::new();
    let mut mock_files = MockFilesTrait::new();

    mock_files
        .expect_load_css()
        .with(eq(None))
        .returning(|_| Err(StartupScriptLoadError::FileNotFound("css_file_path".to_string())));

    let mut quick_lookup_window = QuickLookupWindow { 
        tauri_app_handle: Box::new(mock_tauri_app_handle),
        initialization_script: None,
        current_layer: 0,
        files: Box::new(mock_files),
        quick_lookup_window_settings: setup_quick_lookup_window_settings_example(
            models::BrowserSourceCode{
                js_iife_bundle_file_path: "path/to/file/bundle.js".to_string(),
                css_file_path: None,
            }),
        restart_on_change_file_path: 
            Some("other/file/path/bundle.js".to_string()),
        theme: Theme::Light,
    };

    assert!(quick_lookup_window.initialization_script.is_none());

    assert_eq!(quick_lookup_window.load_startup_script().unwrap_err(),
        StartupScriptLoadError::FileNotFound("css_file_path".to_string()));

}

#[test]
fn handles_load_js_error() {
    let mock_tauri_app_handle = MockTauriAppHandleTrait::new();
    let mut mock_files = MockFilesTrait::new();

    mock_files
        .expect_load_css()
        .with(eq(None))
        .returning(|_| Ok(None));

    mock_files
        .expect_load_js()
        .with(eq("path/to/file/bundle.js".to_string()))
        .returning(|_| Err(StartupScriptLoadError::FileNotFound("js_iife_bundle_file_path".to_string())));

    let mut quick_lookup_window = QuickLookupWindow { 
        tauri_app_handle: Box::new(mock_tauri_app_handle),
        initialization_script: None,
        current_layer: 0,
        files: Box::new(mock_files),
        quick_lookup_window_settings: setup_quick_lookup_window_settings_example(
            models::BrowserSourceCode{
                js_iife_bundle_file_path: "path/to/file/bundle.js".to_string(),
                css_file_path: None,
            }),
        restart_on_change_file_path: 
            Some("other/file/path/bundle.js".to_string()),
        theme: Theme::Dark,
    };

    assert!(quick_lookup_window.initialization_script.is_none());

    assert_eq!(quick_lookup_window.load_startup_script().unwrap_err(),
        StartupScriptLoadError::FileNotFound("js_iife_bundle_file_path".to_string()));

}
