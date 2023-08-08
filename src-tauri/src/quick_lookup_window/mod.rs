// pub trait TauriAppHandleTrait {
//
//     fn open_window(&self, window_name: &str) -> Result<(), String>;
// }

use tauri::Manager;

use crate::joy_input::{Layer, Step};

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
            .title("Joytyping Quick Lookup")
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

    pub fn update_keyboard(&self, layer: Layer, step: Step) -> Result<(), tauri::Error> {
        match self.tauri_app_handle.get_window(WINDOW_LABEL) {
            Some(docs_window) => docs_window.emit("update-keyboard", Payload{
               layer: match layer {
                   Layer::First => 1,
                   Layer::Second => 2,
                   Layer::VisitingFirst(_) => 1,
                   Layer::VisitingSecond(_) => 2,
               },
               step: match step {
                   Step::Step1 => 1,
                   Step::Step2 => 2,
                   Step::Step3 => 3,
                   Step::Step4 => 4,
               },
            }),
            None => Ok(()),
        }
    }
}

#[derive(Clone, serde::Serialize)]
struct Payload {
  layer: u8,
  step: u8,
}
