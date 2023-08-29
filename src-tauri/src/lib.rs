use std::sync::mpsc;

use joy_keyboard::input_controller::InputController;
use joy_keyboard::keys_config::KeysConfig;
use settings::error_display_window::ErrorDisplayWindow;
use settings::{Settings,SettingsDependenciesImpl};
use joy_keyboard::stepper::StepperButton;
use gamepad::gilrs_wrapper::GilrsWrapper;
use gamepad::sticks_interpreter::{SticksInterpreter, AxisClickThresholds};
use joy_keyboard::input_controller::enigo_wrapper::EnigoWrapper;
use quick_lookup_window::{QuickLookupWindow, QuickLookupWindowDependenciesImpl};
    use gamepad::CustomButton;

pub mod joy_keyboard;
pub mod gamepad;
pub mod quick_lookup_window;
pub mod settings;
pub mod settings_data;

pub fn start_main_loop(
    end_signal_mpsc_receiver: mpsc::Receiver<MainLoopInterruption>,
    handle: tauri::AppHandle
    ){
    let mut quick_lookup_window = QuickLookupWindow::new(
        handle.clone(),
        Box::new(QuickLookupWindowDependenciesImpl),
    );
    let mut settings_error_display_window = ErrorDisplayWindow::new(handle);

    'main_loop_initializer_loop: loop {
        // close any open window first while there's time
        let _ = settings_error_display_window.close();

        let mut settings = Settings::new(
            Box::new(SettingsDependenciesImpl),
            "/home/haxwell/.config/joytyping/joytyping.toml".to_string());

        if let Err(e) = settings.load() {
           let _ = settings_error_display_window.open_and_show(e);
        }

        let mut settings_data = settings.get_data().unwrap();

        let active_profile_index_option = settings_data.profiles.iter()
            .position(|profile| profile.name == settings_data.global.default_profile);
                
        let active_profile = settings_data.profiles.remove(match active_profile_index_option {
            Some(idx) => idx,
            None => 0
        });

        let mut gamepad = gamepad::Gamepad::new(
            Box::new(GilrsWrapper::new()),
            Box::new(SticksInterpreter::new(
            AxisClickThresholds::get_from_setting(
                active_profile.left_stick.click_thresholds,
                LeftOrRight::Left),
            AxisClickThresholds::get_from_setting(
                active_profile.right_stick.click_thresholds,
                LeftOrRight::Right),
                
            )),
        );
        let mut joy_keyboard = joy_keyboard::JoyKeyboard::new(
            Box::new(InputController::new(Box::new(EnigoWrapper::new()))),
            Box::new(StepperButton::new()),
            Box::new(StepperButton::new()),
            KeysConfig::from(
                active_profile.keyboard_mode.key_mappings)
        );


        quick_lookup_window.set_window_settings(active_profile.quick_lookup_window);
        match quick_lookup_window.load_startup_script() {
            Err(e) => {
                match e {
                    _ => {
                        println!("Error!");
                    }
                }
            },
            Ok(_) => {
                println!("quick lookup window external script");
            }
        }

        'main_loop: loop {
            //TODO: check if this actually eases the load on the CPU
            std::thread::sleep(std::time::Duration::from_millis(10));
            match end_signal_mpsc_receiver.try_recv() {
                Ok(MainLoopInterruption::ReInitiailze) => {
                    println!("Restarting");
                    break 'main_loop;
                }
                Ok(MainLoopInterruption::Terminate) | Err(mpsc::TryRecvError::Disconnected) => {
                    println!("Terminating.");
                    break 'main_loop_initializer_loop;
                }
                Err(mpsc::TryRecvError::Empty) => {}
            }

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
}

#[derive(Debug,PartialEq,Clone)]
pub enum LeftOrRight {
    Left,
    Right,
}

#[derive(PartialEq,Clone)]
pub enum MainLoopInterruption {
    Terminate,
    ReInitiailze,
}
