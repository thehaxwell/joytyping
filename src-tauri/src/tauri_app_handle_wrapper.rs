use tauri::{window::Window, WindowUrl, Wry};
use tauri::Manager;

#[cfg(test)]
use mockall::{automock, predicate::*};

use crate::settings::data::HeightAndWidth;

#[cfg_attr(test, automock)]
pub trait TauriAppHandleTrait {
    fn get_window(&self, label: &str) -> Option<Window<Wry>>;
    fn create_window<'a>(&self,args: CreateWindowArgs<'a>) -> tauri::Result<Window<Wry>>;
}

pub struct TauriAppHandleWrapper {
    tauri_app_handle: tauri::AppHandle,
}

impl TauriAppHandleWrapper {
    pub fn new(
       tauri_app_handle: tauri::AppHandle,
       ) -> Self {
        Self { 
            tauri_app_handle,
        }
    }
}

impl TauriAppHandleTrait for TauriAppHandleWrapper {
    fn get_window(&self, label: &str) -> Option<Window<Wry>> {
        self.tauri_app_handle.get_window(label) 
    }

    fn create_window(&self,args: CreateWindowArgs) -> tauri::Result<Window<Wry>>{
        let mut builder = tauri::WindowBuilder::new(
            &self.tauri_app_handle,
            args.label, /* the unique window label */
            args.url,
        );

        if let Some(script) = args.initialization_script {
            builder = builder.initialization_script(script);
        }
        if let Some(title) = args.title {
            builder = builder.title(title);
        }
        if let Some(HeightAndWidth{width,height}) = args.inner_size {
            builder = builder.inner_size(width,height);
        }
        if let Some(()) = args.center {
            builder = builder.center();
        }
        if let Some(decorations) = args.decorations {
            builder = builder.decorations(decorations);
        }
        if let Some(always_on_top) = args.always_on_top {
            builder = builder.always_on_top(always_on_top);
        }
        if let Some(skip_taskbar) = args.skip_taskbar {
            builder = builder.skip_taskbar(skip_taskbar);
        }
        if let Some(focused) = args.focused {
            builder = builder.focused(focused);
        }
        builder.build()
    }
}

pub struct CreateWindowArgs<'a> {
    pub label: &'a str,
    pub url: WindowUrl,
    pub initialization_script: Option<&'a str>,
    pub title: Option<&'a str>,
    pub inner_size: Option<HeightAndWidth>,
    pub center: Option<()>,
    pub decorations: Option<bool>,
    pub always_on_top: Option<bool>,
    pub skip_taskbar: Option<bool>,
    pub focused: Option<bool>,
}
