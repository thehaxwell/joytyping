use enigo::Key;
use gilrs::Button;
use mockall::{predicate::*, Sequence};
mod layers_examples; 

use crate::{gamepad::{switch_click_pattern_detector::{MockSwitchClickPatternDetectorTrait, SwitchClickPattern}, dynamic_switch_event_reactions::MockDynamicSwitchEventReactionsTrait, gilrs_events::MockGilrsEventsTrait, Gamepad, Switch, Handle, InputEvent}, settings_data::{SwitchOnClickReaction, KeyboardInput, LayerSpecifier}};
use self::layers_examples::setup_layers_examples;

use super::gilrs_events::stick_switch_interpreter::StickSwitchButton;

// use super::gilrs_wrapper::{GilrsEvent, GilrsEventType};

#[test]
fn tick_responds_correctly_with_keyboard_dynamic_switch_events() {
    let mut mock_switch_click_pattern_detector = MockSwitchClickPatternDetectorTrait::new();
    let mut mock_dynamic_switch_event_responses = MockDynamicSwitchEventReactionsTrait::new();
    let mock_gilrs_events = MockGilrsEventsTrait::new();


    let mut seq = Sequence::new();
    let mut results_vec:Vec<Option<InputEvent>> = Vec::new();


    let switch = Switch::Button(Button::North);
    let keyboard_input = KeyboardInput { key: Key::Layout('i'), modifiers: Vec::new() };
    mock_switch_click_pattern_detector
        .expect_tick()
        .times(1)
        .return_const(Some(SwitchClickPattern::Click(switch.clone())))
        .in_sequence(&mut seq);
    mock_dynamic_switch_event_responses
        .expect_get()
        .with(eq(switch.clone()),eq(Handle::Click))
        .times(1)
        .return_const(SwitchOnClickReaction::Keyboard(
            keyboard_input.clone()))
        .in_sequence(&mut seq);
    results_vec.push(Some(InputEvent::KeyClick(keyboard_input)));


    let switch = Switch::Button(Button::West);
    let keyboard_input = KeyboardInput { key: Key::Tab, modifiers: Vec::new() };
    mock_switch_click_pattern_detector
        .expect_tick()
        .times(1)
        .return_const(Some(SwitchClickPattern::ClickAndHold(switch.clone())))
        .in_sequence(&mut seq);
    mock_dynamic_switch_event_responses
        .expect_get()
        .with(eq(switch.clone()),eq(Handle::ClickAndHold))
        .times(1)
        .return_const(SwitchOnClickReaction::Keyboard(
            keyboard_input.clone()))
        .in_sequence(&mut seq);
    results_vec.push(Some(InputEvent::KeyClick(keyboard_input)));


    let switch = Switch::Button(Button::Select);
    let keyboard_input = KeyboardInput { key: Key::Layout('X'), modifiers: Vec::new() };
    mock_switch_click_pattern_detector
        .expect_tick()
        .times(1)
        .return_const(Some(SwitchClickPattern::DoubleClick(switch.clone())))
        .in_sequence(&mut seq);
    mock_dynamic_switch_event_responses
        .expect_get()
        .with(eq(switch.clone()),eq(Handle::DoubleClick))
        .times(1)
        .return_const(SwitchOnClickReaction::Keyboard(
            keyboard_input.clone()))
        .in_sequence(&mut seq);
    results_vec.push(Some(InputEvent::KeyClick(keyboard_input)));


    let switch = Switch::StickSwitchButton(StickSwitchButton::LeftStickUp);
    let keyboard_input = KeyboardInput { key: Key::Layout('X'), modifiers: vec![ Key::Shift ] };
    mock_switch_click_pattern_detector
        .expect_tick()
        .times(1)
        .return_const(Some(SwitchClickPattern::ClickEnd(switch.clone())))
        .in_sequence(&mut seq);
    mock_dynamic_switch_event_responses
        .expect_get()
        .with(eq(switch.clone()),eq(Handle::ClickEnd))
        .times(1)
        .return_const(SwitchOnClickReaction::Keyboard(
            keyboard_input.clone()))
        .in_sequence(&mut seq);
    results_vec.push(Some(InputEvent::KeyClick(keyboard_input)));


    let switch = Switch::StickSwitchButton(StickSwitchButton::RightStickUp);
    let keyboard_input = KeyboardInput { key: Key::Space, modifiers: vec![ Key::Meta ] };
    mock_switch_click_pattern_detector
        .expect_tick()
        .times(1)
        .return_const(Some(SwitchClickPattern::DoubleClickAndHold(switch.clone())))
        .in_sequence(&mut seq);
    mock_dynamic_switch_event_responses
        .expect_get()
        .with(eq(switch.clone()),eq(Handle::DoubleClickAndHold))
        .times(1)
        .return_const(SwitchOnClickReaction::Keyboard(
            keyboard_input.clone()))
        .in_sequence(&mut seq);
    results_vec.push(Some(InputEvent::KeyClick(keyboard_input)));
    


    let mut gamepad = Gamepad {
       gilrs_events: Box::new(mock_gilrs_events),
       layers: setup_layers_examples(),
       current_layer_index: 0,
       switch_click_pattern_detector: Box::new(mock_switch_click_pattern_detector),
       dynamic_switch_event_responses: Box::new(mock_dynamic_switch_event_responses),
    };

    for result in results_vec.iter() {
        assert_eq!(gamepad.tick(),*result);
        assert_eq!(gamepad.current_layer_index,0);
        assert_eq!(gamepad.layers,setup_layers_examples());
    }
}

#[test]
fn tick_responds_correctly_with_move_to_layer_dynamic_switch_events() {
    let mut mock_switch_click_pattern_detector = MockSwitchClickPatternDetectorTrait::new();
    let mut mock_dynamic_switch_event_responses = MockDynamicSwitchEventReactionsTrait::new();
    let mock_gilrs_events = MockGilrsEventsTrait::new();

    let mut seq = Sequence::new();
    let mut indexes_vec:Vec<usize> = Vec::new();


    let switch = Switch::Button(Button::North);
    let index_in_gamepad = 1;
    mock_switch_click_pattern_detector
        .expect_tick()
        .times(1)
        .return_const(Some(SwitchClickPattern::Click(switch.clone())))
        .in_sequence(&mut seq);
    mock_dynamic_switch_event_responses
        .expect_get()
        .with(eq(switch.clone()),eq(Handle::Click))
        .times(1)
        .return_const(SwitchOnClickReaction::MoveToLayer(LayerSpecifier {
            id: String::new(),
            index_in_gamepad: Some(index_in_gamepad)
        }))
        .in_sequence(&mut seq);
    indexes_vec.push(index_in_gamepad);


    let switch = Switch::Button(Button::East);
    let index_in_gamepad = 123;
    mock_switch_click_pattern_detector
        .expect_tick()
        .times(1)
        .return_const(Some(SwitchClickPattern::ClickAndHold(switch.clone())))
        .in_sequence(&mut seq);
    mock_dynamic_switch_event_responses
        .expect_get()
        .with(eq(switch.clone()),eq(Handle::ClickAndHold))
        .times(1)
        .return_const(SwitchOnClickReaction::MoveToLayer(LayerSpecifier {
            id: String::new(),
            index_in_gamepad: Some(index_in_gamepad)
        }))
        .in_sequence(&mut seq);
    indexes_vec.push(index_in_gamepad);


    let switch = Switch::Button(Button::North);
    let index_in_gamepad = 123;
    mock_switch_click_pattern_detector
        .expect_tick()
        .times(1)
        .return_const(Some(SwitchClickPattern::ClickAndHold(switch.clone())))
        .in_sequence(&mut seq);
    mock_dynamic_switch_event_responses
        .expect_get()
        .with(eq(switch.clone()),eq(Handle::ClickAndHold))
        .times(1)
        .return_const(SwitchOnClickReaction::MoveToLayer(LayerSpecifier {
            id: String::new(),
            index_in_gamepad: Some(index_in_gamepad)
        }))
        .in_sequence(&mut seq);
    indexes_vec.push(index_in_gamepad);


    let switch = Switch::StickSwitchButton(StickSwitchButton::LeftStickLeft);
    let index_in_gamepad = 99;
    mock_switch_click_pattern_detector
        .expect_tick()
        .times(1)
        .return_const(Some(SwitchClickPattern::DoubleClick(switch.clone())))
        .in_sequence(&mut seq);
    mock_dynamic_switch_event_responses
        .expect_get()
        .with(eq(switch.clone()),eq(Handle::DoubleClick))
        .times(1)
        .return_const(SwitchOnClickReaction::MoveToLayer(LayerSpecifier {
            id: String::new(),
            index_in_gamepad: Some(index_in_gamepad)
        }))
        .in_sequence(&mut seq);
    indexes_vec.push(index_in_gamepad);


    let switch = Switch::StickSwitchButton(StickSwitchButton::RightStickRight);
    let index_in_gamepad = 8900;
    mock_switch_click_pattern_detector
        .expect_tick()
        .times(1)
        .return_const(Some(SwitchClickPattern::DoubleClickAndHold(switch.clone())))
        .in_sequence(&mut seq);
    mock_dynamic_switch_event_responses
        .expect_get()
        .with(eq(switch.clone()),eq(Handle::DoubleClickAndHold))
        .times(1)
        .return_const(SwitchOnClickReaction::MoveToLayer(LayerSpecifier {
            id: String::new(),
            index_in_gamepad: Some(index_in_gamepad)
        }))
        .in_sequence(&mut seq);
    indexes_vec.push(index_in_gamepad);


    let switch = Switch::StickSwitchButton(StickSwitchButton::RightStickUp);
    let index_in_gamepad = 89;
    mock_switch_click_pattern_detector
        .expect_tick()
        .times(1)
        .return_const(Some(SwitchClickPattern::ClickEnd(switch.clone())))
        .in_sequence(&mut seq);
    mock_dynamic_switch_event_responses
        .expect_get()
        .with(eq(switch.clone()),eq(Handle::ClickEnd))
        .times(1)
        .return_const(SwitchOnClickReaction::MoveToLayer(LayerSpecifier {
            id: String::new(),
            index_in_gamepad: Some(index_in_gamepad)
        }))
        .in_sequence(&mut seq);
    indexes_vec.push(index_in_gamepad);

    
    let mut gamepad = Gamepad {
       gilrs_events: Box::new(mock_gilrs_events),
       layers: setup_layers_examples(),
       current_layer_index: 0,
       switch_click_pattern_detector: Box::new(mock_switch_click_pattern_detector),
       dynamic_switch_event_responses: Box::new(mock_dynamic_switch_event_responses),
    };

    for result in indexes_vec.iter() {
        assert!(gamepad.tick().is_none());
        assert_eq!(gamepad.current_layer_index,*result);
        assert_eq!(gamepad.layers,setup_layers_examples());
    }
}
