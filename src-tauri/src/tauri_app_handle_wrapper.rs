use tauri::WindowUrl;
use tauri::Manager;

#[cfg(test)]
use mockall::{automock, predicate::*};

use crate::quick_lookup_window::UpdateKeyboardEventPayload;
use crate::models::HeightAndWidth;

#[cfg_attr(test, automock)]
pub trait TauriAppHandleTrait {
    fn show_window(&self, label: &str) -> Result<WindowOperationOutcome,tauri::Error>;
    fn hide_window(&self, label: &str) -> Result<WindowOperationOutcome,tauri::Error>;
    fn close_window(&self, label: &str) -> Result<WindowOperationOutcome,tauri::Error>;
    fn window_is_open(&self, label: &str) -> bool;
    fn emit_window_event(
        &self, label: &str, event: &str, payload: EmitWindowEventPayload) 
        -> Result<WindowOperationOutcome,tauri::Error>;
    fn create_window(&self,args: CreateWindowArgs) -> Result<(),tauri::Error>;
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

    fn operate_on_window<F>(&self, label: &str, func: F)
        -> Result<WindowOperationOutcome,tauri::Error> 
        where F: FnOnce(tauri::Window) -> tauri::Result<()> {
        if let Some(win) = self.tauri_app_handle.get_window(label) {
            func(win)
               .and_then(|()| Ok(WindowOperationOutcome::Success))
        }
        else {
            Ok(WindowOperationOutcome::WindowNotFound)
        }
    }
}

impl TauriAppHandleTrait for TauriAppHandleWrapper {
    fn show_window(&self, label: &str) -> Result<WindowOperationOutcome,tauri::Error> {
        self.operate_on_window(label, |win|win.show())
    }

    fn hide_window(&self, label: &str) -> Result<WindowOperationOutcome,tauri::Error> {
        self.operate_on_window(label, |win|win.hide())
    }

    fn close_window(&self, label: &str) -> Result<WindowOperationOutcome,tauri::Error> {
        self.operate_on_window(label, |win|win.close())
    }

    //TODO: change this to window_is_built
    fn window_is_open(&self, label: &str) -> bool {
        self.tauri_app_handle.get_window(label).is_some()
    }

    fn emit_window_event(
        &self, label: &str, event: &str, payload: EmitWindowEventPayload) 
        -> Result<WindowOperationOutcome,tauri::Error>{
        self.operate_on_window(label, 
            |win| match payload {
                EmitWindowEventPayload::UpdateKeyboardEventPayload(payload)
                    => win.emit(event,payload)
               })
    }

    fn create_window(&self,args: CreateWindowArgs) -> Result<(),tauri::Error>{
        let mut builder = tauri::WindowBuilder::new(
            &self.tauri_app_handle,
            args.label, /* the unique window label */
            args.url,
        );

        if let Some(script) = args.initialization_script {
            builder = builder.initialization_script(&script);
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
        builder.build()?;
        Ok(())
    }
}

#[derive(Debug,PartialEq)]
pub struct CreateWindowArgs {
    pub label: String,
    pub url: WindowUrl,
    pub initialization_script: Option<String>,
    pub title: Option<String>,
    pub inner_size: Option<HeightAndWidth>,
    pub center: Option<()>,
    pub decorations: Option<bool>,
    pub always_on_top: Option<bool>,
    pub skip_taskbar: Option<bool>,
    pub focused: Option<bool>,
}

#[derive(Debug,PartialEq)]
pub enum WindowOperationOutcome {
    Success,
    WindowNotFound,
}

#[derive(Debug,PartialEq)]
pub enum EmitWindowEventPayload {
    UpdateKeyboardEventPayload(UpdateKeyboardEventPayload),
}
