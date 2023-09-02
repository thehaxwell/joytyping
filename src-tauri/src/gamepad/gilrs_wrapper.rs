#[cfg(test)]
use mockall::{automock, predicate::*};

// TODO: change Gilrs to GilrsTrait
#[cfg_attr(test, automock)]
pub trait Gilrs {
    fn next_event(&mut self) -> Option<GilrsEvent>;
    fn get_gamepad_axis_data_value(&self, axis: gilrs::Axis) -> Option<f32>;
}

pub struct GilrsWrapper {
    gilrs: gilrs::Gilrs,
    current_gamepad_id: Option<gilrs::GamepadId>
}

impl GilrsWrapper {
    pub fn new() -> Self {
        Self{
            gilrs: gilrs::Gilrs::new().unwrap(),
            current_gamepad_id: None
        }
    }
}

impl Gilrs for GilrsWrapper {
    fn next_event(&mut self) -> Option<GilrsEvent> {
        // gilrs::Event
        if let Some(gilrs::Event { id, event, time}) = self.gilrs.next_event() {
            self.current_gamepad_id = Some(id);

            let event_wrapped = match event {
                gilrs::ev::EventType::ButtonPressed(button, _code)=>GilrsEventType::ButtonPressed(button),
                gilrs::ev::EventType::ButtonRepeated(button, _code) => GilrsEventType::ButtonRepeated(button),              
                gilrs::ev::EventType::ButtonReleased(button, _code) => GilrsEventType::ButtonReleased(button),              
                gilrs::ev::EventType::ButtonChanged(button, value, _code) => GilrsEventType::ButtonChanged(button,value),
                gilrs::ev::EventType::AxisChanged(axis, value, _code) => GilrsEventType::AxisChanged(axis, value),
                gilrs::ev::EventType::Connected => GilrsEventType::Connected,
                gilrs::ev::EventType::Disconnected => GilrsEventType::Disconnected,
                gilrs::ev::EventType::Dropped => GilrsEventType::Dropped,
            };

            Some(GilrsEvent {
                event: event_wrapped, time
            })
        }
        else {
            None
        }
    }

    fn get_gamepad_axis_data_value(&self, axis: gilrs::Axis) -> Option<f32>{
        if let Some(id) = self.current_gamepad_id {
            if let Some(x_data) = self.gilrs.gamepad(id).axis_data(axis) {
                return Some(x_data.value());
            }
        }
        None
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GilrsEvent {
    pub event: GilrsEventType,
    pub time: std::time::SystemTime
}

use gilrs::ev::{Button,Axis};


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GilrsEventType {
    ButtonPressed(Button),
    ButtonRepeated(Button),
    ButtonReleased(Button),
    ButtonChanged(Button, f32),
    AxisChanged(Axis, f32),
    Connected,
    Disconnected,
    Dropped,
}
