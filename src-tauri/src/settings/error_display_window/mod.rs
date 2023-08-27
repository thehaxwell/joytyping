use tauri::Manager;

use super::SettingsLoadError;

const WINDOW_LABEL: &str = "settings-error";
pub struct ErrorDisplayWindow {
    tauri_app_handle: tauri::AppHandle,
}

impl ErrorDisplayWindow {
    pub fn new(tauri_app_handle: tauri::AppHandle,) -> Self {
        Self { tauri_app_handle,}
    }

    pub fn open_and_show(
        &mut self, error: SettingsLoadError) -> Result<(), tauri::Error> {
        if let Some(_win) = self.tauri_app_handle.get_window(WINDOW_LABEL) {
            panic!("make sure to close the window before opening it again");
        }

        let err_string: &str = &str::replace(&error.to_string(),"`",r#"\`"#);

        tauri::WindowBuilder::new(
            &self.tauri_app_handle,
            WINDOW_LABEL, /* the unique window label */
            tauri::WindowUrl::App("#/settings-load-error".into())
        )
        .title("Error Loading Settings | Joytyping")
        .center()
        .inner_size(500.0,500.0)
        .initialization_script(
            &format!("window.__ERROR_MESSAGE__ = `{}`;",
                     err_string)
        )
        .build()?;

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
