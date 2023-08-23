use crate::joy_keyboard::joy_keyboard_keys_config::{JoyKeyboardKeysConfig, JoyKeyboardKeyConfig, JoyKeyboardKeyMapping};

pub fn joy_keyboard_keys_config() -> JoyKeyboardKeysConfig{
    JoyKeyboardKeysConfig {
        south: JoyKeyboardKeyConfig {
            first_layer_step_1: JoyKeyboardKeyMapping {
                key: Some(enigo::Key::Layout('I')),
                modifiers: None,
            },
            first_layer_step_2: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('U')),
                modifiers: None,
            },
            first_layer_step_3: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('V')),
                modifiers: None,
            },
            first_layer_step_4: JoyKeyboardKeyMapping{
                key: None,
                modifiers: None,
            },
            second_layer_step_1: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('&')),
                modifiers: None,
            },
            second_layer_step_2: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('}')),
                modifiers: None,
            },
            second_layer_step_3: JoyKeyboardKeyMapping{
                key: None,
                modifiers: None,
            },
            second_layer_step_4: JoyKeyboardKeyMapping{
                key: None,
                modifiers: None,
            },
        },
        east: JoyKeyboardKeyConfig {
            first_layer_step_1: JoyKeyboardKeyMapping {
                key: Some(enigo::Key::Return),
                modifiers: None,
            },
            first_layer_step_2: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('S')),
                modifiers: None,
            },
            first_layer_step_3: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('F')),
                modifiers: None,
            },
            first_layer_step_4: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('X')),
                modifiers: None,
            },
            second_layer_step_1: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('^')),
                modifiers: None,
            },
            second_layer_step_2: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout(':')),
                modifiers: None,
            },
            second_layer_step_3: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('|')),
                modifiers: None,
            },
            second_layer_step_4: JoyKeyboardKeyMapping{
                key: None,
                modifiers: None,
            },
        },
        north: JoyKeyboardKeyConfig {
            first_layer_step_1: JoyKeyboardKeyMapping {
                key: Some(enigo::Key::Layout('E')),
                modifiers: None,
            },
            first_layer_step_2: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('H')),
                modifiers: None,
            },
            first_layer_step_3: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('G')),
                modifiers: None,
            },
            first_layer_step_4: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('Q')),
                modifiers: None,
            },
            second_layer_step_1: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('%')),
                modifiers: None,
            },
            second_layer_step_2: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('\"')),
                modifiers: None,
            },
            second_layer_step_3: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Tab),
                modifiers: Some(vec![enigo::Key::Shift]),
            },
            second_layer_step_4: JoyKeyboardKeyMapping{
                key: None,
                modifiers: None,
            },
        },
        west: JoyKeyboardKeyConfig {
            first_layer_step_1: JoyKeyboardKeyMapping {
                key: Some(enigo::Key::Layout('A')),
                modifiers: None,
            },
            first_layer_step_2: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('D')),
                modifiers: None,
            },
            first_layer_step_3: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('Y')),
                modifiers: None,
            },
            first_layer_step_4: JoyKeyboardKeyMapping{
                key: None,
                modifiers: None,
            },
            second_layer_step_1: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('*')),
                modifiers: None,
            },
            second_layer_step_2: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('{')),
                modifiers: None,
            },
            second_layer_step_3: JoyKeyboardKeyMapping{
                key: None,
                modifiers: None,
            },
            second_layer_step_4: JoyKeyboardKeyMapping{
                key: None,
                modifiers: None,
            },
        },
        d_pad_up: JoyKeyboardKeyConfig {
            first_layer_step_1: JoyKeyboardKeyMapping {
                key: Some(enigo::Key::Layout('T')),
                modifiers: None,
            },
            first_layer_step_2: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('L')),
                modifiers: None,
            },
            first_layer_step_3: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('W')),
                modifiers: None,
            },
            first_layer_step_4: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('Z')),
                modifiers: None,
            },
            second_layer_step_1: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('!')),
                modifiers: None,
            },
            second_layer_step_2: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('(')),
                modifiers: None,
            },
            second_layer_step_3: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('_')),
                modifiers: None,
            },
            second_layer_step_4: JoyKeyboardKeyMapping{
                key: None,
                modifiers: None,
            },
        },
        d_pad_down: JoyKeyboardKeyConfig {
            first_layer_step_1: JoyKeyboardKeyMapping {
                key: Some(enigo::Key::Layout('N')),
                modifiers: None,
            },
            first_layer_step_2: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('M')),
                modifiers: None,
            },
            first_layer_step_3: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('K')),
                modifiers: None,
            },
            first_layer_step_4: JoyKeyboardKeyMapping{
                key: None,
                modifiers: None,
            },
            second_layer_step_1: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('#')),
                modifiers: None,
            },
            second_layer_step_2: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('>')),
                modifiers: None,
            },
            second_layer_step_3: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('+')),
                modifiers: None,
            },
            second_layer_step_4: JoyKeyboardKeyMapping{
                key: None,
                modifiers: None,
            },
        },
        d_pad_left: JoyKeyboardKeyConfig {
            first_layer_step_1: JoyKeyboardKeyMapping {
                key: None,
                modifiers: None,
            },
            first_layer_step_2: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('R')),
                modifiers: None,
            },
            first_layer_step_3: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('P')),
                modifiers: None,
            },
            first_layer_step_4: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('J')),
                modifiers: None,
            },
            second_layer_step_1: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('$')),
                modifiers: None,
            },
            second_layer_step_2: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('<')),
                modifiers: None,
            },
            second_layer_step_3: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('~')),
                modifiers: None,
            },
            second_layer_step_4: JoyKeyboardKeyMapping{
                key: None,
                modifiers: None,
            },
        },
        d_pad_right: JoyKeyboardKeyConfig {
            first_layer_step_1: JoyKeyboardKeyMapping {
                key: Some(enigo::Key::Layout('O')),
                modifiers: None,
            },
            first_layer_step_2: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('C')),
                modifiers: None,
            },
            first_layer_step_3: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('B')),
                modifiers: None,
            },
            first_layer_step_4: JoyKeyboardKeyMapping{
                key: None,
                modifiers: None,
            },
            second_layer_step_1: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('@')),
                modifiers: None,
            },
            second_layer_step_2: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout(')')),
                modifiers: None,
            },
            second_layer_step_3: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('?')),
                modifiers: None,
            },
            second_layer_step_4: JoyKeyboardKeyMapping{
                key: None,
                modifiers: None,
            },
        },
        left_stick_up: JoyKeyboardKeyConfig {
            first_layer_step_1: JoyKeyboardKeyMapping {
                key: Some(enigo::Key::Layout('t')),
                modifiers: None,
            },
            first_layer_step_2: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('l')),
                modifiers: None,
            },
            first_layer_step_3: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('w')),
                modifiers: None,
            },
            first_layer_step_4: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('z')),
                modifiers: None,
            },
            second_layer_step_1: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('1')),
                modifiers: None,
            },
            second_layer_step_2: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('9')),
                modifiers: None,
            },
            second_layer_step_3: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('_')),
                modifiers: None,
            },
            second_layer_step_4: JoyKeyboardKeyMapping{
                key: None,
                modifiers: None,
            },
        },
        left_stick_down: JoyKeyboardKeyConfig {
            first_layer_step_1: JoyKeyboardKeyMapping {
                key: Some(enigo::Key::Layout('n')),
                modifiers: None,
            },
            first_layer_step_2: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('m')),
                modifiers: None,
            },
            first_layer_step_3: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('k')),
                modifiers: None,
            },
            first_layer_step_4: JoyKeyboardKeyMapping{
                key: None,
                modifiers: None,
            },
            second_layer_step_1: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('3')),
                modifiers: None,
            },
            second_layer_step_2: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('.')),
                modifiers: None,
            },
            second_layer_step_3: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('=')),
                modifiers: None,
            },
            second_layer_step_4: JoyKeyboardKeyMapping{
                key: None,
                modifiers: None,
            },
        },
        left_stick_left: JoyKeyboardKeyConfig {
            first_layer_step_1: JoyKeyboardKeyMapping {
                key: Some(enigo::Key::Backspace),
                modifiers: None,
            },
            first_layer_step_2: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('r')),
                modifiers: None,
            },
            first_layer_step_3: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('p')),
                modifiers: None,
            },
            first_layer_step_4: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('j')),
                modifiers: None,
            },
            second_layer_step_1: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('4')),
                modifiers: None,
            },
            second_layer_step_2: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout(',')),
                modifiers: None,
            },
            second_layer_step_3: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('`')),
                modifiers: None,
            },
            second_layer_step_4: JoyKeyboardKeyMapping{
                key: None,
                modifiers: None,
            },
        },
        left_stick_right: JoyKeyboardKeyConfig {
            first_layer_step_1: JoyKeyboardKeyMapping {
                key: Some(enigo::Key::Layout('o')),
                modifiers: None,
            },
            first_layer_step_2: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('c')),
                modifiers: None,
            },
            first_layer_step_3: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('b')),
                modifiers: None,
            },
            first_layer_step_4: JoyKeyboardKeyMapping{
                key: None,
                modifiers: None,
            },
            second_layer_step_1: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('2')),
                modifiers: None,
            },
            second_layer_step_2: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('0')),
                modifiers: None,
            },
            second_layer_step_3: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('/')),
                modifiers: None,
            },
            second_layer_step_4: JoyKeyboardKeyMapping{
                key: None,
                modifiers: None,
            },
        },
        right_stick_up: JoyKeyboardKeyConfig {
            first_layer_step_1: JoyKeyboardKeyMapping {
                key: Some(enigo::Key::Layout('e')),
                modifiers: None,
            },
            first_layer_step_2: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('h')),
                modifiers: None,
            },
            first_layer_step_3: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('g')),
                modifiers: None,
            },
            first_layer_step_4: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('q')),
                modifiers: None,
            },
            second_layer_step_1: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('5')),
                modifiers: None,
            },
            second_layer_step_2: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('\'')),
                modifiers: None,
            },
            second_layer_step_3: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Tab),
                modifiers: None,
            },
            second_layer_step_4: JoyKeyboardKeyMapping{
                key: None,
                modifiers: None,
            },
        },
        right_stick_down: JoyKeyboardKeyConfig {
            first_layer_step_1: JoyKeyboardKeyMapping {
                key: Some(enigo::Key::Layout('i')),
                modifiers: None,
            },
            first_layer_step_2: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('u')),
                modifiers: None,
            },
            first_layer_step_3: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('v')),
                modifiers: None,
            },
            first_layer_step_4: JoyKeyboardKeyMapping{
                key: None,
                modifiers: None,
            },
            second_layer_step_1: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('7')),
                modifiers: None,
            },
            second_layer_step_2: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout(']')),
                modifiers: None,
            },
            second_layer_step_3: JoyKeyboardKeyMapping{
                key: None,
                modifiers: None,
            },
            second_layer_step_4: JoyKeyboardKeyMapping{
                key: None,
                modifiers: None,
            },
        },
        right_stick_left: JoyKeyboardKeyConfig {
            first_layer_step_1: JoyKeyboardKeyMapping {
                key: Some(enigo::Key::Layout('a')),
                modifiers: None,
            },
            first_layer_step_2: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('d')),
                modifiers: None,
            },
            first_layer_step_3: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('y')),
                modifiers: None,
            },
            first_layer_step_4: JoyKeyboardKeyMapping{
                key: None,
                modifiers: None,
            },
            second_layer_step_1: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('8')),
                modifiers: None,
            },
            second_layer_step_2: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('[')),
                modifiers: None,
            },
            second_layer_step_3: JoyKeyboardKeyMapping{
                key: None,
                modifiers: None,
            },
            second_layer_step_4: JoyKeyboardKeyMapping{
                key: None,
                modifiers: None,
            },
        },
        right_stick_right: JoyKeyboardKeyConfig {
            first_layer_step_1: JoyKeyboardKeyMapping {
                key: Some(enigo::Key::Space),
                modifiers: None,
            },
            first_layer_step_2: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('s')),
                modifiers: None,
            },
            first_layer_step_3: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('f')),
                modifiers: None,
            },
            first_layer_step_4: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('x')),
                modifiers: None,
            },
            second_layer_step_1: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('6')),
                modifiers: None,
            },
            second_layer_step_2: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout(';')),
                modifiers: None,
            },
            second_layer_step_3: JoyKeyboardKeyMapping{
                key: Some(enigo::Key::Layout('\\')),
                modifiers: None,
            },
            second_layer_step_4: JoyKeyboardKeyMapping{
                key: None,
                modifiers: None,
            },
        },
    }
}
