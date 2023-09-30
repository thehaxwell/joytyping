// pub trait TauriAppHandleTrait {
//
//     fn open_window(&self, window_name: &str) -> Result<(), String>;
// }

use tauri::Manager;

use crate::{settings, gamepad::Switch};

use thiserror::Error;

#[cfg(test)]
use mockall::{automock, predicate::*};

const DEFAULT_QUICK_LOOKUP_INIT_SCRIPT: &str = include_str!("default_startup_script.js");

#[cfg_attr(test, automock)]
pub trait QuickLookupWindowDependencies {
    fn read_to_string(&self, path: &str) -> Result<String, std::io::Error>;
}

pub struct QuickLookupWindowDependenciesImpl;
impl QuickLookupWindowDependencies for QuickLookupWindowDependenciesImpl {
    fn read_to_string(&self, path: &str) -> Result<String, std::io::Error> {
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
}

impl QuickLookupWindow {
    pub fn new(
       tauri_app_handle: tauri::AppHandle,
       dependencies: Box<dyn QuickLookupWindowDependencies>,
       quick_lookup_window_settings: settings::data::QuickLookupWindow,
       ) -> Self {
        Self { 
            tauri_app_handle,
            dependencies,
            initialization_script: DEFAULT_QUICK_LOOKUP_INIT_SCRIPT.to_string(),
            quick_lookup_window_settings,
            current_state: QuickLookupWindowState::Hidden,
            current_layer: 0,
        }
    }

    /// Load startup script from the specified file.
    /// If reading or parsing the file fails, load the default startup script.
    pub fn load_startup_script(&mut self) -> Result<(), StartupScriptLoadError> {
        if let Some(path) 
            = &self.quick_lookup_window_settings.js_bundle_file_path {
            match self.dependencies.read_to_string(path) {
                Err(e) => {
                    self.initialization_script = DEFAULT_QUICK_LOOKUP_INIT_SCRIPT.to_string();
                    match e.kind() {
                        std::io::ErrorKind::NotFound => {
                            Err(StartupScriptLoadError::FileNotFound)
                        },
                        std::io::ErrorKind::PermissionDenied => {
                            Err(StartupScriptLoadError::PermissionDenied)
                        },
                        _ => {
                            Err(StartupScriptLoadError::FileNotReadable)
                        },
                    }
                },
                Ok(rollup_script_str) => {
                    let mut init_script = String::from("window.addEventListener(\"load\", (event) => {");
                    init_script.push_str(&rollup_script_str);
                    init_script.push_str("});");
                    self.initialization_script = init_script;
                    Ok(())
                },
            }
        }
        else{
            self.initialization_script = DEFAULT_QUICK_LOOKUP_INIT_SCRIPT.to_string();
            Ok(())
        }
        .and_then(|()|{
            if let Some(win) 
                = self.tauri_app_handle.get_window(WINDOW_LABEL) {
                    if let Ok(()) = win.close() {
                        self.current_state = QuickLookupWindowState::Hidden;
                    }
                }
            Ok(())
        })
    }
}

impl QuickLookupWindowTrait for QuickLookupWindow {
    fn show_or_open(&mut self, trigger_switch: Switch) -> Result<(), tauri::Error> {
        let window = self.tauri_app_handle.get_window(WINDOW_LABEL);

        if let Some(win) = window {
            win.show()
        } else {
            let window = tauri::WindowBuilder::new(
                &self.tauri_app_handle,
                WINDOW_LABEL, /* the unique window label */
                tauri::WindowUrl::App("quick-lookup.html".into())
            )
            // start by injecting the current_layer
            // in-case it was set while the window didn't exist
            .initialization_script(format!("window.__START_LAYER__= {};{}", 
                                           self.current_layer,
                                           self.initialization_script).as_str())
            .title("Joytyping Quick Lookup");

            window
                .inner_size(
                    self.quick_lookup_window_settings.inner_size.width,
                    self.quick_lookup_window_settings.inner_size.height)
                .center()
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
    #[error("Startup script file not found")]
    FileNotFound,

    #[error("Startup script file not readable")]
    FileNotReadable,

    #[error("OS denied access to startup script file")]
    PermissionDenied
}

enum QuickLookupWindowState {
    Showing(Switch),
    Hidden
}
