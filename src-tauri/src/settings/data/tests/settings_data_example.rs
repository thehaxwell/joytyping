use crate::settings::data::*;
use enigo::{Key, MouseButton};

pub struct SetupConfig {
    pub include_index_in_gamepad: bool,
    pub my_ps3_controller_first_layer_step_1_left_trigger_on_double_click_visit_layer_name: String,
}

pub fn setup_settings_data_example(config: SetupConfig) -> SettingsData {
    let ps4_controller_profile = Profile{
        name: "My PS3 Controller".to_string(),
        quick_lookup_window: QuickLookupWindow{
            inner_size: HeightAndWidth{
                height: 100.0,
                width: 100.0,
            },
        },
        stick_switches_click_thresholds: StickSwitchesClickThresholds {
            left_stick_up: 0.5,
            left_stick_down: 0.5,
            left_stick_left: 0.5,
            left_stick_right: 0.5,
            right_stick_up: 0.5,
            right_stick_down: 0.5,
            right_stick_left: 0.5,
            right_stick_right: 0.5
        },
        layers: vec![
            Layer {
                id: "first-layer-step-1".to_string(),
                switches: Some(Switches {
                    south: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('I'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    east: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Return,
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    north: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('E'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    west: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('A'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    d_pad_up: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('T'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    d_pad_down: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('N'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    d_pad_left: None,
                    d_pad_right: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('O'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_stick_up: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('t'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_stick_down: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('n'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_stick_left: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Backspace,
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_stick_right: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('o'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_stick_up: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('e'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_stick_down: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('i'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_stick_left: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('a'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_stick_right: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Space,
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_trigger: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::VisitLayer(LayerSpecifier {
                            id: "first-layer-step-2".to_string(),
                            index_in_gamepad: 
                                if config.include_index_in_gamepad { Some(1) } else { None },
                        })),
                        on_double_click: Some(SwitchOnClickReaction::MoveToOrVisitLayer(LayerSpecifier {
                            id: "second-layer-step-1".to_string(),
                            index_in_gamepad: 
                                if config.include_index_in_gamepad { Some(4) } else { None },
                        })), 
                    }),
                    left_trigger: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::VisitLayer(LayerSpecifier {
                            id: "first-layer-step-3".to_string(),
                            index_in_gamepad: 
                                if config.include_index_in_gamepad { Some(2) } else { None },
                        })),
                        on_double_click: Some(SwitchOnClickReaction::MoveToOrVisitLayer(LayerSpecifier {
                            id: config
                                .my_ps3_controller_first_layer_step_1_left_trigger_on_double_click_visit_layer_name,
                            index_in_gamepad:
                                if config.include_index_in_gamepad { Some(4) } else { None },
                        })), 
                    }),
                    left_trigger_2: None,
                    right_trigger_2: None,
                }),
                cardinal_levers: None,
            },
            Layer {
                id: "first-layer-step-2".to_string(),
                switches: Some(Switches {
                    south: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('U'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    east: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('S'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    north: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('H'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    west: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('D'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    d_pad_up: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('L'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    d_pad_down: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('M'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    d_pad_left: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('R'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    d_pad_right: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('C'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_stick_up: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('l'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_stick_down: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('m'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_stick_left: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('r'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_stick_right: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('c'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_stick_up: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('h'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_stick_down: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('u'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_stick_left: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('d'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_stick_right: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('s'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_trigger: None,
                    left_trigger: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::VisitLayer(LayerSpecifier {
                            id: "first-layer-step-4".to_string(),
                            index_in_gamepad: 
                                if config.include_index_in_gamepad { Some(3) } else { None },
                        })),
                        on_double_click: None, 
                    }),
                    left_trigger_2: None,
                    right_trigger_2: None,
                }),
                cardinal_levers: None,
            },
            Layer {
                id: "first-layer-step-3".to_string(),
                switches: Some(Switches {
                    south: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('V'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    east: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('F'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    north: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('G'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    west: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('Y'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    d_pad_up: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('W'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    d_pad_down: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('K'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    d_pad_left: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('P'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    d_pad_right: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('B'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_stick_up: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('w'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_stick_down: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('k'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_stick_left: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('p'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_stick_right: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('b'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_stick_up: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('g'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_stick_down: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('v'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_stick_left: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('y'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_stick_right: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('f'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_trigger: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::VisitLayer(LayerSpecifier { 
                            id: "first-layer-step-4".to_string(),
                            index_in_gamepad:
                                if config.include_index_in_gamepad { Some(3) } else { None },
                        })),
                        on_double_click: None, 
                    }),
                    left_trigger: None,
                    left_trigger_2: None,
                    right_trigger_2: None,
                }),
                cardinal_levers: None,
            },
            Layer {
                id: "first-layer-step-4".to_string(),
                switches: Some(Switches {
                    south: None,
                    east: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('X'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    north: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('Q'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    west: None,
                    d_pad_up: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('Z'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    d_pad_down: None,
                    d_pad_left: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('J'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    d_pad_right: None,
                    left_stick_up: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('z'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_stick_down: None,
                    left_stick_left: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('j'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_stick_right: None,
                    right_stick_up: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('q'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_stick_down: None,
                    right_stick_left: None,
                    right_stick_right: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('x'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_trigger: None,
                    left_trigger: None,
                    left_trigger_2: None,
                    right_trigger_2: None,
                }),
                cardinal_levers: None,
            },
            Layer {
                id: "second-layer-step-1".to_string(),
                switches: Some(Switches {
                    south: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('&'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    east: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('^'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    north: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('%'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    west: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('*'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    d_pad_up: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('!'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    d_pad_down: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('#'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    d_pad_left: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('$'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    d_pad_right: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('@'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_stick_up: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('1'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_stick_down: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('3'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_stick_left: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('4'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_stick_right: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('2'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_stick_up: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('5'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_stick_down: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('7'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_stick_left: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('8'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_stick_right: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('6'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_trigger: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::VisitLayer(LayerSpecifier {
                            id: "second-layer-step-2".to_string(),
                            index_in_gamepad: 
                                if config.include_index_in_gamepad { Some(5) } else { None },
                        })),
                        on_double_click: Some(SwitchOnClickReaction::MoveToOrVisitLayer(LayerSpecifier {
                            id: "first-layer-step-1".to_string(),
                            index_in_gamepad:
                                if config.include_index_in_gamepad { Some(0) } else { None },
                        })), 
                    }),
                    left_trigger: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::VisitLayer(LayerSpecifier {
                            id: "second-layer-step-3".to_string(),
                            index_in_gamepad:
                                if config.include_index_in_gamepad { Some(6) } else { None },
                        })),
                        on_double_click: Some(SwitchOnClickReaction::MoveToOrVisitLayer(LayerSpecifier {
                            id: "first-layer-step-1".to_string(),
                            index_in_gamepad:
                                if config.include_index_in_gamepad { Some(0) } else { None },
                        })), 
                    }),
                    left_trigger_2: None,
                    right_trigger_2: None,
                }),
                cardinal_levers: None,
            },
            Layer {
                id: "second-layer-step-2".to_string(),
                switches: Some(Switches {
                    south: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('}'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    east: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout(':'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    north: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('"'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    west: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('{'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    d_pad_up: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('('),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    d_pad_down: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('>'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    d_pad_left: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('<'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    d_pad_right: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('('),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_stick_up: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('9'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_stick_down: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('.'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_stick_left: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout(','),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_stick_right: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('0'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_stick_up: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('\''),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_stick_down: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout(']'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_stick_left: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('['),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_stick_right: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout(';'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_trigger: None,
                    left_trigger: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::VisitLayer(LayerSpecifier {
                            id: "second-layer-step-4".to_string(),
                            index_in_gamepad:
                                if config.include_index_in_gamepad { Some(7) } else { None },
                        })),
                        on_double_click: None, 
                    }),
                    left_trigger_2: None,
                    right_trigger_2: None,
                }),
                cardinal_levers: None,
            },
            Layer {
                id: "second-layer-step-3".to_string(),
                switches: Some(Switches {
                    south: None,
                    east: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('|'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    north: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Tab,
                            modifiers: vec![
                                Key::Shift
                            ]})),
                        on_double_click: None, 
                    }),
                    west: None,
                    d_pad_up: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('_'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    d_pad_down: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('+'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    d_pad_left: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('~'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    d_pad_right: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('?'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_stick_up: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('-'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_stick_down: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('='),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_stick_left: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('`'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_stick_right: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('/'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_stick_up: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Tab,
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_stick_down: None,
                    right_stick_left: None,
                    right_stick_right: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('\\'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_trigger: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::VisitLayer(LayerSpecifier{
                        id: "second-layer-step-4".to_string(),
                        index_in_gamepad:
                            if config.include_index_in_gamepad { Some(7) } else { None },
                        })),
                        on_double_click: None, 
                    }),
                    left_trigger: None,
                    left_trigger_2: None,
                    right_trigger_2: None,
                }),
                cardinal_levers: None,
            },
            Layer {
                id: "second-layer-step-4".to_string(),
                switches: Some(Switches {
                    south: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Mouse(
                            MouseInput{button: MouseButton::Left})),
                        on_double_click: None, 
                    }),
                    east: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Mouse(
                            MouseInput{button: MouseButton::Right})),
                        on_double_click: None, 
                    }),
                    north: None,
                    west: None,
                    d_pad_up: None,
                    d_pad_down: None,
                    d_pad_left: None,
                    d_pad_right: None,
                    left_stick_up: None,
                    left_stick_down: None,
                    left_stick_left: None,
                    left_stick_right: None,
                    right_stick_up: None,
                    right_stick_down: None,
                    right_stick_left: None,
                    right_stick_right: None,
                    right_trigger: None,
                    left_trigger: None,
                    left_trigger_2: None,
                    right_trigger_2: None,
                }),
                cardinal_levers: Some(CardinalLevers{
                    left_stick: Some(SingleCardinalLever::ControlMouseCursor(
                        MouseControl {
                            deadzone_upper_limit: 0.0,
                            scale_factor: 10.0,
                        })),
                    right_stick: Some(SingleCardinalLever::ControlMouseScrollwheel(
                        MouseControl {
                            deadzone_upper_limit: 0.0,
                            scale_factor: 10.0,
                        })),
                })
            },

      ]
    };

    SettingsData {
        profiles: vec![ps4_controller_profile],
        global: Global{default_profile:"My PS3 Controller".to_string()} 
    }
}
