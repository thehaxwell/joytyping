
use crate::{settings::{self, data::QuickLookupWindowTheme}, gamepad::Switch, app_data_directory_manager::AppDataDirectoryManagerTrait};
use crate::tauri_app_handle_wrapper::{self,TauriAppHandleTrait};

#[cfg(test)]
use mockall::{automock, predicate::*};

use self::files::{FilesTrait, StartupScriptLoadError};

pub mod files;

#[cfg(test)]
mod tests;

const DEFAULT_QUICK_LOOKUP_INIT_SCRIPT: &str = include_str!("default_startup_script.js");

#[cfg_attr(test, automock)]
pub trait QuickLookupWindowTrait {
    fn show_or_open(&mut self, trigger_switch: Switch) -> Result<(), tauri::Error>;
    fn hide(&mut self, trigger_switch: Switch) -> Result<(), tauri::Error>;
    fn update(&mut self, layer: usize) -> Result<(), tauri::Error>;
}

const WINDOW_LABEL: &str = "quick-lookup";
pub struct QuickLookupWindow {
    tauri_app_handle: Box<dyn TauriAppHandleTrait>,
    current_state: QuickLookupWindowState,
    current_layer: usize,
    files: Box<dyn FilesTrait>,
    initialization_script: String,
    quick_lookup_window_settings: settings::data::QuickLookupWindow,
    restart_on_change_file_path: Option<String>,
}

impl QuickLookupWindow {
    pub fn new(
       tauri_app_handle: Box<dyn TauriAppHandleTrait>,
       dev_quick_lookup_window_settings: Option<settings::data::QuickLookupWindow>,
       prod_quick_lookup_window_settings: settings::data::QuickLookupWindow,
       files: Box<dyn FilesTrait>,
       ) -> Self {
        Self { 
            tauri_app_handle,
            current_state: QuickLookupWindowState::Hidden,
            initialization_script: DEFAULT_QUICK_LOOKUP_INIT_SCRIPT.to_string(),
            current_layer: 0,
            files,
            quick_lookup_window_settings: 
                dev_quick_lookup_window_settings.clone()
                .unwrap_or(prod_quick_lookup_window_settings.clone()),
            restart_on_change_file_path: 
                dev_quick_lookup_window_settings
                .and_then(|sets|Some(sets.source_code.js_iife_bundle_file_path)),
        }
    }

    /// Load startup script from the specified file.
    /// If reading or parsing the file fails, load the default startup script.
    pub fn load_startup_script(&mut self) -> Result<(), StartupScriptLoadError> {
        self.files.load_and_get_code(self.quick_lookup_window_settings.source_code.clone())
            .map_err(|err|{
                self.initialization_script = DEFAULT_QUICK_LOOKUP_INIT_SCRIPT.to_string();
                err
            })
            .and_then(|init_script|{
                self.initialization_script = init_script;
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

    pub fn watch<F : FnOnce(&std::path::Path) -> Result<(),notify::Error>> (&self,func: F) {
        if let Some(path) 
            = &self.restart_on_change_file_path {
            let _ = func(path.as_ref());
        }
    }
}

impl QuickLookupWindowTrait for QuickLookupWindow {
    fn show_or_open(&mut self, trigger_switch: Switch) -> Result<(), tauri::Error> {
        let window = self.tauri_app_handle.get_window(WINDOW_LABEL);

        if let Some(win) = window {
            win.show()
        } else {
            self.tauri_app_handle.create_window(
                tauri_app_handle_wrapper::CreateWindowArgs{
                    label: WINDOW_LABEL,
                    url: tauri::WindowUrl::App("quick-lookup.html".into()),
                    initialization_script: Some(format!(
                       "window.__START_LAYER__= {};document.documentElement.setAttribute('data-theme','{}');{}", 
                       self.current_layer,
                       match self.quick_lookup_window_settings.theme {
                           Some(QuickLookupWindowTheme::Light) => "light",
                           Some(QuickLookupWindowTheme::Dark) => "dark",
                           None => "light",
                       },
                       self.initialization_script).as_str()),
                    title: Some("Joytyping Quick Lookup"),
                    inner_size: Some(self.quick_lookup_window_settings.inner_size.clone()),
                    center: Some(()),
                    decorations: Some(false),
                    always_on_top: Some(true),
                    skip_taskbar: Some(true),
                    focused: Some(false)
                })?;

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


enum QuickLookupWindowState {
    Showing(Switch),
    Hidden
}
