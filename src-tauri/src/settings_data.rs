use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
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

#[derive(Deserialize, Debug, Clone)]
pub struct KeyMapping {
	pub key: Option<EnigoKey>,
	pub modifiers: Option<Vec<EnigoKey>>,
	pub char_key: Option<char>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KeyboardModeKeyMappingsMappings {
    pub first_layer_step_1: KeyMapping,
    pub first_layer_step_2: KeyMapping,
    pub first_layer_step_3: KeyMapping,
    pub first_layer_step_4: KeyMapping,
    pub second_layer_step_1: KeyMapping,
    pub second_layer_step_2: KeyMapping,
    pub second_layer_step_3: KeyMapping,
    pub second_layer_step_4: KeyMapping,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KeyboardModeKeyMappings {
	pub south: KeyboardModeKeyMappingsMappings,
	pub east: KeyboardModeKeyMappingsMappings,
	pub north: KeyboardModeKeyMappingsMappings,
	pub west: KeyboardModeKeyMappingsMappings,
	pub d_pad_up: KeyboardModeKeyMappingsMappings,
	pub d_pad_down: KeyboardModeKeyMappingsMappings,
	pub d_pad_left: KeyboardModeKeyMappingsMappings,
	pub d_pad_right: KeyboardModeKeyMappingsMappings,
	pub left_stick_up: KeyboardModeKeyMappingsMappings,
	pub left_stick_down: KeyboardModeKeyMappingsMappings,
	pub left_stick_left: KeyboardModeKeyMappingsMappings,
	pub left_stick_right: KeyboardModeKeyMappingsMappings,
	pub right_stick_up: KeyboardModeKeyMappingsMappings,
	pub right_stick_down: KeyboardModeKeyMappingsMappings,
	pub right_stick_left: KeyboardModeKeyMappingsMappings,
	pub right_stick_right: KeyboardModeKeyMappingsMappings,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KeyboardModeConfig {
	pub key_mappings: KeyboardModeKeyMappings,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Profile {
	pub keyboard_mode: KeyboardModeConfig,
	pub name: String,
    pub quick_lookup_window: QuickLookupWindowSettings,
}

#[derive(Deserialize, Debug, Clone)]
pub struct QuickLookupWindowSettings {
    pub inner_width: f64,
    pub inner_height: f64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GlobalSettings {
    pub default_profile: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SettingsData {
	pub profiles: Vec<Profile>,
    pub global: GlobalSettings,
}
