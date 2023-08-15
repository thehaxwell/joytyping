use crate::settings_data::{KeyMapping, EnigoKey, KeyboardModeKeyMappings, KeyboardModeKeyMappingsMappings};

pub struct JoyKeyboardKeyMapping {
	pub key: Option<enigo::Key>,
	pub modifiers: Option<Vec<enigo::Key>>,
}

impl JoyKeyboardKeyMapping {
    fn from(mapping: KeyMapping) -> Self {
        Self {
            key: JoyKeyboardKeyMapping::get_key(mapping.key, mapping.char_key),
            modifiers: JoyKeyboardKeyMapping::get_modifier_keys(mapping.modifiers),
        }
    }
    // If char_key is given, it takes precedence over key
    fn get_key(key_src: Option<EnigoKey>, char_key_src: Option<char>) -> Option<enigo::Key> {
        if let Some(key) = char_key_src {
            Some(enigo::Key::Layout(key))
        }
        else if let Some(key) = key_src {
            JoyKeyboardKeyMapping::to_enigo_key(key)
        }
        else {
            None
        }
    }

    fn get_modifier_keys(modifiers: Option<Vec<EnigoKey>>) -> Option<Vec<enigo::Key>> {
        if let Some(mods) = modifiers {
            let mut enigo_modifiers = Vec::new();
            for modifier in mods {
                if let Some(enigo_modifier) = JoyKeyboardKeyMapping::to_enigo_key(modifier.clone()) {
                    enigo_modifiers.push(enigo_modifier);
                }
            }
            Some(enigo_modifiers)
        }
        else {
            None
        }
    }

    fn to_enigo_key(key: EnigoKey) -> Option<enigo::Key> {
        match key {
            EnigoKey::Alt => Some(enigo::Key::Alt),
            EnigoKey::Backspace => Some(enigo::Key::Backspace),
            EnigoKey::Begin => Some(enigo::Key::Begin),
            EnigoKey::Break => Some(enigo::Key::Break),
            EnigoKey::Cancel => Some(enigo::Key::Cancel),
            EnigoKey::CapsLock => Some(enigo::Key::CapsLock),
            EnigoKey::Clear => Some(enigo::Key::Clear),
            EnigoKey::Control => Some(enigo::Key::Control),
            EnigoKey::Delete => Some(enigo::Key::Delete),
            EnigoKey::DownArrow => Some(enigo::Key::DownArrow),
            EnigoKey::End => Some(enigo::Key::End),
            EnigoKey::Escape => Some(enigo::Key::Escape),
            EnigoKey::Execute => Some(enigo::Key::Execute),
            EnigoKey::F1 => Some(enigo::Key::F1),
            EnigoKey::F2 => Some(enigo::Key::F2),
            EnigoKey::F3 => Some(enigo::Key::F3),
            EnigoKey::F4 => Some(enigo::Key::F4),
            EnigoKey::F5 => Some(enigo::Key::F5),
            EnigoKey::F6 => Some(enigo::Key::F6),
            EnigoKey::F7 => Some(enigo::Key::F7),
            EnigoKey::F8 => Some(enigo::Key::F8),
            EnigoKey::F9 => Some(enigo::Key::F9),
            EnigoKey::F10 => Some(enigo::Key::F10),
            EnigoKey::F11 => Some(enigo::Key::F11),
            EnigoKey::F12 => Some(enigo::Key::F12),
            EnigoKey::F13 => Some(enigo::Key::F13),
            EnigoKey::F14 => Some(enigo::Key::F14),
            EnigoKey::F15 => Some(enigo::Key::F15),
            EnigoKey::F16 => Some(enigo::Key::F16),
            EnigoKey::F17 => Some(enigo::Key::F17),
            EnigoKey::F18 => Some(enigo::Key::F18),
            EnigoKey::F19 => Some(enigo::Key::F19),
            EnigoKey::F20 => Some(enigo::Key::F20),
            EnigoKey::F21 => Some(enigo::Key::F21),
            EnigoKey::F22 => Some(enigo::Key::F22),
            EnigoKey::F23 => Some(enigo::Key::F23),
            EnigoKey::F24 => Some(enigo::Key::F24),
            EnigoKey::F25 => Some(enigo::Key::F25),
            EnigoKey::F26 => Some(enigo::Key::F26),
            EnigoKey::F27 => Some(enigo::Key::F27),
            EnigoKey::F28 => Some(enigo::Key::F28),
            EnigoKey::F29 => Some(enigo::Key::F29),
            EnigoKey::F30 => Some(enigo::Key::F30),
            EnigoKey::F31 => Some(enigo::Key::F31),
            EnigoKey::F32 => Some(enigo::Key::F32),
            EnigoKey::F33 => Some(enigo::Key::F33),
            EnigoKey::F34 => Some(enigo::Key::F34),
            EnigoKey::F35 => Some(enigo::Key::F35),
            EnigoKey::Find => Some(enigo::Key::Find),
            EnigoKey::Hangul => Some(enigo::Key::Hangul),
            EnigoKey::Hanja => Some(enigo::Key::Hanja),
            EnigoKey::Help => Some(enigo::Key::Help),
            EnigoKey::Home => Some(enigo::Key::Home),
            EnigoKey::Insert => Some(enigo::Key::Insert),
            EnigoKey::Kanji => Some(enigo::Key::Kanji),
            EnigoKey::LControl => Some(enigo::Key::LControl),
            EnigoKey::LeftArrow => Some(enigo::Key::LeftArrow),
            EnigoKey::Linefeed => Some(enigo::Key::Linefeed),
            EnigoKey::LMenu => Some(enigo::Key::LMenu),
            EnigoKey::LShift => Some(enigo::Key::LShift),
            EnigoKey::Meta => Some(enigo::Key::Meta),
            EnigoKey::ModeChange => Some(enigo::Key::ModeChange),
            EnigoKey::Numlock => Some(enigo::Key::Numlock),
            EnigoKey::Option => Some(enigo::Key::Option),
            EnigoKey::PageDown => Some(enigo::Key::PageDown),
            EnigoKey::PageUp => Some(enigo::Key::PageUp),
            EnigoKey::Pause => Some(enigo::Key::Pause),
            EnigoKey::Print => Some(enigo::Key::Print),
            EnigoKey::RControl => Some(enigo::Key::RControl),
            EnigoKey::Redo => Some(enigo::Key::Redo),
            EnigoKey::Return => Some(enigo::Key::Return),
            EnigoKey::RightArrow => Some(enigo::Key::RightArrow),
            EnigoKey::RShift => Some(enigo::Key::RShift),
            EnigoKey::ScrollLock => Some(enigo::Key::ScrollLock),
            EnigoKey::Select => Some(enigo::Key::Select),
            EnigoKey::ScriptSwitch => Some(enigo::Key::ScriptSwitch),
            EnigoKey::Shift => Some(enigo::Key::Shift),
            EnigoKey::ShiftLock => Some(enigo::Key::ShiftLock),
            EnigoKey::Space => Some(enigo::Key::Space),
            EnigoKey::SysReq => Some(enigo::Key::SysReq),
            EnigoKey::Tab => Some(enigo::Key::Tab),
            EnigoKey::UpArrow => Some(enigo::Key::UpArrow),
            EnigoKey::Undo => Some(enigo::Key::Undo),
        }
    }
}

pub struct JoyKeyboardKeyConfig {
    pub first_layer_step_1: JoyKeyboardKeyMapping,
    pub first_layer_step_2: JoyKeyboardKeyMapping,
    pub first_layer_step_3: JoyKeyboardKeyMapping,
    pub first_layer_step_4: JoyKeyboardKeyMapping,
    pub second_layer_step_1: JoyKeyboardKeyMapping,
    pub second_layer_step_2: JoyKeyboardKeyMapping,
    pub second_layer_step_3: JoyKeyboardKeyMapping,
    pub second_layer_step_4: JoyKeyboardKeyMapping,
}
impl JoyKeyboardKeyConfig {
    fn from(conf: KeyboardModeKeyMappingsMappings) -> Self {
        JoyKeyboardKeyConfig {
            first_layer_step_1: JoyKeyboardKeyMapping::from(conf.first_layer_step_1),
            first_layer_step_2: JoyKeyboardKeyMapping::from(conf.first_layer_step_2),
            first_layer_step_3: JoyKeyboardKeyMapping::from(conf.first_layer_step_3),
            first_layer_step_4: JoyKeyboardKeyMapping::from(conf.first_layer_step_4),
            second_layer_step_1: JoyKeyboardKeyMapping::from(conf.second_layer_step_1),
            second_layer_step_2: JoyKeyboardKeyMapping::from(conf.second_layer_step_2),
            second_layer_step_3: JoyKeyboardKeyMapping::from(conf.second_layer_step_3),
            second_layer_step_4: JoyKeyboardKeyMapping::from(conf.second_layer_step_4),
        }
    }
}

pub struct JoyKeyboardKeysConfig {
    pub south: JoyKeyboardKeyConfig,
    pub east: JoyKeyboardKeyConfig,
    pub north: JoyKeyboardKeyConfig,
    pub west: JoyKeyboardKeyConfig,
    pub d_pad_up: JoyKeyboardKeyConfig,
    pub d_pad_down: JoyKeyboardKeyConfig,
    pub d_pad_left: JoyKeyboardKeyConfig,
    pub d_pad_right: JoyKeyboardKeyConfig,
    pub left_stick_up: JoyKeyboardKeyConfig,
    pub left_stick_down: JoyKeyboardKeyConfig,
    pub left_stick_left: JoyKeyboardKeyConfig,
    pub left_stick_right: JoyKeyboardKeyConfig,
    pub right_stick_up: JoyKeyboardKeyConfig,
    pub right_stick_down: JoyKeyboardKeyConfig,
    pub right_stick_left: JoyKeyboardKeyConfig,
    pub right_stick_right: JoyKeyboardKeyConfig,
}
impl JoyKeyboardKeysConfig {
    pub fn from(mappings: KeyboardModeKeyMappings) -> Self {
        JoyKeyboardKeysConfig {
            south: JoyKeyboardKeyConfig::from(mappings.south),
            east: JoyKeyboardKeyConfig::from(mappings.east),
            north: JoyKeyboardKeyConfig::from(mappings.north),
            west: JoyKeyboardKeyConfig::from(mappings.west),
            d_pad_up: JoyKeyboardKeyConfig::from(mappings.d_pad_up),
            d_pad_down: JoyKeyboardKeyConfig::from(mappings.d_pad_down),
            d_pad_left: JoyKeyboardKeyConfig::from(mappings.d_pad_left),
            d_pad_right: JoyKeyboardKeyConfig::from(mappings.d_pad_right),
            left_stick_up: JoyKeyboardKeyConfig::from(mappings.left_stick_up),
            left_stick_down: JoyKeyboardKeyConfig::from(mappings.left_stick_down),
            left_stick_left: JoyKeyboardKeyConfig::from(mappings.left_stick_left),
            left_stick_right: JoyKeyboardKeyConfig::from(mappings.left_stick_right),
            right_stick_up: JoyKeyboardKeyConfig::from(mappings.right_stick_up),
            right_stick_down: JoyKeyboardKeyConfig::from(mappings.right_stick_down),
            right_stick_left: JoyKeyboardKeyConfig::from(mappings.right_stick_left),
            right_stick_right: JoyKeyboardKeyConfig::from(mappings.right_stick_right),
        }
    }

}
