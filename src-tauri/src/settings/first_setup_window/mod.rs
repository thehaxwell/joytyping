use crate::{tauri_app_handle_wrapper::{self,TauriAppHandleTrait}, settings::models::HeightAndWidth};

#[cfg(test)]
mod tests;

const WINDOW_LABEL: &str = "settings-first-setup";
pub struct FirstSetupWindow {
    tauri_app_handle: Box<dyn TauriAppHandleTrait>,
}

impl FirstSetupWindow {
    pub fn new(
        tauri_app_handle: Box<dyn TauriAppHandleTrait>,
    ) -> Self {
        Self { tauri_app_handle,}
    }

    #[cfg(not(target_os="windows"))]
    fn get_current_os_name(&self) -> String {
        "Linux".to_string()
    }

    #[cfg(target_os="windows")]
    fn get_current_os_name(&self) -> String {
        "Windows".to_string()
    }

    pub fn open_and_show(&mut self) -> Result<(), tauri::Error> {

        if self.tauri_app_handle.window_is_open(WINDOW_LABEL) {
            panic!("Don't try to open a window when one is already exists");
        }

        self.tauri_app_handle.create_window(
            tauri_app_handle_wrapper::CreateWindowArgs {
            label: WINDOW_LABEL.to_string(), /* the unique window label */
            url: tauri::WindowUrl::App("/styled/setup".into()),
            title: Some("Setup Joytyping to get started".to_string()),
            initialization_script: Some(
                format!("window.__OS_NAME__ = `{}`;",self.get_current_os_name())),
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
