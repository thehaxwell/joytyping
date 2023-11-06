use std::sync::mpsc;

use app_data_directory_manager::{AppDataDirectoryManager, AppDataDirectoryDependenciesImpl};
use gamepad::cardinal_levers_move_detector::{CardinalLeversMoveDetector, self};
use gamepad::gilrs_events::GilrsEvents;
use gamepad::gilrs_events::gilrs_wrapper::GilrsWrapper;
use gamepad::gilrs_events::stick_switch_interpreter::{CardinalCustomButtons, StickSwitchInterpreter, AxisClickThresholds, self};
use gamepad::gilrs_events::trigger_2_switch_interpreter::Trigger2SwitchInterpreter;
use gamepad::layers_navigator::LayersNavigator;
use gamepad::layers_wrapper::LayersWrapper;
use input_controller::enigo_wrapper::EnigoWrapper;
use input_controller::keyboard_input_controller::KeyboardInputController;
use input_controller::mouse_input_controller;
use settings::error_display_window::ErrorDisplayWindow;
use settings::{Settings,SettingsDependenciesImpl};
use notify::{Watcher,RecommendedWatcher, RecursiveMode, Config};
use crate::tauri_app_handle_wrapper::TauriAppHandleWrapper;

use crate::gamepad::switch_click_pattern_detector::SwitchClickPatternDetector;
use quick_lookup_window::{QuickLookupWindow, QuickLookupWindowDependenciesImpl};

//TODO: see if we can remove the pub from these
pub mod settings;
pub mod gamepad;
pub mod input_controller;
pub mod quick_lookup_window;
pub mod app_data_directory_manager;
pub mod tauri_app_handle_wrapper;

pub fn start_main_loop(
    end_signal_mpsc_receiver: mpsc::Receiver<MainLoopInterruption>,
    handle: tauri::AppHandle
    ){
    let mut settings_error_display_window = ErrorDisplayWindow::new(
        Box::new(TauriAppHandleWrapper::new(handle.clone())));


    'main_loop_initializer_loop: loop {
        // close any open window first while there's time
        let _ = settings_error_display_window.close();

        let mut settings = Settings::new(
            Box::new(SettingsDependenciesImpl),
            // "/home/haxwell/.config/joytyping/joytyping.toml".to_string(),
            Box::new(AppDataDirectoryManager::new(
                Box::new(AppDataDirectoryDependenciesImpl))),
            );

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

        // assert_eq!(
        //     active_profile.layers.remove(0).switches.unwrap().east.unwrap().on_click.unwrap().keyboard.unwrap().key,
        //     enigo::Key::Return);

        // assert_eq!(
        //     active_profile.layers.remove(7).switches.unwrap().south.unwrap().on_click.unwrap().mouse.unwrap().button,
        //     enigo::MouseButton::Left);

        // assert_eq!(
        //     active_profile.layers.remove(0).switches.unwrap().left_trigger.unwrap().on_click_and_hold.unwrap().visit_layer.unwrap(),
        //     settings_data::LayerSpecifier{name: "first-layer-step-3".to_string(), pointer: None});
        
        // assert_eq!(
        //     active_profile.clone().layers.remove(7).cardinal_levers.unwrap().left_stick.unwrap(),
        //     settings_data::SingleCardinalLever::ControlMouseCursor(
        //         settings_data::ControlMouseCursorFunction{
        //             center_at: settings_data::ControlMouseCursorCenterCoordinates{x: 0.0, y: 0.0}}));
        //
        // assert_eq!(
        //     active_profile.clone().layers.remove(7).cardinal_levers.unwrap().right_stick.unwrap(),
        //     settings_data::SingleCardinalLever::ControlMouseScrollwheel(
        //         settings_data::ControlMouseScrollwheelFunction{
        //             center_at_y: 0.0}));

        let (tx, rx) = std::sync::mpsc::channel();
        // NOTE: this next line doesn't work if it's in a nested scope(idk why)
        let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();

        let tauri_app_handle_wrapper = TauriAppHandleWrapper::new(handle.clone());


        let mut quick_lookup_window 
         = QuickLookupWindow::new(
            Box::new(tauri_app_handle_wrapper),
            Box::new(QuickLookupWindowDependenciesImpl),
            if let Some(dev_quick_lookup_window_settings) 
                = settings_data.development.and_then(|dev| dev.quick_lookup_window){
        
                if let Some(source_code) = &dev_quick_lookup_window_settings.source_code {
                    watcher
                        .watch(source_code.js_iife_bundle_file_path.as_ref(), RecursiveMode::Recursive)
                        .unwrap(); // TODO: handle these errors
                }
                dev_quick_lookup_window_settings
            } else {
                active_profile.quick_lookup_window
            },
            Box::new(AppDataDirectoryManager::new(
                Box::new(AppDataDirectoryDependenciesImpl))),
        );
        
        if let Err(e) = quick_lookup_window.load_startup_script() {
           let _ = settings_error_display_window.open_and_show(e);
        }

        let mut gamepad = gamepad::Gamepad::new(
            Box::new(GilrsEvents::new(
                Box::new(GilrsWrapper::new()),
                Box::new(StickSwitchInterpreter::new(
                    AxisClickThresholds::get_from_setting(
                        active_profile.stick_switches_click_thresholds,
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
                        active_profile.stick_switches_click_thresholds,
                        LeftOrRight::Right),
                    CardinalCustomButtons {
                        up: stick_switch_interpreter::StickSwitchButton::RightStickUp,
                        down: stick_switch_interpreter::StickSwitchButton::RightStickDown,
                        left: stick_switch_interpreter::StickSwitchButton::RightStickLeft,
                        right: stick_switch_interpreter::StickSwitchButton::RightStickRight,
                    }
                )),
                Box::new(Trigger2SwitchInterpreter::new(
                    active_profile.trigger_2_switches_click_thresholds,
                    LeftOrRight::Left,
                )),
                Box::new(Trigger2SwitchInterpreter::new(
                    active_profile.trigger_2_switches_click_thresholds,
                    LeftOrRight::Right,
                )),
            )),
            Box::new(LayersWrapper::new(active_profile.layers.clone(),active_profile.left_upper_is_d_pad)),
            Box::new(SwitchClickPatternDetector::new()),
            Box::new(LayersNavigator::new(active_profile.layers,active_profile.left_upper_is_d_pad)),
            Box::new(quick_lookup_window),
            Box::new(cardinal_levers_move_detector::mouse::Mouse::new(
                Box::new(CardinalLeversMoveDetector::new()),
                Box::new(CardinalLeversMoveDetector::new()),
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

            input_controller.trigger_input();
            while gamepad.next_event() {}
            if let Some(event) = gamepad.tick() {
                input_controller.react_to_gamepad_event(event);
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
