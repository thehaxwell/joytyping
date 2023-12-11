use crate::{tauri_app_handle_wrapper::{self,TauriAppHandleTrait}, settings::models::HeightAndWidth};

#[cfg(test)]
mod tests;

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

        if self.tauri_app_handle.window_is_open(WINDOW_LABEL) {
            panic!("Don't try to open a window when one is already exists");
        }

        // escape `(tilda) becuase that's what is used in the 
        // initialization_script to delimit the error message string
        let err_string: &str = &str::replace(&error.to_string(),"`",r#"\`"#);

        self.tauri_app_handle.create_window(
            tauri_app_handle_wrapper::CreateWindowArgs {
            label: WINDOW_LABEL.to_string(), /* the unique window label */
            url: tauri::WindowUrl::App("/app/status".into()),
            title: Some("Failed to load Joytyping settings".to_string()),
            initialization_script: Some(
                format!("window.__ERROR_MESSAGE__ = `{}`;",err_string)),
            inner_size: Some(HeightAndWidth{height: 400.0, width: 600.0}),
            center: Some(()),
            decorations: None,
            always_on_top: None,
            skip_taskbar: None,
            focused: None,
        })?;

        self.tauri_app_handle.show_window(WINDOW_LABEL)?;

        Ok(())
    }

    pub fn close(&self) -> Result<(), tauri::Error> {
        self.tauri_app_handle.close_window(WINDOW_LABEL)?;
        Ok(())
    }
}

#[derive(Clone, serde::Serialize)]
struct ErrorDisplayWindowPayload {
    message: String,
}
