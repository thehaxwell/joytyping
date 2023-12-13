use std::sync::mpsc::TryRecvError;

use app_data_directory_manager::{AppDataDirectoryManager, AppDataDirectoryDependenciesImpl};
use gamepad_listener::cardinal_levers_move_detector::{CardinalLeversMoveDetector, self};
use gamepad_listener::gilrs_events::GilrsEvents;
use gamepad_listener::gilrs_events::gilrs_wrapper::{GilrsWrapper, GilrsEventType};
use gamepad_listener::gilrs_events::stick_switch_interpreter::{CardinalCustomButtons, StickSwitchInterpreter, AxisClickThresholds, self};
use gamepad_listener::gilrs_events::trigger_2_switch_interpreter::Trigger2SwitchInterpreter;
use gamepad_listener::layers_navigator::LayersNavigator;
use gamepad_listener::layers_wrapper::LayersWrapper;
use input_controller::enigo_wrapper::EnigoWrapper;
use input_controller::keyboard_input_controller::KeyboardInputController;
use input_controller::mouse_input_controller;
use quick_lookup_window::files::FilesDependenciesImpl;
use settings::loader::error_display_window::ErrorDisplayWindow;
use settings::loader::{Settings,SettingsDependenciesImpl};
use notify::{Watcher,RecommendedWatcher, RecursiveMode, Config};
use crate::tauri_app_handle_wrapper::TauriAppHandleWrapper;
use tauri::Manager;

use crate::gamepad_listener::switch_click_pattern_detector::SwitchClickPatternDetector;
use quick_lookup_window::{QuickLookupWindow, QuickLookupWindowTrait};

use tauri::api::notification::Notification;

//TODO: see if we can remove the pub from these
pub mod settings;
pub mod gamepad_listener;
pub mod input_controller;
pub mod quick_lookup_window;
pub mod app_data_directory_manager;
pub mod tauri_app_handle_wrapper;

pub fn start_main_loop(
    handle: tauri::AppHandle,
    tauri_config_bundle_identifier: String,
    ){
    let mut settings_error_display_window = ErrorDisplayWindow::new(
        Box::new(TauriAppHandleWrapper::new(handle.clone())));

    let mut interrupt_event_reciever = MainLoopInterruptionEventReciever::new(handle.clone());

    'main_loop_initializer_loop: loop {

        // // close any open window first while there's time
        // let _ = settings_error_display_window.close();
        // std::thread::sleep(std::time::Duration::from_millis(100));

        let mut settings = Settings::new(
            Box::new(SettingsDependenciesImpl),
            Box::new(AppDataDirectoryManager::new(
                Box::new(AppDataDirectoryDependenciesImpl))),
            );

        let mut settings_data;
        match settings.load_settings() {
            Ok(data) => settings_data = data,
            Err(e) => {
               let _ = settings_error_display_window.open_and_show(e);
               break 'main_loop_initializer_loop;
            }
        }


        let active_profile_index_option = settings_data.profiles.iter()
            .position(|profile| profile.name == settings_data.global.default_profile);
                
        let active_profile = settings_data.profiles.remove(match active_profile_index_option {
            Some(idx) => idx,
            None => 0
        });

        let active_layout: settings::models::layout::Layout;
        match settings.load_layout(active_profile.layout_settings_relative_file_path.clone().into()) {
            Ok(data) => active_layout = data,
            Err(e) => {
               let _ = settings_error_display_window.open_and_show(e);
               break 'main_loop_initializer_loop;
            }
        }

        let (tx, rx) = std::sync::mpsc::channel();
        // NOTE: this next line doesn't work if it's in a nested scope(idk why)
        let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();

        let mut quick_lookup_window 
         = QuickLookupWindow::new(
           Box::new(TauriAppHandleWrapper::new(handle.clone())),
           settings_data.development.clone().and_then(|dev| dev.quick_lookup_window),
           active_layout.quick_lookup_window,
           Box::new(quick_lookup_window::files::Files::new(
              Box::new(FilesDependenciesImpl),
              Box::new(AppDataDirectoryManager::new(
                  Box::new(AppDataDirectoryDependenciesImpl))),
              // returns path if not in development mode
              settings_data.development.map_or(
                 //TODO: this is too much logic to be in lib.
                 //Try put it away in some struct
                 (||{ 
                   let mut path = std::path::PathBuf::from(
                       active_profile.layout_settings_relative_file_path.clone());
                   path.pop(); // remove the file name from the path
                   Some(path)
                 })(),
              |_|None)
           )),
           active_profile.theme,
        );

        if let Err(e) 
            = quick_lookup_window.conditionally_call_watcher(|path|{
                watcher.watch(path, RecursiveMode::Recursive)
            }) {
               let _ = settings_error_display_window.open_and_show(e);
               break 'main_loop_initializer_loop;
            }

        if let Err(e) = quick_lookup_window.load_startup_script() {
           let _ = settings_error_display_window.open_and_show(e);
           break 'main_loop_initializer_loop;
        }
        if let Err(e) = quick_lookup_window.build() {
            println!("Couldn't build quick-lookup-window: {}",e);
        }

        let mut quick_lookup_window_controller = quick_lookup_window::controller
            ::Controller::new(Box::new(quick_lookup_window));

        let mut gamepad_listener = gamepad_listener::GamepadListener::new(
            Box::new(GilrsEvents::new(
                Box::new(GilrsWrapper::new()),
                Box::new(StickSwitchInterpreter::new(
                    AxisClickThresholds::get_from_setting(
                        active_profile.stick_switches.click_thresholds,
                        LeftOrRight::Left),
                    CardinalCustomButtons {
                        up: stick_switch_interpreter::StickSwitchButton::LeftStickUp,
                        down: stick_switch_interpreter::StickSwitchButton::LeftStickDown,
                        left: stick_switch_interpreter::StickSwitchButton::LeftStickLeft,
                        right: stick_switch_interpreter::StickSwitchButton::LeftStickRight,
                    }
                )),
                Box::new(StickSwitchInterpreter::new(
                    AxisClickThresholds::get_from_setting(
                        active_profile.stick_switches.click_thresholds,
                        LeftOrRight::Right),
                    CardinalCustomButtons {
                        up: stick_switch_interpreter::StickSwitchButton::RightStickUp,
                        down: stick_switch_interpreter::StickSwitchButton::RightStickDown,
                        left: stick_switch_interpreter::StickSwitchButton::RightStickLeft,
                        right: stick_switch_interpreter::StickSwitchButton::RightStickRight,
                    }
                )),
                Box::new(Trigger2SwitchInterpreter::new(
                    active_profile.trigger_2_switches.click_thresholds,
                    LeftOrRight::Left,
                )),
                Box::new(Trigger2SwitchInterpreter::new(
                    active_profile.trigger_2_switches.click_thresholds,
                    LeftOrRight::Right,
                )),
            )),
            Box::new(LayersWrapper::new(active_layout.layers.clone(),active_profile.left_upper_is_d_pad)),
            Box::new(SwitchClickPatternDetector::new(active_profile.switch_click_event_thresholds)),
            Box::new(LayersNavigator::new(active_layout.layers,active_profile.left_upper_is_d_pad)),
            Box::new(cardinal_levers_move_detector::mouse::Mouse::new(
                // mouse_cursor_move_detector
                Box::new(CardinalLeversMoveDetector::new(
                    active_profile.stick_cardinal_levers.deadzone_upper_limits,
                    active_profile.stick_cardinal_levers.mouse_controls.cursor_move_scale_factor,
                )),
                // mouse_scroll_detector
                Box::new(CardinalLeversMoveDetector::new(
                    active_profile.stick_cardinal_levers.deadzone_upper_limits,
                    active_profile.stick_cardinal_levers.mouse_controls.scroll_scale_factor,
                )),
            ))
        );

        let mut input_controller = input_controller::InputController::new(
            Box::new(KeyboardInputController::new(Box::new(EnigoWrapper::new()))),
            Box::new(mouse_input_controller::cursor::Cursor::new(Box::new(EnigoWrapper::new()))),
            Box::new(mouse_input_controller::scroll::Scroll::new(Box::new(EnigoWrapper::new()))),
            Box::new(mouse_input_controller::button::Button::new(Box::new(EnigoWrapper::new()))),
        );

        'main_loop: loop {
            // slow this loop down a little
            std::thread::sleep(std::time::Duration::from_millis(10));

            if let Ok(res) = rx.try_recv() {
                match res {
                    Ok(event) => {
                        println!("Restarting at dev event: {:?}",event);
                        break 'main_loop;
                    },
                    Err(e) => println!("watch error: {:?}", e),
                }
            }

            match interrupt_event_reciever.try_recv() {
                Err(_) => (),
                Ok(MainLoopInterruption::ReInitiailze) => {
                    println!("Restarting");
                    break 'main_loop;
                }
                Ok(MainLoopInterruption::Terminate) => {
                    println!("Terminating.");
                    return;
                }
            }

            input_controller.trigger_input();
            loop {
                match gamepad_listener.next_event() {
                    None => break,
                    Some(GilrsEventType::Connected(Some(gamepad_info))) => {
                        let _ = Notification::new(tauri_config_bundle_identifier.clone())
                            .title("Controller connected")
                            .body(format!("\"{}\" controller connected to Joytyping",gamepad_info.name))
                            .show();

                    }
                    Some(_other) => ()
                }
            }

            match gamepad_listener.next_command() {
                Some(gamepad_listener::Command::InputEvent(event))
                    => input_controller.react_to_gamepad_event(event),
                Some(gamepad_listener::Command::QuickLookupWindowEvent(command))
                    => {let _ = quick_lookup_window_controller.react_to_command(command);},
                Some(gamepad_listener::Command::KeyUp(switch)) => {
                    input_controller.react_to_gamepad_event(gamepad_listener::InputEvent::KeyUp);
                    let _ = quick_lookup_window_controller.react_to_switch_keyup(switch);
                },
                None => (),
            }
        }
    }

    // give a chance to save this loop. If the
    // main_loop_initializer_loop breaks(but not from
    // MainLoopInterruption::Terminate), it can
    // still be restarted with MainLoopInterruption::ReInitiailze
    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
        match interrupt_event_reciever.try_recv() {
            Ok(MainLoopInterruption::ReInitiailze) => {
                start_main_loop(handle.clone(),tauri_config_bundle_identifier);
                break;
            }
            _other => (),
        }
    }
    interrupt_event_reciever.destruct();
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

struct MainLoopInterruptionEventReciever {
    reciever: std::sync::mpsc::Receiver<MainLoopInterruption>,
    handle_id: tauri::EventHandler,
    app_handle: tauri::AppHandle,
}

impl MainLoopInterruptionEventReciever {
    fn new(app_handle: tauri::AppHandle) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();

       let handle_id = app_handle.listen_global("main-loop-interruption", move |event| {
            match event.payload() {
                Some("MainLoopInterruption::ReInitiailze") => {
                    let _ = tx.send(MainLoopInterruption::ReInitiailze);
                },
                Some("MainLoopInterruption::Terminate") => {
                    let _ = tx.send(MainLoopInterruption::Terminate);
                }
                Some(other) => panic!("{}",other),
                _ => (),
            };
        });

        Self {
            reciever: rx,
            handle_id,
            app_handle,
        }
    }

    fn destruct(&self) {
        self.app_handle.unlisten(self.handle_id);
    }

    fn try_recv(&mut self) -> Result<MainLoopInterruption,TryRecvError> {
        self.reciever.try_recv()
    }
}
