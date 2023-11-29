use crate::models::{layout::*, BrowserSourceCode, HeightAndWidth};
use enigo::{Key, MouseButton};

pub struct SetupConfig {
    pub include_index_in_gamepad: bool,
    pub my_ps3_controller_first_layer_step_1_left_trigger_on_double_click_visit_layer_name: String,
    pub my_ps3_controller_quick_lookup_window_inner_size_height: f64,
    pub my_ps3_controller_quick_lookup_window_inner_size_width: f64,
    pub my_ps3_controller_cardinal_levers_second_layer_step_4_right_stick_control_mouse_scrollwheel_scale_factor: f32,
}

pub fn setup_example(config: SetupConfig) -> Layout {
    Layout{
        quick_lookup_window: QuickLookupWindow{
            inner_size: HeightAndWidth{
                height: config.my_ps3_controller_quick_lookup_window_inner_size_height,
                width: config.my_ps3_controller_quick_lookup_window_inner_size_width,
            },
            source_code: BrowserSourceCode{
                js_iife_bundle_file_path: "path/to/file/bundle.js".to_string(),
                css_file_path: None,
            }
        },
        layers: vec![
            Layer {
                id: "first-layer-step-1".to_string(),
                switches: Some(SwitchesAdapter {
                    right_upper_south: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('I'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_upper_east: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Return,
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_upper_north: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('E'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_upper_west: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('A'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_upper_north: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('T'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_upper_south: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('N'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_upper_west: None,
                    left_upper_east: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('O'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_lower_north: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('t'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_lower_south: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('n'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_lower_west: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Backspace,
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_lower_east: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('o'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_lower_north: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('e'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_lower_south: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('i'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_lower_west: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('a'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_lower_east: Some(SwitchEventAndReaction {
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
                switches: Some(SwitchesAdapter {
                    right_upper_south: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('U'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_upper_east: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('S'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_upper_north: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('H'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_upper_west: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('D'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_upper_north: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('L'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_upper_south: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('M'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_upper_west: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('R'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_upper_east: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('C'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_lower_north: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('l'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_lower_south: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('m'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_lower_west: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('r'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_lower_east: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('c'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_lower_north: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('h'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_lower_south: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('u'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_lower_west: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('d'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_lower_east: Some(SwitchEventAndReaction {
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
                switches: Some(SwitchesAdapter {
                    right_upper_south: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('V'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_upper_east: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('F'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_upper_north: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('G'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_upper_west: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('Y'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_upper_north: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('W'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_upper_south: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('K'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_upper_west: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('P'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_upper_east: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('B'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_lower_north: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('w'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_lower_south: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('k'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_lower_west: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('p'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_lower_east: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('b'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_lower_north: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('g'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_lower_south: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('v'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_lower_west: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('y'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_lower_east: Some(SwitchEventAndReaction {
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
                switches: Some(SwitchesAdapter {
                    right_upper_south: None,
                    right_upper_east: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('X'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_upper_north: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('Q'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_upper_west: None,
                    left_upper_north: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('Z'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_upper_south: None,
                    left_upper_west: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('J'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_upper_east: None,
                    left_lower_north: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('z'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_lower_south: None,
                    left_lower_west: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('j'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_lower_east: None,
                    right_lower_north: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('q'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_lower_south: None,
                    right_lower_west: None,
                    right_lower_east: Some(SwitchEventAndReaction {
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
                switches: Some(SwitchesAdapter {
                    right_upper_south: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('&'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_upper_east: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('^'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_upper_north: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('%'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_upper_west: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('*'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_upper_north: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('!'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_upper_south: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('#'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_upper_west: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('$'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_upper_east: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('@'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_lower_north: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('1'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_lower_south: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('3'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_lower_west: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('4'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_lower_east: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('2'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_lower_north: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('5'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_lower_south: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('7'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_lower_west: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('8'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_lower_east: Some(SwitchEventAndReaction {
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
                switches: Some(SwitchesAdapter {
                    right_upper_south: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('}'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_upper_east: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout(':'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_upper_north: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('"'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_upper_west: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('{'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_upper_north: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('('),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_upper_south: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('>'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_upper_west: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('<'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_upper_east: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('('),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_lower_north: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('9'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_lower_south: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('.'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_lower_west: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout(','),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_lower_east: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('0'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_lower_north: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('\''),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_lower_south: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout(']'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_lower_west: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('['),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_lower_east: Some(SwitchEventAndReaction {
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
                switches: Some(SwitchesAdapter {
                    right_upper_south: None,
                    right_upper_east: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('|'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_upper_north: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Tab,
                            modifiers: vec![
                                Key::Shift
                            ]})),
                        on_double_click: None, 
                    }),
                    right_upper_west: None,
                    left_upper_north: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('_'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_upper_south: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('+'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_upper_west: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('~'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_upper_east: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('?'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_lower_north: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('-'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_lower_south: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('='),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_lower_west: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('`'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    left_lower_east: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Layout('/'),
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_lower_north: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                            key: Key::Tab,
                            modifiers: vec![
                            ]})),
                        on_double_click: None, 
                    }),
                    right_lower_south: None,
                    right_lower_west: None,
                    right_lower_east: Some(SwitchEventAndReaction {
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
                switches: Some(SwitchesAdapter {
                    right_upper_south: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Mouse(
                            MouseInput{button: MouseButton::Left})),
                        on_double_click: None, 
                    }),
                    right_upper_east: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::Mouse(
                            MouseInput{button: MouseButton::Right})),
                        on_double_click: None, 
                    }),
                    right_upper_north: None,
                    right_upper_west: None,
                    left_upper_north: None,
                    left_upper_south: None,
                    left_upper_west: None,
                    left_upper_east: None,
                    left_lower_north: None,
                    left_lower_south: None,
                    left_lower_west: None,
                    left_lower_east: None,
                    right_lower_north: None,
                    right_lower_south: None,
                    right_lower_west: None,
                    right_lower_east: None,
                    right_trigger: None,
                    left_trigger: None,
                    left_trigger_2: None,
                    right_trigger_2: None,
                }),
                cardinal_levers: Some(CardinalLevers{
                    left_stick: Some(SingleCardinalLever::ControlMouseCursor(
                        MouseControl {
                            scale_factor: 10.0,
                        })),
                    right_stick: Some(SingleCardinalLever::ControlMouseScrollwheel(
                        MouseControl {
                            scale_factor:
                                config.my_ps3_controller_cardinal_levers_second_layer_step_4_right_stick_control_mouse_scrollwheel_scale_factor,
                        })),
                })
            },

      ]
    }
}
