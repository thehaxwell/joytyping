use enigo::Key;
use gilrs::Button;
use mockall::{predicate::*, Sequence};

use crate::{gamepad::{switch_click_pattern_detector::{MockSwitchClickPatternDetectorTrait, SwitchClickPattern}, gilrs_events::MockGilrsEventsTrait, layers_navigator::MockLayersNavigatorTrait, cardinal_levers_move_detector::{MockCardinalLeversMoveDetectorTrait, self}, Gamepad, InputEvent, Switch}, quick_lookup_window::{QuickLookupWindowTrait, MockQuickLookupWindowTrait}, settings::{self, data::{LayerSpecifier, SwitchEventAndReaction, SwitchOnClickReaction, KeyboardInput}}};

use super::super::{gilrs_events::stick_switch_interpreter::StickSwitchButton, layers_wrapper::MockLayersWrapperTrait, layers_navigator::LayerVisitTrigger};

fn setup_handles_keyboard_input_events(
    layer_num: usize, source_switch_event_and_reaction: SwitchEventAndReaction,pattern: SwitchClickPattern) -> Option<InputEvent> {
        let mut mock_switch_click_pattern_detector = MockSwitchClickPatternDetectorTrait::new();
        let mut mock_layers_wrapper = MockLayersWrapperTrait::new();
        let mock_gilrs_events = MockGilrsEventsTrait::new();
        let mut mock_layers_navigator = MockLayersNavigatorTrait::new();
        let mock_quick_lookup_window = MockQuickLookupWindowTrait::new();
        let mock_mouse_cardinal_levers_move_detector 
            = cardinal_levers_move_detector::mouse::MockMouseTrait::new();

        mock_switch_click_pattern_detector
            .expect_tick()
            .times(1)
            .return_const(Some(pattern.clone()));

        mock_layers_navigator
            .expect_process_current_potential_visit()
            .times(1)
            .with(eq(pattern.clone()))
            .return_const(());

        let switch = match pattern {
            SwitchClickPattern::Click(sw) => sw,
            SwitchClickPattern::ClickAndHold(sw) => sw,
            SwitchClickPattern::DoubleClick(sw) => sw,
            SwitchClickPattern::DoubleClickAndHold(sw) => sw,
            SwitchClickPattern::ClickEnd(sw) => sw,
        };

        mock_layers_wrapper
            .expect_get_switch_event_and_reaction()
            .times(1)
            .with(eq(layer_num),eq(switch))
            .return_const(
                Some(source_switch_event_and_reaction));

        mock_layers_navigator
            .expect_get_current_layer_index()
            .times(1)
            .return_const(layer_num);

        let mut gamepad = Gamepad {
           gilrs_events: Box::new(mock_gilrs_events),
           layers: Box::new(mock_layers_wrapper),
           switch_click_pattern_detector: Box::new(mock_switch_click_pattern_detector),
           layers_navigator: Box::new(mock_layers_navigator),
           quick_lookup_window: Box::new(mock_quick_lookup_window),
           mouse_cardinal_levers_move_detector: 
               Box::new(mock_mouse_cardinal_levers_move_detector),
        };

        gamepad.tick()
}

#[test]
fn handles_keyboard_input_events(){
        let keyboard_input = KeyboardInput{
            key: Key::Layout('I'),
            modifiers: vec![]
        };
       assert_eq!(
           setup_handles_keyboard_input_events(
               0_usize,
               SwitchEventAndReaction {
                    on_click: Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
                    on_double_click: None, 
                },
               SwitchClickPattern::Click(Switch::Button(Button::South)),).unwrap(),
            InputEvent::KeyClick(keyboard_input.clone()));

        let keyboard_input = KeyboardInput{
            key: Key::Layout('*'),
            modifiers: vec![]
        };
       assert_eq!(
           setup_handles_keyboard_input_events(
               1_usize,
               SwitchEventAndReaction {
					on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
                    on_double_click: None, 
				},
               SwitchClickPattern::Click(Switch::Button(Button::South)),).unwrap(),
            InputEvent::KeyClick(keyboard_input.clone()));
       
        let keyboard_input = KeyboardInput{
            key: Key::Tab,
            modifiers: vec![]
        };
       assert_eq!(
           setup_handles_keyboard_input_events(
               2_usize,
               SwitchEventAndReaction {
					on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
                    on_double_click: None, 
				},
               SwitchClickPattern::Click(Switch::Button(Button::South)),).unwrap(),
            InputEvent::KeyClick(keyboard_input.clone()));
       
        let keyboard_input = KeyboardInput{
            key: Key::Tab,
            modifiers: vec![Key::Shift]
        };
       assert_eq!(
           setup_handles_keyboard_input_events(
               10_usize,
               SwitchEventAndReaction {
					on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
                    on_double_click: None, 
				},
               SwitchClickPattern::Click(Switch::Button(Button::South)),).unwrap(),
            InputEvent::KeyClick(keyboard_input.clone()));
       
        let keyboard_input = KeyboardInput{
            key: Key::Layout('a'),
            modifiers: vec![Key::Control, Key::Shift]
        };
       assert_eq!(
           setup_handles_keyboard_input_events(
               100_usize,
               SwitchEventAndReaction {
					on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
                    on_double_click: None, 
				},
               SwitchClickPattern::Click(Switch::Button(Button::South)),).unwrap(),
            InputEvent::KeyClick(keyboard_input.clone()));
       
       
       
       // ----------------
       // different SwitchClickPattern::Click triggering switches
       // ----------------
        let keyboard_input = KeyboardInput{
            key: Key::Layout('I'),
            modifiers: vec![]
        };
       assert_eq!(
           setup_handles_keyboard_input_events(
               0_usize,
               SwitchEventAndReaction {
					on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
                    on_double_click: None, 
				},
               SwitchClickPattern::Click(Switch::Button(Button::LeftTrigger2)),).unwrap(),
            InputEvent::KeyClick(keyboard_input.clone()));
       
        let keyboard_input = KeyboardInput{
            key: Key::Layout('*'),
            modifiers: vec![]
        };
       assert_eq!(
           setup_handles_keyboard_input_events(
               1_usize,
               SwitchEventAndReaction {
					on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
                    on_double_click: None, 
				},
               SwitchClickPattern::Click(Switch::StickSwitchButton(StickSwitchButton::RightStickDown)),).unwrap(),
            InputEvent::KeyClick(keyboard_input.clone()));
       
        let keyboard_input = KeyboardInput{
            key: Key::Tab,
            modifiers: vec![]
        };
       assert_eq!(
           setup_handles_keyboard_input_events(
               2_usize,
               SwitchEventAndReaction {
					on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
                    on_double_click: None, 
				},
               SwitchClickPattern::Click(Switch::StickSwitchButton(StickSwitchButton::LeftStickDown)),).unwrap(),
            InputEvent::KeyClick(keyboard_input.clone()));
       
        let keyboard_input = KeyboardInput{
            key: Key::Tab,
            modifiers: vec![Key::Shift]
        };
       assert_eq!(
           setup_handles_keyboard_input_events(
               10_usize,
               SwitchEventAndReaction {
					on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
                    on_double_click: None, 
				},
               SwitchClickPattern::Click(Switch::Button(Button::Start)),).unwrap(),
            InputEvent::KeyClick(keyboard_input.clone()));
       
        let keyboard_input = KeyboardInput{
            key: Key::Layout('a'),
            modifiers: vec![Key::Control, Key::Shift]
        };
       assert_eq!(
           setup_handles_keyboard_input_events(
               100_usize,
               SwitchEventAndReaction {
					on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
                    on_double_click: None, 
				},
               SwitchClickPattern::Click(Switch::Button(Button::DPadRight)),).unwrap(),
            InputEvent::KeyClick(keyboard_input.clone()));
       
       
        // -------------
        // DoubleClick count as Click if on_double_click wasn't set
        // -------------
        let keyboard_input = KeyboardInput{
            key: Key::Layout('I'),
            modifiers: vec![]
        };
       assert_eq!(
           setup_handles_keyboard_input_events(
               0_usize,
               SwitchEventAndReaction {
					on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
                    on_double_click: None, 
				},
               SwitchClickPattern::DoubleClick(Switch::Button(Button::LeftTrigger2)),).unwrap(),
            InputEvent::KeyClick(keyboard_input.clone()));
       
        let keyboard_input = KeyboardInput{
            key: Key::Layout('*'),
            modifiers: vec![]
        };
       assert_eq!(
           setup_handles_keyboard_input_events(
               1_usize,
               SwitchEventAndReaction {
					on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
                    on_double_click: None, 
				},
               SwitchClickPattern::DoubleClick(Switch::StickSwitchButton(StickSwitchButton::RightStickDown)),).unwrap(),
            InputEvent::KeyClick(keyboard_input.clone()));
       
        let keyboard_input = KeyboardInput{
            key: Key::Tab,
            modifiers: vec![]
        };
       assert_eq!(
           setup_handles_keyboard_input_events(
               2_usize,
               SwitchEventAndReaction {
					on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
                    on_double_click: None, 
				},
               SwitchClickPattern::DoubleClick(Switch::StickSwitchButton(StickSwitchButton::LeftStickDown)),).unwrap(),
            InputEvent::KeyClick(keyboard_input.clone()));
       
        let keyboard_input = KeyboardInput{
            key: Key::Tab,
            modifiers: vec![Key::Shift]
        };
       assert_eq!(
           setup_handles_keyboard_input_events(
               10_usize,
               SwitchEventAndReaction {
					on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
                    on_double_click: None, 
				},
               SwitchClickPattern::DoubleClick(Switch::Button(Button::Start)),).unwrap(),
            InputEvent::KeyClick(keyboard_input.clone()));
       
        let keyboard_input = KeyboardInput{
            key: Key::Layout('a'),
            modifiers: vec![Key::Control, Key::Shift]
        };
       assert_eq!(
           setup_handles_keyboard_input_events(
               100_usize,
               SwitchEventAndReaction {
					on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
                    on_double_click: None, 
				},
               SwitchClickPattern::DoubleClick(Switch::Button(Button::DPadRight)),).unwrap(),
            InputEvent::KeyClick(keyboard_input.clone()));
       
       
       // ----------------
       // SwitchClickPattern::ClickAndHold triggers Keydown
       // ----------------
        let keyboard_input = KeyboardInput{
            key: Key::Layout('I'),
            modifiers: vec![]
        };
       assert_eq!(
           setup_handles_keyboard_input_events(
               0_usize,
               SwitchEventAndReaction {
					on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
                    on_double_click: None, 
				},
               SwitchClickPattern::ClickAndHold(Switch::Button(Button::LeftTrigger2)),).unwrap(),
            InputEvent::KeyDown(keyboard_input.clone()));
       
        let keyboard_input = KeyboardInput{
            key: Key::Layout('*'),
            modifiers: vec![]
        };
       assert_eq!(
           setup_handles_keyboard_input_events(
               1_usize,
               SwitchEventAndReaction {
					on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
                    on_double_click: None, 
				},
               SwitchClickPattern::ClickAndHold(Switch::StickSwitchButton(StickSwitchButton::RightStickDown)),).unwrap(),
            InputEvent::KeyDown(keyboard_input.clone()));
       
        let keyboard_input = KeyboardInput{
            key: Key::Tab,
            modifiers: vec![]
        };
       assert_eq!(
           setup_handles_keyboard_input_events(
               2_usize,
               SwitchEventAndReaction {
					on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
                    on_double_click: None, 
				},
               SwitchClickPattern::ClickAndHold(Switch::StickSwitchButton(StickSwitchButton::LeftStickDown)),).unwrap(),
            InputEvent::KeyDown(keyboard_input.clone()));
       
        let keyboard_input = KeyboardInput{
            key: Key::Tab,
            modifiers: vec![Key::Shift]
        };
       assert_eq!(
           setup_handles_keyboard_input_events(
               10_usize,
               SwitchEventAndReaction {
					on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
                    on_double_click: None, 
				},
               SwitchClickPattern::ClickAndHold(Switch::Button(Button::Start)),).unwrap(),
            InputEvent::KeyDown(keyboard_input.clone()));
       
        let keyboard_input = KeyboardInput{
            key: Key::Layout('a'),
            modifiers: vec![Key::Control, Key::Shift]
        };
       assert_eq!(
           setup_handles_keyboard_input_events(
               100_usize,
               SwitchEventAndReaction {
					on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
                    on_double_click: None, 
				},
               SwitchClickPattern::ClickAndHold(Switch::Button(Button::DPadRight)),).unwrap(),
            InputEvent::KeyDown(keyboard_input.clone()));
       
       
       // ----------------
       // SwitchClickPattern::ClickAndHold triggers Keydown
       // ----------------
        let keyboard_input = KeyboardInput{
            key: Key::Layout('I'),
            modifiers: vec![]
        };
       assert_eq!(
           setup_handles_keyboard_input_events(
               0_usize,
               SwitchEventAndReaction {
					on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
                    on_double_click: None, 
				},
               SwitchClickPattern::ClickAndHold(Switch::Button(Button::LeftTrigger2)),).unwrap(),
            InputEvent::KeyDown(keyboard_input.clone()));
       
        let keyboard_input = KeyboardInput{
            key: Key::Layout('*'),
            modifiers: vec![]
        };
       assert_eq!(
           setup_handles_keyboard_input_events(
               1_usize,
               SwitchEventAndReaction {
					on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
                    on_double_click: None, 
				},
               SwitchClickPattern::ClickAndHold(Switch::StickSwitchButton(StickSwitchButton::RightStickDown)),).unwrap(),
            InputEvent::KeyDown(keyboard_input.clone()));
       
        let keyboard_input = KeyboardInput{
            key: Key::Tab,
            modifiers: vec![]
        };
       assert_eq!(
           setup_handles_keyboard_input_events(
               2_usize,
               SwitchEventAndReaction {
					on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
                    on_double_click: None, 
				},
               SwitchClickPattern::ClickAndHold(Switch::StickSwitchButton(StickSwitchButton::LeftStickDown)),).unwrap(),
            InputEvent::KeyDown(keyboard_input.clone()));
       
        let keyboard_input = KeyboardInput{
            key: Key::Tab,
            modifiers: vec![Key::Shift]
        };
       assert_eq!(
           setup_handles_keyboard_input_events(
               10_usize,
               SwitchEventAndReaction {
					on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
                    on_double_click: None, 
				},
               SwitchClickPattern::ClickAndHold(Switch::Button(Button::Start)),).unwrap(),
            InputEvent::KeyDown(keyboard_input.clone()));
       
        let keyboard_input = KeyboardInput{
            key: Key::Layout('a'),
            modifiers: vec![Key::Control, Key::Shift]
        };
       assert_eq!(
           setup_handles_keyboard_input_events(
               100_usize,
               SwitchEventAndReaction {
					on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
                    on_double_click: None, 
				},
               SwitchClickPattern::ClickAndHold(Switch::Button(Button::DPadRight)),).unwrap(),
            InputEvent::KeyDown(keyboard_input.clone()));
       
       
       // ----------------
       // SwitchClickPattern::DoubleClickAndHold triggers Keydown
       // ----------------
        let keyboard_input = KeyboardInput{
            key: Key::Layout('I'),
            modifiers: vec![]
        };
       assert_eq!(
           setup_handles_keyboard_input_events(
               0_usize,
               SwitchEventAndReaction {
					on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
                    on_double_click: None, 
				},
               SwitchClickPattern::DoubleClickAndHold(Switch::Button(Button::LeftTrigger2)),).unwrap(),
            InputEvent::KeyDown(keyboard_input.clone()));
       
        let keyboard_input = KeyboardInput{
            key: Key::Layout('*'),
            modifiers: vec![]
        };
       assert_eq!(
           setup_handles_keyboard_input_events(
               1_usize,
               SwitchEventAndReaction {
					on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
                    on_double_click: None, 
				},
               SwitchClickPattern::DoubleClickAndHold(Switch::StickSwitchButton(StickSwitchButton::RightStickDown)),).unwrap(),
            InputEvent::KeyDown(keyboard_input.clone()));
       
        let keyboard_input = KeyboardInput{
            key: Key::Tab,
            modifiers: vec![]
        };
       assert_eq!(
           setup_handles_keyboard_input_events(
               2_usize,
               SwitchEventAndReaction {
					on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
                    on_double_click: None, 
				},
               SwitchClickPattern::DoubleClickAndHold(Switch::StickSwitchButton(StickSwitchButton::LeftStickDown)),).unwrap(),
            InputEvent::KeyDown(keyboard_input.clone()));
       
        let keyboard_input = KeyboardInput{
            key: Key::Tab,
            modifiers: vec![Key::Shift]
        };
       assert_eq!(
           setup_handles_keyboard_input_events(
               10_usize,
               SwitchEventAndReaction {
					on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
                    on_double_click: None, 
				},
               SwitchClickPattern::DoubleClickAndHold(Switch::Button(Button::Start)),).unwrap(),
            InputEvent::KeyDown(keyboard_input.clone()));
       
        let keyboard_input = KeyboardInput{
            key: Key::Layout('a'),
            modifiers: vec![Key::Control, Key::Shift]
        };
       assert_eq!(
           setup_handles_keyboard_input_events(
               100_usize,
               SwitchEventAndReaction {
					on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
                    on_double_click: None, 
				},
               SwitchClickPattern::DoubleClickAndHold(Switch::Button(Button::DPadRight)),).unwrap(),
            InputEvent::KeyDown(keyboard_input.clone()));


        // -------------
        // DoubleClick triggers on_double_click key_click when it's set
        // -------------
        let keyboard_input = KeyboardInput{
            key: Key::Layout('I'),
            modifiers: vec![]
        };
       assert_eq!(
           setup_handles_keyboard_input_events(
               0_usize,
               SwitchEventAndReaction {
					on_click: None,
                    on_double_click: Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())), 
				},
               SwitchClickPattern::DoubleClick(Switch::Button(Button::LeftTrigger2)),).unwrap(),
            InputEvent::KeyClick(keyboard_input.clone()));
       
        let keyboard_input = KeyboardInput{
            key: Key::Layout('*'),
            modifiers: vec![]
        };
       assert_eq!(
           setup_handles_keyboard_input_events(
               1_usize,
               SwitchEventAndReaction {
					on_click: None,
                    on_double_click: Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())), 
				},
               SwitchClickPattern::DoubleClick(Switch::StickSwitchButton(StickSwitchButton::RightStickDown)),).unwrap(),
            InputEvent::KeyClick(keyboard_input.clone()));
       
        let keyboard_input = KeyboardInput{
            key: Key::Tab,
            modifiers: vec![]
        };
       assert_eq!(
           setup_handles_keyboard_input_events(
               2_usize,
               SwitchEventAndReaction {
					on_click: None,
                    on_double_click: Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())), 
				},
               SwitchClickPattern::DoubleClick(Switch::StickSwitchButton(StickSwitchButton::LeftStickDown)),).unwrap(),
            InputEvent::KeyClick(keyboard_input.clone()));
       
        let keyboard_input = KeyboardInput{
            key: Key::Tab,
            modifiers: vec![Key::Shift]
        };
       assert_eq!(
           setup_handles_keyboard_input_events(
               10_usize,
               SwitchEventAndReaction {
					on_click: None,
                    on_double_click: Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())), 
				},
               SwitchClickPattern::DoubleClick(Switch::Button(Button::Start)),).unwrap(),
            InputEvent::KeyClick(keyboard_input.clone()));
       
        let keyboard_input = KeyboardInput{
            key: Key::Layout('a'),
            modifiers: vec![Key::Control, Key::Shift]
        };
       assert_eq!(
           setup_handles_keyboard_input_events(
               100_usize,
               SwitchEventAndReaction {
					on_click: None,
                    on_double_click: Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())), 
				},
               SwitchClickPattern::DoubleClick(Switch::Button(Button::DPadRight)),).unwrap(),
            InputEvent::KeyClick(keyboard_input.clone()));
       
       
}

struct SetupGamepadTickHandlesMoveToLayerEventsArgs{
    current_layer_num: usize,
    new_layer_index: usize,
    layer_specifier: LayerSpecifier, 
    source_switch_event_and_reaction: SwitchEventAndReaction,
    pattern: SwitchClickPattern,
}

fn setup_handles_move_to_layer_events(
    args: SetupGamepadTickHandlesMoveToLayerEventsArgs
    ) -> Option<InputEvent> {

        let mut mock_switch_click_pattern_detector = MockSwitchClickPatternDetectorTrait::new();
        let mut mock_layers_wrapper = MockLayersWrapperTrait::new();
        let mock_gilrs_events = MockGilrsEventsTrait::new();
        let mut mock_layers_navigator = MockLayersNavigatorTrait::new();
        let mut mock_quick_lookup_window = MockQuickLookupWindowTrait::new();
        let mut mock_mouse_cardinal_levers_move_detector 
            = cardinal_levers_move_detector::mouse::MockMouseTrait::new();

        mock_switch_click_pattern_detector
            .expect_tick()
            .times(1)
            .return_const(Some(args.pattern.clone()));

        mock_layers_navigator
            .expect_process_current_potential_visit()
            .times(1)
            .with(eq(args.pattern.clone()))
            .return_const(());

        let switch = match args.pattern {
            SwitchClickPattern::Click(sw) => sw,
            SwitchClickPattern::ClickAndHold(sw) => sw,
            SwitchClickPattern::DoubleClick(sw) => sw,
            SwitchClickPattern::DoubleClickAndHold(sw) => sw,
            SwitchClickPattern::ClickEnd(sw) => sw,
        };

        mock_layers_wrapper
            .expect_get_switch_event_and_reaction()
            .times(1)
            .with(eq(args.current_layer_num),eq(switch))
            .return_const(
                Some(args.source_switch_event_and_reaction));

        mock_layers_navigator
            .expect_get_current_layer_index()
            .times(1)
            .return_const(args.current_layer_num);

        mock_layers_navigator
            .expect_move_to_layer()
            .times(1)
            .with(eq(args.layer_specifier))
            .return_const(());

        mock_layers_navigator
            .expect_consumable_get_current_layer_index()
            .times(1)
            .with()
            .return_const(Some(args.new_layer_index));


        mock_quick_lookup_window
            .expect_update()
            .times(1)
            .with(eq(args.new_layer_index))
            .returning(|_| Ok(()));

        mock_layers_wrapper
            .expect_get_cardinal_levers()
            .times(1)
            .with(eq(args.new_layer_index))
            .return_const(None);

        mock_mouse_cardinal_levers_move_detector
            .expect_set_mouse_controls()
            .times(1)
            .with(eq(None))
            .return_const(());

        mock_mouse_cardinal_levers_move_detector
            .expect_tick()
            .times(1)
            .return_const(None);

        let mut gamepad = Gamepad {
           gilrs_events: Box::new(mock_gilrs_events),
           layers: Box::new(mock_layers_wrapper),
           switch_click_pattern_detector: Box::new(mock_switch_click_pattern_detector),
           layers_navigator: Box::new(mock_layers_navigator),
           quick_lookup_window: Box::new(mock_quick_lookup_window),
           mouse_cardinal_levers_move_detector: 
               Box::new(mock_mouse_cardinal_levers_move_detector),
        };

        gamepad.tick()
}

#[test]
fn handles_move_to_layer_events(){
    let new_layer_index: usize = 101_usize;
    let layer_specifier = LayerSpecifier {
        id: "some-id".to_string(), index_in_gamepad: Some(new_layer_index)};
    assert!(setup_handles_move_to_layer_events(
        SetupGamepadTickHandlesMoveToLayerEventsArgs {
            current_layer_num: 100_usize,
            new_layer_index,
            layer_specifier: layer_specifier.clone(),
            source_switch_event_and_reaction: SwitchEventAndReaction {
                on_click: Some(SwitchOnClickReaction::MoveToLayer(layer_specifier.clone())),
                on_double_click: None, 
            },
            pattern: SwitchClickPattern::Click(Switch::Button(Button::DPadRight)),
        }
    ).is_none());

    let new_layer_index: usize = 12_usize;
    let layer_specifier = LayerSpecifier {
        id: "some-other-id".to_string(), index_in_gamepad: Some(new_layer_index)};
    assert!(setup_handles_move_to_layer_events(
        SetupGamepadTickHandlesMoveToLayerEventsArgs {
            current_layer_num: 10_usize,
            new_layer_index,
            layer_specifier: layer_specifier.clone(),
            source_switch_event_and_reaction: SwitchEventAndReaction {
                on_click: None,
                on_double_click: Some(SwitchOnClickReaction::MoveToLayer(layer_specifier.clone())), 
            },
            pattern: SwitchClickPattern::DoubleClick(Switch::Button(Button::South)),
        }
    ).is_none());


    let new_layer_index: usize = 0_usize;
    let layer_specifier = LayerSpecifier {
        id: "some-id".to_string(), index_in_gamepad: Some(new_layer_index)};
    assert!(setup_handles_move_to_layer_events(
        SetupGamepadTickHandlesMoveToLayerEventsArgs {
            current_layer_num: 10_usize,
            new_layer_index,
            layer_specifier: layer_specifier.clone(),
            source_switch_event_and_reaction: SwitchEventAndReaction {
                on_click: Some(SwitchOnClickReaction::MoveToLayer(layer_specifier.clone())),
                on_double_click: None, 
            },
            pattern: SwitchClickPattern::DoubleClick(Switch::Button(Button::South)),
        }
    ).is_none());

}


struct SetupGamepadTickHandlesVisitLayerEventsArgs{
    current_layer_num: usize,
    new_layer_index: usize,
    layer_specifier: LayerSpecifier, 
    source_switch_event_and_reaction: SwitchEventAndReaction,
    pattern: SwitchClickPattern,
}

fn setup_handles_visit_layer_events(
    args: SetupGamepadTickHandlesVisitLayerEventsArgs
    ) -> Option<InputEvent> {

        let mut mock_switch_click_pattern_detector = MockSwitchClickPatternDetectorTrait::new();
        let mut mock_layers_wrapper = MockLayersWrapperTrait::new();
        let mock_gilrs_events = MockGilrsEventsTrait::new();
        let mut mock_layers_navigator = MockLayersNavigatorTrait::new();
        let mut mock_quick_lookup_window = MockQuickLookupWindowTrait::new();
        let mut mock_mouse_cardinal_levers_move_detector 
            = cardinal_levers_move_detector::mouse::MockMouseTrait::new();

        mock_switch_click_pattern_detector
            .expect_tick()
            .times(1)
            .return_const(Some(args.pattern.clone()));

        mock_layers_navigator
            .expect_process_current_potential_visit()
            .times(1)
            .with(eq(args.pattern.clone()))
            .return_const(());

        let switch = match args.pattern.clone() {
            SwitchClickPattern::Click(sw) => sw,
            SwitchClickPattern::ClickAndHold(sw) => sw,
            SwitchClickPattern::DoubleClick(sw) => sw,
            SwitchClickPattern::DoubleClickAndHold(sw) => sw,
            SwitchClickPattern::ClickEnd(sw) => sw,
        };

        mock_layers_wrapper
            .expect_get_switch_event_and_reaction()
            .times(1)
            .with(eq(args.current_layer_num),eq(switch))
            .return_const(
                Some(args.source_switch_event_and_reaction));

        mock_layers_navigator
            .expect_get_current_layer_index()
            .times(1)
            .return_const(args.current_layer_num);

        match args.pattern {
            SwitchClickPattern::Click(sw)=> {
                mock_layers_navigator
                    .expect_visit_layer()
                    .times(1)
                    .with(eq(LayerVisitTrigger::Click(sw)),eq(args.layer_specifier))
                    .return_const(());
            },
            SwitchClickPattern::DoubleClick(sw)=> {
                mock_layers_navigator
                    .expect_visit_layer()
                    .times(1)
                    .with(eq(LayerVisitTrigger::DoubleClick(sw)),eq(args.layer_specifier))
                    .return_const(());
            },
            _ => (),
        };

        mock_layers_navigator
            .expect_consumable_get_current_layer_index()
            .times(1)
            .with()
            .return_const(Some(args.new_layer_index));


        mock_quick_lookup_window
            .expect_update()
            .times(1)
            .with(eq(args.new_layer_index))
            .returning(|_| Ok(()));

        mock_layers_wrapper
            .expect_get_cardinal_levers()
            .times(1)
            .with(eq(args.new_layer_index))
            .return_const(None);

        mock_mouse_cardinal_levers_move_detector
            .expect_set_mouse_controls()
            .times(1)
            .with(eq(None))
            .return_const(());

        mock_mouse_cardinal_levers_move_detector
            .expect_tick()
            .times(1)
            .return_const(None);

        let mut gamepad = Gamepad {
           gilrs_events: Box::new(mock_gilrs_events),
           layers: Box::new(mock_layers_wrapper),
           switch_click_pattern_detector: Box::new(mock_switch_click_pattern_detector),
           layers_navigator: Box::new(mock_layers_navigator),
           quick_lookup_window: Box::new(mock_quick_lookup_window),
           mouse_cardinal_levers_move_detector: 
               Box::new(mock_mouse_cardinal_levers_move_detector),
        };

        gamepad.tick()
}

#[test]
fn handles_visit_layer_events() {
    let new_layer_index: usize = 101_usize;
    let layer_specifier = LayerSpecifier {
        id: "some-id".to_string(), index_in_gamepad: Some(new_layer_index)};
    assert!(setup_handles_visit_layer_events(
        SetupGamepadTickHandlesVisitLayerEventsArgs {
            current_layer_num: 100_usize,
            new_layer_index,
            layer_specifier: layer_specifier.clone(),
            source_switch_event_and_reaction: SwitchEventAndReaction {
                on_click: Some(SwitchOnClickReaction::VisitLayer(layer_specifier.clone())),
                on_double_click: None, 
            },
            pattern: SwitchClickPattern::Click(Switch::Button(Button::DPadRight)),
        }
    ).is_none());


    let new_layer_index: usize = 12_usize;
    let layer_specifier = LayerSpecifier {
        id: "some-other-id".to_string(), index_in_gamepad: Some(new_layer_index)};
    assert!(setup_handles_visit_layer_events(
        SetupGamepadTickHandlesVisitLayerEventsArgs {
            current_layer_num: 10_usize,
            new_layer_index,
            layer_specifier: layer_specifier.clone(),
            source_switch_event_and_reaction: SwitchEventAndReaction {
                on_click: None,
                on_double_click: Some(SwitchOnClickReaction::VisitLayer(layer_specifier.clone())), 
            },
            pattern: SwitchClickPattern::DoubleClick(Switch::Button(Button::South)),
        }
    ).is_none());


    let new_layer_index: usize = 0_usize;
    let layer_specifier = LayerSpecifier {
        id: "some-id".to_string(), index_in_gamepad: Some(new_layer_index)};
    assert!(setup_handles_visit_layer_events(
        SetupGamepadTickHandlesVisitLayerEventsArgs {
            current_layer_num: 10_usize,
            new_layer_index,
            layer_specifier: layer_specifier.clone(),
            source_switch_event_and_reaction: SwitchEventAndReaction {
                on_click: Some(SwitchOnClickReaction::VisitLayer(layer_specifier.clone())),
                on_double_click: None, 
            },
            pattern: SwitchClickPattern::DoubleClick(Switch::Button(Button::South)),
        }
    ).is_none());

}



struct SetupGamepadTickHandlesMoveToOrVisitLayerEventsArgs{
    current_layer_num: usize,
    new_layer_index: usize,
    layer_specifier: LayerSpecifier, 
    source_switch_event_and_reaction: SwitchEventAndReaction,
    pattern: SwitchClickPattern,
}

fn setup_handles_move_to_or_visit_layer_events(
    args: SetupGamepadTickHandlesMoveToOrVisitLayerEventsArgs
    ) -> Option<InputEvent> {

        let mut mock_switch_click_pattern_detector = MockSwitchClickPatternDetectorTrait::new();
        let mut mock_layers_wrapper = MockLayersWrapperTrait::new();
        let mock_gilrs_events = MockGilrsEventsTrait::new();
        let mut mock_layers_navigator = MockLayersNavigatorTrait::new();
        let mut mock_quick_lookup_window = MockQuickLookupWindowTrait::new();
        let mut mock_mouse_cardinal_levers_move_detector 
            = cardinal_levers_move_detector::mouse::MockMouseTrait::new();

        mock_switch_click_pattern_detector
            .expect_tick()
            .times(1)
            .return_const(Some(args.pattern.clone()));

        mock_layers_navigator
            .expect_process_current_potential_visit()
            .times(1)
            .with(eq(args.pattern.clone()))
            .return_const(());

        let switch = match args.pattern.clone() {
            SwitchClickPattern::Click(sw) => sw,
            SwitchClickPattern::ClickAndHold(sw) => sw,
            SwitchClickPattern::DoubleClick(sw) => sw,
            SwitchClickPattern::DoubleClickAndHold(sw) => sw,
            SwitchClickPattern::ClickEnd(sw) => sw,
        };

        mock_layers_wrapper
            .expect_get_switch_event_and_reaction()
            .times(1)
            .with(eq(args.current_layer_num),eq(switch))
            .return_const(
                Some(args.source_switch_event_and_reaction));

        mock_layers_navigator
            .expect_get_current_layer_index()
            .times(1)
            .return_const(args.current_layer_num);

        match args.pattern {
            SwitchClickPattern::Click(sw)=> {
                mock_layers_navigator
                    .expect_move_to_or_visit_layer()
                    .times(1)
                    .with(eq(LayerVisitTrigger::Click(sw)),eq(args.layer_specifier))
                    .return_const(());
            },
            SwitchClickPattern::DoubleClick(sw)=> {
                mock_layers_navigator
                    .expect_move_to_or_visit_layer()
                    .times(1)
                    .with(eq(LayerVisitTrigger::DoubleClick(sw)),eq(args.layer_specifier))
                    .return_const(());
            },
            _ => (),
        };

        mock_layers_navigator
            .expect_consumable_get_current_layer_index()
            .times(1)
            .with()
            .return_const(Some(args.new_layer_index));


        mock_quick_lookup_window
            .expect_update()
            .times(1)
            .with(eq(args.new_layer_index))
            .returning(|_| Ok(()));

        mock_layers_wrapper
            .expect_get_cardinal_levers()
            .times(1)
            .with(eq(args.new_layer_index))
            .return_const(None);

        mock_mouse_cardinal_levers_move_detector
            .expect_set_mouse_controls()
            .times(1)
            .with(eq(None))
            .return_const(());

        mock_mouse_cardinal_levers_move_detector
            .expect_tick()
            .times(1)
            .return_const(None);


        let mut gamepad = Gamepad {
           gilrs_events: Box::new(mock_gilrs_events),
           layers: Box::new(mock_layers_wrapper),
           switch_click_pattern_detector: Box::new(mock_switch_click_pattern_detector),
           layers_navigator: Box::new(mock_layers_navigator),
           quick_lookup_window: Box::new(mock_quick_lookup_window),
           mouse_cardinal_levers_move_detector:
               Box::new(mock_mouse_cardinal_levers_move_detector),
        };

        gamepad.tick()
}

#[test]
fn handles_move_to_or_visit_layer_events() {
    let new_layer_index: usize = 101_usize;
    let layer_specifier = LayerSpecifier {
        id: "some-id".to_string(), index_in_gamepad: Some(new_layer_index)};
    assert!(setup_handles_move_to_or_visit_layer_events(
        SetupGamepadTickHandlesMoveToOrVisitLayerEventsArgs {
            current_layer_num: 100_usize,
            new_layer_index,
            layer_specifier: layer_specifier.clone(),
            source_switch_event_and_reaction: SwitchEventAndReaction {
                on_click: Some(SwitchOnClickReaction::MoveToOrVisitLayer(layer_specifier.clone())),
                on_double_click: None, 
            },
            pattern: SwitchClickPattern::Click(Switch::Button(Button::DPadRight)),
        }
    ).is_none());


    let new_layer_index: usize = 12_usize;
    let layer_specifier = LayerSpecifier {
        id: "some-other-id".to_string(), index_in_gamepad: Some(new_layer_index)};
    assert!(setup_handles_move_to_or_visit_layer_events(
        SetupGamepadTickHandlesMoveToOrVisitLayerEventsArgs {
            current_layer_num: 10_usize,
            new_layer_index,
            layer_specifier: layer_specifier.clone(),
            source_switch_event_and_reaction: SwitchEventAndReaction {
                on_click: None,
                on_double_click: Some(SwitchOnClickReaction::MoveToOrVisitLayer(layer_specifier.clone())), 
            },
            pattern: SwitchClickPattern::DoubleClick(Switch::Button(Button::South)),
        }
    ).is_none());


    let new_layer_index: usize = 0_usize;
    let layer_specifier = LayerSpecifier {
        id: "some-id".to_string(), index_in_gamepad: Some(new_layer_index)};
    assert!(setup_handles_move_to_or_visit_layer_events(
        SetupGamepadTickHandlesMoveToOrVisitLayerEventsArgs {
            current_layer_num: 10_usize,
            new_layer_index,
            layer_specifier: layer_specifier.clone(),
            source_switch_event_and_reaction: SwitchEventAndReaction {
                on_click: Some(SwitchOnClickReaction::MoveToOrVisitLayer(layer_specifier.clone())),
                on_double_click: None, 
            },
            pattern: SwitchClickPattern::DoubleClick(Switch::Button(Button::South)),
        }
    ).is_none());

}



struct SetupGamepadTickHandlesShowQuickLookupWindowEventsArgs{
    current_layer_num: usize,
    new_layer_index: usize,
    source_switch_event_and_reaction: SwitchEventAndReaction,
    pattern: SwitchClickPattern,
}

fn setup_handles_show_quick_lookup_window(
    args: SetupGamepadTickHandlesShowQuickLookupWindowEventsArgs
    ) -> Option<InputEvent> {

        let mut mock_switch_click_pattern_detector = MockSwitchClickPatternDetectorTrait::new();
        let mut mock_layers_wrapper = MockLayersWrapperTrait::new();
        let mock_gilrs_events = MockGilrsEventsTrait::new();
        let mut mock_layers_navigator = MockLayersNavigatorTrait::new();
        let mut mock_quick_lookup_window = MockQuickLookupWindowTrait::new();
        let mut mock_mouse_cardinal_levers_move_detector 
            = cardinal_levers_move_detector::mouse::MockMouseTrait::new();

        mock_switch_click_pattern_detector
            .expect_tick()
            .times(1)
            .return_const(Some(args.pattern.clone()));

        mock_layers_navigator
            .expect_process_current_potential_visit()
            .times(1)
            .with(eq(args.pattern.clone()))
            .return_const(());

        let switch = match args.pattern {
            SwitchClickPattern::Click(sw) => sw,
            SwitchClickPattern::ClickAndHold(sw) => sw,
            SwitchClickPattern::DoubleClick(sw) => sw,
            SwitchClickPattern::DoubleClickAndHold(sw) => sw,
            SwitchClickPattern::ClickEnd(sw) => sw,
        };

        mock_layers_wrapper
            .expect_get_switch_event_and_reaction()
            .times(1)
            .with(eq(args.current_layer_num),eq(switch.clone()))
            .return_const(
                Some(args.source_switch_event_and_reaction));

        mock_layers_navigator
            .expect_get_current_layer_index()
            .times(1)
            .return_const(args.current_layer_num);

        mock_quick_lookup_window
            .expect_show_or_open()
            .times(1)
            .with(eq(switch.clone()))
            .returning(|_| Ok(()));

        mock_layers_navigator
            .expect_consumable_get_current_layer_index()
            .times(1)
            .with()
            .return_const(Some(args.new_layer_index));


        mock_quick_lookup_window
            .expect_update()
            .times(1)
            .with(eq(args.new_layer_index))
            .returning(|_| Ok(()));

        mock_layers_wrapper
            .expect_get_cardinal_levers()
            .times(1)
            .with(eq(args.new_layer_index))
            .return_const(None);

        mock_mouse_cardinal_levers_move_detector
            .expect_set_mouse_controls()
            .times(1)
            .with(eq(None))
            .return_const(());

        mock_mouse_cardinal_levers_move_detector
            .expect_tick()
            .times(1)
            .return_const(None);

        let mut gamepad = Gamepad {
           gilrs_events: Box::new(mock_gilrs_events),
           layers: Box::new(mock_layers_wrapper),
           switch_click_pattern_detector: Box::new(mock_switch_click_pattern_detector),
           layers_navigator: Box::new(mock_layers_navigator),
           quick_lookup_window: Box::new(mock_quick_lookup_window),
           mouse_cardinal_levers_move_detector:
               Box::new(mock_mouse_cardinal_levers_move_detector),
        };

        gamepad.tick()
}

#[test]
fn handles_show_quick_lookup_window(){
    let new_layer_index: usize = 101_usize;
    assert!(setup_handles_show_quick_lookup_window(
        SetupGamepadTickHandlesShowQuickLookupWindowEventsArgs {
            current_layer_num: 100_usize,
            new_layer_index,
            source_switch_event_and_reaction: SwitchEventAndReaction {
                on_click: Some(SwitchOnClickReaction::ShowQuickLookupWindow),
                on_double_click: None, 
            },
            pattern: SwitchClickPattern::Click(Switch::Button(Button::DPadRight)),
        }
    ).is_none());

    let new_layer_index: usize = 12_usize;
    assert!(setup_handles_show_quick_lookup_window(
        SetupGamepadTickHandlesShowQuickLookupWindowEventsArgs {
            current_layer_num: 10_usize,
            new_layer_index,
            source_switch_event_and_reaction: SwitchEventAndReaction {
                on_click: None,
                on_double_click: Some(SwitchOnClickReaction::ShowQuickLookupWindow),
            },
            pattern: SwitchClickPattern::DoubleClick(Switch::Button(Button::South)),
        }
    ).is_none());


    let new_layer_index: usize = 0_usize;
    assert!(setup_handles_show_quick_lookup_window(
        SetupGamepadTickHandlesShowQuickLookupWindowEventsArgs {
            current_layer_num: 10_usize,
            new_layer_index,
            source_switch_event_and_reaction: SwitchEventAndReaction {
                on_click: Some(SwitchOnClickReaction::ShowQuickLookupWindow),
                on_double_click: None, 
            },
            pattern: SwitchClickPattern::DoubleClick(Switch::Button(Button::South)),
        }
    ).is_none());

}


struct SetupGamepadTickHandlesClickEndSwitchPatternArgs {
    pattern: SwitchClickPattern,
}

fn setup_processes_click_end_switch_pattern(
    args: SetupGamepadTickHandlesClickEndSwitchPatternArgs
    ) -> Option<InputEvent> {

        let mut mock_switch_click_pattern_detector = MockSwitchClickPatternDetectorTrait::new();
        let mock_layers_wrapper = MockLayersWrapperTrait::new();
        let mock_gilrs_events = MockGilrsEventsTrait::new();
        let mut mock_layers_navigator = MockLayersNavigatorTrait::new();
        let mut mock_quick_lookup_window = MockQuickLookupWindowTrait::new();
        let mock_mouse_cardinal_levers_move_detector 
            = cardinal_levers_move_detector::mouse::MockMouseTrait::new();

        mock_switch_click_pattern_detector
            .expect_tick()
            .times(1)
            .return_const(Some(args.pattern.clone()));

        mock_layers_navigator
            .expect_process_current_potential_visit()
            .times(1)
            .with(eq(args.pattern.clone()))
            .return_const(());

        let switch = match args.pattern {
            SwitchClickPattern::Click(sw) => sw,
            SwitchClickPattern::ClickAndHold(sw) => sw,
            SwitchClickPattern::DoubleClick(sw) => sw,
            SwitchClickPattern::DoubleClickAndHold(sw) => sw,
            SwitchClickPattern::ClickEnd(sw) => sw,
        };

        mock_layers_navigator
            .expect_undo_last_layer_visit_with_switch()
            .times(1)
            .with(eq(switch.clone()))
            .return_const(());

        mock_quick_lookup_window
            .expect_hide()
            .times(1)
            .with(eq(switch.clone()))
            .returning(|_| Ok(()));


        let mut gamepad = Gamepad {
           gilrs_events: Box::new(mock_gilrs_events),
           layers: Box::new(mock_layers_wrapper),
           switch_click_pattern_detector: Box::new(mock_switch_click_pattern_detector),
           layers_navigator: Box::new(mock_layers_navigator),
           quick_lookup_window: Box::new(mock_quick_lookup_window),
           mouse_cardinal_levers_move_detector:
               Box::new(mock_mouse_cardinal_levers_move_detector),
        };

        gamepad.tick()
}

#[test]
fn processes_click_end_switch_pattern(){
    assert_eq!(setup_processes_click_end_switch_pattern(
        SetupGamepadTickHandlesClickEndSwitchPatternArgs {
            pattern: SwitchClickPattern::ClickEnd(Switch::Button(Button::DPadRight)),
        }
    ).unwrap(),InputEvent::KeyUp);

    assert_eq!(setup_processes_click_end_switch_pattern(
        SetupGamepadTickHandlesClickEndSwitchPatternArgs {
            pattern: SwitchClickPattern::ClickEnd(Switch::Button(Button::South)),
        }
    ).unwrap(),InputEvent::KeyUp);


    assert_eq!(setup_processes_click_end_switch_pattern(
        SetupGamepadTickHandlesClickEndSwitchPatternArgs {
            pattern: SwitchClickPattern::ClickEnd(Switch::Button(Button::South)),
        }
    ).unwrap(),InputEvent::KeyUp);

}
