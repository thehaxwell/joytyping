// pub trait TauriAppHandleTrait {
//
//     fn open_window(&self, window_name: &str) -> Result<(), String>;
// }

use std::path::PathBuf;

use tauri::Manager;

use crate::{settings::{self, data::QuickLookupWindowTheme}, gamepad::Switch, app_data_directory_manager::AppDataDirectoryManagerTrait};

use thiserror::Error;

#[cfg(test)]
use mockall::{automock, predicate::*};

const DEFAULT_QUICK_LOOKUP_INIT_SCRIPT: &str = include_str!("default_startup_script.js");

#[cfg_attr(test, automock)]
pub trait QuickLookupWindowDependencies {
    fn read_to_string(&self, path: PathBuf) -> Result<String, std::io::Error>;
}

pub struct QuickLookupWindowDependenciesImpl;
impl QuickLookupWindowDependencies for QuickLookupWindowDependenciesImpl {
    fn read_to_string(&self, path: PathBuf) -> Result<String, std::io::Error> {
        std::fs::read_to_string(path)
    }
}

#[cfg_attr(test, automock)]
pub trait QuickLookupWindowTrait {
    fn show_or_open(&mut self, trigger_switch: Switch) -> Result<(), tauri::Error>;
    fn hide(&mut self, trigger_switch: Switch) -> Result<(), tauri::Error>;
    fn update(&mut self, layer: usize) -> Result<(), tauri::Error>;
}

const WINDOW_LABEL: &str = "quick-lookup";
pub struct QuickLookupWindow {
    tauri_app_handle: tauri::AppHandle,
    dependencies: Box<dyn QuickLookupWindowDependencies>,
    initialization_script: String,
    quick_lookup_window_settings: settings::data::QuickLookupWindow,
    current_state: QuickLookupWindowState,
    current_layer: usize,
    app_data_directory_manager: Box<dyn AppDataDirectoryManagerTrait>,
}

impl QuickLookupWindow {
    pub fn new(
       tauri_app_handle: tauri::AppHandle,
       dependencies: Box<dyn QuickLookupWindowDependencies>,
       quick_lookup_window_settings: settings::data::QuickLookupWindow,
       app_data_directory_manager: Box<dyn AppDataDirectoryManagerTrait>,
       ) -> Self {
        Self { 
            tauri_app_handle,
            dependencies,
            initialization_script: DEFAULT_QUICK_LOOKUP_INIT_SCRIPT.to_string(),
            quick_lookup_window_settings,
            current_state: QuickLookupWindowState::Hidden,
            current_layer: 0,
            app_data_directory_manager,
        }
    }

    fn load_css(&mut self) -> Result<Option<String>, StartupScriptLoadError> {
        // if let Some(css_path) 
        //     = self.quick_lookup_window_settings.source_code.clone()
        //         .and_then(|src| src.css_file_path) {
        //     match self.dependencies.read_to_string(&css_path) {
        //         Err(e) => {
        //             self.initialization_script = DEFAULT_QUICK_LOOKUP_INIT_SCRIPT.to_string();
        //             return Err(get_file_load_error(e.kind(), "js_bundle_file_path".to_string()))
        //         },
        //         Ok(rollup_script_str) => {
        //             Ok(Some(format!("\
        //                 var styleSheet = document.createElement(\"style\");\n\
        //                 styleSheet.innerText = `{rollup_script_str}`;\n\
        //                 document.head.appendChild(styleSheet);")))
        //         }
        //     }
        // }
        // else {
        //     Ok(None)
        // }

        if let Some(css_path) 
            = self.quick_lookup_window_settings.source_code.clone()
                .and_then(|src| src.css_file_path)
                .and_then(|css_file_path|{
                    match self.app_data_directory_manager
                        .get_app_data_directory() {
                            Ok(mut path) => {
                                path.push(css_file_path);
                                Some(path)
                            }
                            Err(e) => None
                        }
                }) {
            match self.dependencies.read_to_string(css_path) {
                Err(e) => {
                    self.initialization_script = DEFAULT_QUICK_LOOKUP_INIT_SCRIPT.to_string();
                    return Err(get_file_load_error(e.kind(), "js_bundle_file_path".to_string()))
                },
                Ok(rollup_script_str) => {
                    Ok(Some(format!("\
                        var styleSheet = document.createElement(\"style\");\n\
                        styleSheet.innerText = `{rollup_script_str}`;\n\
                        document.head.appendChild(styleSheet);")))
                }
            }
        }
        else {
            Ok(None)
        }
    }

    fn load_js(&mut self) -> Result<Option<String>, StartupScriptLoadError> {
        // if let Some(source_code) 
        //     = &self.quick_lookup_window_settings.source_code {
        //     match self.dependencies.read_to_string(&source_code.js_iife_bundle_file_path) {
        //         Err(e) => {
        //             self.initialization_script = DEFAULT_QUICK_LOOKUP_INIT_SCRIPT.to_string();
        //             return Err(get_file_load_error(e.kind(), "js_bundle_file_path".to_string()))
        //         },
        //         Ok(rollup_script_str) => {
        //             Ok(Some(rollup_script_str))
        //         }
        //     }
        // }
        // else {
        //     Ok(None)
        // }

        if let Some(js_iife_bundle_file_path) 
            = self.quick_lookup_window_settings.source_code.clone()
                .and_then(|src|{
                    match self.app_data_directory_manager
                        .get_app_data_directory() {
                            Ok(mut path) => {
                                path.push(src.js_iife_bundle_file_path);
                                Some(path)
                            }
                            Err(e) => None
                        }
                }) {
            match self.dependencies.read_to_string(js_iife_bundle_file_path) {
                Err(e) => {
                    self.initialization_script = DEFAULT_QUICK_LOOKUP_INIT_SCRIPT.to_string();
                    return Err(get_file_load_error(e.kind(), "js_bundle_file_path".to_string()))
                },
                Ok(rollup_script_str) => {
                    Ok(Some(rollup_script_str))
                }
            }
        }
        else {
            Ok(None)
        }
    }

    /// Load startup script from the specified file.
    /// If reading or parsing the file fails, load the default startup script.
    pub fn load_startup_script(&mut self) -> Result<(), StartupScriptLoadError> {
        self.load_js()
            .and_then(|js_str| self.load_css()
                                   .map(|css_str| (js_str,css_str)))
            .map_err(|err|{
                self.initialization_script = DEFAULT_QUICK_LOOKUP_INIT_SCRIPT.to_string();
                err
            })
            .and_then(|(js_str_opt,css_str_opt)|{
                self.initialization_script = if let Some(js_str) = js_str_opt {
                    let mut init_script = String::from("window.addEventListener(\"load\", (event) => {");
                    if let Some(css_str) = css_str_opt { init_script.push_str(&css_str) };
                    init_script.push_str(&js_str); 
                    init_script.push_str("});");
                    init_script
                }
                else {
                    DEFAULT_QUICK_LOOKUP_INIT_SCRIPT.to_string()
                };
                Ok(())
            })
            .and_then(|()|{
                if let Some(win) 
                    = self.tauri_app_handle.get_window(WINDOW_LABEL) {
                        if let Ok(()) = win.close() {
                            self.current_state = QuickLookupWindowState::Hidden;
                        }
                    }
                Ok(())
            })?;

        Ok(())
    }
}

impl QuickLookupWindowTrait for QuickLookupWindow {
    fn show_or_open(&mut self, trigger_switch: Switch) -> Result<(), tauri::Error> {
        let window = self.tauri_app_handle.get_window(WINDOW_LABEL);

        if let Some(win) = window {
            win.show()
        } else {
            let _window = tauri::WindowBuilder::new(
                &self.tauri_app_handle,
                WINDOW_LABEL, /* the unique window label */
                tauri::WindowUrl::App("quick-lookup.html".into())
            )
            // start by injecting the current_layer
            // in-case it was set while the window didn't exist
            .initialization_script(format!("window.__START_LAYER__= {};document.documentElement.setAttribute('data-theme','{}');{}", 
                                           self.current_layer,
                                           match self.quick_lookup_window_settings.theme {
                                               Some(QuickLookupWindowTheme::Light) => "light",
                                               Some(QuickLookupWindowTheme::Dark) => "dark",
                                               None => "light",
                                           },
                                           self.initialization_script).as_str())
            .title("Joytyping Quick Lookup")
                .inner_size(
                    self.quick_lookup_window_settings.inner_size.width,
                    self.quick_lookup_window_settings.inner_size.height)
                .center()
                .decorations(false)
                .always_on_top(true)
                .skip_taskbar(true)
                .focused(false)
                .build()?;

            Ok(())
        }
        .and_then(|()|{
             self.current_state = QuickLookupWindowState::Showing(trigger_switch);
             Ok(())
        })
    }

    fn hide(&mut self, trigger_switch: Switch) -> Result<(), tauri::Error> {
        // only allow closing a window with the same 
        // switch it was opened with
        if let QuickLookupWindowState::Showing(switch) = &self.current_state {
            if *switch != trigger_switch {
                return Ok(());
            }
        }

        match self.tauri_app_handle.get_window(WINDOW_LABEL) {
            Some(docs_window) => {
                docs_window
                    .hide()
                    .and_then(|()|{
                         self.current_state = QuickLookupWindowState::Hidden;
                         Ok(())
                    })
            },
            None => Ok(())
        }
    }

    fn update(&mut self, layer: usize) -> Result<(), tauri::Error> {
        self.current_layer = layer;
        match self.tauri_app_handle.get_window(WINDOW_LABEL) {
            Some(docs_window) => docs_window.emit("update-keyboard",
                UpdateKeyboardEventPayload{
                    layer,
                }),
            None => Ok(()),
        }
    }

}

#[derive(Clone, serde::Serialize)]
struct UpdateKeyboardEventPayload {
  layer: usize,
}


#[derive(Error, Debug)]
pub enum StartupScriptLoadError {
    #[error("{0} file not found")]
    FileNotFound(String),

    #[error("{0} file not readable")]
    FileNotReadable(String),

    #[error("OS denied access to {0} file")]
    PermissionDenied(String),
}

enum QuickLookupWindowState {
    Showing(Switch),
    Hidden
}

fn get_file_load_error(
    err_kind: std::io::ErrorKind,file_name: String)-> StartupScriptLoadError {
    match err_kind {
        std::io::ErrorKind::NotFound => {
            StartupScriptLoadError::FileNotFound(file_name)
        },
        std::io::ErrorKind::PermissionDenied => {
            StartupScriptLoadError::PermissionDenied(file_name)
        },
        _ => {
            StartupScriptLoadError::FileNotReadable(file_name)
        },
    }
}
