use gilrs::Button;
use mockall::predicate::eq;

use crate::{gamepad_listener::{layers_navigator::{controller::Controller, MockLayersNavigatorTrait}, switch_click_pattern_detector::SwitchClickPattern, Switch}, settings::models::layout::{LayerSpecifier, SwitchEventAndReaction, SwitchOnClickReaction}};

use crate::gamepad_listener::layers_navigator::controller::ControllerTrait;

struct SetupHandlesMoveToLayerEventsArgs{
    layer_specifier: LayerSpecifier, 
    source_switch_event_and_reaction: SwitchEventAndReaction,
    pattern: SwitchClickPattern,
}

fn setup_handles_move_to_layer_events(
    args: SetupHandlesMoveToLayerEventsArgs
    ) -> Option<SwitchOnClickReaction> {
        let mut mock_layers_navigator = MockLayersNavigatorTrait::new();
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

        mock_layers_navigator
            .expect_get_switch_event_and_reaction()
            .times(1)
            .with(eq(switch))
            .return_const(
                Some(args.source_switch_event_and_reaction));

        mock_layers_navigator
            .expect_move_to_layer()
            .times(1)
            .with(eq(args.layer_specifier))
            .return_const(());

        let mut controller = Controller {
           layers_navigator: Box::new(mock_layers_navigator),
        };
        controller.process_switch_event(Some(args.pattern))
}

#[test]
fn handles_move_to_layer_events(){
    let new_layer_index: usize = 101_usize;
    let layer_specifier = LayerSpecifier {
        id: "some-id".to_string(), index_in_gamepad: Some(new_layer_index)};

    assert_eq!(setup_handles_move_to_layer_events(
        SetupHandlesMoveToLayerEventsArgs {
            layer_specifier: layer_specifier.clone(),
            source_switch_event_and_reaction: SwitchEventAndReaction {
                on_click: Some(SwitchOnClickReaction::MoveToLayer(layer_specifier.clone())),
                on_double_click: None, 
            },
            pattern: SwitchClickPattern::Click(Switch::Button(Button::DPadRight)),
        }),
        Some(SwitchOnClickReaction::MoveToLayer(layer_specifier.clone()))
    );

    let new_layer_index: usize = 12_usize;
    let layer_specifier = LayerSpecifier {
        id: "some-other-id".to_string(), index_in_gamepad: Some(new_layer_index)};
    assert_eq!(setup_handles_move_to_layer_events(
        SetupHandlesMoveToLayerEventsArgs {
            layer_specifier: layer_specifier.clone(),
            source_switch_event_and_reaction: SwitchEventAndReaction {
                on_click: None,
                on_double_click: Some(SwitchOnClickReaction::MoveToLayer(layer_specifier.clone())), 
            },
            pattern: SwitchClickPattern::DoubleClick(Switch::Button(Button::South)),
        }),
        Some(SwitchOnClickReaction::MoveToLayer(layer_specifier.clone()))
    );
   
   
    let new_layer_index: usize = 0_usize;
    let layer_specifier = LayerSpecifier {
        id: "some-id".to_string(), index_in_gamepad: Some(new_layer_index)};
    assert_eq!(setup_handles_move_to_layer_events(
        SetupHandlesMoveToLayerEventsArgs {
            layer_specifier: layer_specifier.clone(),
            source_switch_event_and_reaction: SwitchEventAndReaction {
                on_click: Some(SwitchOnClickReaction::MoveToLayer(layer_specifier.clone())),
                on_double_click: None, 
            },
            pattern: SwitchClickPattern::DoubleClick(Switch::Button(Button::South)),
        }),
        Some(SwitchOnClickReaction::MoveToLayer(layer_specifier.clone()))
    );
}

#[test]
fn click_and_hold_never_triggers_move_to_layer(){
    let new_layer_index: usize = 101_usize;
    let layer_specifier = LayerSpecifier {
        id: "some-id".to_string(), index_in_gamepad: Some(new_layer_index)};

    let source_switch_event_and_reaction = SwitchEventAndReaction {
        on_click: Some(SwitchOnClickReaction::MoveToLayer(layer_specifier.clone())),
        on_double_click: Some(SwitchOnClickReaction::MoveToLayer(layer_specifier.clone())), 
    };
    let pattern = SwitchClickPattern::ClickAndHold(Switch::Button(Button::DPadRight));

    let mut mock_layers_navigator = MockLayersNavigatorTrait::new();

    mock_layers_navigator
        .expect_process_current_potential_visit()
        .times(1)
        .with(eq(pattern.clone()))
        .return_const(());

    let switch = match pattern.clone() {
        SwitchClickPattern::Click(sw) => sw,
        SwitchClickPattern::ClickAndHold(sw) => sw,
        SwitchClickPattern::DoubleClick(sw) => sw,
        SwitchClickPattern::DoubleClickAndHold(sw) => sw,
        SwitchClickPattern::ClickEnd(sw) => sw,
    };

    mock_layers_navigator
        .expect_get_switch_event_and_reaction()
        .times(1)
        .with(eq(switch))
        .return_const(
            Some(source_switch_event_and_reaction));

    let mut controller = Controller {
       layers_navigator: Box::new(mock_layers_navigator),
    };
    
    assert_eq!(
        controller.process_switch_event(Some(pattern)),
        Some(SwitchOnClickReaction::MoveToLayer(layer_specifier.clone()))
    );

}

#[test]
fn double_click_and_hold_never_triggers_move_to_layer(){
    let new_layer_index: usize = 101_usize;
    let layer_specifier = LayerSpecifier {
        id: "some-id".to_string(), index_in_gamepad: Some(new_layer_index)};

    let source_switch_event_and_reaction = SwitchEventAndReaction {
        on_click: Some(SwitchOnClickReaction::MoveToLayer(layer_specifier.clone())),
        on_double_click: Some(SwitchOnClickReaction::MoveToLayer(layer_specifier.clone())), 
    };
    let pattern = SwitchClickPattern::DoubleClickAndHold(Switch::Button(Button::DPadRight));

    let mut mock_layers_navigator = MockLayersNavigatorTrait::new();

    mock_layers_navigator
        .expect_process_current_potential_visit()
        .times(1)
        .with(eq(pattern.clone()))
        .return_const(());

    let switch = match pattern.clone() {
        SwitchClickPattern::Click(sw) => sw,
        SwitchClickPattern::ClickAndHold(sw) => sw,
        SwitchClickPattern::DoubleClick(sw) => sw,
        SwitchClickPattern::DoubleClickAndHold(sw) => sw,
        SwitchClickPattern::ClickEnd(sw) => sw,
    };

    mock_layers_navigator
        .expect_get_switch_event_and_reaction()
        .times(1)
        .with(eq(switch))
        .return_const(
            Some(source_switch_event_and_reaction));

    let mut controller = Controller {
       layers_navigator: Box::new(mock_layers_navigator),
    };
    
    assert_eq!(
        controller.process_switch_event(Some(pattern)),
        Some(SwitchOnClickReaction::MoveToLayer(layer_specifier.clone()))
    );
}
