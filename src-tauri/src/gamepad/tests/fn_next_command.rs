use enigo::Key;
use gilrs::Button;
use mockall::predicate::*;

use crate::{gamepad::{switch_click_pattern_detector::{MockSwitchClickPatternDetectorTrait, SwitchClickPattern}, gilrs_events::MockGilrsEventsTrait, layers_navigator::MockLayersNavigatorTrait, cardinal_levers_move_detector, Gamepad, InputEvent, Switch, Command, QuickLookupWindowEvent}, settings::models::layout::{LayerSpecifier, SwitchEventAndReaction, SwitchOnClickReaction, KeyboardInput}};

use super::super::{gilrs_events::stick_switch_interpreter::StickSwitchButton, layers_wrapper::MockLayersWrapperTrait, layers_navigator::LayerVisitTrigger};

fn setup_handles_keyboard_input_events(
    layer_num: usize,
    source_switch_event_and_reaction: SwitchEventAndReaction,
    pattern: SwitchClickPattern) -> Option<Command> {
        let mut mock_switch_click_pattern_detector = MockSwitchClickPatternDetectorTrait::new();
        let mut mock_layers_wrapper = MockLayersWrapperTrait::new();
        let mock_gilrs_events = MockGilrsEventsTrait::new();
        let mut mock_layers_navigator = MockLayersNavigatorTrait::new();
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
           mouse_cardinal_levers_move_detector: 
               Box::new(mock_mouse_cardinal_levers_move_detector),
        };

        gamepad.next_command()
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
            Command::InputEvent(InputEvent::KeyClick(keyboard_input.clone())));

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
            Command::InputEvent(InputEvent::KeyClick(keyboard_input.clone())));
       
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
            Command::InputEvent(InputEvent::KeyClick(keyboard_input.clone())));
       
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
            Command::InputEvent(InputEvent::KeyClick(keyboard_input.clone())));
       
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
            Command::InputEvent(InputEvent::KeyClick(keyboard_input.clone())));
       
       
       
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
            Command::InputEvent(InputEvent::KeyClick(keyboard_input.clone())));
       
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
            Command::InputEvent(InputEvent::KeyClick(keyboard_input.clone())));
       
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
            Command::InputEvent(InputEvent::KeyClick(keyboard_input.clone())));
       
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
            Command::InputEvent(InputEvent::KeyClick(keyboard_input.clone())));
       
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
            Command::InputEvent(InputEvent::KeyClick(keyboard_input.clone())));
       
       
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
            Command::InputEvent(InputEvent::KeyClick(keyboard_input.clone())));
       
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
            Command::InputEvent(InputEvent::KeyClick(keyboard_input.clone())));
       
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
            Command::InputEvent(InputEvent::KeyClick(keyboard_input.clone())));
       
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
            Command::InputEvent(InputEvent::KeyClick(keyboard_input.clone())));
       
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
            Command::InputEvent(InputEvent::KeyClick(keyboard_input.clone())));
       
       
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
            Command::InputEvent(InputEvent::KeyDown(keyboard_input.clone())));
       
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
            Command::InputEvent(InputEvent::KeyDown(keyboard_input.clone())));
       
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
            Command::InputEvent(InputEvent::KeyDown(keyboard_input.clone())));
       
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
            Command::InputEvent(InputEvent::KeyDown(keyboard_input.clone())));
       
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
            Command::InputEvent(InputEvent::KeyDown(keyboard_input.clone())));
       
       
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
            Command::InputEvent(InputEvent::KeyDown(keyboard_input.clone())));
       
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
            Command::InputEvent(InputEvent::KeyDown(keyboard_input.clone())));
       
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
            Command::InputEvent(InputEvent::KeyDown(keyboard_input.clone())));
       
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
            Command::InputEvent(InputEvent::KeyDown(keyboard_input.clone())));
       
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
            Command::InputEvent(InputEvent::KeyDown(keyboard_input.clone())));
       
       
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
            Command::InputEvent(InputEvent::KeyDown(keyboard_input.clone())));
       
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
            Command::InputEvent(InputEvent::KeyDown(keyboard_input.clone())));
       
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
            Command::InputEvent(InputEvent::KeyDown(keyboard_input.clone())));
       
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
            Command::InputEvent(InputEvent::KeyDown(keyboard_input.clone())));
       
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
            Command::InputEvent(InputEvent::KeyDown(keyboard_input.clone())));


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
            Command::InputEvent(InputEvent::KeyClick(keyboard_input.clone())));
       
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
            Command::InputEvent(InputEvent::KeyClick(keyboard_input.clone())));
       
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
            Command::InputEvent(InputEvent::KeyClick(keyboard_input.clone())));
       
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
            Command::InputEvent(InputEvent::KeyClick(keyboard_input.clone())));
       
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
            Command::InputEvent(InputEvent::KeyClick(keyboard_input.clone())));
       
       
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
    ) -> Option<Command> {

        let mut mock_switch_click_pattern_detector = MockSwitchClickPatternDetectorTrait::new();
        let mut mock_layers_wrapper = MockLayersWrapperTrait::new();
        let mock_gilrs_events = MockGilrsEventsTrait::new();
        let mut mock_layers_navigator = MockLayersNavigatorTrait::new();
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

        mock_layers_wrapper
            .expect_get_cardinal_levers()
            .times(1)
            .with(eq(args.new_layer_index))
            .returning(|_| None);

        mock_mouse_cardinal_levers_move_detector
            .expect_set_mouse_controls()
            .times(1)
            .with(eq(None))
            .return_const(());

        let mut gamepad = Gamepad {
           gilrs_events: Box::new(mock_gilrs_events),
           layers: Box::new(mock_layers_wrapper),
           switch_click_pattern_detector: Box::new(mock_switch_click_pattern_detector),
           layers_navigator: Box::new(mock_layers_navigator),
           mouse_cardinal_levers_move_detector: 
               Box::new(mock_mouse_cardinal_levers_move_detector),
        };

        gamepad.next_command()
}

#[test]
fn handles_move_to_layer_events(){
    let new_layer_index: usize = 101_usize;
    let layer_specifier = LayerSpecifier {
        id: "some-id".to_string(), index_in_gamepad: Some(new_layer_index)};
    assert_eq!(setup_handles_move_to_layer_events(
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
    ).unwrap(),
    Command::QuickLookupWindowEvent(QuickLookupWindowEvent::EmitCurrentLayerNotification(new_layer_index)));

    let new_layer_index: usize = 12_usize;
    let layer_specifier = LayerSpecifier {
        id: "some-other-id".to_string(), index_in_gamepad: Some(new_layer_index)};
    assert_eq!(setup_handles_move_to_layer_events(
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
    ).unwrap(),
    Command::QuickLookupWindowEvent(QuickLookupWindowEvent::EmitCurrentLayerNotification(new_layer_index)));
    
    
    let new_layer_index: usize = 0_usize;
    let layer_specifier = LayerSpecifier {
        id: "some-id".to_string(), index_in_gamepad: Some(new_layer_index)};
    assert_eq!(setup_handles_move_to_layer_events(
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
    ).unwrap(),
    Command::QuickLookupWindowEvent(QuickLookupWindowEvent::EmitCurrentLayerNotification(new_layer_index)));
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
    ) -> Option<Command> {

        let mut mock_switch_click_pattern_detector = MockSwitchClickPatternDetectorTrait::new();
        let mut mock_layers_wrapper = MockLayersWrapperTrait::new();
        let mock_gilrs_events = MockGilrsEventsTrait::new();
        let mut mock_layers_navigator = MockLayersNavigatorTrait::new();
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

        mock_layers_wrapper
            .expect_get_cardinal_levers()
            .times(1)
            .with(eq(args.new_layer_index))
            .returning(|_| None);

        mock_mouse_cardinal_levers_move_detector
            .expect_set_mouse_controls()
            .times(1)
            .with(eq(None))
            .return_const(());

        let mut gamepad = Gamepad {
           gilrs_events: Box::new(mock_gilrs_events),
           layers: Box::new(mock_layers_wrapper),
           switch_click_pattern_detector: Box::new(mock_switch_click_pattern_detector),
           layers_navigator: Box::new(mock_layers_navigator),
           mouse_cardinal_levers_move_detector: 
               Box::new(mock_mouse_cardinal_levers_move_detector),
        };

        gamepad.next_command()
}

#[test]
fn handles_visit_layer_events() {
    let new_layer_index: usize = 101_usize;
    let layer_specifier = LayerSpecifier {
        id: "some-id".to_string(), index_in_gamepad: Some(new_layer_index)};
    assert_eq!(setup_handles_visit_layer_events(
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
    ).unwrap(),
    Command::QuickLookupWindowEvent(QuickLookupWindowEvent::EmitCurrentLayerNotification(new_layer_index)));


    let new_layer_index: usize = 12_usize;
    let layer_specifier = LayerSpecifier {
        id: "some-other-id".to_string(), index_in_gamepad: Some(new_layer_index)};
    assert_eq!(setup_handles_visit_layer_events(
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
    ).unwrap(),
    Command::QuickLookupWindowEvent(QuickLookupWindowEvent::EmitCurrentLayerNotification(new_layer_index)));


    let new_layer_index: usize = 0_usize;
    let layer_specifier = LayerSpecifier {
        id: "some-id".to_string(), index_in_gamepad: Some(new_layer_index)};
    assert_eq!(setup_handles_visit_layer_events(
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
    ).unwrap(),
    Command::QuickLookupWindowEvent(QuickLookupWindowEvent::EmitCurrentLayerNotification(new_layer_index)));

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
    ) -> Option<Command> {

        let mut mock_switch_click_pattern_detector = MockSwitchClickPatternDetectorTrait::new();
        let mut mock_layers_wrapper = MockLayersWrapperTrait::new();
        let mock_gilrs_events = MockGilrsEventsTrait::new();
        let mut mock_layers_navigator = MockLayersNavigatorTrait::new();
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


        mock_layers_wrapper
            .expect_get_cardinal_levers()
            .times(1)
            .with(eq(args.new_layer_index))
            .returning(|_| None);

        mock_mouse_cardinal_levers_move_detector
            .expect_set_mouse_controls()
            .times(1)
            .with(eq(None))
            .return_const(());


        let mut gamepad = Gamepad {
           gilrs_events: Box::new(mock_gilrs_events),
           layers: Box::new(mock_layers_wrapper),
           switch_click_pattern_detector: Box::new(mock_switch_click_pattern_detector),
           layers_navigator: Box::new(mock_layers_navigator),
           mouse_cardinal_levers_move_detector:
               Box::new(mock_mouse_cardinal_levers_move_detector),
        };

        gamepad.next_command()
}

#[test]
fn handles_move_to_or_visit_layer_events() {
    let new_layer_index: usize = 101_usize;
    let layer_specifier = LayerSpecifier {
        id: "some-id".to_string(), index_in_gamepad: Some(new_layer_index)};
    assert_eq!(setup_handles_move_to_or_visit_layer_events(
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
    ).unwrap(),
    Command::QuickLookupWindowEvent(QuickLookupWindowEvent::EmitCurrentLayerNotification(new_layer_index)));


    let new_layer_index: usize = 12_usize;
    let layer_specifier = LayerSpecifier {
        id: "some-other-id".to_string(), index_in_gamepad: Some(new_layer_index)};
    assert_eq!(setup_handles_move_to_or_visit_layer_events(
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
    ).unwrap(),
    Command::QuickLookupWindowEvent(QuickLookupWindowEvent::EmitCurrentLayerNotification(new_layer_index)));


    let new_layer_index: usize = 0_usize;
    let layer_specifier = LayerSpecifier {
        id: "some-id".to_string(), index_in_gamepad: Some(new_layer_index)};
    assert_eq!(setup_handles_move_to_or_visit_layer_events(
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
    ).unwrap(),
    Command::QuickLookupWindowEvent(QuickLookupWindowEvent::EmitCurrentLayerNotification(new_layer_index)));

}



struct SetupGamepadTickHandlesShowQuickLookupWindowEventsArgs{
    current_layer_num: usize,
    source_switch_event_and_reaction: SwitchEventAndReaction,
    pattern: SwitchClickPattern,
}

fn setup_handles_show_quick_lookup_window(
    args: SetupGamepadTickHandlesShowQuickLookupWindowEventsArgs
    ) -> Option<Command> {

        let mut mock_switch_click_pattern_detector = MockSwitchClickPatternDetectorTrait::new();
        let mut mock_layers_wrapper = MockLayersWrapperTrait::new();
        let mock_gilrs_events = MockGilrsEventsTrait::new();
        let mut mock_layers_navigator = MockLayersNavigatorTrait::new();
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

        let mut gamepad = Gamepad {
           gilrs_events: Box::new(mock_gilrs_events),
           layers: Box::new(mock_layers_wrapper),
           switch_click_pattern_detector: Box::new(mock_switch_click_pattern_detector),
           layers_navigator: Box::new(mock_layers_navigator),
           mouse_cardinal_levers_move_detector:
               Box::new(mock_mouse_cardinal_levers_move_detector),
        };

        gamepad.next_command()
}

#[test]
fn handles_show_quick_lookup_window(){
    assert_eq!(setup_handles_show_quick_lookup_window(
        SetupGamepadTickHandlesShowQuickLookupWindowEventsArgs {
            current_layer_num: 100_usize,
            source_switch_event_and_reaction: SwitchEventAndReaction {
                on_click: Some(SwitchOnClickReaction::ShowQuickLookupWindowOnHold),
                on_double_click: None, 
            },
            pattern: SwitchClickPattern::Click(Switch::Button(Button::DPadRight)),
        }
    ).unwrap(),
    Command::QuickLookupWindowEvent(QuickLookupWindowEvent::ShowBySwitch(Switch::Button(Button::DPadRight))));
    
    assert_eq!(setup_handles_show_quick_lookup_window(
        SetupGamepadTickHandlesShowQuickLookupWindowEventsArgs {
            current_layer_num: 10_usize,
            source_switch_event_and_reaction: SwitchEventAndReaction {
                on_click: None,
                on_double_click: Some(SwitchOnClickReaction::ShowQuickLookupWindowOnHold),
            },
            pattern: SwitchClickPattern::DoubleClick(Switch::Button(Button::South)),
        }
    ).unwrap(),
    Command::QuickLookupWindowEvent(QuickLookupWindowEvent::ShowBySwitch(Switch::Button(Button::South))));
    
    
    assert_eq!(setup_handles_show_quick_lookup_window(
        SetupGamepadTickHandlesShowQuickLookupWindowEventsArgs {
            current_layer_num: 10_usize,
            source_switch_event_and_reaction: SwitchEventAndReaction {
                on_click: Some(SwitchOnClickReaction::ShowQuickLookupWindowOnHold),
                on_double_click: None, 
            },
            pattern: SwitchClickPattern::DoubleClick(Switch::Button(Button::South)),
        }
    ).unwrap(),
    Command::QuickLookupWindowEvent(QuickLookupWindowEvent::ShowBySwitch(Switch::Button(Button::South))));
    
}


struct SetupGamepadTickHandlesClickEndSwitchPatternArgs {
    pattern: SwitchClickPattern,
}

fn setup_processes_click_end_switch_pattern(
    args: SetupGamepadTickHandlesClickEndSwitchPatternArgs
    ) -> Option<Command> {

        let mut mock_switch_click_pattern_detector = MockSwitchClickPatternDetectorTrait::new();
        let mock_layers_wrapper = MockLayersWrapperTrait::new();
        let mock_gilrs_events = MockGilrsEventsTrait::new();
        let mut mock_layers_navigator = MockLayersNavigatorTrait::new();
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

        let mut gamepad = Gamepad {
           gilrs_events: Box::new(mock_gilrs_events),
           layers: Box::new(mock_layers_wrapper),
           switch_click_pattern_detector: Box::new(mock_switch_click_pattern_detector),
           layers_navigator: Box::new(mock_layers_navigator),
           mouse_cardinal_levers_move_detector:
               Box::new(mock_mouse_cardinal_levers_move_detector),
        };

        gamepad.next_command()
}

#[test]
fn processes_click_end_switch_pattern(){
    assert_eq!(setup_processes_click_end_switch_pattern(
        SetupGamepadTickHandlesClickEndSwitchPatternArgs {
            pattern: SwitchClickPattern::ClickEnd(Switch::Button(Button::DPadRight)),
        }
    ).unwrap(),Command::KeyUp(Switch::Button(Button::DPadRight)));

    assert_eq!(setup_processes_click_end_switch_pattern(
        SetupGamepadTickHandlesClickEndSwitchPatternArgs {
            pattern: SwitchClickPattern::ClickEnd(Switch::Button(Button::South)),
        }
    ).unwrap(),Command::KeyUp(Switch::Button(Button::South)));


    assert_eq!(setup_processes_click_end_switch_pattern(
        SetupGamepadTickHandlesClickEndSwitchPatternArgs {
            pattern: SwitchClickPattern::ClickEnd(Switch::Button(Button::South)),
        }
    ).unwrap(),Command::KeyUp(Switch::Button(Button::South)));

}


struct SetupProcessesMouseMoveEventsArgs {
    mouse_input_event: Option<InputEvent>,
}

fn setup_processes_mouse_move_events(
    args: SetupProcessesMouseMoveEventsArgs,
    ) -> Option<Command> {

        let mut mock_switch_click_pattern_detector = MockSwitchClickPatternDetectorTrait::new();
        let mock_layers_wrapper = MockLayersWrapperTrait::new();
        let mock_gilrs_events = MockGilrsEventsTrait::new();
        let mut mock_layers_navigator = MockLayersNavigatorTrait::new();
        let mut mock_mouse_cardinal_levers_move_detector 
            = cardinal_levers_move_detector::mouse::MockMouseTrait::new();

        mock_switch_click_pattern_detector
            .expect_tick()
            .times(1)
            .return_const(None);

        mock_layers_navigator
            .expect_consumable_get_current_layer_index()
            .times(1)
            .with()
            .return_const(None);

        mock_mouse_cardinal_levers_move_detector
            .expect_tick()
            .times(1)
            .with()
            .return_const(args.mouse_input_event);

        let mut gamepad = Gamepad {
           gilrs_events: Box::new(mock_gilrs_events),
           layers: Box::new(mock_layers_wrapper),
           switch_click_pattern_detector: Box::new(mock_switch_click_pattern_detector),
           layers_navigator: Box::new(mock_layers_navigator),
           mouse_cardinal_levers_move_detector:
               Box::new(mock_mouse_cardinal_levers_move_detector),
        };

        gamepad.next_command()
}

#[test]
fn processes_mouse_move_events(){
    let event = Command::InputEvent(InputEvent::MoveMouseCursor(2,13));
    assert_eq!(setup_processes_mouse_move_events(
        SetupProcessesMouseMoveEventsArgs {
            mouse_input_event: Some(InputEvent::MoveMouseCursor(2,13)),
        }
    ).unwrap(),event);

    let event = Command::InputEvent(InputEvent::MoveMouseCursor(1,2));
    assert_eq!(setup_processes_mouse_move_events(
        SetupProcessesMouseMoveEventsArgs {
            mouse_input_event: Some(InputEvent::MoveMouseCursor(1,2)),
        }
    ).unwrap(),event);

    let event = Command::InputEvent(InputEvent::MouseScroll(102,1120));
    assert_eq!(setup_processes_mouse_move_events(
        SetupProcessesMouseMoveEventsArgs {
            mouse_input_event: Some(InputEvent::MouseScroll(102,1120)),
        }
    ).unwrap(),event);

    let event = Command::InputEvent(InputEvent::MouseScroll(5245,589));
    assert_eq!(setup_processes_mouse_move_events(
        SetupProcessesMouseMoveEventsArgs {
            mouse_input_event: Some(InputEvent::MouseScroll(5245,589)),
        }
    ).unwrap(),event);

    assert!(setup_processes_mouse_move_events(
        SetupProcessesMouseMoveEventsArgs {
            mouse_input_event: None,
        }
    ).is_none());
}

fn setup_handles_boost_mouse_by_multiplier_events(
    layer_num: usize,
    source_switch_event_and_reaction: SwitchEventAndReaction,
    pattern: SwitchClickPattern) -> Option<Command> {
        let mut mock_switch_click_pattern_detector = MockSwitchClickPatternDetectorTrait::new();
        let mut mock_layers_wrapper = MockLayersWrapperTrait::new();
        let mock_gilrs_events = MockGilrsEventsTrait::new();
        let mut mock_layers_navigator = MockLayersNavigatorTrait::new();
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
           mouse_cardinal_levers_move_detector: 
               Box::new(mock_mouse_cardinal_levers_move_detector),
        };

        gamepad.next_command()
}

#[test]
fn handles_boost_mouse_by_multiplier_events(){
   assert_eq!(
       setup_handles_boost_mouse_by_multiplier_events(
           0_usize,
           SwitchEventAndReaction {
                on_click: Some(SwitchOnClickReaction::BoostMouseCursorByMultiplier(3)),
                on_double_click: None, 
            },
           SwitchClickPattern::Click(Switch::Button(Button::South)),).unwrap(),
        Command::InputEvent(InputEvent::BoostMouseCursor(3)));

   assert_eq!(
       setup_handles_boost_mouse_by_multiplier_events(
           2_usize,
           SwitchEventAndReaction {
                on_click: Some(SwitchOnClickReaction::BoostMouseCursorByMultiplier(100)),
                on_double_click: None, 
            },
           SwitchClickPattern::Click(Switch::Button(Button::South)),).unwrap(),
        Command::InputEvent(InputEvent::BoostMouseCursor(100)));

   assert_eq!(
       setup_handles_boost_mouse_by_multiplier_events(
           99_usize,
           SwitchEventAndReaction {
                on_click: Some(SwitchOnClickReaction::BoostMouseCursorByMultiplier(99999)),
                on_double_click: None, 
            },
           SwitchClickPattern::Click(Switch::Button(Button::South)),).unwrap(),
        Command::InputEvent(InputEvent::BoostMouseCursor(99999)));



   // ----------------
   // different SwitchClickPattern::Click triggering switches
   // ----------------
   assert_eq!(
       setup_handles_boost_mouse_by_multiplier_events(
           0_usize,
           SwitchEventAndReaction {
                on_click: Some(SwitchOnClickReaction::BoostMouseCursorByMultiplier(3)),
                on_double_click: None, 
            },
           SwitchClickPattern::Click(Switch::Button(Button::LeftTrigger2)),).unwrap(),
        Command::InputEvent(InputEvent::BoostMouseCursor(3)));

   assert_eq!(
       setup_handles_boost_mouse_by_multiplier_events(
           2_usize,
           SwitchEventAndReaction {
                on_click: Some(SwitchOnClickReaction::BoostMouseCursorByMultiplier(100)),
                on_double_click: None, 
            },
           SwitchClickPattern::Click(Switch::StickSwitchButton(StickSwitchButton::LeftStickUp)),).unwrap(),
        Command::InputEvent(InputEvent::BoostMouseCursor(100)));

   assert_eq!(
       setup_handles_boost_mouse_by_multiplier_events(
           99_usize,
           SwitchEventAndReaction {
                on_click: Some(SwitchOnClickReaction::BoostMouseCursorByMultiplier(99999)),
                on_double_click: None, 
            },
           SwitchClickPattern::Click(Switch::StickSwitchButton(StickSwitchButton::LeftStickLeft)),).unwrap(),
        Command::InputEvent(InputEvent::BoostMouseCursor(99999)));

    // -------------
    // DoubleClick count as Click if on_double_click wasn't set
    // -------------
   assert_eq!(
       setup_handles_boost_mouse_by_multiplier_events(
           0_usize,
           SwitchEventAndReaction {
                on_click: Some(SwitchOnClickReaction::BoostMouseCursorByMultiplier(3)),
                on_double_click: None, 
            },
           SwitchClickPattern::DoubleClick(Switch::Button(Button::South)),).unwrap(),
        Command::InputEvent(InputEvent::BoostMouseCursor(3)));

   assert_eq!(
       setup_handles_boost_mouse_by_multiplier_events(
           2_usize,
           SwitchEventAndReaction {
                on_click: Some(SwitchOnClickReaction::BoostMouseCursorByMultiplier(100)),
                on_double_click: None, 
            },
           SwitchClickPattern::DoubleClick(Switch::Button(Button::South)),).unwrap(),
        Command::InputEvent(InputEvent::BoostMouseCursor(100)));

   assert_eq!(
       setup_handles_boost_mouse_by_multiplier_events(
           99_usize,
           SwitchEventAndReaction {
                on_click: Some(SwitchOnClickReaction::BoostMouseCursorByMultiplier(99999)),
                on_double_click: None, 
            },
           SwitchClickPattern::DoubleClick(Switch::Button(Button::South)),).unwrap(),
        Command::InputEvent(InputEvent::BoostMouseCursor(99999)));


    // -------------
    // DoubleClick triggers on_double_click key_click when it's set
    // -------------

   assert_eq!(
       setup_handles_boost_mouse_by_multiplier_events(
           0_usize,
           SwitchEventAndReaction {
                on_click: None,
                on_double_click: Some(SwitchOnClickReaction::BoostMouseCursorByMultiplier(3)), 
            },
           SwitchClickPattern::DoubleClick(Switch::Button(Button::South)),).unwrap(),
        Command::InputEvent(InputEvent::BoostMouseCursor(3)));

   assert_eq!(
       setup_handles_boost_mouse_by_multiplier_events(
           2_usize,
           SwitchEventAndReaction {
                on_click: None,
                on_double_click: Some(SwitchOnClickReaction::BoostMouseCursorByMultiplier(100)), 
            },
           SwitchClickPattern::DoubleClick(Switch::Button(Button::South)),).unwrap(),
        Command::InputEvent(InputEvent::BoostMouseCursor(100)));

   assert_eq!(
       setup_handles_boost_mouse_by_multiplier_events(
           99_usize,
           SwitchEventAndReaction {
                on_click: None,
                on_double_click: Some(SwitchOnClickReaction::BoostMouseCursorByMultiplier(99999)), 
            },
           SwitchClickPattern::DoubleClick(Switch::Button(Button::South)),).unwrap(),
        Command::InputEvent(InputEvent::BoostMouseCursor(99999)));

				//        // ----------------
				//        // SwitchClickPattern::ClickAndHold triggers Keydown
				//        // ----------------
				//         let keyboard_input = KeyboardInput{
				//             key: Key::Layout('I'),
				//             modifiers: vec![]
				//         };
				//        assert_eq!(
				//            setup_handles_keyboard_input_events(
				//                0_usize,
				//                SwitchEventAndReaction {
				// 	on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
				//                     on_double_click: None, 
				// },
				//                SwitchClickPattern::ClickAndHold(Switch::Button(Button::LeftTrigger2)),).unwrap(),
				//             Command::InputEvent(InputEvent::KeyDown(keyboard_input.clone())));
				//       
				//         let keyboard_input = KeyboardInput{
				//             key: Key::Layout('*'),
				//             modifiers: vec![]
				//         };
				//        assert_eq!(
				//            setup_handles_keyboard_input_events(
				//                1_usize,
				//                SwitchEventAndReaction {
				// 	on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
				//                     on_double_click: None, 
				// },
				//                SwitchClickPattern::ClickAndHold(Switch::StickSwitchButton(StickSwitchButton::RightStickDown)),).unwrap(),
				//             Command::InputEvent(InputEvent::KeyDown(keyboard_input.clone())));
				//       
				//         let keyboard_input = KeyboardInput{
				//             key: Key::Tab,
				//             modifiers: vec![]
				//         };
				//        assert_eq!(
				//            setup_handles_keyboard_input_events(
				//                2_usize,
				//                SwitchEventAndReaction {
				// 	on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
				//                     on_double_click: None, 
				// },
				//                SwitchClickPattern::ClickAndHold(Switch::StickSwitchButton(StickSwitchButton::LeftStickDown)),).unwrap(),
				//             Command::InputEvent(InputEvent::KeyDown(keyboard_input.clone())));
				//       
				//         let keyboard_input = KeyboardInput{
				//             key: Key::Tab,
				//             modifiers: vec![Key::Shift]
				//         };
				//        assert_eq!(
				//            setup_handles_keyboard_input_events(
				//                10_usize,
				//                SwitchEventAndReaction {
				// 	on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
				//                     on_double_click: None, 
				// },
				//                SwitchClickPattern::ClickAndHold(Switch::Button(Button::Start)),).unwrap(),
				//             Command::InputEvent(InputEvent::KeyDown(keyboard_input.clone())));
				//       
				//         let keyboard_input = KeyboardInput{
				//             key: Key::Layout('a'),
				//             modifiers: vec![Key::Control, Key::Shift]
				//         };
				//        assert_eq!(
				//            setup_handles_keyboard_input_events(
				//                100_usize,
				//                SwitchEventAndReaction {
				// 	on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
				//                     on_double_click: None, 
				// },
				//                SwitchClickPattern::ClickAndHold(Switch::Button(Button::DPadRight)),).unwrap(),
				//             Command::InputEvent(InputEvent::KeyDown(keyboard_input.clone())));
				//       
				//       
				//        // ----------------
				//        // SwitchClickPattern::ClickAndHold triggers Keydown
				//        // ----------------
				//         let keyboard_input = KeyboardInput{
				//             key: Key::Layout('I'),
				//             modifiers: vec![]
				//         };
				//        assert_eq!(
				//            setup_handles_keyboard_input_events(
				//                0_usize,
				//                SwitchEventAndReaction {
				// 	on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
				//                     on_double_click: None, 
				// },
				//                SwitchClickPattern::ClickAndHold(Switch::Button(Button::LeftTrigger2)),).unwrap(),
				//             Command::InputEvent(InputEvent::KeyDown(keyboard_input.clone())));
				//       
				//         let keyboard_input = KeyboardInput{
				//             key: Key::Layout('*'),
				//             modifiers: vec![]
				//         };
				//        assert_eq!(
				//            setup_handles_keyboard_input_events(
				//                1_usize,
				//                SwitchEventAndReaction {
				// 	on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
				//                     on_double_click: None, 
				// },
				//                SwitchClickPattern::ClickAndHold(Switch::StickSwitchButton(StickSwitchButton::RightStickDown)),).unwrap(),
				//             Command::InputEvent(InputEvent::KeyDown(keyboard_input.clone())));
				//       
				//         let keyboard_input = KeyboardInput{
				//             key: Key::Tab,
				//             modifiers: vec![]
				//         };
				//        assert_eq!(
				//            setup_handles_keyboard_input_events(
				//                2_usize,
				//                SwitchEventAndReaction {
				// 	on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
				//                     on_double_click: None, 
				// },
				//                SwitchClickPattern::ClickAndHold(Switch::StickSwitchButton(StickSwitchButton::LeftStickDown)),).unwrap(),
				//             Command::InputEvent(InputEvent::KeyDown(keyboard_input.clone())));
				//       
				//         let keyboard_input = KeyboardInput{
				//             key: Key::Tab,
				//             modifiers: vec![Key::Shift]
				//         };
				//        assert_eq!(
				//            setup_handles_keyboard_input_events(
				//                10_usize,
				//                SwitchEventAndReaction {
				// 	on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
				//                     on_double_click: None, 
				// },
				//                SwitchClickPattern::ClickAndHold(Switch::Button(Button::Start)),).unwrap(),
				//             Command::InputEvent(InputEvent::KeyDown(keyboard_input.clone())));
				//       
				//         let keyboard_input = KeyboardInput{
				//             key: Key::Layout('a'),
				//             modifiers: vec![Key::Control, Key::Shift]
				//         };
				//        assert_eq!(
				//            setup_handles_keyboard_input_events(
				//                100_usize,
				//                SwitchEventAndReaction {
				// 	on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
				//                     on_double_click: None, 
				// },
				//                SwitchClickPattern::ClickAndHold(Switch::Button(Button::DPadRight)),).unwrap(),
				//             Command::InputEvent(InputEvent::KeyDown(keyboard_input.clone())));
				//       
				//       
				//        // ----------------
				//        // SwitchClickPattern::DoubleClickAndHold triggers Keydown
				//        // ----------------
				//         let keyboard_input = KeyboardInput{
				//             key: Key::Layout('I'),
				//             modifiers: vec![]
				//         };
				//        assert_eq!(
				//            setup_handles_keyboard_input_events(
				//                0_usize,
				//                SwitchEventAndReaction {
				// 	on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
				//                     on_double_click: None, 
				// },
				//                SwitchClickPattern::DoubleClickAndHold(Switch::Button(Button::LeftTrigger2)),).unwrap(),
				//             Command::InputEvent(InputEvent::KeyDown(keyboard_input.clone())));
				//       
				//         let keyboard_input = KeyboardInput{
				//             key: Key::Layout('*'),
				//             modifiers: vec![]
				//         };
				//        assert_eq!(
				//            setup_handles_keyboard_input_events(
				//                1_usize,
				//                SwitchEventAndReaction {
				// 	on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
				//                     on_double_click: None, 
				// },
				//                SwitchClickPattern::DoubleClickAndHold(Switch::StickSwitchButton(StickSwitchButton::RightStickDown)),).unwrap(),
				//             Command::InputEvent(InputEvent::KeyDown(keyboard_input.clone())));
				//       
				//         let keyboard_input = KeyboardInput{
				//             key: Key::Tab,
				//             modifiers: vec![]
				//         };
				//        assert_eq!(
				//            setup_handles_keyboard_input_events(
				//                2_usize,
				//                SwitchEventAndReaction {
				// 	on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
				//                     on_double_click: None, 
				// },
				//                SwitchClickPattern::DoubleClickAndHold(Switch::StickSwitchButton(StickSwitchButton::LeftStickDown)),).unwrap(),
				//             Command::InputEvent(InputEvent::KeyDown(keyboard_input.clone())));
				//       
				//         let keyboard_input = KeyboardInput{
				//             key: Key::Tab,
				//             modifiers: vec![Key::Shift]
				//         };
				//        assert_eq!(
				//            setup_handles_keyboard_input_events(
				//                10_usize,
				//                SwitchEventAndReaction {
				// 	on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
				//                     on_double_click: None, 
				// },
				//                SwitchClickPattern::DoubleClickAndHold(Switch::Button(Button::Start)),).unwrap(),
				//             Command::InputEvent(InputEvent::KeyDown(keyboard_input.clone())));
				//       
				//         let keyboard_input = KeyboardInput{
				//             key: Key::Layout('a'),
				//             modifiers: vec![Key::Control, Key::Shift]
				//         };
				//        assert_eq!(
				//            setup_handles_keyboard_input_events(
				//                100_usize,
				//                SwitchEventAndReaction {
				// 	on_click:Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())),
				//                     on_double_click: None, 
				// },
				//                SwitchClickPattern::DoubleClickAndHold(Switch::Button(Button::DPadRight)),).unwrap(),
				//             Command::InputEvent(InputEvent::KeyDown(keyboard_input.clone())));
				//
				//
				//         // -------------
				//         // DoubleClick triggers on_double_click key_click when it's set
				//         // -------------
				//         let keyboard_input = KeyboardInput{
				//             key: Key::Layout('I'),
				//             modifiers: vec![]
				//         };
				//        assert_eq!(
				//            setup_handles_keyboard_input_events(
				//                0_usize,
				//                SwitchEventAndReaction {
				// 	on_click: None,
				//                     on_double_click: Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())), 
				// },
				//                SwitchClickPattern::DoubleClick(Switch::Button(Button::LeftTrigger2)),).unwrap(),
				//             Command::InputEvent(InputEvent::KeyClick(keyboard_input.clone())));
				//       
				//         let keyboard_input = KeyboardInput{
				//             key: Key::Layout('*'),
				//             modifiers: vec![]
				//         };
				//        assert_eq!(
				//            setup_handles_keyboard_input_events(
				//                1_usize,
				//                SwitchEventAndReaction {
				// 	on_click: None,
				//                     on_double_click: Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())), 
				// },
				//                SwitchClickPattern::DoubleClick(Switch::StickSwitchButton(StickSwitchButton::RightStickDown)),).unwrap(),
				//             Command::InputEvent(InputEvent::KeyClick(keyboard_input.clone())));
				//       
				//         let keyboard_input = KeyboardInput{
				//             key: Key::Tab,
				//             modifiers: vec![]
				//         };
				//        assert_eq!(
				//            setup_handles_keyboard_input_events(
				//                2_usize,
				//                SwitchEventAndReaction {
				// 	on_click: None,
				//                     on_double_click: Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())), 
				// },
				//                SwitchClickPattern::DoubleClick(Switch::StickSwitchButton(StickSwitchButton::LeftStickDown)),).unwrap(),
				//             Command::InputEvent(InputEvent::KeyClick(keyboard_input.clone())));
				//       
				//         let keyboard_input = KeyboardInput{
				//             key: Key::Tab,
				//             modifiers: vec![Key::Shift]
				//         };
				//        assert_eq!(
				//            setup_handles_keyboard_input_events(
				//                10_usize,
				//                SwitchEventAndReaction {
				// 	on_click: None,
				//                     on_double_click: Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())), 
				// },
				//                SwitchClickPattern::DoubleClick(Switch::Button(Button::Start)),).unwrap(),
				//             Command::InputEvent(InputEvent::KeyClick(keyboard_input.clone())));
				//       
				//         let keyboard_input = KeyboardInput{
				//             key: Key::Layout('a'),
				//             modifiers: vec![Key::Control, Key::Shift]
				//         };
				//        assert_eq!(
				//            setup_handles_keyboard_input_events(
				//                100_usize,
				//                SwitchEventAndReaction {
				// 	on_click: None,
				//                     on_double_click: Some(SwitchOnClickReaction::Keyboard(keyboard_input.clone())), 
				// },
				//                SwitchClickPattern::DoubleClick(Switch::Button(Button::DPadRight)),).unwrap(),
				//             Command::InputEvent(InputEvent::KeyClick(keyboard_input.clone())));
       
       
}
