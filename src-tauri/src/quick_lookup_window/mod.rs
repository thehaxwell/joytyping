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
    fn update(&self, layer: usize) -> Result<(), tauri::Error>;
}

const WINDOW_LABEL: &str = "quick-lookup";
pub struct QuickLookupWindow {
    tauri_app_handle: tauri::AppHandle,
    dependencies: Box<dyn QuickLookupWindowDependencies>,
    initialization_script: String,
    quick_lookup_window_settings: Option<settings::data::QuickLookupWindow>,
    current_state: QuickLookupWindowState,
}

impl QuickLookupWindow {
    pub fn new(tauri_app_handle: tauri::AppHandle,
               dependencies: Box<dyn QuickLookupWindowDependencies>) -> Self {
        Self { 
            tauri_app_handle,
            dependencies,
            initialization_script: DEFAULT_QUICK_LOOKUP_INIT_SCRIPT.to_string(),
            quick_lookup_window_settings: None,
            current_state: QuickLookupWindowState::Hidden,
        }
    }
    pub fn set_window_settings(&mut self, settings: settings::data::QuickLookupWindow) {
        self.quick_lookup_window_settings = Some(settings);
    }

    /// Load startup script from the specified file.
    /// If reading or parsing the file fails, load the default startup script.
    pub fn load_startup_script(&mut self) -> Result<(), StartupScriptLoadError> {
        match self.dependencies.read_to_string(
            "/home/haxwell/.config/joytyping/quick-lookup/build/bundle.js") {
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
            .initialization_script(&self.initialization_script)
            .title("Joytyping Quick Lookup");

            // TODO: consider using https://lib.rs/crates/tap
            // instead of this if-let
            if let Some(settings) = &self.quick_lookup_window_settings {
                window
                .inner_size(settings.inner_size.width, settings.inner_size.height)
                .center()
                .build()?;
            }
            else {
                window.build()?;
            }

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

    fn update(&self, layer: usize) -> Result<(), tauri::Error> {
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
