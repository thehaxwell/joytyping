use crate::{joy_keyboard::keys_config::{KeysConfig, SingleKeyConfig, KeyClickConfig}, settings_data::{self, EnigoKey}};

pub fn joy_keyboard_keys_config() -> KeysConfig{
    KeysConfig {
        south: SingleKeyConfig {
            first_layer_step_1: KeyClickConfig {
                key: Some(enigo::Key::Layout('I')),
                modifiers: None,
            },
            first_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('U')),
                modifiers: None,
            },
            first_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('V')),
                modifiers: None,
            },
            first_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: None,
            },
            second_layer_step_1: KeyClickConfig{
                key: Some(enigo::Key::Layout('&')),
                modifiers: None,
            },
            second_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('}')),
                modifiers: None,
            },
            second_layer_step_3: KeyClickConfig{
                key: None,
                modifiers: None,
            },
            second_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: None,
            },
        },
        east: SingleKeyConfig {
            first_layer_step_1: KeyClickConfig {
                key: Some(enigo::Key::Return),
                modifiers: None,
            },
            first_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('S')),
                modifiers: None,
            },
            first_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('F')),
                modifiers: None,
            },
            first_layer_step_4: KeyClickConfig{
                key: Some(enigo::Key::Layout('X')),
                modifiers: None,
            },
            second_layer_step_1: KeyClickConfig{
                key: Some(enigo::Key::Layout('^')),
                modifiers: None,
            },
            second_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout(':')),
                modifiers: None,
            },
            second_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('|')),
                modifiers: None,
            },
            second_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: None,
            },
        },
        north: SingleKeyConfig {
            first_layer_step_1: KeyClickConfig {
                key: Some(enigo::Key::Layout('E')),
                modifiers: None,
            },
            first_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('H')),
                modifiers: None,
            },
            first_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('G')),
                modifiers: None,
            },
            first_layer_step_4: KeyClickConfig{
                key: Some(enigo::Key::Layout('Q')),
                modifiers: None,
            },
            second_layer_step_1: KeyClickConfig{
                key: Some(enigo::Key::Layout('%')),
                modifiers: None,
            },
            second_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('\"')),
                modifiers: None,
            },
            second_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Tab),
                modifiers: Some(vec![enigo::Key::Shift]),
            },
            second_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: None,
            },
        },
        west: SingleKeyConfig {
            first_layer_step_1: KeyClickConfig {
                key: Some(enigo::Key::Layout('A')),
                modifiers: None,
            },
            first_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('D')),
                modifiers: None,
            },
            first_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('Y')),
                modifiers: None,
            },
            first_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: None,
            },
            second_layer_step_1: KeyClickConfig{
                key: Some(enigo::Key::Layout('*')),
                modifiers: None,
            },
            second_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('{')),
                modifiers: None,
            },
            second_layer_step_3: KeyClickConfig{
                key: None,
                modifiers: None,
            },
            second_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: None,
            },
        },
        d_pad_up: SingleKeyConfig {
            first_layer_step_1: KeyClickConfig {
                key: Some(enigo::Key::Layout('T')),
                modifiers: None,
            },
            first_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('L')),
                modifiers: None,
            },
            first_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('W')),
                modifiers: None,
            },
            first_layer_step_4: KeyClickConfig{
                key: Some(enigo::Key::Layout('Z')),
                modifiers: None,
            },
            second_layer_step_1: KeyClickConfig{
                key: Some(enigo::Key::Layout('!')),
                modifiers: None,
            },
            second_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('(')),
                modifiers: None,
            },
            second_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('_')),
                modifiers: None,
            },
            second_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: None,
            },
        },
        d_pad_down: SingleKeyConfig {
            first_layer_step_1: KeyClickConfig {
                key: Some(enigo::Key::Layout('N')),
                modifiers: None,
            },
            first_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('M')),
                modifiers: None,
            },
            first_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('K')),
                modifiers: None,
            },
            first_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: None,
            },
            second_layer_step_1: KeyClickConfig{
                key: Some(enigo::Key::Layout('#')),
                modifiers: None,
            },
            second_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('>')),
                modifiers: None,
            },
            second_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('+')),
                modifiers: None,
            },
            second_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: None,
            },
        },
        d_pad_left: SingleKeyConfig {
            first_layer_step_1: KeyClickConfig {
                key: None,
                modifiers: None,
            },
            first_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('R')),
                modifiers: None,
            },
            first_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('P')),
                modifiers: None,
            },
            first_layer_step_4: KeyClickConfig{
                key: Some(enigo::Key::Layout('J')),
                modifiers: None,
            },
            second_layer_step_1: KeyClickConfig{
                key: Some(enigo::Key::Layout('$')),
                modifiers: None,
            },
            second_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('<')),
                modifiers: None,
            },
            second_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('~')),
                modifiers: None,
            },
            second_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: None,
            },
        },
        d_pad_right: SingleKeyConfig {
            first_layer_step_1: KeyClickConfig {
                key: Some(enigo::Key::Layout('O')),
                modifiers: None,
            },
            first_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('C')),
                modifiers: None,
            },
            first_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('B')),
                modifiers: None,
            },
            first_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: None,
            },
            second_layer_step_1: KeyClickConfig{
                key: Some(enigo::Key::Layout('@')),
                modifiers: None,
            },
            second_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout(')')),
                modifiers: None,
            },
            second_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('?')),
                modifiers: None,
            },
            second_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: None,
            },
        },
        left_stick_up: SingleKeyConfig {
            first_layer_step_1: KeyClickConfig {
                key: Some(enigo::Key::Layout('t')),
                modifiers: None,
            },
            first_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('l')),
                modifiers: None,
            },
            first_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('w')),
                modifiers: None,
            },
            first_layer_step_4: KeyClickConfig{
                key: Some(enigo::Key::Layout('z')),
                modifiers: None,
            },
            second_layer_step_1: KeyClickConfig{
                key: Some(enigo::Key::Layout('1')),
                modifiers: None,
            },
            second_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('9')),
                modifiers: None,
            },
            second_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('_')),
                modifiers: None,
            },
            second_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: None,
            },
        },
        left_stick_down: SingleKeyConfig {
            first_layer_step_1: KeyClickConfig {
                key: Some(enigo::Key::Layout('n')),
                modifiers: None,
            },
            first_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('m')),
                modifiers: None,
            },
            first_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('k')),
                modifiers: None,
            },
            first_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: None,
            },
            second_layer_step_1: KeyClickConfig{
                key: Some(enigo::Key::Layout('3')),
                modifiers: None,
            },
            second_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('.')),
                modifiers: None,
            },
            second_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('=')),
                modifiers: None,
            },
            second_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: None,
            },
        },
        left_stick_left: SingleKeyConfig {
            first_layer_step_1: KeyClickConfig {
                key: Some(enigo::Key::Backspace),
                modifiers: None,
            },
            first_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('r')),
                modifiers: None,
            },
            first_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('p')),
                modifiers: None,
            },
            first_layer_step_4: KeyClickConfig{
                key: Some(enigo::Key::Layout('j')),
                modifiers: None,
            },
            second_layer_step_1: KeyClickConfig{
                key: Some(enigo::Key::Layout('4')),
                modifiers: None,
            },
            second_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout(',')),
                modifiers: None,
            },
            second_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('`')),
                modifiers: None,
            },
            second_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: None,
            },
        },
        left_stick_right: SingleKeyConfig {
            first_layer_step_1: KeyClickConfig {
                key: Some(enigo::Key::Layout('o')),
                modifiers: None,
            },
            first_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('c')),
                modifiers: None,
            },
            first_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('b')),
                modifiers: None,
            },
            first_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: None,
            },
            second_layer_step_1: KeyClickConfig{
                key: Some(enigo::Key::Layout('2')),
                modifiers: None,
            },
            second_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('0')),
                modifiers: None,
            },
            second_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('/')),
                modifiers: None,
            },
            second_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: None,
            },
        },
        right_stick_up: SingleKeyConfig {
            first_layer_step_1: KeyClickConfig {
                key: Some(enigo::Key::Layout('e')),
                modifiers: None,
            },
            first_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('h')),
                modifiers: None,
            },
            first_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('g')),
                modifiers: None,
            },
            first_layer_step_4: KeyClickConfig{
                key: Some(enigo::Key::Layout('q')),
                modifiers: None,
            },
            second_layer_step_1: KeyClickConfig{
                key: Some(enigo::Key::Layout('5')),
                modifiers: None,
            },
            second_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('\'')),
                modifiers: None,
            },
            second_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Tab),
                modifiers: None,
            },
            second_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: None,
            },
        },
        right_stick_down: SingleKeyConfig {
            first_layer_step_1: KeyClickConfig {
                key: Some(enigo::Key::Layout('i')),
                modifiers: None,
            },
            first_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('u')),
                modifiers: None,
            },
            first_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('v')),
                modifiers: None,
            },
            first_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: None,
            },
            second_layer_step_1: KeyClickConfig{
                key: Some(enigo::Key::Layout('7')),
                modifiers: None,
            },
            second_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout(']')),
                modifiers: None,
            },
            second_layer_step_3: KeyClickConfig{
                key: None,
                modifiers: None,
            },
            second_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: None,
            },
        },
        right_stick_left: SingleKeyConfig {
            first_layer_step_1: KeyClickConfig {
                key: Some(enigo::Key::Layout('a')),
                modifiers: None,
            },
            first_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('d')),
                modifiers: None,
            },
            first_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('y')),
                modifiers: None,
            },
            first_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: None,
            },
            second_layer_step_1: KeyClickConfig{
                key: Some(enigo::Key::Layout('8')),
                modifiers: None,
            },
            second_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('[')),
                modifiers: None,
            },
            second_layer_step_3: KeyClickConfig{
                key: None,
                modifiers: None,
            },
            second_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: None,
            },
        },
        right_stick_right: SingleKeyConfig {
            first_layer_step_1: KeyClickConfig {
                key: Some(enigo::Key::Space),
                modifiers: None,
            },
            first_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout('s')),
                modifiers: None,
            },
            first_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('f')),
                modifiers: None,
            },
            first_layer_step_4: KeyClickConfig{
                key: Some(enigo::Key::Layout('x')),
                modifiers: None,
            },
            second_layer_step_1: KeyClickConfig{
                key: Some(enigo::Key::Layout('6')),
                modifiers: None,
            },
            second_layer_step_2: KeyClickConfig{
                key: Some(enigo::Key::Layout(';')),
                modifiers: None,
            },
            second_layer_step_3: KeyClickConfig{
                key: Some(enigo::Key::Layout('\\')),
                modifiers: None,
            },
            second_layer_step_4: KeyClickConfig{
                key: None,
                modifiers: None,
            },
        },
    }
}

pub fn settings_data_keys_config() -> settings_data::KeyboardModeKeyMappings{
settings_data::KeyboardModeKeyMappings {
        south: settings_data::KeyboardModeSingleKeyMapping {
            first_layer_step_1: settings_data::KeyClickConfig {
                char_key: Some('I'),
                key: None,
                modifiers: None,
            },
            first_layer_step_2: settings_data::KeyClickConfig{
                char_key: Some('U'),
                key: None,
                modifiers: None,
            },
            first_layer_step_3: settings_data::KeyClickConfig{
                char_key: Some('V'),
                key: None,
                modifiers: None,
            },
            first_layer_step_4: settings_data::KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
            second_layer_step_1: settings_data::KeyClickConfig{
                char_key: Some('&'),
                key: None,
                modifiers: None,
            },
            second_layer_step_2: settings_data::KeyClickConfig{
                char_key: Some('}'),
                key: None,
                modifiers: None,
            },
            second_layer_step_3: settings_data::KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
            second_layer_step_4: settings_data::KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
        },
        east: settings_data::KeyboardModeSingleKeyMapping {
            first_layer_step_1: settings_data::KeyClickConfig {
                char_key: None,
                key: Some(EnigoKey::Return),
                modifiers: None,
            },
            first_layer_step_2: settings_data::KeyClickConfig{
                char_key: Some('S'),
                key: None,
                modifiers: None,
            },
            first_layer_step_3: settings_data::KeyClickConfig{
                char_key: Some('F'),
                key: None,
                modifiers: None,
            },
            first_layer_step_4: settings_data::KeyClickConfig{
                char_key: Some('X'),
                key: None,
                modifiers: None,
            },
            second_layer_step_1: settings_data::KeyClickConfig{
                char_key: Some('^'),
                key: None,
                modifiers: None,
            },
            second_layer_step_2: settings_data::KeyClickConfig{
                char_key: Some(':'),
                key: None,
                modifiers: None,
            },
            second_layer_step_3: settings_data::KeyClickConfig{
                char_key: Some('|'),
                key: None,
                modifiers: None,
            },
            second_layer_step_4: settings_data::KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
        },
        north: settings_data::KeyboardModeSingleKeyMapping {
            first_layer_step_1: settings_data::KeyClickConfig {
                char_key: Some('E'),
                key: None,
                modifiers: None,
            },
            first_layer_step_2: settings_data::KeyClickConfig{
                char_key: Some('H'),
                key: None,
                modifiers: None,
            },
            first_layer_step_3: settings_data::KeyClickConfig{
                char_key: Some('G'),
                key: None,
                modifiers: None,
            },
            first_layer_step_4: settings_data::KeyClickConfig{
                char_key: Some('Q'),
                key: None,
                modifiers: None,
            },
            second_layer_step_1: settings_data::KeyClickConfig{
                char_key: Some('%'),
                key: None,
                modifiers: None,
            },
            second_layer_step_2: settings_data::KeyClickConfig{
                char_key: Some('\"'),
                key: None,
                modifiers: None,
            },
            second_layer_step_3: settings_data::KeyClickConfig{
                char_key: None,
                key: Some(EnigoKey::Tab),
                modifiers: Some(vec![EnigoKey::Shift]),
            },
            second_layer_step_4: settings_data::KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
        },
        west: settings_data::KeyboardModeSingleKeyMapping {
            first_layer_step_1: settings_data::KeyClickConfig {
                char_key: Some('A'),
                key: None,
                modifiers: None,
            },
            first_layer_step_2: settings_data::KeyClickConfig{
                char_key: Some('D'),
                key: None,
                modifiers: None,
            },
            first_layer_step_3: settings_data::KeyClickConfig{
                char_key: Some('Y'),
                key: None,
                modifiers: None,
            },
            first_layer_step_4: settings_data::KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
            second_layer_step_1: settings_data::KeyClickConfig{
                char_key: Some('*'),
                key: None,
                modifiers: None,
            },
            second_layer_step_2: settings_data::KeyClickConfig{
                char_key: Some('{'),
                key: None,
                modifiers: None,
            },
            second_layer_step_3: settings_data::KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
            second_layer_step_4: settings_data::KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
        },
        d_pad_up: settings_data::KeyboardModeSingleKeyMapping {
            first_layer_step_1: settings_data::KeyClickConfig {
                char_key: Some('T'),
                key: None,
                modifiers: None,
            },
            first_layer_step_2: settings_data::KeyClickConfig{
                char_key: Some('L'),
                key: None,
                modifiers: None,
            },
            first_layer_step_3: settings_data::KeyClickConfig{
                char_key: Some('W'),
                key: None,
                modifiers: None,
            },
            first_layer_step_4: settings_data::KeyClickConfig{
                char_key: Some('Z'),
                key: None,
                modifiers: None,
            },
            second_layer_step_1: settings_data::KeyClickConfig{
                char_key: Some('!'),
                key: None,
                modifiers: None,
            },
            second_layer_step_2: settings_data::KeyClickConfig{
                char_key: Some('('),
                key: None,
                modifiers: None,
            },
            second_layer_step_3: settings_data::KeyClickConfig{
                char_key: Some('_'),
                key: None,
                modifiers: None,
            },
            second_layer_step_4: settings_data::KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
        },
        d_pad_down: settings_data::KeyboardModeSingleKeyMapping {
            first_layer_step_1: settings_data::KeyClickConfig {
                char_key: Some('N'),
                key: None,
                modifiers: None,
            },
            first_layer_step_2: settings_data::KeyClickConfig{
                char_key: Some('M'),
                key: None,
                modifiers: None,
            },
            first_layer_step_3: settings_data::KeyClickConfig{
                char_key: Some('K'),
                key: None,
                modifiers: None,
            },
            first_layer_step_4: settings_data::KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
            second_layer_step_1: settings_data::KeyClickConfig{
                char_key: Some('#'),
                key: None,
                modifiers: None,
            },
            second_layer_step_2: settings_data::KeyClickConfig{
                char_key: Some('>'),
                key: None,
                modifiers: None,
            },
            second_layer_step_3: settings_data::KeyClickConfig{
                char_key: Some('+'),
                key: None,
                modifiers: None,
            },
            second_layer_step_4: settings_data::KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
        },
        d_pad_left: settings_data::KeyboardModeSingleKeyMapping {
            first_layer_step_1: settings_data::KeyClickConfig {
                char_key: None,
                key: None,
                modifiers: None,
            },
            first_layer_step_2: settings_data::KeyClickConfig{
                char_key: Some('R'),
                key: None,
                modifiers: None,
            },
            first_layer_step_3: settings_data::KeyClickConfig{
                char_key: Some('P'),
                key: None,
                modifiers: None,
            },
            first_layer_step_4: settings_data::KeyClickConfig{
                char_key: Some('J'),
                key: None,
                modifiers: None,
            },
            second_layer_step_1: settings_data::KeyClickConfig{
                char_key: Some('$'),
                key: None,
                modifiers: None,
            },
            second_layer_step_2: settings_data::KeyClickConfig{
                char_key: Some('<'),
                key: None,
                modifiers: None,
            },
            second_layer_step_3: settings_data::KeyClickConfig{
                char_key: Some('~'),
                key: None,
                modifiers: None,
            },
            second_layer_step_4: settings_data::KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
        },
        d_pad_right: settings_data::KeyboardModeSingleKeyMapping {
            first_layer_step_1: settings_data::KeyClickConfig {
                char_key: Some('O'),
                key: None,
                modifiers: None,
            },
            first_layer_step_2: settings_data::KeyClickConfig{
                char_key: Some('C'),
                key: None,
                modifiers: None,
            },
            first_layer_step_3: settings_data::KeyClickConfig{
                char_key: Some('B'),
                key: None,
                modifiers: None,
            },
            first_layer_step_4: settings_data::KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
            second_layer_step_1: settings_data::KeyClickConfig{
                char_key: Some('@'),
                key: None,
                modifiers: None,
            },
            second_layer_step_2: settings_data::KeyClickConfig{
                char_key: Some(')'),
                key: None,
                modifiers: None,
            },
            second_layer_step_3: settings_data::KeyClickConfig{
                char_key: Some('?'),
                key: None,
                modifiers: None,
            },
            second_layer_step_4: settings_data::KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
        },
        left_stick_up: settings_data::KeyboardModeSingleKeyMapping {
            first_layer_step_1: settings_data::KeyClickConfig {
                char_key: Some('t'),
                key: None,
                modifiers: None,
            },
            first_layer_step_2: settings_data::KeyClickConfig{
                char_key: Some('l'),
                key: None,
                modifiers: None,
            },
            first_layer_step_3: settings_data::KeyClickConfig{
                char_key: Some('w'),
                key: None,
                modifiers: None,
            },
            first_layer_step_4: settings_data::KeyClickConfig{
                char_key: Some('z'),
                key: None,
                modifiers: None,
            },
            second_layer_step_1: settings_data::KeyClickConfig{
                char_key: Some('1'),
                key: None,
                modifiers: None,
            },
            second_layer_step_2: settings_data::KeyClickConfig{
                char_key: Some('9'),
                key: None,
                modifiers: None,
            },
            second_layer_step_3: settings_data::KeyClickConfig{
                char_key: Some('_'),
                key: None,
                modifiers: None,
            },
            second_layer_step_4: settings_data::KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
        },
        left_stick_down: settings_data::KeyboardModeSingleKeyMapping {
            first_layer_step_1: settings_data::KeyClickConfig {
                char_key: Some('n'),
                key: None,
                modifiers: None,
            },
            first_layer_step_2: settings_data::KeyClickConfig{
                char_key: Some('m'),
                key: None,
                modifiers: None,
            },
            first_layer_step_3: settings_data::KeyClickConfig{
                char_key: Some('k'),
                key: None,
                modifiers: None,
            },
            first_layer_step_4: settings_data::KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
            second_layer_step_1: settings_data::KeyClickConfig{
                char_key: Some('3'),
                key: None,
                modifiers: None,
            },
            second_layer_step_2: settings_data::KeyClickConfig{
                char_key: Some('.'),
                key: None,
                modifiers: None,
            },
            second_layer_step_3: settings_data::KeyClickConfig{
                char_key: Some('='),
                key: None,
                modifiers: None,
            },
            second_layer_step_4: settings_data::KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
        },
        left_stick_left: settings_data::KeyboardModeSingleKeyMapping {
            first_layer_step_1: settings_data::KeyClickConfig {
                char_key: None,
                key: Some(EnigoKey::Backspace),
                modifiers: None,
            },
            first_layer_step_2: settings_data::KeyClickConfig{
                char_key: Some('r'),
                key: None,
                modifiers: None,
            },
            first_layer_step_3: settings_data::KeyClickConfig{
                char_key: Some('p'),
                key: None,
                modifiers: None,
            },
            first_layer_step_4: settings_data::KeyClickConfig{
                char_key: Some('j'),
                key: None,
                modifiers: None,
            },
            second_layer_step_1: settings_data::KeyClickConfig{
                char_key: Some('4'),
                key: None,
                modifiers: None,
            },
            second_layer_step_2: settings_data::KeyClickConfig{
                char_key: Some(','),
                key: None,
                modifiers: None,
            },
            second_layer_step_3: settings_data::KeyClickConfig{
                char_key: Some('`'),
                key: None,
                modifiers: None,
            },
            second_layer_step_4: settings_data::KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
        },
        left_stick_right: settings_data::KeyboardModeSingleKeyMapping {
            first_layer_step_1: settings_data::KeyClickConfig {
                char_key: Some('o'),
                key: None,
                modifiers: None,
            },
            first_layer_step_2: settings_data::KeyClickConfig{
                char_key: Some('c'),
                key: None,
                modifiers: None,
            },
            first_layer_step_3: settings_data::KeyClickConfig{
                char_key: Some('b'),
                key: None,
                modifiers: None,
            },
            first_layer_step_4: settings_data::KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
            second_layer_step_1: settings_data::KeyClickConfig{
                char_key: Some('2'),
                key: None,
                modifiers: None,
            },
            second_layer_step_2: settings_data::KeyClickConfig{
                char_key: Some('0'),
                key: None,
                modifiers: None,
            },
            second_layer_step_3: settings_data::KeyClickConfig{
                char_key: Some('/'),
                key: None,
                modifiers: None,
            },
            second_layer_step_4: settings_data::KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
        },
        right_stick_up: settings_data::KeyboardModeSingleKeyMapping {
            first_layer_step_1: settings_data::KeyClickConfig {
                char_key: Some('e'),
                key: None,
                modifiers: None,
            },
            first_layer_step_2: settings_data::KeyClickConfig{
                char_key: Some('h'),
                key: None,
                modifiers: None,
            },
            first_layer_step_3: settings_data::KeyClickConfig{
                char_key: Some('g'),
                key: None,
                modifiers: None,
            },
            first_layer_step_4: settings_data::KeyClickConfig{
                char_key: Some('q'),
                key: None,
                modifiers: None,
            },
            second_layer_step_1: settings_data::KeyClickConfig{
                char_key: Some('5'),
                key: None,
                modifiers: None,
            },
            second_layer_step_2: settings_data::KeyClickConfig{
                char_key: Some('\''),
                key: None,
                modifiers: None,
            },
            second_layer_step_3: settings_data::KeyClickConfig{
                char_key: None,
                key: Some(EnigoKey::Tab),
                modifiers: None,
            },
            second_layer_step_4: settings_data::KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
        },
        right_stick_down: settings_data::KeyboardModeSingleKeyMapping {
            first_layer_step_1: settings_data::KeyClickConfig {
                char_key: Some('i'),
                key: None,
                modifiers: None,
            },
            first_layer_step_2: settings_data::KeyClickConfig{
                char_key: Some('u'),
                key: None,
                modifiers: None,
            },
            first_layer_step_3: settings_data::KeyClickConfig{
                char_key: Some('v'),
                key: None,
                modifiers: None,
            },
            first_layer_step_4: settings_data::KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
            second_layer_step_1: settings_data::KeyClickConfig{
                char_key: Some('7'),
                key: None,
                modifiers: None,
            },
            second_layer_step_2: settings_data::KeyClickConfig{
                char_key: Some(']'),
                key: None,
                modifiers: None,
            },
            second_layer_step_3: settings_data::KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
            second_layer_step_4: settings_data::KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
        },
        right_stick_left: settings_data::KeyboardModeSingleKeyMapping {
            first_layer_step_1: settings_data::KeyClickConfig {
                char_key: Some('a'),
                key: None,
                modifiers: None,
            },
            first_layer_step_2: settings_data::KeyClickConfig{
                char_key: Some('d'),
                key: None,
                modifiers: None,
            },
            first_layer_step_3: settings_data::KeyClickConfig{
                char_key: Some('y'),
                key: None,
                modifiers: None,
            },
            first_layer_step_4: settings_data::KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
            second_layer_step_1: settings_data::KeyClickConfig{
                char_key: Some('8'),
                key: None,
                modifiers: None,
            },
            second_layer_step_2: settings_data::KeyClickConfig{
                char_key: Some('['),
                key: None,
                modifiers: None,
            },
            second_layer_step_3: settings_data::KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
            second_layer_step_4: settings_data::KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
        },
        right_stick_right: settings_data::KeyboardModeSingleKeyMapping {
            first_layer_step_1: settings_data::KeyClickConfig {
                char_key: None,
                key: Some(EnigoKey::Space),
                modifiers: None,
            },
            first_layer_step_2: settings_data::KeyClickConfig{
                char_key: Some('s'),
                key: None,
                modifiers: None,
            },
            first_layer_step_3: settings_data::KeyClickConfig{
                char_key: Some('f'),
                key: None,
                modifiers: None,
            },
            first_layer_step_4: settings_data::KeyClickConfig{
                char_key: Some('x'),
                key: None,
                modifiers: None,
            },
            second_layer_step_1: settings_data::KeyClickConfig{
                char_key: Some('6'),
                key: None,
                modifiers: None,
            },
            second_layer_step_2: settings_data::KeyClickConfig{
                char_key: Some(';'),
                key: None,
                modifiers: None,
            },
            second_layer_step_3: settings_data::KeyClickConfig{
                char_key: Some('\\'),
                key: None,
                modifiers: None,
            },
            second_layer_step_4: settings_data::KeyClickConfig{
                char_key: None,
                key: None,
                modifiers: None,
            },
        },
        }
}

