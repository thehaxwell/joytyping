use enigo::Key;

use self::setup_example::setup_example;

use self::setup_example::SetupConfig;

use super::*;

mod setup_example;

fn setup_settings_data_toggle_index_in_gamepad(include_index_in_gamepad: bool) -> Layout {
    setup_example(SetupConfig {
       include_index_in_gamepad,
       my_ps3_controller_first_layer_step_1_left_trigger_on_double_click_visit_layer_name:
           "second-layer-step-1".to_string(),
        my_ps3_controller_quick_lookup_window_inner_size_height: 100.0,
        my_ps3_controller_quick_lookup_window_inner_size_width: 100.0,
    })
}

#[test]
fn settings_data_validate_and_clone_and_set_layer_pointers_works() {
    assert_eq!(
        setup_settings_data_toggle_index_in_gamepad(false)
            .validate_and_clone_and_set_layer_pointers().unwrap(), 
            setup_settings_data_toggle_index_in_gamepad(true));
}

#[test]
fn settings_data_validate_and_clone_and_set_layer_pointers_gives_the_right_error() {
    let res = setup_example(SetupConfig {
       include_index_in_gamepad: false,
       my_ps3_controller_first_layer_step_1_left_trigger_on_double_click_visit_layer_name:
           "seconda-layer-step-1".to_string(),
       my_ps3_controller_quick_lookup_window_inner_size_height: 100.0,
       my_ps3_controller_quick_lookup_window_inner_size_width: 100.0,
    }).validate_and_clone_and_set_layer_pointers();

    assert_eq!(
        res.unwrap_err(),
        format!("{}\n{}\n{}\n{}\n{}\n{}\n{}",
        "Error in",
        "> layers: \"first-layer-step-1\"",
        "   > switches",
        "      > left_trigger",
        "         > on_double_click",
        "            > move_to_or_visit_layer",
        "No layer found having the id \"seconda-layer-step-1\"",));


    let res = setup_example(SetupConfig {
       include_index_in_gamepad: false,
       my_ps3_controller_first_layer_step_1_left_trigger_on_double_click_visit_layer_name:
           "second-layer-step-1".to_string(),
       my_ps3_controller_quick_lookup_window_inner_size_height: 100.0,
       my_ps3_controller_quick_lookup_window_inner_size_width: -100.0,
    }).validate_and_clone_and_set_layer_pointers();

    assert_eq!(
        res.unwrap_err(),
        format!("{}\n{}\n{}\n{}\n{}",
        "Error in",
        "> quick_lookup_window",
        "   > inner_size",
        "      > width",
        "value (-100) is lower than the minimum acceptable 0.0",));


    let res = setup_example(SetupConfig {
       include_index_in_gamepad: false,
       my_ps3_controller_first_layer_step_1_left_trigger_on_double_click_visit_layer_name:
           "second-layer-step-1".to_string(),
       my_ps3_controller_quick_lookup_window_inner_size_height: -1.002,
       my_ps3_controller_quick_lookup_window_inner_size_width: 100.0,
    }).validate_and_clone_and_set_layer_pointers();

    assert_eq!(
        res.unwrap_err(),
        format!("{}\n{}\n{}\n{}\n{}",
        "Error in",
        "> quick_lookup_window",
        "   > inner_size",
        "      > height",
        "value (-1.002) is lower than the minimum acceptable 0.0",));

}


// -------- Individual structs ------------
#[test]
fn layer_get_switches_works(){
    // left_upper_is_d_pad == true
    
    let layer = Layer {
        id: "layer-id".to_string(),
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
                    index_in_gamepad: Some(1),
                })),
                on_double_click: Some(SwitchOnClickReaction::MoveToOrVisitLayer(LayerSpecifier {
                    id: "second-layer-step-1".to_string(),
                    index_in_gamepad: Some(4),
                })), 
            }),
            left_trigger: Some(SwitchEventAndReaction {
                on_click: Some(SwitchOnClickReaction::VisitLayer(LayerSpecifier {
                    id: "first-layer-step-3".to_string(),
                    index_in_gamepad: Some(2),
                })),
                on_double_click: Some(SwitchOnClickReaction::MoveToOrVisitLayer(LayerSpecifier {
                    id: "second-layer-step-1".to_string(),
                    index_in_gamepad: Some(4),
                })), 
            }),
            left_trigger_2: None,
            right_trigger_2: None,
        }),
        cardinal_levers: None,
    };

    let res = layer.get_switches(true).unwrap();

    let expected = Switches {
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
                index_in_gamepad: Some(1),
            })),
            on_double_click: Some(SwitchOnClickReaction::MoveToOrVisitLayer(LayerSpecifier {
                id: "second-layer-step-1".to_string(),
                index_in_gamepad: Some(4),
            })), 
        }),
        left_trigger: Some(SwitchEventAndReaction {
            on_click: Some(SwitchOnClickReaction::VisitLayer(LayerSpecifier {
                id: "first-layer-step-3".to_string(),
                index_in_gamepad: Some(2),
            })),
            on_double_click: Some(SwitchOnClickReaction::MoveToOrVisitLayer(LayerSpecifier {
                id: "second-layer-step-1".to_string(),
                index_in_gamepad: Some(4),
            })), 
        }),
        left_trigger_2: None,
        right_trigger_2: None,
    };

    assert_eq!(res, expected);


    // left_upper_is_d_pad == false

    let res = layer.get_switches(false).unwrap();

    let expected = Switches {
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
                key: Key::Layout('t'),
                modifiers: vec![
                ]})),
            on_double_click: None, 
        }),
        d_pad_down: Some(SwitchEventAndReaction {
            on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                key: Key::Layout('n'),
                modifiers: vec![
                ]})),
            on_double_click: None, 
        }),
        d_pad_left: Some(SwitchEventAndReaction {
            on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                key: Key::Backspace,
                modifiers: vec![
                ]})),
            on_double_click: None, 
        }),
        d_pad_right: Some(SwitchEventAndReaction {
            on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                key: Key::Layout('o'),
                modifiers: vec![
                ]})),
            on_double_click: None, 
        }),
        left_stick_up: Some(SwitchEventAndReaction {
            on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                key: Key::Layout('T'),
                modifiers: vec![
                ]})),
            on_double_click: None, 
        }),
        left_stick_down: Some(SwitchEventAndReaction {
            on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                key: Key::Layout('N'),
                modifiers: vec![
                ]})),
            on_double_click: None, 
        }),
        left_stick_left: None,
        left_stick_right: Some(SwitchEventAndReaction {
            on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                key: Key::Layout('O'),
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
                index_in_gamepad: Some(1),
            })),
            on_double_click: Some(SwitchOnClickReaction::MoveToOrVisitLayer(LayerSpecifier {
                id: "second-layer-step-1".to_string(),
                index_in_gamepad: Some(4),
            })), 
        }),
        left_trigger: Some(SwitchEventAndReaction {
            on_click: Some(SwitchOnClickReaction::VisitLayer(LayerSpecifier {
                id: "first-layer-step-3".to_string(),
                index_in_gamepad: Some(2),
            })),
            on_double_click: Some(SwitchOnClickReaction::MoveToOrVisitLayer(LayerSpecifier {
                id: "second-layer-step-1".to_string(),
                index_in_gamepad: Some(4),
            })), 
        }),
        left_trigger_2: None,
        right_trigger_2: None,
    };
    assert_eq!(res, expected);


    // switches not set

    let layer = Layer {
        id: "layer-id".to_string(),
        switches: None,
        cardinal_levers: None,
    };

    assert!(layer.get_switches(true).is_none());
    assert!(layer.get_switches(false).is_none());
}
