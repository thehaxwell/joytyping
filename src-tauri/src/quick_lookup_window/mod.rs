// pub trait TauriAppHandleTrait {
//
//     fn open_window(&self, window_name: &str) -> Result<(), String>;
// }

use tauri::Manager;

pub struct QuickLookupWindow {
    tauri_app_handle: tauri::AppHandle,
    window_is_open: bool,
}

impl QuickLookupWindow {
    pub fn new(tauri_app_handle: tauri::AppHandle) -> Self {
        Self { tauri_app_handle,
        window_is_open: false,}
    }

    pub fn is_open(&self) -> bool {
        self.window_is_open
    }

    pub fn open(&mut self) -> Result<tauri::Window, tauri::Error> {
      // TODO: DI the window builder and App
        let res = tauri::WindowBuilder::new(
            &self.tauri_app_handle,
            "external", /* the unique window label */
            tauri::WindowUrl::App("#/quick-lookup".into())
        )
        .build();
        if let Ok(_) = res {
            self.window_is_open = true;
        }
        res
    } 

    pub fn close(&mut self) -> Result<(), tauri::Error> {
        let docs_window = self.tauri_app_handle.get_window("external").unwrap();
        let res = docs_window.close();
        if let Ok(_) = res {
            self.window_is_open = false;
        }
        res
    }

}
