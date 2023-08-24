use crate::settings_data::{SettingsData, Profile, Global, QuickLookupWindow, Axis, AxisClickThresholds, HeightAndWidth, KeyboardModeConfig, KeyboardModeKeyMappings, KeyClickConfig, KeyboardModeSingleKeyMapping, EnigoKey};

pub fn setup_settings_data_example() -> SettingsData {
    let ps4_controller_profile = Profile{
        name: "My PS3 Controller".to_string(),
        left_stick: Axis{
            click_thresholds: AxisClickThresholds {
                up: 0.5,
                down: 0.5,
                left: 0.5,
                right: 0.5,
            }
        },
        right_stick: Axis{
            click_thresholds: AxisClickThresholds {
                up: 0.5,
                down: 0.5,
                left: 0.5,
                right: 0.5,
            }
        },
        quick_lookup_window: QuickLookupWindow{
            inner_size: HeightAndWidth{
                height: 100.0,
                width: 100.0,
            },
        },
        keyboard_mode: KeyboardModeConfig {
            key_mappings: 
KeyboardModeKeyMappings {
        south: KeyboardModeSingleKeyMapping {
            first_layer_step_1: KeyClickConfig {
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('I'),
                key: None,
            },
            first_layer_step_2: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('U'),
                key: None,
            },
            first_layer_step_3: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('V'),
                key: None,
            },
            first_layer_step_4: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
            },
            second_layer_step_1: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('&'),
                key: None,
            },
            second_layer_step_2: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('}'),
                key: None,
            },
            second_layer_step_3: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
            },
            second_layer_step_4: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
            },
        },
        east: KeyboardModeSingleKeyMapping {
            first_layer_step_1: KeyClickConfig {
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: Some(EnigoKey::Return),
            },
            first_layer_step_2: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('S'),
                key: None,
            },
            first_layer_step_3: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('F'),
                key: None,
            },
            first_layer_step_4: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('X'),
                key: None,
            },
            second_layer_step_1: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('^'),
                key: None,
            },
            second_layer_step_2: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some(':'),
                key: None,
                
            },
            second_layer_step_3: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('|'),
                key: None,
                
            },
            second_layer_step_4: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
        },
        north: KeyboardModeSingleKeyMapping {
            first_layer_step_1: KeyClickConfig {
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('E'),
                key: None,
                
            },
            first_layer_step_2: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('H'),
                key: None,
                
            },
            first_layer_step_3: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('G'),
                key: None,
                
            },
            first_layer_step_4: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('Q'),
                key: None,
                
            },
            second_layer_step_1: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('%'),
                key: None,
                
            },
            second_layer_step_2: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('\"'),
                key: None,
                
            },
            second_layer_step_3: KeyClickConfig{
				modifier_1: Some(EnigoKey::Shift),
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: Some(EnigoKey::Tab),
            },
            second_layer_step_4: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
        },
        west: KeyboardModeSingleKeyMapping {
            first_layer_step_1: KeyClickConfig {
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('A'),
                key: None,
                
            },
            first_layer_step_2: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('D'),
                key: None,
                
            },
            first_layer_step_3: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('Y'),
                key: None,
                
            },
            first_layer_step_4: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
            second_layer_step_1: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('*'),
                key: None,
                
            },
            second_layer_step_2: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('{'),
                key: None,
                
            },
            second_layer_step_3: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
            second_layer_step_4: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
        },
        d_pad_up: KeyboardModeSingleKeyMapping {
            first_layer_step_1: KeyClickConfig {
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('T'),
                key: None,
                
            },
            first_layer_step_2: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('L'),
                key: None,
                
            },
            first_layer_step_3: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('W'),
                key: None,
                
            },
            first_layer_step_4: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('Z'),
                key: None,
                
            },
            second_layer_step_1: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('!'),
                key: None,
                
            },
            second_layer_step_2: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('('),
                key: None,
                
            },
            second_layer_step_3: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('_'),
                key: None,
                
            },
            second_layer_step_4: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
        },
        d_pad_down: KeyboardModeSingleKeyMapping {
            first_layer_step_1: KeyClickConfig {
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('N'),
                key: None,
                
            },
            first_layer_step_2: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('M'),
                key: None,
                
            },
            first_layer_step_3: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('K'),
                key: None,
                
            },
            first_layer_step_4: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
            second_layer_step_1: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('#'),
                key: None,
                
            },
            second_layer_step_2: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('>'),
                key: None,
                
            },
            second_layer_step_3: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('+'),
                key: None,
                
            },
            second_layer_step_4: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
        },
        d_pad_left: KeyboardModeSingleKeyMapping {
            first_layer_step_1: KeyClickConfig {
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
            first_layer_step_2: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('R'),
                key: None,
                
            },
            first_layer_step_3: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('P'),
                key: None,
                
            },
            first_layer_step_4: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('J'),
                key: None,
                
            },
            second_layer_step_1: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('$'),
                key: None,
                
            },
            second_layer_step_2: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('<'),
                key: None,
                
            },
            second_layer_step_3: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('~'),
                key: None,
                
            },
            second_layer_step_4: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
        },
        d_pad_right: KeyboardModeSingleKeyMapping {
            first_layer_step_1: KeyClickConfig {
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('O'),
                key: None,
                
            },
            first_layer_step_2: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('C'),
                key: None,
                
            },
            first_layer_step_3: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('B'),
                key: None,
                
            },
            first_layer_step_4: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
            second_layer_step_1: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('@'),
                key: None,
                
            },
            second_layer_step_2: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some(')'),
                key: None,
                
            },
            second_layer_step_3: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('?'),
                key: None,
                
            },
            second_layer_step_4: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
        },
        left_stick_up: KeyboardModeSingleKeyMapping {
            first_layer_step_1: KeyClickConfig {
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('t'),
                key: None,
                
            },
            first_layer_step_2: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('l'),
                key: None,
                
            },
            first_layer_step_3: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('w'),
                key: None,
                
            },
            first_layer_step_4: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('z'),
                key: None,
                
            },
            second_layer_step_1: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('1'),
                key: None,
                
            },
            second_layer_step_2: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('9'),
                key: None,
                
            },
            second_layer_step_3: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('_'),
                key: None,
                
            },
            second_layer_step_4: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
        },
        left_stick_down: KeyboardModeSingleKeyMapping {
            first_layer_step_1: KeyClickConfig {
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('n'),
                key: None,
                
            },
            first_layer_step_2: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('m'),
                key: None,
                
            },
            first_layer_step_3: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('k'),
                key: None,
                
            },
            first_layer_step_4: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
            second_layer_step_1: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('3'),
                key: None,
                
            },
            second_layer_step_2: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('.'),
                key: None,
                
            },
            second_layer_step_3: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('='),
                key: None,
                
            },
            second_layer_step_4: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
        },
        left_stick_left: KeyboardModeSingleKeyMapping {
            first_layer_step_1: KeyClickConfig {
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: Some(EnigoKey::Backspace),
                
            },
            first_layer_step_2: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('r'),
                key: None,
                
            },
            first_layer_step_3: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('p'),
                key: None,
                
            },
            first_layer_step_4: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('j'),
                key: None,
                
            },
            second_layer_step_1: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('4'),
                key: None,
                
            },
            second_layer_step_2: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some(','),
                key: None,
                
            },
            second_layer_step_3: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('`'),
                key: None,
                
            },
            second_layer_step_4: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
        },
        left_stick_right: KeyboardModeSingleKeyMapping {
            first_layer_step_1: KeyClickConfig {
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('o'),
                key: None,
                
            },
            first_layer_step_2: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('c'),
                key: None,
                
            },
            first_layer_step_3: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('b'),
                key: None,
                
            },
            first_layer_step_4: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
            second_layer_step_1: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('2'),
                key: None,
                
            },
            second_layer_step_2: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('0'),
                key: None,
                
            },
            second_layer_step_3: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('/'),
                key: None,
                
            },
            second_layer_step_4: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
        },
        right_stick_up: KeyboardModeSingleKeyMapping {
            first_layer_step_1: KeyClickConfig {
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('e'),
                key: None,
                
            },
            first_layer_step_2: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('h'),
                key: None,
                
            },
            first_layer_step_3: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('g'),
                key: None,
                
            },
            first_layer_step_4: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('q'),
                key: None,
                
            },
            second_layer_step_1: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('5'),
                key: None,
                
            },
            second_layer_step_2: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('\''),
                key: None,
                
            },
            second_layer_step_3: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: Some(EnigoKey::Tab),
                
            },
            second_layer_step_4: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
        },
        right_stick_down: KeyboardModeSingleKeyMapping {
            first_layer_step_1: KeyClickConfig {
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('i'),
                key: None,
                
            },
            first_layer_step_2: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('u'),
                key: None,
                
            },
            first_layer_step_3: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('v'),
                key: None,
                
            },
            first_layer_step_4: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
            second_layer_step_1: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('7'),
                key: None,
                
            },
            second_layer_step_2: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some(']'),
                key: None,
                
            },
            second_layer_step_3: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
            second_layer_step_4: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
        },
        right_stick_left: KeyboardModeSingleKeyMapping {
            first_layer_step_1: KeyClickConfig {
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('a'),
                key: None,
                
            },
            first_layer_step_2: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('d'),
                key: None,
                
            },
            first_layer_step_3: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('y'),
                key: None,
                
            },
            first_layer_step_4: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
            second_layer_step_1: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('8'),
                key: None,
                
            },
            second_layer_step_2: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('['),
                key: None,
                
            },
            second_layer_step_3: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
            second_layer_step_4: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
        },
        right_stick_right: KeyboardModeSingleKeyMapping {
            first_layer_step_1: KeyClickConfig {
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: Some(EnigoKey::Space),
                
            },
            first_layer_step_2: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('s'),
                key: None,
                
            },
            first_layer_step_3: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('f'),
                key: None,
                
            },
            first_layer_step_4: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('x'),
                key: None,
                
            },
            second_layer_step_1: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('6'),
                key: None,
                
            },
            second_layer_step_2: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some(';'),
                key: None,
                
            },
            second_layer_step_3: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('\\'),
                key: None,
                
            },
            second_layer_step_4: KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
        },
        }

        }

    };

    SettingsData {
        profiles: vec![ps4_controller_profile],
        global: Global{default_profile:"My PS3 Controller".to_string()} 
    }
}
