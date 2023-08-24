use crate::{joy_keyboard::keys_config::{KeysConfig, SingleKeyConfig, KeyClickConfig}, settings_data::{self, EnigoKey}};

pub fn joy_keyboard_keys_config() -> KeysConfig{
    KeysConfig {
        south: SingleKeyConfig {
            first_layer_step_1: KeyClickConfig {
                key: Some(enigo::Key::Layout('I')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('U')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('V')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_1: KeyClickConfig{
                key: Some(enigo::Key::Layout('&')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('}')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_3: KeyClickConfig{
                key: None,
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: [None,None,None,None,None],
            },
        },
        east: SingleKeyConfig {
            first_layer_step_1: KeyClickConfig {
                key: Some(enigo::Key::Return),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('S')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('F')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_4: KeyClickConfig{
                key: Some(enigo::Key::Layout('X')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_1: KeyClickConfig{
                key: Some(enigo::Key::Layout('^')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout(':')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('|')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: [None,None,None,None,None],
            },
        },
        north: SingleKeyConfig {
            first_layer_step_1: KeyClickConfig {
                key: Some(enigo::Key::Layout('E')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('H')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('G')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_4: KeyClickConfig{
                key: Some(enigo::Key::Layout('Q')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_1: KeyClickConfig{
                key: Some(enigo::Key::Layout('%')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('\"')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Tab),
                modifiers: [Some(enigo::Key::Shift),None,None,None,None],
            },
            second_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: [None,None,None,None,None],
            },
        },
        west: SingleKeyConfig {
            first_layer_step_1: KeyClickConfig {
                key: Some(enigo::Key::Layout('A')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('D')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('Y')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_1: KeyClickConfig{
                key: Some(enigo::Key::Layout('*')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('{')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_3: KeyClickConfig{
                key: None,
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: [None,None,None,None,None],
            },
        },
        d_pad_up: SingleKeyConfig {
            first_layer_step_1: KeyClickConfig {
                key: Some(enigo::Key::Layout('T')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('L')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('W')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_4: KeyClickConfig{
                key: Some(enigo::Key::Layout('Z')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_1: KeyClickConfig{
                key: Some(enigo::Key::Layout('!')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('(')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('_')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: [None,None,None,None,None],
            },
        },
        d_pad_down: SingleKeyConfig {
            first_layer_step_1: KeyClickConfig {
                key: Some(enigo::Key::Layout('N')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('M')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('K')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_1: KeyClickConfig{
                key: Some(enigo::Key::Layout('#')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('>')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('+')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: [None,None,None,None,None],
            },
        },
        d_pad_left: SingleKeyConfig {
            first_layer_step_1: KeyClickConfig {
                key: None,
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('R')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('P')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_4: KeyClickConfig{
                key: Some(enigo::Key::Layout('J')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_1: KeyClickConfig{
                key: Some(enigo::Key::Layout('$')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('<')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('~')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: [None,None,None,None,None],
            },
        },
        d_pad_right: SingleKeyConfig {
            first_layer_step_1: KeyClickConfig {
                key: Some(enigo::Key::Layout('O')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('C')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('B')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_1: KeyClickConfig{
                key: Some(enigo::Key::Layout('@')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout(')')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('?')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: [None,None,None,None,None],
            },
        },
        left_stick_up: SingleKeyConfig {
            first_layer_step_1: KeyClickConfig {
                key: Some(enigo::Key::Layout('t')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('l')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('w')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_4: KeyClickConfig{
                key: Some(enigo::Key::Layout('z')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_1: KeyClickConfig{
                key: Some(enigo::Key::Layout('1')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('9')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('_')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: [None,None,None,None,None],
            },
        },
        left_stick_down: SingleKeyConfig {
            first_layer_step_1: KeyClickConfig {
                key: Some(enigo::Key::Layout('n')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('m')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('k')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_1: KeyClickConfig{
                key: Some(enigo::Key::Layout('3')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('.')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('=')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: [None,None,None,None,None],
            },
        },
        left_stick_left: SingleKeyConfig {
            first_layer_step_1: KeyClickConfig {
                key: Some(enigo::Key::Backspace),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('r')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('p')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_4: KeyClickConfig{
                key: Some(enigo::Key::Layout('j')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_1: KeyClickConfig{
                key: Some(enigo::Key::Layout('4')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout(',')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('`')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: [None,None,None,None,None],
            },
        },
        left_stick_right: SingleKeyConfig {
            first_layer_step_1: KeyClickConfig {
                key: Some(enigo::Key::Layout('o')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('c')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('b')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_1: KeyClickConfig{
                key: Some(enigo::Key::Layout('2')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('0')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('/')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: [None,None,None,None,None],
            },
        },
        right_stick_up: SingleKeyConfig {
            first_layer_step_1: KeyClickConfig {
                key: Some(enigo::Key::Layout('e')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('h')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('g')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_4: KeyClickConfig{
                key: Some(enigo::Key::Layout('q')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_1: KeyClickConfig{
                key: Some(enigo::Key::Layout('5')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('\'')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Tab),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: [None,None,None,None,None],
            },
        },
        right_stick_down: SingleKeyConfig {
            first_layer_step_1: KeyClickConfig {
                key: Some(enigo::Key::Layout('i')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('u')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('v')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_1: KeyClickConfig{
                key: Some(enigo::Key::Layout('7')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout(']')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_3: KeyClickConfig{
                key: None,
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: [None,None,None,None,None],
            },
        },
        right_stick_left: SingleKeyConfig {
            first_layer_step_1: KeyClickConfig {
                key: Some(enigo::Key::Layout('a')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('d')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('y')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_1: KeyClickConfig{
                key: Some(enigo::Key::Layout('8')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('[')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_3: KeyClickConfig{
                key: None,
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: [None,None,None,None,None],
            },
        },
        right_stick_right: SingleKeyConfig {
            first_layer_step_1: KeyClickConfig {
                key: Some(enigo::Key::Space),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('s')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('f')),
                modifiers: [None,None,None,None,None],
            },
            first_layer_step_4: KeyClickConfig{
                key: Some(enigo::Key::Layout('x')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_1: KeyClickConfig{
                key: Some(enigo::Key::Layout('6')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout(';')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('\\')),
                modifiers: [None,None,None,None,None],
            },
            second_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: [None,None,None,None,None],
            },
        },
    }
}

pub fn settings_data_keys_config() -> settings_data::KeyboardModeKeyMappings{
settings_data::KeyboardModeKeyMappings {

        south: settings_data::KeyboardModeSingleKeyMapping {
            first_layer_step_1: settings_data::KeyClickConfig {
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('I'),
                key: None,
            },
            first_layer_step_2: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('U'),
                key: None,
            },
            first_layer_step_3: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('V'),
                key: None,
            },
            first_layer_step_4: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
            },
            second_layer_step_1: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('&'),
                key: None,
            },
            second_layer_step_2: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('}'),
                key: None,
            },
            second_layer_step_3: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
            },
            second_layer_step_4: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
            },
        },
        east: settings_data::KeyboardModeSingleKeyMapping {
            first_layer_step_1: settings_data::KeyClickConfig {
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: Some(EnigoKey::Return),
            },
            first_layer_step_2: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('S'),
                key: None,
            },
            first_layer_step_3: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('F'),
                key: None,
            },
            first_layer_step_4: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('X'),
                key: None,
            },
            second_layer_step_1: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('^'),
                key: None,
            },
            second_layer_step_2: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some(':'),
                key: None,
                
            },
            second_layer_step_3: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('|'),
                key: None,
                
            },
            second_layer_step_4: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
        },
        north: settings_data::KeyboardModeSingleKeyMapping {
            first_layer_step_1: settings_data::KeyClickConfig {
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('E'),
                key: None,
                
            },
            first_layer_step_2: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('H'),
                key: None,
                
            },
            first_layer_step_3: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('G'),
                key: None,
                
            },
            first_layer_step_4: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('Q'),
                key: None,
                
            },
            second_layer_step_1: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('%'),
                key: None,
                
            },
            second_layer_step_2: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('\"'),
                key: None,
                
            },
            second_layer_step_3: settings_data::KeyClickConfig{
				modifier_1: Some(EnigoKey::Shift),
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: Some(EnigoKey::Tab),
            },
            second_layer_step_4: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
        },
        west: settings_data::KeyboardModeSingleKeyMapping {
            first_layer_step_1: settings_data::KeyClickConfig {
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('A'),
                key: None,
                
            },
            first_layer_step_2: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('D'),
                key: None,
                
            },
            first_layer_step_3: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('Y'),
                key: None,
                
            },
            first_layer_step_4: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
            second_layer_step_1: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('*'),
                key: None,
                
            },
            second_layer_step_2: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('{'),
                key: None,
                
            },
            second_layer_step_3: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
            second_layer_step_4: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
        },
        d_pad_up: settings_data::KeyboardModeSingleKeyMapping {
            first_layer_step_1: settings_data::KeyClickConfig {
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('T'),
                key: None,
                
            },
            first_layer_step_2: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('L'),
                key: None,
                
            },
            first_layer_step_3: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('W'),
                key: None,
                
            },
            first_layer_step_4: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('Z'),
                key: None,
                
            },
            second_layer_step_1: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('!'),
                key: None,
                
            },
            second_layer_step_2: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('('),
                key: None,
                
            },
            second_layer_step_3: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('_'),
                key: None,
                
            },
            second_layer_step_4: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
        },
        d_pad_down: settings_data::KeyboardModeSingleKeyMapping {
            first_layer_step_1: settings_data::KeyClickConfig {
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('N'),
                key: None,
                
            },
            first_layer_step_2: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('M'),
                key: None,
                
            },
            first_layer_step_3: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('K'),
                key: None,
                
            },
            first_layer_step_4: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
            second_layer_step_1: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('#'),
                key: None,
                
            },
            second_layer_step_2: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('>'),
                key: None,
                
            },
            second_layer_step_3: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('+'),
                key: None,
                
            },
            second_layer_step_4: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
        },
        d_pad_left: settings_data::KeyboardModeSingleKeyMapping {
            first_layer_step_1: settings_data::KeyClickConfig {
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
            first_layer_step_2: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('R'),
                key: None,
                
            },
            first_layer_step_3: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('P'),
                key: None,
                
            },
            first_layer_step_4: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('J'),
                key: None,
                
            },
            second_layer_step_1: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('$'),
                key: None,
                
            },
            second_layer_step_2: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('<'),
                key: None,
                
            },
            second_layer_step_3: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('~'),
                key: None,
                
            },
            second_layer_step_4: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
        },
        d_pad_right: settings_data::KeyboardModeSingleKeyMapping {
            first_layer_step_1: settings_data::KeyClickConfig {
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('O'),
                key: None,
                
            },
            first_layer_step_2: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('C'),
                key: None,
                
            },
            first_layer_step_3: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('B'),
                key: None,
                
            },
            first_layer_step_4: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
            second_layer_step_1: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('@'),
                key: None,
                
            },
            second_layer_step_2: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some(')'),
                key: None,
                
            },
            second_layer_step_3: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('?'),
                key: None,
                
            },
            second_layer_step_4: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
        },
        left_stick_up: settings_data::KeyboardModeSingleKeyMapping {
            first_layer_step_1: settings_data::KeyClickConfig {
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('t'),
                key: None,
                
            },
            first_layer_step_2: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('l'),
                key: None,
                
            },
            first_layer_step_3: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('w'),
                key: None,
                
            },
            first_layer_step_4: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('z'),
                key: None,
                
            },
            second_layer_step_1: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('1'),
                key: None,
                
            },
            second_layer_step_2: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('9'),
                key: None,
                
            },
            second_layer_step_3: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('_'),
                key: None,
                
            },
            second_layer_step_4: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
        },
        left_stick_down: settings_data::KeyboardModeSingleKeyMapping {
            first_layer_step_1: settings_data::KeyClickConfig {
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('n'),
                key: None,
                
            },
            first_layer_step_2: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('m'),
                key: None,
                
            },
            first_layer_step_3: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('k'),
                key: None,
                
            },
            first_layer_step_4: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
            second_layer_step_1: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('3'),
                key: None,
                
            },
            second_layer_step_2: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('.'),
                key: None,
                
            },
            second_layer_step_3: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('='),
                key: None,
                
            },
            second_layer_step_4: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
        },
        left_stick_left: settings_data::KeyboardModeSingleKeyMapping {
            first_layer_step_1: settings_data::KeyClickConfig {
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: Some(EnigoKey::Backspace),
                
            },
            first_layer_step_2: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('r'),
                key: None,
                
            },
            first_layer_step_3: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('p'),
                key: None,
                
            },
            first_layer_step_4: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('j'),
                key: None,
                
            },
            second_layer_step_1: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('4'),
                key: None,
                
            },
            second_layer_step_2: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some(','),
                key: None,
                
            },
            second_layer_step_3: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('`'),
                key: None,
                
            },
            second_layer_step_4: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
        },
        left_stick_right: settings_data::KeyboardModeSingleKeyMapping {
            first_layer_step_1: settings_data::KeyClickConfig {
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('o'),
                key: None,
                
            },
            first_layer_step_2: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('c'),
                key: None,
                
            },
            first_layer_step_3: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('b'),
                key: None,
                
            },
            first_layer_step_4: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
            second_layer_step_1: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('2'),
                key: None,
                
            },
            second_layer_step_2: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('0'),
                key: None,
                
            },
            second_layer_step_3: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('/'),
                key: None,
                
            },
            second_layer_step_4: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
        },
        right_stick_up: settings_data::KeyboardModeSingleKeyMapping {
            first_layer_step_1: settings_data::KeyClickConfig {
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('e'),
                key: None,
                
            },
            first_layer_step_2: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('h'),
                key: None,
                
            },
            first_layer_step_3: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('g'),
                key: None,
                
            },
            first_layer_step_4: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('q'),
                key: None,
                
            },
            second_layer_step_1: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('5'),
                key: None,
                
            },
            second_layer_step_2: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('\''),
                key: None,
                
            },
            second_layer_step_3: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: Some(EnigoKey::Tab),
                
            },
            second_layer_step_4: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
        },
        right_stick_down: settings_data::KeyboardModeSingleKeyMapping {
            first_layer_step_1: settings_data::KeyClickConfig {
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('i'),
                key: None,
                
            },
            first_layer_step_2: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('u'),
                key: None,
                
            },
            first_layer_step_3: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('v'),
                key: None,
                
            },
            first_layer_step_4: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
            second_layer_step_1: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('7'),
                key: None,
                
            },
            second_layer_step_2: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some(']'),
                key: None,
                
            },
            second_layer_step_3: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
            second_layer_step_4: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
        },
        right_stick_left: settings_data::KeyboardModeSingleKeyMapping {
            first_layer_step_1: settings_data::KeyClickConfig {
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('a'),
                key: None,
                
            },
            first_layer_step_2: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('d'),
                key: None,
                
            },
            first_layer_step_3: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('y'),
                key: None,
                
            },
            first_layer_step_4: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
            second_layer_step_1: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('8'),
                key: None,
                
            },
            second_layer_step_2: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('['),
                key: None,
                
            },
            second_layer_step_3: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
            second_layer_step_4: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: None,
                
            },
        },
        right_stick_right: settings_data::KeyboardModeSingleKeyMapping {
            first_layer_step_1: settings_data::KeyClickConfig {
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: None,
                key: Some(EnigoKey::Space),
                
            },
            first_layer_step_2: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('s'),
                key: None,
                
            },
            first_layer_step_3: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('f'),
                key: None,
                
            },
            first_layer_step_4: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('x'),
                key: None,
                
            },
            second_layer_step_1: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('6'),
                key: None,
                
            },
            second_layer_step_2: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some(';'),
                key: None,
                
            },
            second_layer_step_3: settings_data::KeyClickConfig{
				modifier_1: None,
				modifier_2: None,
				modifier_3: None,
				modifier_4: None,
				modifier_5: None,
                char_key: Some('\\'),
                key: None,
                
            },
            second_layer_step_4: settings_data::KeyClickConfig{
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

