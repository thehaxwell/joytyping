use serde::Deserialize;

#[derive(Deserialize, Debug, Copy, Clone, PartialEq)]
pub enum EnigoKey {
    Alt,
    Backspace,
    Begin,
    Break,
    Cancel,
    CapsLock,
    Clear,
    Control,
    Delete,
    DownArrow,
    End,
    Escape,
    Execute,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    F26,
    F27,
    F28,
    F29,
    F30,
    F31,
    F32,
    F33,
    F34,
    F35,
    Find,
    Hangul,
    Hanja,
    Help,
    Home,
    Insert,
    Kanji,
    LControl,
    LeftArrow,
    Linefeed,
    LMenu,
    LShift,
    Meta,
    ModeChange,
    Numlock,
    Option,
    PageDown,
    PageUp,
    Pause,
    Print,
    RControl,
    Redo,
    Return,
    RightArrow,
    RShift,
    ScrollLock,
    Select,
    ScriptSwitch,
    Shift,
    ShiftLock,
    Space,
    SysReq,
    Tab,
    Undo,
    UpArrow,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct KeyClickConfig {
	pub key: Option<EnigoKey>,
	pub modifier_1: Option<EnigoKey>,
	pub modifier_2: Option<EnigoKey>,
	pub modifier_3: Option<EnigoKey>,
	pub modifier_4: Option<EnigoKey>,
	pub modifier_5: Option<EnigoKey>,
	pub char_key: Option<char>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct KeyboardModeSingleKeyMapping {
    pub first_layer_step_1: KeyClickConfig,
    pub first_layer_step_2: KeyClickConfig,
    pub first_layer_step_3: KeyClickConfig,
    pub first_layer_step_4: KeyClickConfig,
    pub second_layer_step_1: KeyClickConfig,
    pub second_layer_step_2: KeyClickConfig,
    pub second_layer_step_3: KeyClickConfig,
    pub second_layer_step_4: KeyClickConfig,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct KeyboardModeKeyMappings {
	pub south: KeyboardModeSingleKeyMapping,
	pub east: KeyboardModeSingleKeyMapping,
	pub north: KeyboardModeSingleKeyMapping,
	pub west: KeyboardModeSingleKeyMapping,
	pub d_pad_up: KeyboardModeSingleKeyMapping,
	pub d_pad_down: KeyboardModeSingleKeyMapping,
	pub d_pad_left: KeyboardModeSingleKeyMapping,
	pub d_pad_right: KeyboardModeSingleKeyMapping,
	pub left_stick_up: KeyboardModeSingleKeyMapping,
	pub left_stick_down: KeyboardModeSingleKeyMapping,
	pub left_stick_left: KeyboardModeSingleKeyMapping,
	pub left_stick_right: KeyboardModeSingleKeyMapping,
	pub right_stick_up: KeyboardModeSingleKeyMapping,
	pub right_stick_down: KeyboardModeSingleKeyMapping,
	pub right_stick_left: KeyboardModeSingleKeyMapping,
	pub right_stick_right: KeyboardModeSingleKeyMapping,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct KeyboardModeConfig {
	pub key_mappings: KeyboardModeKeyMappings,
}

#[derive(Deserialize, Debug, Clone, PartialEq,Copy)]
pub struct AxisClickThresholds {
    pub up: f32,
    pub right: f32,
    pub down: f32,
    pub left: f32,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Axis {
    pub click_thresholds: AxisClickThresholds,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Profile {
	pub keyboard_mode: KeyboardModeConfig,
	pub name: String,
    pub quick_lookup_window: QuickLookupWindow,
    pub left_stick: Axis,
    pub right_stick: Axis,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct HeightAndWidth {
    pub width: f64,
    pub height: f64,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct QuickLookupWindow {
    pub inner_size: HeightAndWidth,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Global {
    pub default_profile: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct SettingsData {
	pub profiles: Vec<Profile>,
    pub global: Global,
}

// NEW

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct SettingsData1 {
	pub profiles: Vec<Profile1>,
    pub global: Global,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Profile1 {
	// pub keyboard_mode: KeyboardModeConfig,
	pub name: String,
    pub quick_lookup_window: QuickLookupWindow,
    // pub left_stick: Axis,
    // pub right_stick: Axis,
    pub layers: Vec<Layer>,
    pub stick_switches_click_thresholds: StickSwitchesClickThresholds,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct StickSwitchesClickThresholds {
    pub left_stick_up: f32,
    pub left_stick_down: f32,
    pub left_stick_left: f32,
    pub left_stick_right: f32,
    pub right_stick_up: f32,
    pub right_stick_down: f32,
    pub right_stick_left: f32,
    pub right_stick_right: f32,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Layer {
    pub switches: Option<Switches>,
    // pub levers
    pub cardinal_levers: Option<CardinalLevers>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct CardinalLevers {
    pub left_stick: CardinalLeverFunction,
    pub right_stick: CardinalLeverFunction,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct CardinalLeverFunction {
    pub control_mouse_cursor: Option<ControlMouseCursorFunction>,
    pub control_mouse_scrollwheel: Option<ControlMouseScrollwheelFunction>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ControlMouseCursorFunction {
    center_at: ControlMouseCursorCenterCoordinates,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ControlMouseCursorCenterCoordinates {
    pub x: f32,
    pub y: f32,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ControlMouseScrollwheelFunction {
    center_at_y: f32,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Switches {
	pub south: Option<SwitchEventAndReaction>,
	pub east: Option<SwitchEventAndReaction>,
	pub north: Option<SwitchEventAndReaction>,
	pub west: Option<SwitchEventAndReaction>,
	pub d_pad_up: Option<SwitchEventAndReaction>,
	pub d_pad_down: Option<SwitchEventAndReaction>,
	pub d_pad_left: Option<SwitchEventAndReaction>,
	pub d_pad_right: Option<SwitchEventAndReaction>,
	pub left_stick_up: Option<SwitchEventAndReaction>,
	pub left_stick_down: Option<SwitchEventAndReaction>,
	pub left_stick_left: Option<SwitchEventAndReaction>,
	pub left_stick_right: Option<SwitchEventAndReaction>,
	pub right_stick_up: Option<SwitchEventAndReaction>,
	pub right_stick_down: Option<SwitchEventAndReaction>,
	pub right_stick_left: Option<SwitchEventAndReaction>,
	pub right_stick_right: Option<SwitchEventAndReaction>,
	pub right_trigger: Option<SwitchEventAndReaction>,
	pub left_trigger: Option<SwitchEventAndReaction>,

}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct SwitchEventAndReaction {
    on_click: Option<SwitchOnClickReaction>,
    on_click_and_hold: Option<SwitchOnClickReaction>,
    on_double_click: Option<SwitchOnClickReaction>,
    on_double_click_and_hold: Option<SwitchOnClickReaction>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct SwitchOnClickReaction {
    // first 3 taken from KeyClickConfig
	pub key: Option<EnigoKey>,
	pub modifiers: Option<Vec<EnigoKey>>,
	pub char_key: Option<char>,

    pub visit_layer: Option<String>,
    pub move_to_layer: Option<String>,
}

impl SwitchOnClickReaction {
    // If char_key is given, it takes precedence over key
    pub fn get_enigo_key(&self) -> Option<enigo::Key> {
        if let Some(key) = self.char_key {
            Some(enigo::Key::Layout(key))
        }
        else if let Some(key) = self.key {
            Some(SwitchOnClickReaction::to_enigo_key(key))
        }
        else {
            None
        }
    }

    pub fn get_enigo_modifiers(&self) -> Option<Vec<enigo::Key>> {
        if let Some(mods) = &self.modifiers {
            let keys = mods
                .iter()
                .map(|modifier|SwitchOnClickReaction::to_enigo_key(*modifier))
                .collect();
            Some(keys)
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
