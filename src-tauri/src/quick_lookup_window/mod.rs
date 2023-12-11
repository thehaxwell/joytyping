use crate::{gamepad::Switch, tauri_app_handle_wrapper::{WindowOperationOutcome, EmitWindowEventPayload}, settings::models::{self, main_config::Theme}};
use crate::tauri_app_handle_wrapper::{self,TauriAppHandleTrait};

#[cfg(test)]
use mockall::{automock, predicate::*};

use self::files::{FilesTrait, StartupScriptLoadError};

pub mod files;

#[cfg(test)]
mod tests;


#[cfg_attr(test, automock)]
pub trait QuickLookupWindowTrait {
    fn show(&mut self, trigger_switch: Switch) -> Result<WindowOperationOutcome, tauri::Error>;
    fn build(&mut self) -> Result<(), tauri::Error>;
    fn hide(&mut self, trigger_switch: Switch) -> Result<(), tauri::Error>;
    fn update(&mut self, layer: usize) -> Result<(), tauri::Error>;
}

const WINDOW_LABEL: &str = "quick-lookup";
pub struct QuickLookupWindow {
    tauri_app_handle: Box<dyn TauriAppHandleTrait>,
    current_state: QuickLookupWindowState,
    current_layer: usize,
    files: Box<dyn FilesTrait>,
    initialization_script: Option<String>,
    quick_lookup_window_settings: models::QuickLookupWindow,
    restart_on_change_file_path: Option<String>,
    theme: Theme,
}

impl QuickLookupWindow {
    pub fn new(
       tauri_app_handle: Box<dyn TauriAppHandleTrait>,
       dev_quick_lookup_window_settings: Option<models::QuickLookupWindow>,
       prod_quick_lookup_window_settings: models::QuickLookupWindow,
       files: Box<dyn FilesTrait>,
       theme: Theme,
       ) -> Self {
        Self { 
            tauri_app_handle,
            current_state: QuickLookupWindowState::Hidden,
            initialization_script: None,
            current_layer: 0,
            files,
            quick_lookup_window_settings: 
                dev_quick_lookup_window_settings.clone()
                .unwrap_or(prod_quick_lookup_window_settings.clone()),
            restart_on_change_file_path: 
                dev_quick_lookup_window_settings
                .and_then(|sets|Some(sets.source_code.js_iife_bundle_file_path)),
            theme,
        }
    }

    /// Load startup script from the specified file.
    /// If reading or parsing the file fails, load the default startup script.
    pub fn load_startup_script(&mut self) -> Result<(), StartupScriptLoadError> {
        let mut init_script 
            = String::from("window.addEventListener(\"load\", (event) => {");

        init_script.push_str(
            &format!("document.documentElement.setAttribute('data-theme','{}');",
               match self.theme {
                   Theme::Light => "light",
                   Theme::Dark => "dark",
               }));

        if let Some(css_str) = self.files
            .load_css(self.quick_lookup_window_settings.source_code.css_file_path.clone())? {
            init_script.push_str(&css_str) 
        };
            init_script.push_str(&self.files.load_js(
                self.quick_lookup_window_settings.source_code.js_iife_bundle_file_path.clone()
            )?); 
            init_script.push_str("});");

        self.initialization_script = Some(init_script);

        if let Ok(WindowOperationOutcome::Success) 
            = self.tauri_app_handle.close_window(WINDOW_LABEL) {
                self.current_state = QuickLookupWindowState::Hidden;
        }

        Ok(())
    }

    pub fn conditionally_call_watcher<F>(&self,func: F) -> Result<(),String> 
        where F: FnOnce(&std::path::Path) -> Result<(),notify::Error> {
        if let Some(path) 
            = &self.restart_on_change_file_path {
            func(path.as_ref())
                .map_err(|e|
                    format!("{}\n{}\n{}\n{}\n{}\n{}",
                    "Error in",
                    "> development",
                    "   > quick_lookup_window",
                    "      > source_code",
                    "         > js_iife_bundle_file_path",
                    e.to_string(),))
        }
        else {
            Ok(())
        }
    }
}

impl Drop for QuickLookupWindow {
    fn drop(&mut self) {
        let _ = self.tauri_app_handle.close_window(WINDOW_LABEL);
    }
}

impl QuickLookupWindowTrait for QuickLookupWindow {
    fn build(&mut self) -> Result<(), tauri::Error> {
        self.tauri_app_handle.create_window(
            tauri_app_handle_wrapper::CreateWindowArgs{
                label: WINDOW_LABEL.to_string(),
                url: tauri::WindowUrl::App("/quick-lookup".into()),
                initialization_script: 
                    if let Some(init_script) = &self.initialization_script {
                        Some(format!(
                           "window.__START_LAYER__= {};{}", 
                           // "{}window.__START_LAYER__= {};{}", 
                           // // remove all styles
                           // r#"addEventListener("DOMContentLoaded", (event) => {
                           //     document.querySelectorAll('[style]')
                           //        .forEach(el => el.removeAttribute('style'));
                           //      document.querySelectorAll('link[rel="stylesheet"], style')
                           //        .forEach(el => el.parentNode.removeChild(el));
                           //  });"#,
                           self.current_layer,
                           init_script))
                    } else {
                        None
                    },
                title: Some("Joytyping Quick Lookup".to_string()),
                inner_size: Some(self.quick_lookup_window_settings.inner_size.clone()),
                center: Some(()),
                decorations: Some(false),
                always_on_top: Some(true),
                skip_taskbar: Some(true),
                focused: Some(false)
            })?;
        Ok(())
    }

    fn show(&mut self, trigger_switch: Switch) -> Result<WindowOperationOutcome, tauri::Error> {
        let res = self.tauri_app_handle.show_window(WINDOW_LABEL)?;
        self.current_state = QuickLookupWindowState::Showing(trigger_switch);
        Ok(res)
    }

    fn hide(&mut self, trigger_switch: Switch) -> Result<(), tauri::Error> {
        // only allow closing a window with the same 
        // switch it was opened with
        if let QuickLookupWindowState::Showing(switch) = &self.current_state {
            if *switch != trigger_switch {
                return Ok(());
            }
        }

        if self.tauri_app_handle.hide_window(WINDOW_LABEL)?
            == WindowOperationOutcome::Success {
             self.current_state = QuickLookupWindowState::Hidden;
        }
        Ok(())
    }

    fn update(&mut self, layer: usize) -> Result<(), tauri::Error> {
        self.current_layer = layer;
        self.tauri_app_handle.emit_window_event(
            WINDOW_LABEL,
            "update-keyboard",
            EmitWindowEventPayload::UpdateKeyboardEventPayload(
                UpdateKeyboardEventPayload{
                layer,
            }))?;
        Ok(())
    }

}

#[derive(Clone, serde::Serialize, Debug, PartialEq)]
pub struct UpdateKeyboardEventPayload {
  layer: usize,
}

#[derive(Debug,PartialEq)]
enum QuickLookupWindowState {
    Showing(Switch),
    Hidden
}
