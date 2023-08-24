use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq)]
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
