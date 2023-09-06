use enigo::MouseButton;
use serde::Deserialize;
use std::fmt;
use serde::de::{self, Deserializer, Visitor, MapAccess};

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct SettingsData {
	pub profiles: Vec<Profile>,
    pub global: Global,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Global {
    pub default_profile: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Profile {
	pub name: String,
    pub quick_lookup_window: QuickLookupWindow,
    pub layers: Vec<Layer>,
    pub stick_switches_click_thresholds: StickSwitchesClickThresholds,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct QuickLookupWindow {
    pub inner_size: HeightAndWidth,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct HeightAndWidth {
    pub width: f64,
    pub height: f64,
}
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
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
    pub id: String,
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
    pub center_at: ControlMouseCursorCenterCoordinates,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ControlMouseCursorCenterCoordinates {
    pub x: f32,
    pub y: f32,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ControlMouseScrollwheelFunction {
    pub center_at_y: f32,
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
    pub on_click: Option<SwitchOnClickReaction>,
    pub on_click_and_hold: Option<SwitchOnClickReaction>,
    pub on_double_click: Option<SwitchOnClickReaction>,
    pub on_double_click_and_hold: Option<SwitchOnClickReaction>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct SwitchOnClickReaction {
    pub keyboard: Option<KeyboardInput>,

    pub mouse: Option<MouseInput>,

    pub visit_layer: Option<String>,
    pub move_to_layer: Option<String>,
}

#[derive(Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct MouseInput {
    #[serde(with = "EnigoMouseButtonDef")]
    pub button: enigo::MouseButton,
}

#[derive(Deserialize, Debug, Copy, Clone, PartialEq)]
#[serde(remote = "MouseButton")]
pub enum EnigoMouseButtonDef {
    Left,
    Middle,
    Right,
    Back,
    Forward,
    ScrollUp,
    ScrollDown,
    ScrollLeft,
    ScrollRight,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KeyboardInput {
    pub key: enigo::Key,
    pub modifiers: Vec<enigo::Key>,
}

#[derive(Debug)]
enum KeyboardInputError {
    UnknownVariant(String)
}

impl KeyboardInput {
    fn new(
        key_arg: String,
        modifiers_arg_option: Option<Vec<String>>,) -> Result<KeyboardInput, KeyboardInputError> {

        let key = 
            if key_arg.chars().count() == 1 {
                enigo::Key::Layout(key_arg.chars().next().unwrap())}
            else {
                KeyboardInput::to_enigo_key(key_arg.clone())?
            };

        let mut modifiers = Vec::new();
        if let Some(modifiers_arg) = modifiers_arg_option {
            for modifier in modifiers_arg {
                match KeyboardInput::to_enigo_key(modifier) {
                    Ok(key) => modifiers.push(key),
                    Err(err) => return Err(err)
                }
            }
        }
        Ok(Self{key, modifiers})
    }

    fn to_enigo_key(key: String) -> Result<enigo::Key, KeyboardInputError> {
        match key.as_str() {
            "Alt" => Ok(enigo::Key::Alt),
            "Backspace" => Ok(enigo::Key::Backspace),
            "Begin" => Ok(enigo::Key::Begin),
            "Break" => Ok(enigo::Key::Break),
            "Cancel" => Ok(enigo::Key::Cancel),
            "CapsLock" => Ok(enigo::Key::CapsLock),
            "Clear" => Ok(enigo::Key::Clear),
            "Control" => Ok(enigo::Key::Control),
            "Delete" => Ok(enigo::Key::Delete),
            "DownArrow" => Ok(enigo::Key::DownArrow),
            "End" => Ok(enigo::Key::End),
            "Escape" => Ok(enigo::Key::Escape),
            "Execute" => Ok(enigo::Key::Execute),
            "F1" => Ok(enigo::Key::F1),
            "F2" => Ok(enigo::Key::F2),
            "F3" => Ok(enigo::Key::F3),
            "F4" => Ok(enigo::Key::F4),
            "F5" => Ok(enigo::Key::F5),
            "F6" => Ok(enigo::Key::F6),
            "F7" => Ok(enigo::Key::F7),
            "F8" => Ok(enigo::Key::F8),
            "F9" => Ok(enigo::Key::F9),
            "F10" => Ok(enigo::Key::F10),
            "F11" => Ok(enigo::Key::F11),
            "F12" => Ok(enigo::Key::F12),
            "F13" => Ok(enigo::Key::F13),
            "F14" => Ok(enigo::Key::F14),
            "F15" => Ok(enigo::Key::F15),
            "F16" => Ok(enigo::Key::F16),
            "F17" => Ok(enigo::Key::F17),
            "F18" => Ok(enigo::Key::F18),
            "F19" => Ok(enigo::Key::F19),
            "F20" => Ok(enigo::Key::F20),
            "F21" => Ok(enigo::Key::F21),
            "F22" => Ok(enigo::Key::F22),
            "F23" => Ok(enigo::Key::F23),
            "F24" => Ok(enigo::Key::F24),
            "F25" => Ok(enigo::Key::F25),
            "F26" => Ok(enigo::Key::F26),
            "F27" => Ok(enigo::Key::F27),
            "F28" => Ok(enigo::Key::F28),
            "F29" => Ok(enigo::Key::F29),
            "F30" => Ok(enigo::Key::F30),
            "F31" => Ok(enigo::Key::F31),
            "F32" => Ok(enigo::Key::F32),
            "F33" => Ok(enigo::Key::F33),
            "F34" => Ok(enigo::Key::F34),
            "F35" => Ok(enigo::Key::F35),
            "Find" => Ok(enigo::Key::Find),
            "Hangul" => Ok(enigo::Key::Hangul),
            "Hanja" => Ok(enigo::Key::Hanja),
            "Help" => Ok(enigo::Key::Help),
            "Home" => Ok(enigo::Key::Home),
            "Insert" => Ok(enigo::Key::Insert),
            "Kanji" => Ok(enigo::Key::Kanji),
            "LControl" => Ok(enigo::Key::LControl),
            "LeftArrow" => Ok(enigo::Key::LeftArrow),
            "Linefeed" => Ok(enigo::Key::Linefeed),
            "LMenu" => Ok(enigo::Key::LMenu),
            "LShift" => Ok(enigo::Key::LShift),
            "Meta" => Ok(enigo::Key::Meta),
            "ModeChange" => Ok(enigo::Key::ModeChange),
            "Numlock" => Ok(enigo::Key::Numlock),
            "Option" => Ok(enigo::Key::Option),
            "PageDown" => Ok(enigo::Key::PageDown),
            "PageUp" => Ok(enigo::Key::PageUp),
            "Pause" => Ok(enigo::Key::Pause),
            "Print" => Ok(enigo::Key::Print),
            "RControl" => Ok(enigo::Key::RControl),
            "Redo" => Ok(enigo::Key::Redo),
            "Return" => Ok(enigo::Key::Return),
            "RightArrow" => Ok(enigo::Key::RightArrow),
            "RShift" => Ok(enigo::Key::RShift),
            "ScrollLock" => Ok(enigo::Key::ScrollLock),
            "Select" => Ok(enigo::Key::Select),
            "ScriptSwitch" => Ok(enigo::Key::ScriptSwitch),
            "Shift" => Ok(enigo::Key::Shift),
            "ShiftLock" => Ok(enigo::Key::ShiftLock),
            "Space" => Ok(enigo::Key::Space),
            "SysReq" => Ok(enigo::Key::SysReq),
            "Tab" => Ok(enigo::Key::Tab),
            "UpArrow" => Ok(enigo::Key::UpArrow),
            "Undo" => Ok(enigo::Key::Undo),
            _other => return Err(KeyboardInputError::UnknownVariant(key)),
        }
    }
}

impl<'de> Deserialize<'de> for KeyboardInput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field { Key, Modifiers }


        struct KeyboardInputVisitor;

        impl<'de> Visitor<'de> for KeyboardInputVisitor {
            type Value = KeyboardInput;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct KeyboardInput")
            }

            // NOTE: I didn't implemented visit_seq because I only plan on importing toml
            // fn visit_seq<V>(self, mut seq: V) -> Result<KeyboardInput, V::Error>

            fn visit_map<V>(self, mut map: V) -> Result<KeyboardInput, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut key = None;
                let mut modifiers = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        Field::Key => {
                            if key.is_some() {
                                return Err(de::Error::duplicate_field("key"));
                            }
                            key = Some(map.next_value()?);
                        }
                        Field::Modifiers => {
                            if modifiers.is_some() {
                                return Err(de::Error::duplicate_field("modifiers"));
                            }
                            modifiers = Some(map.next_value()?);
                        }
                    }
                }
                let key = key.ok_or_else(|| de::Error::missing_field("key"))?;
                // let modifiers = modifiers.ok_or_else(|| de::Error::missing_field("modifiers"))?;

                let result = KeyboardInput::new(key, modifiers).or_else(|err| match err {
                    KeyboardInputError::UnknownVariant(var)
                        => return Err(de::Error::unknown_variant(&var, 
                            // Hard coded some valid values
                            &["Meta","Alt", "Control", "..."])),
                })?;
                Ok(result)

            }
        }

        const FIELDS: &'static [&'static str] = &["key", "modifiers"];
        deserializer.deserialize_struct("KeyboardInput", FIELDS, KeyboardInputVisitor)
    }
}
