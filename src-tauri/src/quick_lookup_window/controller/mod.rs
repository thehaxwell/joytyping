use crate::{tauri_app_handle_wrapper::WindowOperationOutcome, gamepad_listener::Switch};

use super::QuickLookupWindowTrait;

#[cfg(test)]
mod tests;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait ControllerTrait {
    fn show_until_switch_keyup(&mut self, switch: Switch) -> Result<(), tauri::Error>;
    fn emit_current_layer_notification(&mut self, new_layer_index: usize) -> Result<(), tauri::Error>;
    fn toggle_by_switch(&mut self, trigger_switch: Switch) -> Result<(), tauri::Error>;
    fn react_to_keyup(&mut self, trigger_switch: Switch) -> Result<(), tauri::Error>;
}

pub struct Controller {
    quick_lookup_window: Box<dyn QuickLookupWindowTrait>,
    current_state: QuickLookupWindowState,
}
impl Controller {
    pub fn new(
        quick_lookup_window: Box<dyn QuickLookupWindowTrait>,
        )-> Self {
        Self {
            quick_lookup_window,
            current_state: QuickLookupWindowState::Hidden,
        }
    }

    //TODO: delete
    // pub fn react_to_command(&mut self, command: QuickLookupWindowEvent) -> Result<(), tauri::Error> {
    //     match command {
    //         QuickLookupWindowEvent::ShowUntilSwitchKeyup(switch) => {
    //             self.quick_lookup_window.show()?;
    //             self.current_state = QuickLookupWindowState::Showing{
    //                 switch: switch.clone(), hide_on_keyup:true};
    //         },
    //         QuickLookupWindowEvent::EmitCurrentLayerNotification(
    //             new_layer_index) => self.quick_lookup_window.update(new_layer_index)?,
    //         QuickLookupWindowEvent::ToggleBySwitch(trigger_switch) 
    //             => match &self.current_state {
    //                 QuickLookupWindowState::Showing{switch, hide_on_keyup: _} => {
    //                     if *switch == trigger_switch {
    //                         self.hide()?;
    //                     }
    //                 },
    //                 QuickLookupWindowState::Hidden => {
    //                     self.quick_lookup_window.show()?;
    //                     self.current_state = QuickLookupWindowState::Showing{
    //                         switch: trigger_switch, hide_on_keyup: false};
    //                 },
    //             },
    //     }
    //     Ok(())
    // }

    //TODO: delete
    pub fn react_to_switch_keyup(&mut self, trigger_switch: Switch) -> Result<(), tauri::Error> {
        if let QuickLookupWindowState::Showing{switch, hide_on_keyup} = &self.current_state {
            if *switch==trigger_switch && *hide_on_keyup {
                return self.hide();
            }
        }
        Ok(())
    }

    fn hide(&mut self) -> Result<(), tauri::Error> {
        if self.quick_lookup_window.hide()?
            == WindowOperationOutcome::Success {
             self.current_state = QuickLookupWindowState::Hidden;
        }
        Ok(())
    }
}

impl ControllerTrait for Controller {
    fn show_until_switch_keyup(&mut self, switch: Switch) -> Result<(), tauri::Error> {
        self.quick_lookup_window.show()?;
        self.current_state = QuickLookupWindowState::Showing{
            switch: switch.clone(), hide_on_keyup:true};
        Ok(())
    }

    fn emit_current_layer_notification(&mut self, new_layer_index: usize) -> Result<(), tauri::Error> {
        self.quick_lookup_window.update(new_layer_index)
    }

    fn toggle_by_switch(&mut self, trigger_switch: Switch) -> Result<(), tauri::Error> {
        match &self.current_state {
            QuickLookupWindowState::Showing{switch, hide_on_keyup: _} => {
                if *switch == trigger_switch {
                    self.hide()?;
                }
            },
            QuickLookupWindowState::Hidden => {
                self.quick_lookup_window.show()?;
                self.current_state = QuickLookupWindowState::Showing{
                    switch: trigger_switch, hide_on_keyup: false};
            },
        }
        Ok(())
    }

    fn react_to_keyup(&mut self, trigger_switch: Switch) -> Result<(), tauri::Error> {
        if let QuickLookupWindowState::Showing{switch, hide_on_keyup} = &self.current_state {
            if *switch==trigger_switch && *hide_on_keyup {
                return self.hide();
            }
        }
        Ok(())
    }
}

#[derive(Debug,PartialEq)]
enum QuickLookupWindowState {
    Showing{ switch: Switch, hide_on_keyup: bool },
    Hidden
}
