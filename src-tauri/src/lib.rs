use quick_lookup_window::QuickLookupWindow;

pub mod joy_input;
pub mod gamepad;
pub mod quick_lookup_window;

pub fn run(mut gamepad: gamepad::Gamepad, mut joy_keyboard: joy_input::JoyKeyboard, mut quick_lookup_window: QuickLookupWindow){
    use gamepad::CustomButton;
    use joy_input::GamepadKeyConfig;

    loop {
        while let Some(event) = gamepad.next_event() {
            match event {
                gamepad::GamepadEvent::ButtonPressed(button)=> {
                print!("ButtonPressed: {:?}\n",button);
                    let key_to_click = match button{
                        CustomButton::Base(gilrs::Button::South) => Some(
                            GamepadKeyConfig { 
                                first_layer_step_1: Some(enigo::Key::Layout('I')),
                                first_layer_step_2: Some(enigo::Key::Layout('U')),
                                first_layer_step_3: Some(enigo::Key::Layout('V')),
                                first_layer_step_4: None,
                                second_layer_step_1: Some(enigo::Key::Layout('&')),
                                second_layer_step_2: Some(enigo::Key::Layout('}')),
                                second_layer_step_3: None,
                                second_layer_step_4: None,
                            },),
                        CustomButton::Base(gilrs::Button::East) => Some(
                            GamepadKeyConfig { 
                                first_layer_step_1: Some(enigo::Key::Return),
                                first_layer_step_2: Some(enigo::Key::Layout('S')),
                                first_layer_step_3: Some(enigo::Key::Layout('F')),
                                first_layer_step_4: Some(enigo::Key::Layout('X')), 
                                second_layer_step_1: Some(enigo::Key::Layout('^')),
                                second_layer_step_2: Some(enigo::Key::Layout(':')),
                                second_layer_step_3: Some(enigo::Key::Layout('|')),                                second_layer_step_4: None,
                            },),
                        CustomButton::Base(gilrs::Button::North) => Some(
                            GamepadKeyConfig { 
                                first_layer_step_1: Some(enigo::Key::Layout('E')),
                                first_layer_step_2: Some(enigo::Key::Layout('H')),
                                first_layer_step_3: Some(enigo::Key::Layout('G')),
                                first_layer_step_4: Some(enigo::Key::Layout('Q')), 
                                second_layer_step_1: Some(enigo::Key::Layout('%')), 
                                second_layer_step_2: Some(enigo::Key::Layout('"')), 
//TODO: turn this next one to backwards-tab (shift+tab)
                                second_layer_step_3: Some(enigo::Key::Tab),
                                second_layer_step_4: None,
                            },),
                        CustomButton::Base(gilrs::Button::West) => Some(
                            GamepadKeyConfig { 
                                first_layer_step_1: Some(enigo::Key::Layout('A')),
                                first_layer_step_2: Some(enigo::Key::Layout('D')),
                                first_layer_step_3: Some(enigo::Key::Layout('Y')),
                                first_layer_step_4: None,
                                second_layer_step_1: Some(enigo::Key::Layout('*')),
                                second_layer_step_2: Some(enigo::Key::Layout('{')),
                                second_layer_step_3: None,
                                second_layer_step_4: None,
                            },),
                        CustomButton::Base(gilrs::Button::DPadUp) => Some(
                            GamepadKeyConfig { 
                                first_layer_step_1: Some(enigo::Key::Layout('T')),
                                first_layer_step_2: Some(enigo::Key::Layout('L')),
                                first_layer_step_3: Some(enigo::Key::Layout('W')),
                                first_layer_step_4: Some(enigo::Key::Layout('Z')), 
                                second_layer_step_1: Some(enigo::Key::Layout('!')),
                                second_layer_step_2: Some(enigo::Key::Layout('(')),
                                second_layer_step_3: Some(enigo::Key::Layout('_')),
                                second_layer_step_4: None,
                            },),
                        CustomButton::Base(gilrs::Button::DPadDown) => Some(
                            GamepadKeyConfig { 
                                first_layer_step_1: Some(enigo::Key::Layout('N')),
                                first_layer_step_2: Some(enigo::Key::Layout('M')),
                                first_layer_step_3: Some(enigo::Key::Layout('K')),
                                first_layer_step_4: None,
                                second_layer_step_1: Some(enigo::Key::Layout('#')),
                                second_layer_step_2: Some(enigo::Key::Layout('>')),
                                second_layer_step_3: Some(enigo::Key::Layout('+')),
                                second_layer_step_4: None,
                            },),
                        CustomButton::Base(gilrs::Button::DPadLeft) => Some(
                            GamepadKeyConfig { 
                                first_layer_step_1: None,
                                first_layer_step_2: Some(enigo::Key::Layout('R')),
                                first_layer_step_3: Some(enigo::Key::Layout('P')),
                                first_layer_step_4: Some(enigo::Key::Layout('J')),
                                second_layer_step_1: Some(enigo::Key::Layout('$')),
                                second_layer_step_2: Some(enigo::Key::Layout('<')),
                                second_layer_step_3: Some(enigo::Key::Layout('~')),
                                second_layer_step_4: None,
                            },),
                        CustomButton::Base(gilrs::Button::DPadRight) => Some(
                            GamepadKeyConfig { 
                                first_layer_step_1: Some(enigo::Key::Layout('O')),
                                first_layer_step_2: Some(enigo::Key::Layout('C')),
                                first_layer_step_3: Some(enigo::Key::Layout('B')),
                                first_layer_step_4: None,
                                second_layer_step_1: Some(enigo::Key::Layout('@')),
                                second_layer_step_2: Some(enigo::Key::Layout(')')),
                                second_layer_step_3: Some(enigo::Key::Layout('?')),
                                second_layer_step_4: None,
                            },),
                        CustomButton::Base(gilrs::Button::Unknown) => None,
                        CustomButton::LeftStickUp => Some(
                            GamepadKeyConfig { 
                                first_layer_step_1: Some(enigo::Key::Layout('t')),
                                first_layer_step_2: Some(enigo::Key::Layout('l')),
                                first_layer_step_3: Some(enigo::Key::Layout('w')),
                                first_layer_step_4: Some(enigo::Key::Layout('z')), 
                                second_layer_step_1: Some(enigo::Key::Layout('1')), 
                                second_layer_step_2: Some(enigo::Key::Layout('9')), 
                                second_layer_step_3: Some(enigo::Key::Layout('-')), 
                                second_layer_step_4: None,
                            },),
                        CustomButton::LeftStickDown => Some(
                            GamepadKeyConfig { 
                                first_layer_step_1: Some(enigo::Key::Layout('n')),
                                first_layer_step_2: Some(enigo::Key::Layout('m')),
                                first_layer_step_3: Some(enigo::Key::Layout('k')),
                                first_layer_step_4: None,
                                second_layer_step_1: Some(enigo::Key::Layout('3')), 
                                second_layer_step_2: Some(enigo::Key::Layout('.')), 
                                second_layer_step_3: Some(enigo::Key::Layout('=')), 
                                second_layer_step_4: None,
                            },),
                        CustomButton::LeftStickLeft => Some(
                            GamepadKeyConfig { 
                                first_layer_step_1: Some(enigo::Key::Backspace),
                                first_layer_step_2: Some(enigo::Key::Layout('r')),
                                first_layer_step_3: Some(enigo::Key::Layout('p')),
                                first_layer_step_4: Some(enigo::Key::Layout('j')), 
                                second_layer_step_1: Some(enigo::Key::Layout('4')), 
                                second_layer_step_2: Some(enigo::Key::Layout(',')), 
                                second_layer_step_3: Some(enigo::Key::Layout('`')), 
                                second_layer_step_4: None,
                            },),
                        CustomButton::LeftStickRight => Some(
                            GamepadKeyConfig { 
                                first_layer_step_1: Some(enigo::Key::Layout('o')),
                                first_layer_step_2: Some(enigo::Key::Layout('c')),
                                first_layer_step_3: Some(enigo::Key::Layout('b')),
                                first_layer_step_4: None,
                                second_layer_step_1: Some(enigo::Key::Layout('2')), 
                                second_layer_step_2: Some(enigo::Key::Layout('0')), 
                                second_layer_step_3: Some(enigo::Key::Layout('/')), 
                                second_layer_step_4: None,
                            },),
                        CustomButton::RightStickUp => Some(
                            GamepadKeyConfig { 
                                first_layer_step_1: Some(enigo::Key::Layout('e')),
                                first_layer_step_2: Some(enigo::Key::Layout('h')),
                                first_layer_step_3: Some(enigo::Key::Layout('g')),
                                first_layer_step_4: Some(enigo::Key::Layout('q')), 
                                second_layer_step_1: Some(enigo::Key::Layout('5')), 
                                second_layer_step_2: Some(enigo::Key::Layout('\'')), 
                                second_layer_step_3: Some(enigo::Key::Tab), 
                                second_layer_step_4: None,
                            },),
                        CustomButton::RightStickDown => Some(
                            GamepadKeyConfig { 
                                first_layer_step_1: Some(enigo::Key::Layout('i')),
                                first_layer_step_2: Some(enigo::Key::Layout('u')),
                                first_layer_step_3: Some(enigo::Key::Layout('v')),
                                first_layer_step_4: None,
                                second_layer_step_1: Some(enigo::Key::Layout('7')), 
                                second_layer_step_2: Some(enigo::Key::Layout(']')), 
                                second_layer_step_3: None,
                                second_layer_step_4: None,
                            },),
                        CustomButton::RightStickLeft => Some(
                            GamepadKeyConfig { 
                                first_layer_step_1: Some(enigo::Key::Layout('a')),
                                first_layer_step_2: Some(enigo::Key::Layout('d')),
                                first_layer_step_3: Some(enigo::Key::Layout('y')),
                                first_layer_step_4: None,
                                second_layer_step_1: Some(enigo::Key::Layout('8')), 
                                second_layer_step_2: Some(enigo::Key::Layout('[')), 
                                second_layer_step_3: None,
                                second_layer_step_4: None,
                            },),
                        CustomButton::RightStickRight => Some(
                            GamepadKeyConfig { 
                                first_layer_step_1: Some(enigo::Key::Space),
                                first_layer_step_2: Some(enigo::Key::Layout('s')),
                                first_layer_step_3: Some(enigo::Key::Layout('f')),
                                first_layer_step_4: Some(enigo::Key::Layout('x')), 
                                second_layer_step_1: Some(enigo::Key::Layout('6')), 
                                second_layer_step_2: Some(enigo::Key::Layout(';')), 
                                second_layer_step_3: Some(enigo::Key::Layout('\\')), 
                                second_layer_step_4: None,
                            },),
                        CustomButton::Base(gilrs::Button::C) => None,
                        CustomButton::Base(gilrs::Button::Z) => None,
                        CustomButton::Base(gilrs::Button::LeftTrigger) => {
                            joy_keyboard.set_l1_mod_is_down(true);
                            None
                        },
                        CustomButton::Base(gilrs::Button::LeftTrigger2) => {
                          quick_lookup_window.open();
                          None
                        },
                        CustomButton::Base(gilrs::Button::RightTrigger) => {
                            joy_keyboard.set_r1_mod_is_down(true);
                            None
                        },
                        CustomButton::Base(gilrs::Button::RightTrigger2) => None,
                        CustomButton::Base(gilrs::Button::Select) => None,
                        CustomButton::Base(gilrs::Button::Start) => None,
                        CustomButton::Base(gilrs::Button::Mode) => None,
                        CustomButton::Base(gilrs::Button::LeftThumb) => None,
                        CustomButton::Base(gilrs::Button::RightThumb) => None,
                    };

                    if let Some(key) = key_to_click {
                        joy_keyboard.key_click(key);
                    }
                },
                gamepad::GamepadEvent::ButtonReleased(button) => {
                    print!("ButtonReleased: {:?}\n",button);
                    match button {
                        CustomButton::Base(gilrs::Button::LeftTrigger) => joy_keyboard.set_l1_mod_is_down(false),
                        CustomButton::Base(gilrs::Button::RightTrigger) => joy_keyboard.set_r1_mod_is_down(false),
                        CustomButton::Base(gilrs::Button::LeftTrigger2) => {
                          quick_lookup_window.close();
                        },
                        _other => ()
                    }
                },
            };
        }
    
    }
}
