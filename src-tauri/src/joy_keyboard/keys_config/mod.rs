use crate::settings_data;
use crate::settings_data::{EnigoKey, KeyboardModeKeyMappings, KeyboardModeSingleKeyMapping};

#[cfg(test)]
mod tests;

#[derive(Debug,PartialEq)]
pub struct KeyClickConfig {
	pub key: Option<enigo::Key>,
	pub modifiers: [Option<enigo::Key>;5],
}

impl KeyClickConfig {
    fn from(mapping: settings_data::KeyClickConfig) -> Self {
        Self {
            key: KeyClickConfig::get_key(mapping.key, mapping.char_key),
            // modifiers: KeyClickConfig::get_modifier_keys(mapping.modifiers),
            modifiers: [
                if mapping.modifier_1.is_some() {
                    Some(KeyClickConfig::to_enigo_key(mapping.modifier_1.unwrap().clone()))
                } else {None},
                if mapping.modifier_2.is_some() {
                    Some(KeyClickConfig::to_enigo_key(mapping.modifier_2.unwrap().clone()))
                } else {None},
                if mapping.modifier_3.is_some() {
                    Some(KeyClickConfig::to_enigo_key(mapping.modifier_3.unwrap().clone()))
                } else {None},
                if mapping.modifier_4.is_some() {
                    Some(KeyClickConfig::to_enigo_key(mapping.modifier_4.unwrap().clone()))
                } else {None},
                if mapping.modifier_5.is_some() {
                    Some(KeyClickConfig::to_enigo_key(mapping.modifier_5.unwrap().clone()))
                } else {None},
            ]
        }
    }
    // If char_key is given, it takes precedence over key
    fn get_key(key_src: Option<EnigoKey>, char_key_src: Option<char>) -> Option<enigo::Key> {
        if let Some(key) = char_key_src {
            Some(enigo::Key::Layout(key))
        }
        else if let Some(key) = key_src {
            Some(KeyClickConfig::to_enigo_key(key))
        }
        else {
            None
        }
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

#[derive(Debug,PartialEq)]
pub struct SingleKeyConfig {
    pub first_layer_step_1: KeyClickConfig,
    pub first_layer_step_2: KeyClickConfig,
    pub first_layer_step_3: KeyClickConfig,
    pub first_layer_step_4: KeyClickConfig,
    pub second_layer_step_1: KeyClickConfig,
    pub second_layer_step_2: KeyClickConfig,
    pub second_layer_step_3: KeyClickConfig,
    pub second_layer_step_4: KeyClickConfig,
}
impl SingleKeyConfig {
    fn from(conf: KeyboardModeSingleKeyMapping) -> Self {
        SingleKeyConfig {
            first_layer_step_1: KeyClickConfig::from(conf.first_layer_step_1),
            first_layer_step_2: KeyClickConfig::from(conf.first_layer_step_2),
            first_layer_step_3: KeyClickConfig::from(conf.first_layer_step_3),
            first_layer_step_4: KeyClickConfig::from(conf.first_layer_step_4),
            second_layer_step_1: KeyClickConfig::from(conf.second_layer_step_1),
            second_layer_step_2: KeyClickConfig::from(conf.second_layer_step_2),
            second_layer_step_3: KeyClickConfig::from(conf.second_layer_step_3),
            second_layer_step_4: KeyClickConfig::from(conf.second_layer_step_4),
        }
    }
}

#[derive(Debug,PartialEq)]
pub struct KeysConfig {
    pub south: SingleKeyConfig,
    pub east: SingleKeyConfig,
    pub north: SingleKeyConfig,
    pub west: SingleKeyConfig,
    pub d_pad_up: SingleKeyConfig,
    pub d_pad_down: SingleKeyConfig,
    pub d_pad_left: SingleKeyConfig,
    pub d_pad_right: SingleKeyConfig,
    pub left_stick_up: SingleKeyConfig,
    pub left_stick_down: SingleKeyConfig,
    pub left_stick_left: SingleKeyConfig,
    pub left_stick_right: SingleKeyConfig,
    pub right_stick_up: SingleKeyConfig,
    pub right_stick_down: SingleKeyConfig,
    pub right_stick_left: SingleKeyConfig,
    pub right_stick_right: SingleKeyConfig,
}
impl KeysConfig {
    pub fn from(mappings: KeyboardModeKeyMappings) -> Self {
        KeysConfig {
            south: SingleKeyConfig::from(mappings.south),
            east: SingleKeyConfig::from(mappings.east),
            north: SingleKeyConfig::from(mappings.north),
            west: SingleKeyConfig::from(mappings.west),
            d_pad_up: SingleKeyConfig::from(mappings.d_pad_up),
            d_pad_down: SingleKeyConfig::from(mappings.d_pad_down),
            d_pad_left: SingleKeyConfig::from(mappings.d_pad_left),
            d_pad_right: SingleKeyConfig::from(mappings.d_pad_right),
            left_stick_up: SingleKeyConfig::from(mappings.left_stick_up),
            left_stick_down: SingleKeyConfig::from(mappings.left_stick_down),
            left_stick_left: SingleKeyConfig::from(mappings.left_stick_left),
            left_stick_right: SingleKeyConfig::from(mappings.left_stick_right),
            right_stick_up: SingleKeyConfig::from(mappings.right_stick_up),
            right_stick_down: SingleKeyConfig::from(mappings.right_stick_down),
            right_stick_left: SingleKeyConfig::from(mappings.right_stick_left),
            right_stick_right: SingleKeyConfig::from(mappings.right_stick_right),
        }
    }

}
