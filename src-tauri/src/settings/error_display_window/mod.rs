use crate::tauri_app_handle_wrapper::{self,TauriAppHandleTrait};

use super::{SettingsLoadError, data::HeightAndWidth};

const WINDOW_LABEL: &str = "settings-error";
pub struct ErrorDisplayWindow {
    tauri_app_handle: Box<dyn TauriAppHandleTrait>,
}

impl ErrorDisplayWindow {
    pub fn new(
        tauri_app_handle: Box<dyn TauriAppHandleTrait>,
    ) -> Self {
        Self { tauri_app_handle,}
    }

    pub fn open_and_show<S: ToString>(
        &mut self, error: S) -> Result<(), tauri::Error> {
        if let Some(_win) = self.tauri_app_handle.get_window(WINDOW_LABEL) {
            panic!("make sure to close the window before opening it again");
        }

        let err_string: &str = &str::replace(&error.to_string(),"`",r#"\`"#);

        self.tauri_app_handle.create_window(
            tauri_app_handle_wrapper::CreateWindowArgs {
            label: WINDOW_LABEL, /* the unique window label */
            url: tauri::WindowUrl::App("#/settings-load-error".into()),
            title: Some("Error Loading Settings | Joytyping"),
            initialization_script: Some(
                &format!("window.__ERROR_MESSAGE__ = `{}`;",err_string)),
            inner_size: Some(HeightAndWidth{height: 500.0, width: 500.0}),
            center: Some(()),
            decorations: None,
            always_on_top: None,
            skip_taskbar: None,
            focused: None,
        })?;

        Ok(())
    }

    pub fn close(&self) -> Result<(), tauri::Error> {
        match self.tauri_app_handle.get_window(WINDOW_LABEL) {
            Some(docs_window) => docs_window.close(),
            None => Ok(())
        }
    }
}

#[derive(Clone, serde::Serialize)]
struct ErrorDisplayWindowPayload {
    message: String,
}
