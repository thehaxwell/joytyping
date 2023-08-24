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
                char_key: Some('I'),
                key: None,
                modifiers: None,
            },
            first_layer_step_2: KeyClickConfig{
                char_key: Some('U'),
                key: None,
                modifiers: None,
            },
            first_layer_step_3: KeyClickConfig{
                char_key: Some('V'),
                key: None,
                modifiers: None,
            },
            first_layer_step_4: KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
            second_layer_step_1: KeyClickConfig{
                char_key: Some('&'),
                key: None,
                modifiers: None,
            },
            second_layer_step_2: KeyClickConfig{
                char_key: Some('}'),
                key: None,
                modifiers: None,
            },
            second_layer_step_3: KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
            second_layer_step_4: KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
        },
        east: KeyboardModeSingleKeyMapping {
            first_layer_step_1: KeyClickConfig {
                char_key: None,
                key: Some(EnigoKey::Return),
                modifiers: None,
            },
            first_layer_step_2: KeyClickConfig{
                char_key: Some('S'),
                key: None,
                modifiers: None,
            },
            first_layer_step_3: KeyClickConfig{
                char_key: Some('F'),
                key: None,
                modifiers: None,
            },
            first_layer_step_4: KeyClickConfig{
                char_key: Some('X'),
                key: None,
                modifiers: None,
            },
            second_layer_step_1: KeyClickConfig{
                char_key: Some('^'),
                key: None,
                modifiers: None,
            },
            second_layer_step_2: KeyClickConfig{
                char_key: Some(':'),
                key: None,
                modifiers: None,
            },
            second_layer_step_3: KeyClickConfig{
                char_key: Some('|'),
                key: None,
                modifiers: None,
            },
            second_layer_step_4: KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
        },
        north: KeyboardModeSingleKeyMapping {
            first_layer_step_1: KeyClickConfig {
                char_key: Some('E'),
                key: None,
                modifiers: None,
            },
            first_layer_step_2: KeyClickConfig{
                char_key: Some('H'),
                key: None,
                modifiers: None,
            },
            first_layer_step_3: KeyClickConfig{
                char_key: Some('G'),
                key: None,
                modifiers: None,
            },
            first_layer_step_4: KeyClickConfig{
                char_key: Some('Q'),
                key: None,
                modifiers: None,
            },
            second_layer_step_1: KeyClickConfig{
                char_key: Some('%'),
                key: None,
                modifiers: None,
            },
            second_layer_step_2: KeyClickConfig{
                char_key: Some('\"'),
                key: None,
                modifiers: None,
            },
            second_layer_step_3: KeyClickConfig{
                char_key: None,
                key: Some(EnigoKey::Tab),
                modifiers: Some(vec![EnigoKey::Shift]),
            },
            second_layer_step_4: KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
        },
        west: KeyboardModeSingleKeyMapping {
            first_layer_step_1: KeyClickConfig {
                char_key: Some('A'),
                key: None,
                modifiers: None,
            },
            first_layer_step_2: KeyClickConfig{
                char_key: Some('D'),
                key: None,
                modifiers: None,
            },
            first_layer_step_3: KeyClickConfig{
                char_key: Some('Y'),
                key: None,
                modifiers: None,
            },
            first_layer_step_4: KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
            second_layer_step_1: KeyClickConfig{
                char_key: Some('*'),
                key: None,
                modifiers: None,
            },
            second_layer_step_2: KeyClickConfig{
                char_key: Some('{'),
                key: None,
                modifiers: None,
            },
            second_layer_step_3: KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
            second_layer_step_4: KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
        },
        d_pad_up: KeyboardModeSingleKeyMapping {
            first_layer_step_1: KeyClickConfig {
                char_key: Some('T'),
                key: None,
                modifiers: None,
            },
            first_layer_step_2: KeyClickConfig{
                char_key: Some('L'),
                key: None,
                modifiers: None,
            },
            first_layer_step_3: KeyClickConfig{
                char_key: Some('W'),
                key: None,
                modifiers: None,
            },
            first_layer_step_4: KeyClickConfig{
                char_key: Some('Z'),
                key: None,
                modifiers: None,
            },
            second_layer_step_1: KeyClickConfig{
                char_key: Some('!'),
                key: None,
                modifiers: None,
            },
            second_layer_step_2: KeyClickConfig{
                char_key: Some('('),
                key: None,
                modifiers: None,
            },
            second_layer_step_3: KeyClickConfig{
                char_key: Some('_'),
                key: None,
                modifiers: None,
            },
            second_layer_step_4: KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
        },
        d_pad_down: KeyboardModeSingleKeyMapping {
            first_layer_step_1: KeyClickConfig {
                char_key: Some('N'),
                key: None,
                modifiers: None,
            },
            first_layer_step_2: KeyClickConfig{
                char_key: Some('M'),
                key: None,
                modifiers: None,
            },
            first_layer_step_3: KeyClickConfig{
                char_key: Some('K'),
                key: None,
                modifiers: None,
            },
            first_layer_step_4: KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
            second_layer_step_1: KeyClickConfig{
                char_key: Some('#'),
                key: None,
                modifiers: None,
            },
            second_layer_step_2: KeyClickConfig{
                char_key: Some('>'),
                key: None,
                modifiers: None,
            },
            second_layer_step_3: KeyClickConfig{
                char_key: Some('+'),
                key: None,
                modifiers: None,
            },
            second_layer_step_4: KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
        },
        d_pad_left: KeyboardModeSingleKeyMapping {
            first_layer_step_1: KeyClickConfig {
                char_key: None,
                key: None,
                modifiers: None,
            },
            first_layer_step_2: KeyClickConfig{
                char_key: Some('R'),
                key: None,
                modifiers: None,
            },
            first_layer_step_3: KeyClickConfig{
                char_key: Some('P'),
                key: None,
                modifiers: None,
            },
            first_layer_step_4: KeyClickConfig{
                char_key: Some('J'),
                key: None,
                modifiers: None,
            },
            second_layer_step_1: KeyClickConfig{
                char_key: Some('$'),
                key: None,
                modifiers: None,
            },
            second_layer_step_2: KeyClickConfig{
                char_key: Some('<'),
                key: None,
                modifiers: None,
            },
            second_layer_step_3: KeyClickConfig{
                char_key: Some('~'),
                key: None,
                modifiers: None,
            },
            second_layer_step_4: KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
        },
        d_pad_right: KeyboardModeSingleKeyMapping {
            first_layer_step_1: KeyClickConfig {
                char_key: Some('O'),
                key: None,
                modifiers: None,
            },
            first_layer_step_2: KeyClickConfig{
                char_key: Some('C'),
                key: None,
                modifiers: None,
            },
            first_layer_step_3: KeyClickConfig{
                char_key: Some('B'),
                key: None,
                modifiers: None,
            },
            first_layer_step_4: KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
            second_layer_step_1: KeyClickConfig{
                char_key: Some('@'),
                key: None,
                modifiers: None,
            },
            second_layer_step_2: KeyClickConfig{
                char_key: Some(')'),
                key: None,
                modifiers: None,
            },
            second_layer_step_3: KeyClickConfig{
                char_key: Some('?'),
                key: None,
                modifiers: None,
            },
            second_layer_step_4: KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
        },
        left_stick_up: KeyboardModeSingleKeyMapping {
            first_layer_step_1: KeyClickConfig {
                char_key: Some('t'),
                key: None,
                modifiers: None,
            },
            first_layer_step_2: KeyClickConfig{
                char_key: Some('l'),
                key: None,
                modifiers: None,
            },
            first_layer_step_3: KeyClickConfig{
                char_key: Some('w'),
                key: None,
                modifiers: None,
            },
            first_layer_step_4: KeyClickConfig{
                char_key: Some('z'),
                key: None,
                modifiers: None,
            },
            second_layer_step_1: KeyClickConfig{
                char_key: Some('1'),
                key: None,
                modifiers: None,
            },
            second_layer_step_2: KeyClickConfig{
                char_key: Some('9'),
                key: None,
                modifiers: None,
            },
            second_layer_step_3: KeyClickConfig{
                char_key: Some('_'),
                key: None,
                modifiers: None,
            },
            second_layer_step_4: KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
        },
        left_stick_down: KeyboardModeSingleKeyMapping {
            first_layer_step_1: KeyClickConfig {
                char_key: Some('n'),
                key: None,
                modifiers: None,
            },
            first_layer_step_2: KeyClickConfig{
                char_key: Some('m'),
                key: None,
                modifiers: None,
            },
            first_layer_step_3: KeyClickConfig{
                char_key: Some('k'),
                key: None,
                modifiers: None,
            },
            first_layer_step_4: KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
            second_layer_step_1: KeyClickConfig{
                char_key: Some('3'),
                key: None,
                modifiers: None,
            },
            second_layer_step_2: KeyClickConfig{
                char_key: Some('.'),
                key: None,
                modifiers: None,
            },
            second_layer_step_3: KeyClickConfig{
                char_key: Some('='),
                key: None,
                modifiers: None,
            },
            second_layer_step_4: KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
        },
        left_stick_left: KeyboardModeSingleKeyMapping {
            first_layer_step_1: KeyClickConfig {
                char_key: None,
                key: Some(EnigoKey::Backspace),
                modifiers: None,
            },
            first_layer_step_2: KeyClickConfig{
                char_key: Some('r'),
                key: None,
                modifiers: None,
            },
            first_layer_step_3: KeyClickConfig{
                char_key: Some('p'),
                key: None,
                modifiers: None,
            },
            first_layer_step_4: KeyClickConfig{
                char_key: Some('j'),
                key: None,
                modifiers: None,
            },
            second_layer_step_1: KeyClickConfig{
                char_key: Some('4'),
                key: None,
                modifiers: None,
            },
            second_layer_step_2: KeyClickConfig{
                char_key: Some(','),
                key: None,
                modifiers: None,
            },
            second_layer_step_3: KeyClickConfig{
                char_key: Some('`'),
                key: None,
                modifiers: None,
            },
            second_layer_step_4: KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
        },
        left_stick_right: KeyboardModeSingleKeyMapping {
            first_layer_step_1: KeyClickConfig {
                char_key: Some('o'),
                key: None,
                modifiers: None,
            },
            first_layer_step_2: KeyClickConfig{
                char_key: Some('c'),
                key: None,
                modifiers: None,
            },
            first_layer_step_3: KeyClickConfig{
                char_key: Some('b'),
                key: None,
                modifiers: None,
            },
            first_layer_step_4: KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
            second_layer_step_1: KeyClickConfig{
                char_key: Some('2'),
                key: None,
                modifiers: None,
            },
            second_layer_step_2: KeyClickConfig{
                char_key: Some('0'),
                key: None,
                modifiers: None,
            },
            second_layer_step_3: KeyClickConfig{
                char_key: Some('/'),
                key: None,
                modifiers: None,
            },
            second_layer_step_4: KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
        },
        right_stick_up: KeyboardModeSingleKeyMapping {
            first_layer_step_1: KeyClickConfig {
                char_key: Some('e'),
                key: None,
                modifiers: None,
            },
            first_layer_step_2: KeyClickConfig{
                char_key: Some('h'),
                key: None,
                modifiers: None,
            },
            first_layer_step_3: KeyClickConfig{
                char_key: Some('g'),
                key: None,
                modifiers: None,
            },
            first_layer_step_4: KeyClickConfig{
                char_key: Some('q'),
                key: None,
                modifiers: None,
            },
            second_layer_step_1: KeyClickConfig{
                char_key: Some('5'),
                key: None,
                modifiers: None,
            },
            second_layer_step_2: KeyClickConfig{
                char_key: Some('\''),
                key: None,
                modifiers: None,
            },
            second_layer_step_3: KeyClickConfig{
                char_key: None,
                key: Some(EnigoKey::Tab),
                modifiers: None,
            },
            second_layer_step_4: KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
        },
        right_stick_down: KeyboardModeSingleKeyMapping {
            first_layer_step_1: KeyClickConfig {
                char_key: Some('i'),
                key: None,
                modifiers: None,
            },
            first_layer_step_2: KeyClickConfig{
                char_key: Some('u'),
                key: None,
                modifiers: None,
            },
            first_layer_step_3: KeyClickConfig{
                char_key: Some('v'),
                key: None,
                modifiers: None,
            },
            first_layer_step_4: KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
            second_layer_step_1: KeyClickConfig{
                char_key: Some('7'),
                key: None,
                modifiers: None,
            },
            second_layer_step_2: KeyClickConfig{
                char_key: Some(']'),
                key: None,
                modifiers: None,
            },
            second_layer_step_3: KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
            second_layer_step_4: KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
        },
        right_stick_left: KeyboardModeSingleKeyMapping {
            first_layer_step_1: KeyClickConfig {
                char_key: Some('a'),
                key: None,
                modifiers: None,
            },
            first_layer_step_2: KeyClickConfig{
                char_key: Some('d'),
                key: None,
                modifiers: None,
            },
            first_layer_step_3: KeyClickConfig{
                char_key: Some('y'),
                key: None,
                modifiers: None,
            },
            first_layer_step_4: KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
            second_layer_step_1: KeyClickConfig{
                char_key: Some('8'),
                key: None,
                modifiers: None,
            },
            second_layer_step_2: KeyClickConfig{
                char_key: Some('['),
                key: None,
                modifiers: None,
            },
            second_layer_step_3: KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
            second_layer_step_4: KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
        },
        right_stick_right: KeyboardModeSingleKeyMapping {
            first_layer_step_1: KeyClickConfig {
                char_key: None,
                key: Some(EnigoKey::Space),
                modifiers: None,
            },
            first_layer_step_2: KeyClickConfig{
                char_key: Some('s'),
                key: None,
                modifiers: None,
            },
            first_layer_step_3: KeyClickConfig{
                char_key: Some('f'),
                key: None,
                modifiers: None,
            },
            first_layer_step_4: KeyClickConfig{
                char_key: Some('x'),
                key: None,
                modifiers: None,
            },
            second_layer_step_1: KeyClickConfig{
                char_key: Some('6'),
                key: None,
                modifiers: None,
            },
            second_layer_step_2: KeyClickConfig{
                char_key: Some(';'),
                key: None,
                modifiers: None,
            },
            second_layer_step_3: KeyClickConfig{
                char_key: Some('\\'),
                key: None,
                modifiers: None,
            },
            second_layer_step_4: KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
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
