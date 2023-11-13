use crate::{settings::data, tauri_app_handle_wrapper::MockTauriAppHandleTrait, quick_lookup_window::{files::MockFilesTrait, QuickLookupWindow, QuickLookupWindowState}};

fn setup_quick_lookup_window_settings_example() -> data::QuickLookupWindow {
    data::QuickLookupWindow{
        theme: Some(data::QuickLookupWindowTheme::Light),
        inner_size: data::HeightAndWidth{
            height: 100.0,
            width: 100.0,
        },
        source_code: data::BrowserSourceCode{
            js_iife_bundle_file_path: "path/to/file/bundle.js".to_string(),
            css_file_path: None,
        }
    }
}

#[test]
fn works_with_dev_quick_lookup_window_settings_not_set(){
    let mock_tauri_app_handle = MockTauriAppHandleTrait::new();
    let mock_files = MockFilesTrait::new();

    let quick_lookup_window = QuickLookupWindow::new(
        Box::new(mock_tauri_app_handle),
        None,
        setup_quick_lookup_window_settings_example(),
        Box::new(mock_files),
    );

    assert_eq!(quick_lookup_window.current_state, QuickLookupWindowState::Hidden);
    assert!(quick_lookup_window.initialization_script.is_none());
    assert_eq!(quick_lookup_window.current_layer, 0);
    assert_eq!(quick_lookup_window.quick_lookup_window_settings,setup_quick_lookup_window_settings_example());
    assert!(quick_lookup_window.restart_on_change_file_path.is_none());

}

#[test]
fn works_with_dev_quick_lookup_window_settings_set(){
    let mock_tauri_app_handle = MockTauriAppHandleTrait::new();
    let mock_files = MockFilesTrait::new();

    let quick_lookup_window = QuickLookupWindow::new(
        Box::new(mock_tauri_app_handle),
        Some(data::QuickLookupWindow{
            theme: Some(data::QuickLookupWindowTheme::Dark),
            inner_size: data::HeightAndWidth{
                height: 100.0,
                width: 100.0,
            },
            source_code: data::BrowserSourceCode{
                js_iife_bundle_file_path: "path/to/dev/file/bundle.js".to_string(),
                css_file_path: None,
            }
        }),
        setup_quick_lookup_window_settings_example(),
        Box::new(mock_files),
    );

    assert_eq!(quick_lookup_window.current_state, QuickLookupWindowState::Hidden);
    assert!(quick_lookup_window.initialization_script.is_none());
    assert_eq!(quick_lookup_window.current_layer, 0);
    assert_eq!(quick_lookup_window.quick_lookup_window_settings,data::QuickLookupWindow{
            theme: Some(data::QuickLookupWindowTheme::Dark),
            inner_size: data::HeightAndWidth{
                height: 100.0,
                width: 100.0,
            },
            source_code: data::BrowserSourceCode{
                js_iife_bundle_file_path: "path/to/dev/file/bundle.js".to_string(),
                css_file_path: None,
            }
        });
    assert_eq!(quick_lookup_window.restart_on_change_file_path.unwrap(), "path/to/dev/file/bundle.js".to_string());

}