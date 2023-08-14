use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct KeyMapping {
	pub key: Option<String>,
	pub modifiers: Option<Vec<String>>,
	pub char_key: Option<String>,
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
}

#[derive(Deserialize, Debug, Clone)]
pub struct SettingsData {
	pub profiles: Vec<Profile>,
}
