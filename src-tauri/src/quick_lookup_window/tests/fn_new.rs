use crate::{settings::models::{self, main_config::Theme}, tauri_app_handle_wrapper::MockTauriAppHandleTrait, quick_lookup_window::{files::MockFilesTrait, QuickLookupWindow}};

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
fn works_with_dev_quick_lookup_window_settings_not_set(){
    let mock_tauri_app_handle = MockTauriAppHandleTrait::new();
    let mock_files = MockFilesTrait::new();

    let quick_lookup_window = QuickLookupWindow::new(
        Box::new(mock_tauri_app_handle),
        None,
        setup_quick_lookup_window_settings_example(),
        Box::new(mock_files),
        Theme::Light
    );

    assert!(quick_lookup_window.initialization_script.is_none());
    assert_eq!(quick_lookup_window.current_layer, 0);
    assert_eq!(quick_lookup_window.quick_lookup_window_settings,setup_quick_lookup_window_settings_example());
    assert!(quick_lookup_window.restart_on_change_file_path.is_none());
    assert_eq!(quick_lookup_window.theme, Theme::Light);

}

#[test]
fn works_with_dev_quick_lookup_window_settings_set(){
    let mock_tauri_app_handle = MockTauriAppHandleTrait::new();
    let mock_files = MockFilesTrait::new();

    let quick_lookup_window = QuickLookupWindow::new(
        Box::new(mock_tauri_app_handle),
        Some(models::QuickLookupWindow{
            inner_size: models::HeightAndWidth{
                height: 100.0,
                width: 100.0,
            },
            source_code: models::BrowserSourceCode{
                js_iife_bundle_file_path: "path/to/dev/file/bundle.js".to_string(),
                css_file_path: None,
            }
        }),
        setup_quick_lookup_window_settings_example(),
        Box::new(mock_files),
        Theme::Dark
    );

    assert!(quick_lookup_window.initialization_script.is_none());
    assert_eq!(quick_lookup_window.current_layer, 0);
    assert_eq!(quick_lookup_window.quick_lookup_window_settings,models::QuickLookupWindow{
            inner_size: models::HeightAndWidth{
                height: 100.0,
                width: 100.0,
            },
            source_code: models::BrowserSourceCode{
                js_iife_bundle_file_path: "path/to/dev/file/bundle.js".to_string(),
                css_file_path: None,
            }
        });
    assert_eq!(quick_lookup_window.restart_on_change_file_path.clone().unwrap(), "path/to/dev/file/bundle.js".to_string());
    assert_eq!(quick_lookup_window.theme, Theme::Dark);

}
