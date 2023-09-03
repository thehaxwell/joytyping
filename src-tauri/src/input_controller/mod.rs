use std::time::Duration;
use crate::settings_data::{EnigoKey, EnigoMouseButton, SwitchOnClickReaction};

use self::enigo_wrapper::EnigoTrait;

pub mod enigo_wrapper;

#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg(test)]
mod tests;

const DELAY_DURATION: Duration = Duration::from_millis(500);

//TODO: split into two structs: KeyboardInputController and MouseInputController
#[cfg_attr(test, automock)]
pub trait InputControllerTrait {
    fn key_down(&mut self, key_to_click: InputShape);
    fn key_up(&mut self);
    fn trigger_input(&mut self);
    fn update_mouse_cursor_x_axis(&mut self, value: Option<i32>);
    fn update_mouse_cursor_y_axis(&mut self, value: Option<i32>);
}

pub struct InputController {
    enigo: Box<dyn EnigoTrait>,
    active_key: Option<InputShape>,
    instant_to_start_delay_from: Option<std::time::Instant>,
    mouse_cursor_x_move: Option<i32>,
    mouse_cursor_y_move: Option<i32>,
}

impl InputController {
    pub fn new(
        enigo: Box<dyn EnigoTrait>,
    ) -> Self {
        Self{
            enigo,
            active_key: None,
            instant_to_start_delay_from: None,
            mouse_cursor_x_move: None,
            mouse_cursor_y_move: None,
        }
    }

    fn allow_input(&self) -> bool {
        if let Some(instant) = self.instant_to_start_delay_from {
            if instant.elapsed() <= DELAY_DURATION {
                return false;
            }
        }
        true
    }

}

impl InputControllerTrait for InputController {
    // The key is immediately triggered and then after DELAY_DURATION
    // it will be allowed to be triggered many times by calling trigger_input()
    // until key_up() is called (or key_down() is called again).
    fn key_down(&mut self, key_to_click: InputShape) {
        self.active_key = Some(key_to_click);
        self.instant_to_start_delay_from = None;
        self.trigger_input();
        self.instant_to_start_delay_from = Some(std::time::Instant::now());
    }

    fn key_up(&mut self) {
        self.active_key = None;
        self.instant_to_start_delay_from = None;
    }

    fn trigger_input(&mut self) {
        if let Some(active_key) = &self.active_key {
            match active_key {
                InputShape::Key(key) => {
                    if !self.allow_input() {
                        return;
                    }
                    for modifier in key.modifiers.as_slice() {
                            println!("-> {:?}",modifier);
                            self.enigo.key_down(*modifier);
                    }

                    println!("-> {:?}",key.key);
                    self.enigo.key_click(key.key);

                    for modifier in key.modifiers.as_slice() {
                            println!("-> {:?}",modifier);
                            self.enigo.key_up(*modifier);
                    }
                    self.instant_to_start_delay_from = None;
                },
                InputShape::MouseKey(key) => self.enigo.mouse_down(key.mouse_button),
            }
        }

        if self.mouse_cursor_x_move.is_some()
            || self.mouse_cursor_y_move.is_some(){
            self.enigo.mouse_move_relative(
                self.mouse_cursor_x_move.unwrap_or(0),
                self.mouse_cursor_y_move.unwrap_or(0)
            );
        }
    }

    fn update_mouse_cursor_x_axis(&mut self, value: Option<i32>){
        self.mouse_cursor_x_move = value;
    }

    fn update_mouse_cursor_y_axis(&mut self, value: Option<i32>){
        self.mouse_cursor_y_move = value;
    }
}

#[derive(Debug, PartialEq)]
pub enum InputShape {
    Key(KeyInputShape),
    MouseKey(MouseKeyInputShape)
}

#[derive(Debug, PartialEq)]
pub struct KeyInputShape {
	pub key: enigo::Key,
	pub modifiers: Vec<enigo::Key>,
}

impl KeyInputShape {
    pub fn from_switch_event_reaction(config: SwitchOnClickReaction) -> Option<Self> {
        let key_option = KeyInputShape::get_enigo_key(config.char_key, config.key);
        if let Some(key) = key_option {
            if let Some(mods) = config.modifiers {
                return Some(Self {
                    key,
                    modifiers: KeyInputShape::get_enigo_modifiers(mods)
                });
            }
        }
        None
    }

    // the parameters are listed in order of priority
    pub fn get_enigo_key(
        char_key: Option<char>,
        key: Option<EnigoKey>) -> Option<enigo::Key> {
        if let Some(key) = char_key {
            Some(enigo::Key::Layout(key))
        }
        else if let Some(key) = key {
            Some(KeyInputShape::to_enigo_key(key))
        }
        else {
            None
        }
    }

    pub fn get_enigo_modifiers(mods: Vec<EnigoKey>) -> Vec<enigo::Key> {
            mods
                .iter()
                .map(|modifier|KeyInputShape::to_enigo_key(*modifier))
                .collect()
    }

    fn to_enigo_key(key: EnigoKey) -> enigo::Key {
        match key {
            EnigoKey::Alt => enigo::Key::Alt,
            EnigoKey::Backspace => enigo::Key::Backspace,
            EnigoKey::Begin => enigo::Key::Begin,
            EnigoKey::Break => enigo::Key::Break,
            EnigoKey::Cancel => enigo::Key::Cancel,
            EnigoKey::CapsLock => enigo::Key::CapsLock,
            EnigoKey::Clear => enigo::Key::Clear,
            EnigoKey::Control => enigo::Key::Control,
            EnigoKey::Delete => enigo::Key::Delete,
            EnigoKey::DownArrow => enigo::Key::DownArrow,
            EnigoKey::End => enigo::Key::End,
            EnigoKey::Escape => enigo::Key::Escape,
            EnigoKey::Execute => enigo::Key::Execute,
            EnigoKey::F1 => enigo::Key::F1,
            EnigoKey::F2 => enigo::Key::F2,
            EnigoKey::F3 => enigo::Key::F3,
            EnigoKey::F4 => enigo::Key::F4,
            EnigoKey::F5 => enigo::Key::F5,
            EnigoKey::F6 => enigo::Key::F6,
            EnigoKey::F7 => enigo::Key::F7,
            EnigoKey::F8 => enigo::Key::F8,
            EnigoKey::F9 => enigo::Key::F9,
            EnigoKey::F10 => enigo::Key::F10,
            EnigoKey::F11 => enigo::Key::F11,
            EnigoKey::F12 => enigo::Key::F12,
            EnigoKey::F13 => enigo::Key::F13,
            EnigoKey::F14 => enigo::Key::F14,
            EnigoKey::F15 => enigo::Key::F15,
            EnigoKey::F16 => enigo::Key::F16,
            EnigoKey::F17 => enigo::Key::F17,
            EnigoKey::F18 => enigo::Key::F18,
            EnigoKey::F19 => enigo::Key::F19,
            EnigoKey::F20 => enigo::Key::F20,
            EnigoKey::F21 => enigo::Key::F21,
            EnigoKey::F22 => enigo::Key::F22,
            EnigoKey::F23 => enigo::Key::F23,
            EnigoKey::F24 => enigo::Key::F24,
            EnigoKey::F25 => enigo::Key::F25,
            EnigoKey::F26 => enigo::Key::F26,
            EnigoKey::F27 => enigo::Key::F27,
            EnigoKey::F28 => enigo::Key::F28,
            EnigoKey::F29 => enigo::Key::F29,
            EnigoKey::F30 => enigo::Key::F30,
            EnigoKey::F31 => enigo::Key::F31,
            EnigoKey::F32 => enigo::Key::F32,
            EnigoKey::F33 => enigo::Key::F33,
            EnigoKey::F34 => enigo::Key::F34,
            EnigoKey::F35 => enigo::Key::F35,
            EnigoKey::Find => enigo::Key::Find,
            EnigoKey::Hangul => enigo::Key::Hangul,
            EnigoKey::Hanja => enigo::Key::Hanja,
            EnigoKey::Help => enigo::Key::Help,
            EnigoKey::Home => enigo::Key::Home,
            EnigoKey::Insert => enigo::Key::Insert,
            EnigoKey::Kanji => enigo::Key::Kanji,
            EnigoKey::LControl => enigo::Key::LControl,
            EnigoKey::LeftArrow => enigo::Key::LeftArrow,
            EnigoKey::Linefeed => enigo::Key::Linefeed,
            EnigoKey::LMenu => enigo::Key::LMenu,
            EnigoKey::LShift => enigo::Key::LShift,
            EnigoKey::Meta => enigo::Key::Meta,
            EnigoKey::ModeChange => enigo::Key::ModeChange,
            EnigoKey::Numlock => enigo::Key::Numlock,
            EnigoKey::Option => enigo::Key::Option,
            EnigoKey::PageDown => enigo::Key::PageDown,
            EnigoKey::PageUp => enigo::Key::PageUp,
            EnigoKey::Pause => enigo::Key::Pause,
            EnigoKey::Print => enigo::Key::Print,
            EnigoKey::RControl => enigo::Key::RControl,
            EnigoKey::Redo => enigo::Key::Redo,
            EnigoKey::Return => enigo::Key::Return,
            EnigoKey::RightArrow => enigo::Key::RightArrow,
            EnigoKey::RShift => enigo::Key::RShift,
            EnigoKey::ScrollLock => enigo::Key::ScrollLock,
            EnigoKey::Select => enigo::Key::Select,
            EnigoKey::ScriptSwitch => enigo::Key::ScriptSwitch,
            EnigoKey::Shift => enigo::Key::Shift,
            EnigoKey::ShiftLock => enigo::Key::ShiftLock,
            EnigoKey::Space => enigo::Key::Space,
            EnigoKey::SysReq => enigo::Key::SysReq,
            EnigoKey::Tab => enigo::Key::Tab,
            EnigoKey::UpArrow => enigo::Key::UpArrow,
            EnigoKey::Undo => enigo::Key::Undo,
        }
    }
}


#[derive(Debug, PartialEq)]
pub struct MouseKeyInputShape {
	pub mouse_button: enigo::MouseButton,
}

impl MouseKeyInputShape {
    pub fn from_switch_event_reaction(config: SwitchOnClickReaction) -> Option<Self> {
        if let Some(mouse_button) = config.mouse_button {
            Some(Self { mouse_button: MouseKeyInputShape::get_enigo_key(mouse_button)})
        }
        else {
            None
        }
    }

    // the parameters are listed in order of priority
    fn get_enigo_key(mouse_button: EnigoMouseButton) -> enigo::MouseButton {
        let translated_button = match mouse_button {
            EnigoMouseButton::Left => enigo::MouseButton::Left,
            EnigoMouseButton::Middle => enigo::MouseButton::Middle,
            EnigoMouseButton::Right => enigo::MouseButton::Right,
            EnigoMouseButton::Back => enigo::MouseButton::Back,
            EnigoMouseButton::Forward => enigo::MouseButton::Forward,
            EnigoMouseButton::ScrollUp => enigo::MouseButton::ScrollUp,
            EnigoMouseButton::ScrollDown => enigo::MouseButton::ScrollDown,
            EnigoMouseButton::ScrollLeft => enigo::MouseButton::ScrollLeft,
            EnigoMouseButton::ScrollRight => enigo::MouseButton::ScrollRight,
        };
        translated_button
    }

}
