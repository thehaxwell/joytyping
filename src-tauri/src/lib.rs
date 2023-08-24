use quick_lookup_window::QuickLookupWindow;

pub mod joy_keyboard;
pub mod gamepad;
pub mod quick_lookup_window;
pub mod settings;
pub mod settings_data;

pub fn run(mut gamepad: gamepad::Gamepad, mut joy_keyboard: joy_keyboard::JoyKeyboard, mut quick_lookup_window: QuickLookupWindow,
           ) {
    use gamepad::CustomButton;
    //TODO: check if this actually eases the load on the CPU
    let loop_sleep_duration = std::time::Duration::from_millis(10);
    loop {
        std::thread::sleep(loop_sleep_duration);
        joy_keyboard.trigger_input();
        while let Some(event) = gamepad.next_event() {
            match event {
                gamepad::GamepadEvent::ButtonPressed(button)=> {
                print!("ButtonPressed: {:?}\n",button);
                    match button{
                        CustomButton::Base(gilrs::Button::LeftTrigger) => {
                            joy_keyboard.set_l1_mod_is_down(true);
                            let _ = quick_lookup_window.update_keyboard(
                                joy_keyboard.get_current_layer(),
                                joy_keyboard.get_current_step()
                            );
                        },
                        CustomButton::Base(gilrs::Button::LeftTrigger2) => {
                          // quick_lookup_window.open();
                          let _ = quick_lookup_window.show_or_open();
                        },
                        CustomButton::Base(gilrs::Button::RightTrigger) => {
                            joy_keyboard.set_r1_mod_is_down(true);
                            let _ = quick_lookup_window.update_keyboard(
                                joy_keyboard.get_current_layer(),
                                joy_keyboard.get_current_step()
                            );
                        },
                        _ => {
                            joy_keyboard.button_pressed(button);
                        },
                    };

                },
                gamepad::GamepadEvent::ButtonReleased(button) => {
                    print!("ButtonReleased: {:?}\n",button);
                    match button {
                        CustomButton::Base(gilrs::Button::LeftTrigger) => {
                            joy_keyboard.set_l1_mod_is_down(false);
                            let _ = quick_lookup_window.update_keyboard(
                                joy_keyboard.get_current_layer(),
                                joy_keyboard.get_current_step()
                            );
                        },
                        CustomButton::Base(gilrs::Button::RightTrigger) => {
                            joy_keyboard.set_r1_mod_is_down(false);
                            let _ = quick_lookup_window.update_keyboard(
                                joy_keyboard.get_current_layer(),
                                joy_keyboard.get_current_step()
                            );
                        },
                        CustomButton::Base(gilrs::Button::LeftTrigger2) => {
                          let _ = quick_lookup_window.hide();
                        },
                        _ => {
                            joy_keyboard.button_released();
                        },
                    }
                },
            };
        }
    
    }
}

#[derive(PartialEq,Clone)]
pub enum LeftOrRight {
    Left,
    Right,
}
