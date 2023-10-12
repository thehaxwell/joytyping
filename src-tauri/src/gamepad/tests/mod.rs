use enigo::Key;
use gilrs::Button;
use mockall::{predicate::*, Sequence};

use crate::{gamepad::{tests::layers_examples::setup_layers_examples, switch_click_pattern_detector::{MockSwitchClickPatternDetectorTrait, SwitchClickPattern}, gilrs_events::MockGilrsEventsTrait, layers_navigator::MockLayersNavigatorTrait, cardinal_levers_move_detector::MockCardinalLeversMoveDetectorTrait, Gamepad, InputEvent, Switch}, quick_lookup_window::{QuickLookupWindowTrait, MockQuickLookupWindowTrait}, settings::{self, data::LayerSpecifier}};

use super::gilrs_events::stick_switch_interpreter::StickSwitchButton;

mod layers_examples; 

#[test]
fn gamepad_tick_returns_key_click_and_key_down_input_events(){
    [
        (SwitchClickPattern::Click(Switch::Button(Button::South)),
            0_usize,
            InputEvent::KeyClick(settings::data::KeyboardInput { key: Key::Layout('I'), modifiers: Vec::new() })),
        (SwitchClickPattern::Click(Switch::StickSwitchButton(StickSwitchButton::RightStickDown)),
            0_usize,
            InputEvent::KeyClick(settings::data::KeyboardInput { key: Key::Layout('i'), modifiers: Vec::new() })),
        (SwitchClickPattern::Click(Switch::Button(Button::DPadDown)),
            1_usize,
            InputEvent::KeyClick(settings::data::KeyboardInput { key: Key::Layout('M'), modifiers: Vec::new() })),
        (SwitchClickPattern::Click(Switch::StickSwitchButton(StickSwitchButton::LeftStickUp)),
            3_usize,
            InputEvent::KeyClick(settings::data::KeyboardInput { key: Key::Layout('z'), modifiers: Vec::new() })),
        (SwitchClickPattern::Click(Switch::StickSwitchButton(StickSwitchButton::LeftStickUp)),
            4_usize,
            InputEvent::KeyClick(settings::data::KeyboardInput { key: Key::Layout('1'), modifiers: Vec::new() })),


        // DoubleClick count as Click if on_double_click wasn't set
        (SwitchClickPattern::DoubleClick(Switch::Button(Button::South)),
            0_usize,
            InputEvent::KeyClick(settings::data::KeyboardInput { key: Key::Layout('I'), modifiers: Vec::new() })),
        (SwitchClickPattern::DoubleClick(Switch::StickSwitchButton(StickSwitchButton::RightStickDown)),
            0_usize,
            InputEvent::KeyClick(settings::data::KeyboardInput { key: Key::Layout('i'), modifiers: Vec::new() })),
        (SwitchClickPattern::DoubleClick(Switch::Button(Button::DPadDown)),
            1_usize,
            InputEvent::KeyClick(settings::data::KeyboardInput { key: Key::Layout('M'), modifiers: Vec::new() })),
        (SwitchClickPattern::DoubleClick(Switch::StickSwitchButton(StickSwitchButton::LeftStickUp)),
            3_usize,
            InputEvent::KeyClick(settings::data::KeyboardInput { key: Key::Layout('z'), modifiers: Vec::new() })),
        (SwitchClickPattern::DoubleClick(Switch::StickSwitchButton(StickSwitchButton::LeftStickUp)),
            4_usize,
            InputEvent::KeyClick(settings::data::KeyboardInput { key: Key::Layout('1'), modifiers: Vec::new() })),

        // ClickAndHold
        (SwitchClickPattern::ClickAndHold(Switch::Button(Button::South)),
            0_usize,
            InputEvent::KeyDown(settings::data::KeyboardInput { key: Key::Layout('I'), modifiers: Vec::new() })),
        (SwitchClickPattern::ClickAndHold(Switch::StickSwitchButton(StickSwitchButton::RightStickDown)),
            0_usize,
            InputEvent::KeyDown(settings::data::KeyboardInput { key: Key::Layout('i'), modifiers: Vec::new() })),
        (SwitchClickPattern::ClickAndHold(Switch::Button(Button::DPadDown)),
            1_usize,
            InputEvent::KeyDown(settings::data::KeyboardInput { key: Key::Layout('M'), modifiers: Vec::new() })),
        (SwitchClickPattern::ClickAndHold(Switch::StickSwitchButton(StickSwitchButton::LeftStickUp)),
            3_usize,
            InputEvent::KeyDown(settings::data::KeyboardInput { key: Key::Layout('z'), modifiers: Vec::new() })),
        (SwitchClickPattern::ClickAndHold(Switch::StickSwitchButton(StickSwitchButton::LeftStickUp)),
            4_usize,
            InputEvent::KeyDown(settings::data::KeyboardInput { key: Key::Layout('1'), modifiers: Vec::new() })),

        // DoubleClickAndHold count as Click if on_double_click wasn't set
        // and yet on_click is set as SwitchOnClickReaction::Keyboard
        (SwitchClickPattern::DoubleClickAndHold(Switch::Button(Button::South)),
            0_usize,
            InputEvent::KeyDown(settings::data::KeyboardInput { key: Key::Layout('I'), modifiers: Vec::new() })),
        (SwitchClickPattern::DoubleClickAndHold(Switch::StickSwitchButton(StickSwitchButton::RightStickDown)),
            0_usize,
            InputEvent::KeyDown(settings::data::KeyboardInput { key: Key::Layout('i'), modifiers: Vec::new() })),
        (SwitchClickPattern::DoubleClickAndHold(Switch::Button(Button::DPadDown)),
            1_usize,
            InputEvent::KeyDown(settings::data::KeyboardInput { key: Key::Layout('M'), modifiers: Vec::new() })),
        (SwitchClickPattern::DoubleClickAndHold(Switch::StickSwitchButton(StickSwitchButton::LeftStickUp)),
            3_usize,
            InputEvent::KeyDown(settings::data::KeyboardInput { key: Key::Layout('z'), modifiers: Vec::new() })),
        (SwitchClickPattern::DoubleClickAndHold(Switch::StickSwitchButton(StickSwitchButton::LeftStickUp)),
            4_usize,
            InputEvent::KeyDown(settings::data::KeyboardInput { key: Key::Layout('1'), modifiers: Vec::new() })),
    ]
    .iter()
    .for_each(|(pattern,layer_num,result_event)|{
        let mut mock_switch_click_pattern_detector = MockSwitchClickPatternDetectorTrait::new();
        let mock_gilrs_events = MockGilrsEventsTrait::new();
        let mut mock_layers_navigator = MockLayersNavigatorTrait::new();
        let mock_quick_lookup_window = MockQuickLookupWindowTrait::new();
        let mock_mouse_cursor_move_detector = MockCardinalLeversMoveDetectorTrait::new();
        let mock_scroll_detector = MockCardinalLeversMoveDetectorTrait::new();

        mock_switch_click_pattern_detector
            .expect_tick()
            .times(1)
            .return_const(Some(pattern.clone()));

        mock_layers_navigator
            .expect_process_current_potential_visit()
            .times(1)
            .with(eq(pattern.clone()))
            .return_const(());

        mock_layers_navigator
            .expect_get_current_layer_index()
            .times(1)
            .return_const(*layer_num);

        let mut gamepad = Gamepad {
           gilrs_events: Box::new(mock_gilrs_events),
           layers: setup_layers_examples(),
           switch_click_pattern_detector: Box::new(mock_switch_click_pattern_detector),
           layers_navigator: Box::new(mock_layers_navigator),
           quick_lookup_window: Box::new(mock_quick_lookup_window),
           mouse_cursor_move_detector: Box::new(mock_mouse_cursor_move_detector),
           mouse_scroll_detector: Box::new(mock_scroll_detector),
        };

        assert_eq!(gamepad.tick().unwrap(),*result_event);
    });
}

// #[test]
// fn gamepad_tick_processes_move_to_layer(){
//     [
//         // (SwitchClickPattern::Click(Switch::Button(Button::RightTrigger)),
//         //     0_usize,
//         //     LayerSpecifier{id: "first-layer-step-2".to_string(), index_in_gamepad: Some(1)}),
//         (SwitchClickPattern::Click(Switch::Button(Button::RightTrigger)),
//             0_usize,),
//     ]
//     .iter()
//     // .for_each(|(pattern,layer_num,layer_specifier)|{
//     .for_each(|(pattern,layer_num)|{
//         let mut mock_switch_click_pattern_detector = MockSwitchClickPatternDetectorTrait::new();
//         let mock_gilrs_events = MockGilrsEventsTrait::new();
//         let mut mock_layers_navigator = MockLayersNavigatorTrait::new();
//         let mock_quick_lookup_window = MockQuickLookupWindowTrait::new();
//         let mock_mouse_cursor_move_detector = MockCardinalLeversMoveDetectorTrait::new();
//         let mock_scroll_detector = MockCardinalLeversMoveDetectorTrait::new();
//
//         mock_switch_click_pattern_detector
//             .expect_tick()
//             .times(1)
//             .return_const(Some(pattern.clone()));
//
//         mock_layers_navigator
//             .expect_process_current_potential_visit()
//             .times(1)
//             .with(eq(pattern.clone()))
//             .return_const(());
//
//         mock_layers_navigator
//             .expect_get_current_layer_index()
//             .times(1)
//             .return_const(*layer_num);
//
//         mock_layers_navigator
//             .expect_move_to_layer()
//             .times(1)
//             // .with(eq(layer_specifier))
//             .with(eq(LayerSpecifier{id: "first-layer-step-2".to_string(), index_in_gamepad: Some(1)}))
//             .return_const(());
//
//         let mut gamepad = Gamepad {
//            gilrs_events: Box::new(mock_gilrs_events),
//            layers: setup_layers_examples(),
//            switch_click_pattern_detector: Box::new(mock_switch_click_pattern_detector),
//            layers_navigator: Box::new(mock_layers_navigator),
//            quick_lookup_window: Box::new(mock_quick_lookup_window),
//            mouse_cursor_move_detector: Box::new(mock_mouse_cursor_move_detector),
//            mouse_scroll_detector: Box::new(mock_scroll_detector),
//         };
//
//         assert!(gamepad.tick().is_none());
//     });
// }

// // use super::gilrs_wrapper::{GilrsEvent, GilrsEventType};
//
// #[test]
// fn click_end_to_backtrack_from_top_of_layer_visit_vec() {
//     let mut mock_switch_click_pattern_detector = MockSwitchClickPatternDetectorTrait::new();
//     let mock_gilrs_events = MockGilrsEventsTrait::new();
//
//     mock_switch_click_pattern_detector
//         .expect_tick()
//         .times(1)
//         .return_const(Some(SwitchClickPattern::ClickEnd(Switch::Button(Button::LeftTrigger))));
//
//     // simulate the case where the user:
//     // 1. Hold RightTrigger to go to layer 2 (index 1), then without releasing,
//     // 2. Hold LeftTrigger to go to layer 4 (index 3)
//     let layer_visits = vec![
//        LayerVisit{
//             trigger_switch: Switch::Button(Button::RightTrigger),
//             from_index: 0,
//             to_index: 1,
//        },
//        LayerVisit{
//             trigger_switch: Switch::Button(Button::LeftTrigger),
//             from_index: 1,
//             to_index: 3,
//        },
//    ];
//
//     let mut gamepad = Gamepad {
//        gilrs_events: Box::new(mock_gilrs_events),
//        layers: setup_layers_examples(),
//        current_layer_index: 3,
//        switch_click_pattern_detector: Box::new(mock_switch_click_pattern_detector),
//        layer_visits: layer_visits.clone(),
//        potential_layer_visit: None,
//        layer_visits_specified_for_each_layer: vec![
//            (0, vec![
//                LayerVisit{
//                    trigger_switch: Switch::Button(Button::RightTrigger),
//                    from_index: 0,
//                    to_index: 1,
//                },
//                LayerVisit{
//                    trigger_switch: Switch::Button(Button::LeftTrigger),
//                    from_index: 0,
//                    to_index: 2,
//                },
//            ]),
//            (1, vec![
//                LayerVisit{
//                    trigger_switch: Switch::Button(Button::LeftTrigger),
//                    from_index: 1,
//                    to_index: 3,
//                },
//            ])
//        ],
//     };
//
//     assert_eq!(gamepad.tick().unwrap(),InputEvent::KeyUp);
//     assert_eq!(gamepad.layer_visits.len(), 1);
//     assert_eq!(gamepad.layer_visits[0], layer_visits[0].clone());
//
//     assert!(gamepad.potential_layer_visit.is_none());
// }
//
// #[test]
// fn click_end_to_backtrack_from_middle_of_layer_visit_vec() {
//     let mut mock_switch_click_pattern_detector = MockSwitchClickPatternDetectorTrait::new();
//     let mock_gilrs_events = MockGilrsEventsTrait::new();
//
//     mock_switch_click_pattern_detector
//         .expect_tick()
//         .times(1)
//         .return_const(Some(SwitchClickPattern::ClickEnd(Switch::Button(Button::RightTrigger))));
//
//     // simulate the case where the user:
//     // 1. Hold RightTrigger to go to layer 2 (index 1), then without releasing,
//     // 2. Hold LeftTrigger to go to layer 4 (index 3)
//     let layer_visits = vec![
//        LayerVisit{
//             trigger_switch: Switch::Button(Button::RightTrigger),
//             from_index: 0,
//             to_index: 1,
//        },
//        LayerVisit{
//             trigger_switch: Switch::Button(Button::LeftTrigger),
//             from_index: 1,
//             to_index: 3,
//        },
//    ];
//
//     let mut gamepad = Gamepad {
//        gilrs_events: Box::new(mock_gilrs_events),
//        layers: setup_layers_examples(),
//        current_layer_index: 3,
//        switch_click_pattern_detector: Box::new(mock_switch_click_pattern_detector),
//        layer_visits: layer_visits.clone(),
//        potential_layer_visit: None,
//        layer_visits_specified_for_each_layer: vec![
//            (0, vec![
//                LayerVisit{
//                    trigger_switch: Switch::Button(Button::RightTrigger),
//                    from_index: 0,
//                    to_index: 1,
//                },
//                LayerVisit{
//                    trigger_switch: Switch::Button(Button::LeftTrigger),
//                    from_index: 0,
//                    to_index: 2,
//                },
//            ]),
//            (1, vec![
//                LayerVisit{
//                    trigger_switch: Switch::Button(Button::LeftTrigger),
//                    from_index: 1,
//                    to_index: 3,
//                },
//            ])
//        ],
//     };
//
//     assert_eq!(gamepad.tick().unwrap(),InputEvent::KeyUp);
//     assert_eq!(gamepad.layer_visits.len(), 1);
//     assert_eq!(gamepad.layer_visits[0], LayerVisit{
//             trigger_switch: Switch::Button(Button::LeftTrigger),
//             from_index: 0,
//             to_index: 2,
//        });
//
//     assert!(gamepad.potential_layer_visit.is_none());
// }
//
// #[test]
// fn tick_move_to_and_visit_layer_are_handled_correctly() {
//     let mut mock_switch_click_pattern_detector = MockSwitchClickPatternDetectorTrait::new();
//     let mock_gilrs_events = MockGilrsEventsTrait::new();
//
//     // DoubleClick RightTrigger to go (via MoveToOrVisitLayer)
//     // to second-layer-step-1 (index 4)
//
//     mock_switch_click_pattern_detector
//         .expect_tick()
//         .times(1)
//         .return_const(Some(SwitchClickPattern::DoubleClick(Switch::Button(Button::RightTrigger))));
//
//     let mut gamepad = Gamepad {
//        gilrs_events: Box::new(mock_gilrs_events),
//        layers: setup_layers_examples(),
//        current_layer_index: 0,
//        switch_click_pattern_detector: Box::new(mock_switch_click_pattern_detector),
//        layer_visits: Vec::new(),
//        potential_layer_visit: None,
//        layer_visits_specified_for_each_layer: Vec::new(),
//     };
//
//     assert!(gamepad.tick().is_none());
//     assert_eq!(gamepad.current_layer_index,4);
//     assert_eq!(gamepad.layer_visits.len(), 0);
//     assert_eq!(gamepad.potential_layer_visit.unwrap(),
//         LayerVisit {
//             trigger_switch: Switch::Button(Button::RightTrigger),
//             from_index: 0,
//             to_index: 4,
//         });
// }
//
//
//
//
//
//
//
//
// // #[test]
// // fn tick_responds_correctly_with_keyboard_dynamic_switch_events() {
// //     let mut mock_switch_click_pattern_detector = MockSwitchClickPatternDetectorTrait::new();
// //     let mut mock_dynamic_switch_event_responses = MockDynamicSwitchEventReactionsTrait::new();
// //     let mock_gilrs_events = MockGilrsEventsTrait::new();
// //
// //
// //     let mut seq = Sequence::new();
// //     let mut results_vec:Vec<Option<InputEvent>> = Vec::new();
// //
// //
// //     let switch = Switch::Button(Button::North);
// //     let keyboard_input = KeyboardInput { key: Key::Layout('i'), modifiers: Vec::new() };
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::Click(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::Click))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::Keyboard(
// //             keyboard_input.clone()))
// //         .in_sequence(&mut seq);
// //     results_vec.push(Some(InputEvent::KeyClick(keyboard_input)));
// //
// //
// //     let switch = Switch::Button(Button::West);
// //     let keyboard_input = KeyboardInput { key: Key::Tab, modifiers: Vec::new() };
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::ClickAndHold(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::ClickAndHold))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::Keyboard(
// //             keyboard_input.clone()))
// //         .in_sequence(&mut seq);
// //     results_vec.push(Some(InputEvent::KeyClick(keyboard_input)));
// //
// //
// //     let switch = Switch::Button(Button::Select);
// //     let keyboard_input = KeyboardInput { key: Key::Layout('X'), modifiers: Vec::new() };
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::DoubleClick(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::DoubleClick))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::Keyboard(
// //             keyboard_input.clone()))
// //         .in_sequence(&mut seq);
// //     results_vec.push(Some(InputEvent::KeyClick(keyboard_input)));
// //
// //
// //     let switch = Switch::StickSwitchButton(StickSwitchButton::LeftStickUp);
// //     let keyboard_input = KeyboardInput { key: Key::Layout('X'), modifiers: vec![ Key::Shift ] };
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::ClickEnd(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::ClickEnd))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::Keyboard(
// //             keyboard_input.clone()))
// //         .in_sequence(&mut seq);
// //     results_vec.push(Some(InputEvent::KeyClick(keyboard_input)));
// //
// //
// //     let switch = Switch::StickSwitchButton(StickSwitchButton::RightStickUp);
// //     let keyboard_input = KeyboardInput { key: Key::Space, modifiers: vec![ Key::Meta ] };
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::DoubleClickAndHold(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::DoubleClickAndHold))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::Keyboard(
// //             keyboard_input.clone()))
// //         .in_sequence(&mut seq);
// //     results_vec.push(Some(InputEvent::KeyClick(keyboard_input)));
// //    
// //
// //
// //     let mut gamepad = Gamepad {
// //        gilrs_events: Box::new(mock_gilrs_events),
// //        layers: setup_layers_examples(),
// //        current_layer_index: 0,
// //        switch_click_pattern_detector: Box::new(mock_switch_click_pattern_detector),
// //        dynamic_switch_event_responses: Box::new(mock_dynamic_switch_event_responses),
// //     };
// //
// //     for result in results_vec.iter() {
// //         assert_eq!(gamepad.tick(),*result);
// //         assert_eq!(gamepad.current_layer_index,0);
// //         assert_eq!(gamepad.layers,setup_layers_examples());
// //     }
// // }
// //
// // #[test]
// // fn tick_responds_correctly_with_move_to_layer_dynamic_switch_events() {
// //     let mut mock_switch_click_pattern_detector = MockSwitchClickPatternDetectorTrait::new();
// //     let mut mock_dynamic_switch_event_responses = MockDynamicSwitchEventReactionsTrait::new();
// //     let mock_gilrs_events = MockGilrsEventsTrait::new();
// //
// //     let mut seq = Sequence::new();
// //     let mut indexes_vec:Vec<usize> = Vec::new();
// //
// //
// //     let switch = Switch::Button(Button::North);
// //     let index_in_gamepad = 1;
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::Click(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::Click))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::MoveToLayer(LayerSpecifier {
// //             id: String::new(),
// //             index_in_gamepad: Some(index_in_gamepad)
// //         }))
// //         .in_sequence(&mut seq);
// //     indexes_vec.push(index_in_gamepad);
// //
// //
// //     let switch = Switch::Button(Button::East);
// //     let index_in_gamepad = 123;
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::ClickAndHold(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::ClickAndHold))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::MoveToLayer(LayerSpecifier {
// //             id: String::new(),
// //             index_in_gamepad: Some(index_in_gamepad)
// //         }))
// //         .in_sequence(&mut seq);
// //     indexes_vec.push(index_in_gamepad);
// //
// //
// //     let switch = Switch::Button(Button::North);
// //     let index_in_gamepad = 123;
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::ClickAndHold(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::ClickAndHold))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::MoveToLayer(LayerSpecifier {
// //             id: String::new(),
// //             index_in_gamepad: Some(index_in_gamepad)
// //         }))
// //         .in_sequence(&mut seq);
// //     indexes_vec.push(index_in_gamepad);
// //
// //
// //     let switch = Switch::StickSwitchButton(StickSwitchButton::LeftStickLeft);
// //     let index_in_gamepad = 99;
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::DoubleClick(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::DoubleClick))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::MoveToLayer(LayerSpecifier {
// //             id: String::new(),
// //             index_in_gamepad: Some(index_in_gamepad)
// //         }))
// //         .in_sequence(&mut seq);
// //     indexes_vec.push(index_in_gamepad);
// //
// //
// //     let switch = Switch::StickSwitchButton(StickSwitchButton::RightStickRight);
// //     let index_in_gamepad = 8900;
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::DoubleClickAndHold(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::DoubleClickAndHold))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::MoveToLayer(LayerSpecifier {
// //             id: String::new(),
// //             index_in_gamepad: Some(index_in_gamepad)
// //         }))
// //         .in_sequence(&mut seq);
// //     indexes_vec.push(index_in_gamepad);
// //
// //
// //     let switch = Switch::StickSwitchButton(StickSwitchButton::RightStickUp);
// //     let index_in_gamepad = 89;
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::ClickEnd(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::ClickEnd))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::MoveToLayer(LayerSpecifier {
// //             id: String::new(),
// //             index_in_gamepad: Some(index_in_gamepad)
// //         }))
// //         .in_sequence(&mut seq);
// //     indexes_vec.push(index_in_gamepad);
// //
// //    
// //     let mut gamepad = Gamepad {
// //        gilrs_events: Box::new(mock_gilrs_events),
// //        layers: setup_layers_examples(),
// //        current_layer_index: 0,
// //        switch_click_pattern_detector: Box::new(mock_switch_click_pattern_detector),
// //        dynamic_switch_event_responses: Box::new(mock_dynamic_switch_event_responses),
// //     };
// //
// //     for result in indexes_vec.iter() {
// //         assert!(gamepad.tick().is_none());
// //         assert_eq!(gamepad.current_layer_index,*result);
// //         assert_eq!(gamepad.layers,setup_layers_examples());
// //     }
// // }
// //
// // #[test]
// // fn tick_responds_correctly_with_visit_layer_dynamic_switch_events() {
// //     let mut mock_switch_click_pattern_detector = MockSwitchClickPatternDetectorTrait::new();
// //     let mut mock_dynamic_switch_event_responses = MockDynamicSwitchEventReactionsTrait::new();
// //     let mock_gilrs_events = MockGilrsEventsTrait::new();
// //
// //     let mut seq = Sequence::new();
// //     let mut indexes_vec:Vec<usize> = Vec::new();
// //
// //
// //     let switch = Switch::Button(Button::North);
// //     let index_in_gamepad = 1;
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::Click(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::Click))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::VisitLayer(LayerSpecifier {
// //             id: String::new(),
// //             index_in_gamepad: Some(index_in_gamepad)
// //         }))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_add()
// //         .with(eq(switch.clone()),eq(
// //             DynamicSwitchEventReaction::ClickEnd(
// //                 EventReactionOrNesting::Reaction(
// //                     SwitchOnClickReaction::MoveToLayer(
// //                         LayerSpecifier{
// //                             id: String::new(),
// //                             index_in_gamepad: Some(*indexes_vec.last().or(Some(&0)).unwrap())
// //                         })))))
// //         .times(1)
// //         .return_const(())
// //         .in_sequence(&mut seq);
// //     indexes_vec.push(index_in_gamepad);
// //
// //
// //     let switch = Switch::Button(Button::East);
// //     let index_in_gamepad = 123;
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::ClickAndHold(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::ClickAndHold))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::VisitLayer(LayerSpecifier {
// //             id: String::new(),
// //             index_in_gamepad: Some(index_in_gamepad)
// //         }))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_add()
// //         .with(eq(switch.clone()),eq(
// //             DynamicSwitchEventReaction::ClickEnd(
// //                 EventReactionOrNesting::Reaction(
// //                     SwitchOnClickReaction::MoveToLayer(
// //                         LayerSpecifier{
// //                             id: String::new(),
// //                             index_in_gamepad: Some(*indexes_vec.last().or(Some(&0)).unwrap())
// //                         })))))
// //         .times(1)
// //         .return_const(())
// //         .in_sequence(&mut seq);
// //     indexes_vec.push(index_in_gamepad);
// //    
// //    
// //     let switch = Switch::Button(Button::North);
// //     let index_in_gamepad = 123;
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::ClickAndHold(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::ClickAndHold))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::VisitLayer(LayerSpecifier {
// //             id: String::new(),
// //             index_in_gamepad: Some(index_in_gamepad)
// //         }))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_add()
// //         .with(eq(switch.clone()),eq(
// //             DynamicSwitchEventReaction::ClickEnd(
// //                 EventReactionOrNesting::Reaction(
// //                     SwitchOnClickReaction::MoveToLayer(
// //                         LayerSpecifier{
// //                             id: String::new(),
// //                             index_in_gamepad: Some(*indexes_vec.last().or(Some(&0)).unwrap())
// //                         })))))
// //         .times(1)
// //         .return_const(())
// //         .in_sequence(&mut seq);
// //     indexes_vec.push(index_in_gamepad);
// //    
// //    
// //     let switch = Switch::StickSwitchButton(StickSwitchButton::LeftStickLeft);
// //     let index_in_gamepad = 99;
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::DoubleClick(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::DoubleClick))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::VisitLayer(LayerSpecifier {
// //             id: String::new(),
// //             index_in_gamepad: Some(index_in_gamepad)
// //         }))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_add()
// //         .with(eq(switch.clone()),eq(
// //             DynamicSwitchEventReaction::ClickEnd(
// //                 EventReactionOrNesting::Reaction(
// //                     SwitchOnClickReaction::MoveToLayer(
// //                         LayerSpecifier{
// //                             id: String::new(),
// //                             index_in_gamepad: Some(*indexes_vec.last().or(Some(&0)).unwrap())
// //                         })))))
// //         .times(1)
// //         .return_const(())
// //         .in_sequence(&mut seq);
// //     indexes_vec.push(index_in_gamepad);
// //    
// //    
// //     let switch = Switch::StickSwitchButton(StickSwitchButton::RightStickRight);
// //     let index_in_gamepad = 8900;
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::DoubleClickAndHold(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::DoubleClickAndHold))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::VisitLayer(LayerSpecifier {
// //             id: String::new(),
// //             index_in_gamepad: Some(index_in_gamepad)
// //         }))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_add()
// //         .with(eq(switch.clone()),eq(
// //             DynamicSwitchEventReaction::ClickEnd(
// //                 EventReactionOrNesting::Reaction(
// //                     SwitchOnClickReaction::MoveToLayer(
// //                         LayerSpecifier{
// //                             id: String::new(),
// //                             index_in_gamepad: Some(*indexes_vec.last().or(Some(&0)).unwrap())
// //                         })))))
// //         .times(1)
// //         .return_const(())
// //         .in_sequence(&mut seq);
// //     indexes_vec.push(index_in_gamepad);
// //    
// //    
// //     let switch = Switch::StickSwitchButton(StickSwitchButton::RightStickUp);
// //     let index_in_gamepad = 89;
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::ClickEnd(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::ClickEnd))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::VisitLayer(LayerSpecifier {
// //             id: String::new(),
// //             index_in_gamepad: Some(index_in_gamepad)
// //         }))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_add()
// //         .with(eq(switch.clone()),eq(
// //             DynamicSwitchEventReaction::ClickEnd(
// //                 EventReactionOrNesting::Reaction(
// //                     SwitchOnClickReaction::MoveToLayer(
// //                         LayerSpecifier{
// //                             id: String::new(),
// //                             index_in_gamepad: Some(*indexes_vec.last().or(Some(&0)).unwrap())
// //                         })))))
// //         .times(1)
// //         .return_const(())
// //         .in_sequence(&mut seq);
// //     indexes_vec.push(index_in_gamepad);
// //
// //    
// //     let mut gamepad = Gamepad {
// //        gilrs_events: Box::new(mock_gilrs_events),
// //        layers: setup_layers_examples(),
// //        current_layer_index: 0,
// //        switch_click_pattern_detector: Box::new(mock_switch_click_pattern_detector),
// //        dynamic_switch_event_responses: Box::new(mock_dynamic_switch_event_responses),
// //     };
// //
// //     for result in indexes_vec.iter() {
// //         assert!(gamepad.tick().is_none());
// //         assert_eq!(gamepad.current_layer_index,*result);
// //         assert_eq!(gamepad.layers,setup_layers_examples());
// //     }
// // }
// //
// // #[test]
// // fn tick_responds_correctly_with_move_to_or_visit_layer_dynamic_switch_events() {
// //     let mut mock_switch_click_pattern_detector = MockSwitchClickPatternDetectorTrait::new();
// //     let mut mock_dynamic_switch_event_responses = MockDynamicSwitchEventReactionsTrait::new();
// //     let mock_gilrs_events = MockGilrsEventsTrait::new();
// //
// //     let mut seq = Sequence::new();
// //     let mut indexes_vec:Vec<usize> = Vec::new();
// //
// //
// //     let switch = Switch::Button(Button::North);
// //     let index_in_gamepad = 1;
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::Click(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::Click))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::MoveToOrVisitLayer(LayerSpecifier {
// //             id: String::new(),
// //             index_in_gamepad: Some(index_in_gamepad)
// //         }))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_add()
// //         .with(eq(switch.clone()),eq(
// //             DynamicSwitchEventReaction::DoubleClickAndHold(
// //                 EventReactionOrNesting::Nesting(
// //                     Box::new(DynamicSwitchEventReaction::ClickEnd(
// //                         EventReactionOrNesting::Reaction(
// //                             SwitchOnClickReaction::MoveToLayer(
// //                                 LayerSpecifier {
// //                                     id: String::new(),
// //                                     index_in_gamepad: Some(*indexes_vec.last().or(Some(&0)).unwrap())
// //                     }))))))))
// //         .times(1)
// //         .return_const(())
// //         .in_sequence(&mut seq);
// //     indexes_vec.push(index_in_gamepad);
// //
// //
// //     let switch = Switch::Button(Button::East);
// //     let index_in_gamepad = 123;
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::ClickAndHold(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::ClickAndHold))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::MoveToOrVisitLayer(LayerSpecifier {
// //             id: String::new(),
// //             index_in_gamepad: Some(index_in_gamepad)
// //         }))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_add()
// //         .with(eq(switch.clone()),eq(
// //             DynamicSwitchEventReaction::DoubleClickAndHold(
// //                 EventReactionOrNesting::Nesting(
// //                     Box::new(DynamicSwitchEventReaction::ClickEnd(
// //                         EventReactionOrNesting::Reaction(
// //                             SwitchOnClickReaction::MoveToLayer(
// //                                 LayerSpecifier {
// //                                     id: String::new(),
// //                                     index_in_gamepad: Some(*indexes_vec.last().or(Some(&0)).unwrap())
// //                     }))))))))
// //         .times(1)
// //         .return_const(())
// //         .in_sequence(&mut seq);
// //     indexes_vec.push(index_in_gamepad);
// //    
// //    
// //     let switch = Switch::Button(Button::North);
// //     let index_in_gamepad = 123;
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::ClickAndHold(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::ClickAndHold))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::MoveToOrVisitLayer(LayerSpecifier {
// //             id: String::new(),
// //             index_in_gamepad: Some(index_in_gamepad)
// //         }))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_add()
// //         .with(eq(switch.clone()),eq(
// //             DynamicSwitchEventReaction::DoubleClickAndHold(
// //                 EventReactionOrNesting::Nesting(
// //                     Box::new(DynamicSwitchEventReaction::ClickEnd(
// //                         EventReactionOrNesting::Reaction(
// //                             SwitchOnClickReaction::MoveToLayer(
// //                                 LayerSpecifier {
// //                                     id: String::new(),
// //                                     index_in_gamepad: Some(*indexes_vec.last().or(Some(&0)).unwrap())
// //                     }))))))))
// //         .times(1)
// //         .return_const(())
// //         .in_sequence(&mut seq);
// //     indexes_vec.push(index_in_gamepad);
// //    
// //    
// //     let switch = Switch::StickSwitchButton(StickSwitchButton::LeftStickLeft);
// //     let index_in_gamepad = 99;
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::DoubleClick(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::DoubleClick))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::MoveToOrVisitLayer(LayerSpecifier {
// //             id: String::new(),
// //             index_in_gamepad: Some(index_in_gamepad)
// //         }))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_add()
// //         .with(eq(switch.clone()),eq(
// //             DynamicSwitchEventReaction::DoubleClickAndHold(
// //                 EventReactionOrNesting::Nesting(
// //                     Box::new(DynamicSwitchEventReaction::ClickEnd(
// //                         EventReactionOrNesting::Reaction(
// //                             SwitchOnClickReaction::MoveToLayer(
// //                                 LayerSpecifier {
// //                                     id: String::new(),
// //                                     index_in_gamepad: Some(*indexes_vec.last().or(Some(&0)).unwrap())
// //                     }))))))))
// //         .times(1)
// //         .return_const(())
// //         .in_sequence(&mut seq);
// //     indexes_vec.push(index_in_gamepad);
// //    
// //    
// //     let switch = Switch::StickSwitchButton(StickSwitchButton::RightStickRight);
// //     let index_in_gamepad = 8900;
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::DoubleClickAndHold(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::DoubleClickAndHold))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::MoveToOrVisitLayer(LayerSpecifier {
// //             id: String::new(),
// //             index_in_gamepad: Some(index_in_gamepad)
// //         }))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_add()
// //         .with(eq(switch.clone()),eq(
// //             DynamicSwitchEventReaction::DoubleClickAndHold(
// //                 EventReactionOrNesting::Nesting(
// //                     Box::new(DynamicSwitchEventReaction::ClickEnd(
// //                         EventReactionOrNesting::Reaction(
// //                             SwitchOnClickReaction::MoveToLayer(
// //                                 LayerSpecifier {
// //                                     id: String::new(),
// //                                     index_in_gamepad: Some(*indexes_vec.last().or(Some(&0)).unwrap())
// //                     }))))))))
// //         .times(1)
// //         .return_const(())
// //         .in_sequence(&mut seq);
// //     indexes_vec.push(index_in_gamepad);
// //    
// //    
// //     let switch = Switch::StickSwitchButton(StickSwitchButton::RightStickUp);
// //     let index_in_gamepad = 89;
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::ClickEnd(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::ClickEnd))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::MoveToOrVisitLayer(LayerSpecifier {
// //             id: String::new(),
// //             index_in_gamepad: Some(index_in_gamepad)
// //         }))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_add()
// //         .with(eq(switch.clone()),eq(
// //             DynamicSwitchEventReaction::DoubleClickAndHold(
// //                 EventReactionOrNesting::Nesting(
// //                     Box::new(DynamicSwitchEventReaction::ClickEnd(
// //                         EventReactionOrNesting::Reaction(
// //                             SwitchOnClickReaction::MoveToLayer(
// //                                 LayerSpecifier {
// //                                     id: String::new(),
// //                                     index_in_gamepad: Some(*indexes_vec.last().or(Some(&0)).unwrap())
// //                     }))))))))
// //         .times(1)
// //         .return_const(())
// //         .in_sequence(&mut seq);
// //     indexes_vec.push(index_in_gamepad);
// //
// //    
// //     let mut gamepad = Gamepad {
// //        gilrs_events: Box::new(mock_gilrs_events),
// //        layers: setup_layers_examples(),
// //        current_layer_index: 0,
// //        switch_click_pattern_detector: Box::new(mock_switch_click_pattern_detector),
// //        dynamic_switch_event_responses: Box::new(mock_dynamic_switch_event_responses),
// //     };
// //
// //     for result in indexes_vec.iter() {
// //         assert!(gamepad.tick().is_none());
// //         assert_eq!(gamepad.current_layer_index,*result);
// //         assert_eq!(gamepad.layers,setup_layers_examples());
// //     }
// // }
//
//
//
//
//
//
//
// // #[test]
// // fn tick_responds_correctly_with_keyboard_dynamic_switch_events() {
// //     let mut mock_switch_click_pattern_detector = MockSwitchClickPatternDetectorTrait::new();
// //     let mut mock_dynamic_switch_event_responses = MockDynamicSwitchEventReactionsTrait::new();
// //     let mock_gilrs_events = MockGilrsEventsTrait::new();
// //
// //
// //     let mut seq = Sequence::new();
// //     let mut results_vec:Vec<Option<InputEvent>> = Vec::new();
// //
// //
// //     let switch = Switch::Button(Button::North);
// //     let keyboard_input = KeyboardInput { key: Key::Layout('i'), modifiers: Vec::new() };
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::Click(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::Click))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::Keyboard(
// //             keyboard_input.clone()))
// //         .in_sequence(&mut seq);
// //     results_vec.push(Some(InputEvent::KeyClick(keyboard_input)));
// //
// //
// //     let switch = Switch::Button(Button::West);
// //     let keyboard_input = KeyboardInput { key: Key::Tab, modifiers: Vec::new() };
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::ClickAndHold(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::ClickAndHold))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::Keyboard(
// //             keyboard_input.clone()))
// //         .in_sequence(&mut seq);
// //     results_vec.push(Some(InputEvent::KeyClick(keyboard_input)));
// //
// //
// //     let switch = Switch::Button(Button::Select);
// //     let keyboard_input = KeyboardInput { key: Key::Layout('X'), modifiers: Vec::new() };
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::DoubleClick(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::DoubleClick))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::Keyboard(
// //             keyboard_input.clone()))
// //         .in_sequence(&mut seq);
// //     results_vec.push(Some(InputEvent::KeyClick(keyboard_input)));
// //
// //
// //     let switch = Switch::StickSwitchButton(StickSwitchButton::LeftStickUp);
// //     let keyboard_input = KeyboardInput { key: Key::Layout('X'), modifiers: vec![ Key::Shift ] };
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::ClickEnd(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::ClickEnd))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::Keyboard(
// //             keyboard_input.clone()))
// //         .in_sequence(&mut seq);
// //     results_vec.push(Some(InputEvent::KeyClick(keyboard_input)));
// //
// //
// //     let switch = Switch::StickSwitchButton(StickSwitchButton::RightStickUp);
// //     let keyboard_input = KeyboardInput { key: Key::Space, modifiers: vec![ Key::Meta ] };
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::DoubleClickAndHold(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::DoubleClickAndHold))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::Keyboard(
// //             keyboard_input.clone()))
// //         .in_sequence(&mut seq);
// //     results_vec.push(Some(InputEvent::KeyClick(keyboard_input)));
// //    
// //
// //
// //     let mut gamepad = Gamepad {
// //        gilrs_events: Box::new(mock_gilrs_events),
// //        layers: setup_layers_examples(),
// //        current_layer_index: 0,
// //        switch_click_pattern_detector: Box::new(mock_switch_click_pattern_detector),
// //        dynamic_switch_event_responses: Box::new(mock_dynamic_switch_event_responses),
// //     };
// //
// //     for result in results_vec.iter() {
// //         assert_eq!(gamepad.tick(),*result);
// //         assert_eq!(gamepad.current_layer_index,0);
// //         assert_eq!(gamepad.layers,setup_layers_examples());
// //     }
// // }
// //
// // #[test]
// // fn tick_responds_correctly_with_move_to_layer_dynamic_switch_events() {
// //     let mut mock_switch_click_pattern_detector = MockSwitchClickPatternDetectorTrait::new();
// //     let mut mock_dynamic_switch_event_responses = MockDynamicSwitchEventReactionsTrait::new();
// //     let mock_gilrs_events = MockGilrsEventsTrait::new();
// //
// //     let mut seq = Sequence::new();
// //     let mut indexes_vec:Vec<usize> = Vec::new();
// //
// //
// //     let switch = Switch::Button(Button::North);
// //     let index_in_gamepad = 1;
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::Click(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::Click))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::MoveToLayer(LayerSpecifier {
// //             id: String::new(),
// //             index_in_gamepad: Some(index_in_gamepad)
// //         }))
// //         .in_sequence(&mut seq);
// //     indexes_vec.push(index_in_gamepad);
// //
// //
// //     let switch = Switch::Button(Button::East);
// //     let index_in_gamepad = 123;
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::ClickAndHold(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::ClickAndHold))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::MoveToLayer(LayerSpecifier {
// //             id: String::new(),
// //             index_in_gamepad: Some(index_in_gamepad)
// //         }))
// //         .in_sequence(&mut seq);
// //     indexes_vec.push(index_in_gamepad);
// //
// //
// //     let switch = Switch::Button(Button::North);
// //     let index_in_gamepad = 123;
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::ClickAndHold(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::ClickAndHold))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::MoveToLayer(LayerSpecifier {
// //             id: String::new(),
// //             index_in_gamepad: Some(index_in_gamepad)
// //         }))
// //         .in_sequence(&mut seq);
// //     indexes_vec.push(index_in_gamepad);
// //
// //
// //     let switch = Switch::StickSwitchButton(StickSwitchButton::LeftStickLeft);
// //     let index_in_gamepad = 99;
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::DoubleClick(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::DoubleClick))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::MoveToLayer(LayerSpecifier {
// //             id: String::new(),
// //             index_in_gamepad: Some(index_in_gamepad)
// //         }))
// //         .in_sequence(&mut seq);
// //     indexes_vec.push(index_in_gamepad);
// //
// //
// //     let switch = Switch::StickSwitchButton(StickSwitchButton::RightStickRight);
// //     let index_in_gamepad = 8900;
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::DoubleClickAndHold(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::DoubleClickAndHold))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::MoveToLayer(LayerSpecifier {
// //             id: String::new(),
// //             index_in_gamepad: Some(index_in_gamepad)
// //         }))
// //         .in_sequence(&mut seq);
// //     indexes_vec.push(index_in_gamepad);
// //
// //
// //     let switch = Switch::StickSwitchButton(StickSwitchButton::RightStickUp);
// //     let index_in_gamepad = 89;
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::ClickEnd(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::ClickEnd))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::MoveToLayer(LayerSpecifier {
// //             id: String::new(),
// //             index_in_gamepad: Some(index_in_gamepad)
// //         }))
// //         .in_sequence(&mut seq);
// //     indexes_vec.push(index_in_gamepad);
// //
// //    
// //     let mut gamepad = Gamepad {
// //        gilrs_events: Box::new(mock_gilrs_events),
// //        layers: setup_layers_examples(),
// //        current_layer_index: 0,
// //        switch_click_pattern_detector: Box::new(mock_switch_click_pattern_detector),
// //        dynamic_switch_event_responses: Box::new(mock_dynamic_switch_event_responses),
// //     };
// //
// //     for result in indexes_vec.iter() {
// //         assert!(gamepad.tick().is_none());
// //         assert_eq!(gamepad.current_layer_index,*result);
// //         assert_eq!(gamepad.layers,setup_layers_examples());
// //     }
// // }
// //
// // #[test]
// // fn tick_responds_correctly_with_visit_layer_dynamic_switch_events() {
// //     let mut mock_switch_click_pattern_detector = MockSwitchClickPatternDetectorTrait::new();
// //     let mut mock_dynamic_switch_event_responses = MockDynamicSwitchEventReactionsTrait::new();
// //     let mock_gilrs_events = MockGilrsEventsTrait::new();
// //
// //     let mut seq = Sequence::new();
// //     let mut indexes_vec:Vec<usize> = Vec::new();
// //
// //
// //     let switch = Switch::Button(Button::North);
// //     let index_in_gamepad = 1;
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::Click(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::Click))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::VisitLayer(LayerSpecifier {
// //             id: String::new(),
// //             index_in_gamepad: Some(index_in_gamepad)
// //         }))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_add()
// //         .with(eq(switch.clone()),eq(
// //             DynamicSwitchEventReaction::ClickEnd(
// //                 EventReactionOrNesting::Reaction(
// //                     SwitchOnClickReaction::MoveToLayer(
// //                         LayerSpecifier{
// //                             id: String::new(),
// //                             index_in_gamepad: Some(*indexes_vec.last().or(Some(&0)).unwrap())
// //                         })))))
// //         .times(1)
// //         .return_const(())
// //         .in_sequence(&mut seq);
// //     indexes_vec.push(index_in_gamepad);
// //
// //
// //     let switch = Switch::Button(Button::East);
// //     let index_in_gamepad = 123;
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::ClickAndHold(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::ClickAndHold))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::VisitLayer(LayerSpecifier {
// //             id: String::new(),
// //             index_in_gamepad: Some(index_in_gamepad)
// //         }))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_add()
// //         .with(eq(switch.clone()),eq(
// //             DynamicSwitchEventReaction::ClickEnd(
// //                 EventReactionOrNesting::Reaction(
// //                     SwitchOnClickReaction::MoveToLayer(
// //                         LayerSpecifier{
// //                             id: String::new(),
// //                             index_in_gamepad: Some(*indexes_vec.last().or(Some(&0)).unwrap())
// //                         })))))
// //         .times(1)
// //         .return_const(())
// //         .in_sequence(&mut seq);
// //     indexes_vec.push(index_in_gamepad);
// //    
// //    
// //     let switch = Switch::Button(Button::North);
// //     let index_in_gamepad = 123;
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::ClickAndHold(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::ClickAndHold))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::VisitLayer(LayerSpecifier {
// //             id: String::new(),
// //             index_in_gamepad: Some(index_in_gamepad)
// //         }))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_add()
// //         .with(eq(switch.clone()),eq(
// //             DynamicSwitchEventReaction::ClickEnd(
// //                 EventReactionOrNesting::Reaction(
// //                     SwitchOnClickReaction::MoveToLayer(
// //                         LayerSpecifier{
// //                             id: String::new(),
// //                             index_in_gamepad: Some(*indexes_vec.last().or(Some(&0)).unwrap())
// //                         })))))
// //         .times(1)
// //         .return_const(())
// //         .in_sequence(&mut seq);
// //     indexes_vec.push(index_in_gamepad);
// //    
// //    
// //     let switch = Switch::StickSwitchButton(StickSwitchButton::LeftStickLeft);
// //     let index_in_gamepad = 99;
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::DoubleClick(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::DoubleClick))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::VisitLayer(LayerSpecifier {
// //             id: String::new(),
// //             index_in_gamepad: Some(index_in_gamepad)
// //         }))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_add()
// //         .with(eq(switch.clone()),eq(
// //             DynamicSwitchEventReaction::ClickEnd(
// //                 EventReactionOrNesting::Reaction(
// //                     SwitchOnClickReaction::MoveToLayer(
// //                         LayerSpecifier{
// //                             id: String::new(),
// //                             index_in_gamepad: Some(*indexes_vec.last().or(Some(&0)).unwrap())
// //                         })))))
// //         .times(1)
// //         .return_const(())
// //         .in_sequence(&mut seq);
// //     indexes_vec.push(index_in_gamepad);
// //    
// //    
// //     let switch = Switch::StickSwitchButton(StickSwitchButton::RightStickRight);
// //     let index_in_gamepad = 8900;
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::DoubleClickAndHold(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::DoubleClickAndHold))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::VisitLayer(LayerSpecifier {
// //             id: String::new(),
// //             index_in_gamepad: Some(index_in_gamepad)
// //         }))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_add()
// //         .with(eq(switch.clone()),eq(
// //             DynamicSwitchEventReaction::ClickEnd(
// //                 EventReactionOrNesting::Reaction(
// //                     SwitchOnClickReaction::MoveToLayer(
// //                         LayerSpecifier{
// //                             id: String::new(),
// //                             index_in_gamepad: Some(*indexes_vec.last().or(Some(&0)).unwrap())
// //                         })))))
// //         .times(1)
// //         .return_const(())
// //         .in_sequence(&mut seq);
// //     indexes_vec.push(index_in_gamepad);
// //    
// //    
// //     let switch = Switch::StickSwitchButton(StickSwitchButton::RightStickUp);
// //     let index_in_gamepad = 89;
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::ClickEnd(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::ClickEnd))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::VisitLayer(LayerSpecifier {
// //             id: String::new(),
// //             index_in_gamepad: Some(index_in_gamepad)
// //         }))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_add()
// //         .with(eq(switch.clone()),eq(
// //             DynamicSwitchEventReaction::ClickEnd(
// //                 EventReactionOrNesting::Reaction(
// //                     SwitchOnClickReaction::MoveToLayer(
// //                         LayerSpecifier{
// //                             id: String::new(),
// //                             index_in_gamepad: Some(*indexes_vec.last().or(Some(&0)).unwrap())
// //                         })))))
// //         .times(1)
// //         .return_const(())
// //         .in_sequence(&mut seq);
// //     indexes_vec.push(index_in_gamepad);
// //
// //    
// //     let mut gamepad = Gamepad {
// //        gilrs_events: Box::new(mock_gilrs_events),
// //        layers: setup_layers_examples(),
// //        current_layer_index: 0,
// //        switch_click_pattern_detector: Box::new(mock_switch_click_pattern_detector),
// //        dynamic_switch_event_responses: Box::new(mock_dynamic_switch_event_responses),
// //     };
// //
// //     for result in indexes_vec.iter() {
// //         assert!(gamepad.tick().is_none());
// //         assert_eq!(gamepad.current_layer_index,*result);
// //         assert_eq!(gamepad.layers,setup_layers_examples());
// //     }
// // }
// //
// // #[test]
// // fn tick_responds_correctly_with_move_to_or_visit_layer_dynamic_switch_events() {
// //     let mut mock_switch_click_pattern_detector = MockSwitchClickPatternDetectorTrait::new();
// //     let mut mock_dynamic_switch_event_responses = MockDynamicSwitchEventReactionsTrait::new();
// //     let mock_gilrs_events = MockGilrsEventsTrait::new();
// //
// //     let mut seq = Sequence::new();
// //     let mut indexes_vec:Vec<usize> = Vec::new();
// //
// //
// //     let switch = Switch::Button(Button::North);
// //     let index_in_gamepad = 1;
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::Click(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::Click))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::MoveToOrVisitLayer(LayerSpecifier {
// //             id: String::new(),
// //             index_in_gamepad: Some(index_in_gamepad)
// //         }))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_add()
// //         .with(eq(switch.clone()),eq(
// //             DynamicSwitchEventReaction::DoubleClickAndHold(
// //                 EventReactionOrNesting::Nesting(
// //                     Box::new(DynamicSwitchEventReaction::ClickEnd(
// //                         EventReactionOrNesting::Reaction(
// //                             SwitchOnClickReaction::MoveToLayer(
// //                                 LayerSpecifier {
// //                                     id: String::new(),
// //                                     index_in_gamepad: Some(*indexes_vec.last().or(Some(&0)).unwrap())
// //                     }))))))))
// //         .times(1)
// //         .return_const(())
// //         .in_sequence(&mut seq);
// //     indexes_vec.push(index_in_gamepad);
// //
// //
// //     let switch = Switch::Button(Button::East);
// //     let index_in_gamepad = 123;
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::ClickAndHold(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::ClickAndHold))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::MoveToOrVisitLayer(LayerSpecifier {
// //             id: String::new(),
// //             index_in_gamepad: Some(index_in_gamepad)
// //         }))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_add()
// //         .with(eq(switch.clone()),eq(
// //             DynamicSwitchEventReaction::DoubleClickAndHold(
// //                 EventReactionOrNesting::Nesting(
// //                     Box::new(DynamicSwitchEventReaction::ClickEnd(
// //                         EventReactionOrNesting::Reaction(
// //                             SwitchOnClickReaction::MoveToLayer(
// //                                 LayerSpecifier {
// //                                     id: String::new(),
// //                                     index_in_gamepad: Some(*indexes_vec.last().or(Some(&0)).unwrap())
// //                     }))))))))
// //         .times(1)
// //         .return_const(())
// //         .in_sequence(&mut seq);
// //     indexes_vec.push(index_in_gamepad);
// //    
// //    
// //     let switch = Switch::Button(Button::North);
// //     let index_in_gamepad = 123;
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::ClickAndHold(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::ClickAndHold))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::MoveToOrVisitLayer(LayerSpecifier {
// //             id: String::new(),
// //             index_in_gamepad: Some(index_in_gamepad)
// //         }))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_add()
// //         .with(eq(switch.clone()),eq(
// //             DynamicSwitchEventReaction::DoubleClickAndHold(
// //                 EventReactionOrNesting::Nesting(
// //                     Box::new(DynamicSwitchEventReaction::ClickEnd(
// //                         EventReactionOrNesting::Reaction(
// //                             SwitchOnClickReaction::MoveToLayer(
// //                                 LayerSpecifier {
// //                                     id: String::new(),
// //                                     index_in_gamepad: Some(*indexes_vec.last().or(Some(&0)).unwrap())
// //                     }))))))))
// //         .times(1)
// //         .return_const(())
// //         .in_sequence(&mut seq);
// //     indexes_vec.push(index_in_gamepad);
// //    
// //    
// //     let switch = Switch::StickSwitchButton(StickSwitchButton::LeftStickLeft);
// //     let index_in_gamepad = 99;
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::DoubleClick(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::DoubleClick))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::MoveToOrVisitLayer(LayerSpecifier {
// //             id: String::new(),
// //             index_in_gamepad: Some(index_in_gamepad)
// //         }))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_add()
// //         .with(eq(switch.clone()),eq(
// //             DynamicSwitchEventReaction::DoubleClickAndHold(
// //                 EventReactionOrNesting::Nesting(
// //                     Box::new(DynamicSwitchEventReaction::ClickEnd(
// //                         EventReactionOrNesting::Reaction(
// //                             SwitchOnClickReaction::MoveToLayer(
// //                                 LayerSpecifier {
// //                                     id: String::new(),
// //                                     index_in_gamepad: Some(*indexes_vec.last().or(Some(&0)).unwrap())
// //                     }))))))))
// //         .times(1)
// //         .return_const(())
// //         .in_sequence(&mut seq);
// //     indexes_vec.push(index_in_gamepad);
// //    
// //    
// //     let switch = Switch::StickSwitchButton(StickSwitchButton::RightStickRight);
// //     let index_in_gamepad = 8900;
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::DoubleClickAndHold(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::DoubleClickAndHold))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::MoveToOrVisitLayer(LayerSpecifier {
// //             id: String::new(),
// //             index_in_gamepad: Some(index_in_gamepad)
// //         }))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_add()
// //         .with(eq(switch.clone()),eq(
// //             DynamicSwitchEventReaction::DoubleClickAndHold(
// //                 EventReactionOrNesting::Nesting(
// //                     Box::new(DynamicSwitchEventReaction::ClickEnd(
// //                         EventReactionOrNesting::Reaction(
// //                             SwitchOnClickReaction::MoveToLayer(
// //                                 LayerSpecifier {
// //                                     id: String::new(),
// //                                     index_in_gamepad: Some(*indexes_vec.last().or(Some(&0)).unwrap())
// //                     }))))))))
// //         .times(1)
// //         .return_const(())
// //         .in_sequence(&mut seq);
// //     indexes_vec.push(index_in_gamepad);
// //    
// //    
// //     let switch = Switch::StickSwitchButton(StickSwitchButton::RightStickUp);
// //     let index_in_gamepad = 89;
// //     mock_switch_click_pattern_detector
// //         .expect_tick()
// //         .times(1)
// //         .return_const(Some(SwitchClickPattern::ClickEnd(switch.clone())))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_get()
// //         .with(eq(switch.clone()),eq(Handle::ClickEnd))
// //         .times(1)
// //         .return_const(SwitchOnClickReaction::MoveToOrVisitLayer(LayerSpecifier {
// //             id: String::new(),
// //             index_in_gamepad: Some(index_in_gamepad)
// //         }))
// //         .in_sequence(&mut seq);
// //     mock_dynamic_switch_event_responses
// //         .expect_add()
// //         .with(eq(switch.clone()),eq(
// //             DynamicSwitchEventReaction::DoubleClickAndHold(
// //                 EventReactionOrNesting::Nesting(
// //                     Box::new(DynamicSwitchEventReaction::ClickEnd(
// //                         EventReactionOrNesting::Reaction(
// //                             SwitchOnClickReaction::MoveToLayer(
// //                                 LayerSpecifier {
// //                                     id: String::new(),
// //                                     index_in_gamepad: Some(*indexes_vec.last().or(Some(&0)).unwrap())
// //                     }))))))))
// //         .times(1)
// //         .return_const(())
// //         .in_sequence(&mut seq);
// //     indexes_vec.push(index_in_gamepad);
// //
// //    
// //     let mut gamepad = Gamepad {
// //        gilrs_events: Box::new(mock_gilrs_events),
// //        layers: setup_layers_examples(),
// //        current_layer_index: 0,
// //        switch_click_pattern_detector: Box::new(mock_switch_click_pattern_detector),
// //        dynamic_switch_event_responses: Box::new(mock_dynamic_switch_event_responses),
// //     };
// //
// //     for result in indexes_vec.iter() {
// //         assert!(gamepad.tick().is_none());
// //         assert_eq!(gamepad.current_layer_index,*result);
// //         assert_eq!(gamepad.layers,setup_layers_examples());
// //     }
// // }
