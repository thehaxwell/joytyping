use crate::{tauri_app_handle_wrapper::WindowOperationOutcome, gamepad::{Switch, QuickLookupWindowEvent}};

use super::QuickLookupWindowTrait;

#[cfg(test)]
mod tests;

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

    pub fn react_to_command(&mut self, command: QuickLookupWindowEvent) -> Result<(), tauri::Error> {
        match command {
            QuickLookupWindowEvent::ShowUntilSwitchKeyup(switch) => {
                self.quick_lookup_window.show()?;
                self.current_state = QuickLookupWindowState::Showing{
                    switch: switch.clone(), hide_on_keyup:true};
            },
            QuickLookupWindowEvent::EmitCurrentLayerNotification(
                new_layer_index) => self.quick_lookup_window.update(new_layer_index)?,
            QuickLookupWindowEvent::ToggleBySwitch(trigger_switch) 
                => match &self.current_state {
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
                },
        }
        Ok(())
    }

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

#[derive(Debug,PartialEq)]
enum QuickLookupWindowState {
    Showing{ switch: Switch, hide_on_keyup: bool },
    Hidden
}
