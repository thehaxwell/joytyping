use std::sync::mpsc;

use gamepad::gilrs_wrapper::GilrsWrapper;
use gamepad::stick_switch_interpreter::{CardinalCustomButtons, StickSwitchInterpreter, AxisClickThresholds, self};
use input_controller::{KeyboardInputController,KeyboardInputControllerTrait};
use input_controller::enigo_wrapper::{EnigoWrapper, EnigoTrait};
use settings::error_display_window::ErrorDisplayWindow;
use settings::{Settings,SettingsDependenciesImpl};

use crate::input_controller::{MouseInputController, MouseInputControllerTrait};

pub mod settings;
pub mod settings_data;
pub mod gamepad;
pub mod input_controller;

pub fn start_main_loop(
    end_signal_mpsc_receiver: mpsc::Receiver<MainLoopInterruption>,
    handle: tauri::AppHandle
    ){
    let mut settings_error_display_window = ErrorDisplayWindow::new(handle);

    'main_loop_initializer_loop: loop {
        // close any open window first while there's time
        let _ = settings_error_display_window.close();

        let mut settings = Settings::new(
            Box::new(SettingsDependenciesImpl),
            "/home/haxwell/.config/joytyping/joytyping1.toml".to_string());

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

        fn new_switch_click_pattern_detector()-> Box<dyn
            gamepad::layer_node
            ::switch_click_pattern_detector
            ::SwitchClickPatternDetectorTrait>{
            Box::new(gamepad::layer_node
               ::switch_click_pattern_detector::SwitchClickPatternDetector::new())
        }
        fn new_layer_node(
            source: settings_data::Layer,
            pointers: &Vec<gamepad::layer_node::LayerNodeRef>,
            switch_click_pattern_detector: Option<Box<dyn gamepad::layer_node
               ::switch_click_pattern_detector::SwitchClickPatternDetectorTrait>>,
        ) -> gamepad::layer_node::LayerNode {
            gamepad::layer_node::LayerNode::new(
                source,
                pointers,
                switch_click_pattern_detector,
            )
        }

        let mut gamepad = gamepad::Gamepad::new(
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
            active_profile.layers,
            new_layer_node,
            new_switch_click_pattern_detector,
        );

        let mut keyboard_input_controller = KeyboardInputController::new(Box::new(EnigoWrapper::new()));
        // let mut mouse_input_controller = MouseInputController::new(Box::new(EnigoWrapper::new()));

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

            keyboard_input_controller.trigger_input();
            // mouse_input_controller.trigger_input();
            while gamepad.next_event() {}
            match gamepad.tick() {
                Some(gamepad::layer_node::InputEvent::KeyClick(key))
                => {
                    keyboard_input_controller.key_up();
                    keyboard_input_controller.key_click(key);
                }
                Some(gamepad::layer_node::InputEvent::KeyDown(key))
                => {
                    keyboard_input_controller.key_up();
                    keyboard_input_controller.key_down(key);
                }
                Some(gamepad::layer_node::InputEvent::KeyUp)
                => {
                    keyboard_input_controller.key_up();
                }
                None => (),
            };
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
