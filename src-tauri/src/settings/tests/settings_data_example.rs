use crate::settings::data::*;

pub fn setup_settings_data_example() -> SettingsData {
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
          id: String::from("first-layer-step-1"),
          cardinal_levers: None,
          switches: Some(Switches {
            d_pad_left: None,
            south: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('I'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            east: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                key: Some(EnigoKey::Return),
                char_key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            north: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('E'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            west: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('A'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            d_pad_up: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('T'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            d_pad_down: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('N'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            d_pad_right: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('O'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            left_stick_up: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('t'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            left_stick_down: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('n'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            left_stick_left: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                key: Some(EnigoKey::Backspace),
                char_key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            left_stick_right: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('o'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            right_stick_up: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('e'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            right_stick_down: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('i'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            right_stick_left: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('a'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            right_stick_right: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                key: Some(EnigoKey::Space),
                char_key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            left_trigger: Some(SwitchEventAndReaction{
              on_click: None,
              on_click_and_hold: Some(SwitchOnClickReaction{
                visit_layer: Some("first-layer-step-3".to_string()),
                key: None,
                char_key: None,
                modifiers: None,
                move_to_layer: None,
                mouse_button: None,
              }),
              on_double_click: Some(SwitchOnClickReaction{
                move_to_layer: Some("second-layer-step-1".to_string()),
                key: None,
                char_key: None,
                modifiers: None,
                visit_layer: None,
                mouse_button: None,
              }),
              on_double_click_and_hold: Some(SwitchOnClickReaction{
                visit_layer: Some("second-layer-step-1".to_string()),
                key: None,
                char_key: None,
                modifiers: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            right_trigger: Some(SwitchEventAndReaction{
              on_click: None,
              on_click_and_hold: Some(SwitchOnClickReaction{
                visit_layer: Some("first-layer-step-2".to_string()),
                key: None,
                char_key: None,
                modifiers: None,
                move_to_layer: None,
                mouse_button: None,
              }),
              on_double_click: Some(SwitchOnClickReaction{
                move_to_layer: Some("second-layer-step-1".to_string()),
                key: None,
                char_key: None,
                modifiers: None,
                visit_layer: None,
                mouse_button: None,
              }),
              on_double_click_and_hold: Some(SwitchOnClickReaction{
                visit_layer: Some("second-layer-step-1".to_string()),
                key: None,
                char_key: None,
                modifiers: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            left_trigger_2: None,
            right_trigger_2: None,
          })
        },
        Layer {
          id: String::from("first-layer-step-2"),
          cardinal_levers: None,
          switches: Some(Switches {
            right_trigger: None,
            south: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('U'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            east: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('S'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            north: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('H'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            west: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('D'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            d_pad_up: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('L'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            d_pad_down: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('M'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            d_pad_left: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('R'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            d_pad_right: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('C'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            left_stick_up: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('l'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            left_stick_down: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('m'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            left_stick_left: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('r'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            left_stick_right: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('c'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            right_stick_up: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('h'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            right_stick_down: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('u'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            right_stick_left: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('d'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            right_stick_right: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('s'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            left_trigger: Some(SwitchEventAndReaction{
              on_click: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click_and_hold: Some(SwitchOnClickReaction{
                visit_layer: Some("first-layer-step-4".to_string()),
                key: None,
                char_key: None,
                modifiers: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            left_trigger_2: None,
            right_trigger_2: None,
          }),
        },
        Layer {
          id: String::from("first-layer-step-3"),
          cardinal_levers: None,
          switches: Some(Switches {
            left_trigger: None,
            south: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('V'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            east: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('F'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            north: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('G'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            west: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('Y'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            d_pad_up: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('W'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            d_pad_down: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('K'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            d_pad_left: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('P'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            d_pad_right: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('B'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            left_stick_up: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('w'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            left_stick_down: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('k'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            left_stick_left: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('p'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            left_stick_right: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('b'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            right_stick_up: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('g'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            right_stick_down: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('v'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            right_stick_left: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('y'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            right_stick_right: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('f'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            right_trigger: Some(SwitchEventAndReaction{
              on_click: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click_and_hold: Some(SwitchOnClickReaction{
                visit_layer: Some("first-layer-step-4".to_string()),
                key: None,
                char_key: None,
                modifiers: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            left_trigger_2: None,
            right_trigger_2: None,
          })
        },
        Layer {
          id: String::from("first-layer-step-4"),
          cardinal_levers: None,
          switches: Some(Switches {
              d_pad_down: None,
              d_pad_right: None,
              left_stick_down: None,
              left_stick_right: None,
              right_stick_down: None,
              right_stick_left: None,
              south: None,
              west: None,
              left_trigger: None,
              right_trigger: None,
            left_trigger_2: None,
            right_trigger_2: None,
            east: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('X'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            north: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('Q'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            d_pad_up: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('Z'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            d_pad_left: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('J'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            left_stick_up: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('z'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            left_stick_left: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('j'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            right_stick_up: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('q'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            right_stick_right: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('x'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
          }),
        },
        Layer {
          id: String::from("second-layer-step-1"),
          cardinal_levers: None,
          switches: Some(Switches {
            south: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('&'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            east: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('^'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            north: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('%'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            west: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('*'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            d_pad_up: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('!'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            d_pad_down: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('#'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            d_pad_left: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('$'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            d_pad_right: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('@'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            left_stick_up: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('1'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            left_stick_down: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('3'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            left_stick_left: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('4'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            left_stick_right: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('2'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            right_stick_up: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('5'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            right_stick_down: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('7'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            right_stick_left: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('8'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            right_stick_right: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('6'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            left_trigger: Some(SwitchEventAndReaction{
              on_click: None,
              on_click_and_hold: Some(SwitchOnClickReaction{
                visit_layer: Some("second-layer-step-3".to_string()),
                key: None,
                char_key: None,
                modifiers: None,
                move_to_layer: None,
                mouse_button: None,
              }),
              on_double_click: Some(SwitchOnClickReaction{
                move_to_layer: Some("first-layer-step-1".to_string()),
                key: None,
                char_key: None,
                modifiers: None,
                visit_layer: None,
                mouse_button: None,
              }),
              on_double_click_and_hold: Some(SwitchOnClickReaction{
                visit_layer: Some("first-layer-step-1".to_string()),
                key: None,
                char_key: None,
                modifiers: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            right_trigger: Some(SwitchEventAndReaction{
              on_click: None,
              on_click_and_hold: Some(SwitchOnClickReaction{
                visit_layer: Some("second-layer-step-2".to_string()),
                key: None,
                char_key: None,
                modifiers: None,
                move_to_layer: None,
                mouse_button: None,
              }),
              on_double_click: Some(SwitchOnClickReaction{
                move_to_layer: Some("first-layer-step-1".to_string()),
                key: None,
                char_key: None,
                modifiers: None,
                visit_layer: None,
                mouse_button: None,
              }),
              on_double_click_and_hold: Some(SwitchOnClickReaction{
                visit_layer: Some("first-layer-step-1".to_string()),
                key: None,
                char_key: None,
                modifiers: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            left_trigger_2: None,
            right_trigger_2: None,
          }),
        },
        Layer {
          id: String::from("second-layer-step-2"),
          cardinal_levers: None,
          switches: Some(Switches {
            right_trigger: None,
            south: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('}'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            east: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some(':'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            north: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('\"'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            west: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('{'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            d_pad_up: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('('),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            d_pad_down: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('>'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            d_pad_left: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('<'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            d_pad_right: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('('),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            left_stick_up: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('9'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            left_stick_down: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('.'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            left_stick_left: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some(','),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            left_stick_right: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('0'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            right_stick_up: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('\''),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            right_stick_down: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some(']'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            right_stick_left: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('['),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            right_stick_right: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some(';'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            left_trigger: Some(SwitchEventAndReaction{
              on_click: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click_and_hold: Some(SwitchOnClickReaction{
                visit_layer: Some("second-layer-step-4".to_string()),
                key: None,
                char_key: None,
                modifiers: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            left_trigger_2: None,
            right_trigger_2: None,
          }),
        },
        Layer {
          id: String::from("second-layer-step-3"),
          cardinal_levers: None,
          switches: Some(Switches {
            left_trigger: None,
            right_stick_down: None,
            right_stick_left: None,
            south: None,
            west: None,
            east: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('|'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            north: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                key: Some(EnigoKey::Tab),
                char_key: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
                modifiers: Some(vec![
                  EnigoKey::Shift
                ])
              }),
            }),
            d_pad_up: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('_'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            d_pad_down: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('+'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            d_pad_left: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('~'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            d_pad_right: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('?'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            left_stick_up: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('_'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            left_stick_down: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('='),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            left_stick_left: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('`'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            left_stick_right: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('/'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            right_stick_up: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                key: Some(EnigoKey::Tab),
                char_key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            right_stick_right: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                char_key: Some('\\'),
                key: None,
                modifiers: None,
                visit_layer: None,
                move_to_layer: None,
                mouse_button: None,
              })
            }),
            right_trigger: Some(SwitchEventAndReaction{
              on_click: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click_and_hold: Some(SwitchOnClickReaction{
                visit_layer: Some("second-layer-step-4".to_string()),
                char_key: None,
                key: None,
                modifiers: None,
                mouse_button: None,
                move_to_layer: None,
              })
            }),
            left_trigger_2: None,
            right_trigger_2: None,
          }),
        },
        Layer {
          id: String::from("second-layer-step-4"),
          switches: Some(Switches {
	        east: None,
	        north: None,
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
            south: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                  mouse_button: Some(EnigoMouseButton::Left),
                  char_key: None,
                  key: None,
                  modifiers: None,
                  visit_layer: None,
                  move_to_layer: None,
              }),
            }),
            west: Some(SwitchEventAndReaction{
              on_click_and_hold: None,
              on_double_click: None,
              on_double_click_and_hold: None,
              on_click: Some(SwitchOnClickReaction{
                  mouse_button: Some(EnigoMouseButton::Right),
                  key: None,
                  char_key: None,
                  modifiers: None,
                  move_to_layer: None,
                  visit_layer: None,
              }),
            }),
          }),
          cardinal_levers: Some(CardinalLevers {
            left_stick: CardinalLeverFunction{
              control_mouse_cursor: Some(ControlMouseCursorFunction{
                center_at: ControlMouseCursorCenterCoordinates{
                  x: 0.0,
                  y: 0.0
                }
              }),
              control_mouse_scrollwheel: None,
            },
            right_stick: CardinalLeverFunction{
              control_mouse_scrollwheel: Some(ControlMouseScrollwheelFunction{
                center_at_y: 0.0
              }),
              control_mouse_cursor: None,
            },
          })
        }
      ]
    };

    SettingsData {
        profiles: vec![ps4_controller_profile],
        global: Global{default_profile:"My PS3 Controller".to_string()} 
    }
}
