// pub trait TauriAppHandleTrait {
//
//     fn open_window(&self, window_name: &str) -> Result<(), String>;
// }

use tauri::Manager;

const WINDOW_LABEL: &str = "quick-lookup";
pub struct QuickLookupWindow {
    tauri_app_handle: tauri::AppHandle,
}

impl QuickLookupWindow {
    pub fn new(tauri_app_handle: tauri::AppHandle) -> Self {
        Self { tauri_app_handle,}
    }

    pub fn show_or_open(&mut self) -> Result<(), tauri::Error> {
        let window = self.tauri_app_handle.get_window(WINDOW_LABEL);

        if let Some(win) = window {
            win.show()
        } else {
            let _ = tauri::WindowBuilder::new(
                &self.tauri_app_handle,
                WINDOW_LABEL, /* the unique window label */
                tauri::WindowUrl::App("#/quick-lookup".into())
            )
            .build()?;
            Ok(())
        }
    }

    pub fn hide(&self) -> Result<(), tauri::Error> {
        match self.tauri_app_handle.get_window(WINDOW_LABEL) {
            Some(docs_window) => docs_window.hide(),
            None => Ok(())
        }
    }

}
