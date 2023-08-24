use crate::joy_keyboard::keys_config::{KeysConfig, SingleKeyConfig, KeyClickConfig};

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
