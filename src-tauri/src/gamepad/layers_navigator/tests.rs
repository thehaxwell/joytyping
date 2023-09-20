use gilrs::Button;

use crate::{gamepad::{layers_navigator::{LayersNavigator,LayersNavigatorTrait, LayerVisit, AvailableLayerVisitsFromLayer}, Switch, switch_click_pattern_detector::SwitchClickPattern}, settings_data::LayerSpecifier};


#[test]
fn click_end_to_backtrack_from_top_of_layer_visit_vec() {
    // simulate tick emitted ClickEnd LeftTrigger

    // simulate the case where the user:
    // 1. Hold RightTrigger to go to layer 2 (index 1), then without releasing,
    // 2. Hold LeftTrigger to go to layer 4 (index 3)
    let layer_visits = vec![
       LayerVisit{
            trigger_switch: Switch::Button(Button::RightTrigger),
            from_index: 0,
            to_index: 1,
       },
       LayerVisit{
            trigger_switch: Switch::Button(Button::LeftTrigger),
            from_index: 1,
            to_index: 3,
       },
   ];

    let mut gamepad = LayersNavigator {
       current_layer_index: 3,
       layer_visits: layer_visits.clone(),
       potential_layer_visit: None,
       layers_and_their_available_layer_visits: vec![
            AvailableLayerVisitsFromLayer {
                index_in_gamepad: 0,
                layer_visits: vec![
                   LayerVisit{
                       trigger_switch: Switch::Button(Button::RightTrigger),
                       from_index: 0,
                       to_index: 1,
                   },
                   LayerVisit{
                       trigger_switch: Switch::Button(Button::LeftTrigger),
                       from_index: 0,
                       to_index: 2,
                   },
               ]
            },
            AvailableLayerVisitsFromLayer {
                index_in_gamepad: 1,
                layer_visits:vec![
                   LayerVisit{
                       trigger_switch: Switch::Button(Button::LeftTrigger),
                       from_index: 1,
                       to_index: 3,
                   },
                ],
            }
       ],
    };

    let switch = Switch::Button(Button::LeftTrigger);
    gamepad.process_current_potential_visit(
            SwitchClickPattern::ClickEnd(switch.clone()));
    gamepad.undo_last_layer_visit_with_switch(switch);

    assert_eq!(gamepad.layer_visits.len(), 1);
    assert_eq!(gamepad.layer_visits[0], layer_visits[0].clone());

    assert!(gamepad.potential_layer_visit.is_none());
}


#[test]
fn click_end_to_backtrack_from_middle_of_layer_visit_vec() {
    // simulate tick emitted ClickEnd RightTrigger

    // simulate the case where the user:
    // 1. Hold RightTrigger to go to layer 2 (index 1), then without releasing,
    // 2. Hold LeftTrigger to go to layer 4 (index 3)
    let layer_visits = vec![
       LayerVisit{
            trigger_switch: Switch::Button(Button::RightTrigger),
            from_index: 0,
            to_index: 1,
       },
       LayerVisit{
            trigger_switch: Switch::Button(Button::LeftTrigger),
            from_index: 1,
            to_index: 3,
       },
   ];

    let mut gamepad = LayersNavigator {
       current_layer_index: 3,
       layer_visits: layer_visits.clone(),
       potential_layer_visit: None,
       layers_and_their_available_layer_visits: vec![
            AvailableLayerVisitsFromLayer {
                index_in_gamepad: 0,
                layer_visits: vec![
                   LayerVisit{
                       trigger_switch: Switch::Button(Button::RightTrigger),
                       from_index: 0,
                       to_index: 1,
                   },
                   LayerVisit{
                       trigger_switch: Switch::Button(Button::LeftTrigger),
                       from_index: 0,
                       to_index: 2,
                   },
                ],
            },
            AvailableLayerVisitsFromLayer {
                index_in_gamepad: 0,
                layer_visits: vec![
                   LayerVisit{
                       trigger_switch: Switch::Button(Button::LeftTrigger),
                       from_index: 1,
                       to_index: 3,
                   },
                ],
            }
       ],
    };

    let switch = Switch::Button(Button::RightTrigger);
    gamepad.process_current_potential_visit(
            SwitchClickPattern::ClickEnd(switch.clone()));
    gamepad.undo_last_layer_visit_with_switch(switch);

    assert_eq!(gamepad.layer_visits.len(), 1);
    assert_eq!(gamepad.layer_visits[0], LayerVisit{
            trigger_switch: Switch::Button(Button::LeftTrigger),
            from_index: 0,
            to_index: 2,
       });

    assert!(gamepad.potential_layer_visit.is_none());
}


#[test]
fn tick_move_to_and_visit_layer_are_handled_correctly() {
    // Simulate DoubleClick RightTrigger to go (via MoveToOrVisitLayer)
    // to second-layer-step-1 (index 4)

    let mut gamepad = LayersNavigator {
       current_layer_index: 0,
       layer_visits: Vec::new(),
       potential_layer_visit: None,
       layers_and_their_available_layer_visits: Vec::new(),
    };

    let switch = Switch::Button(Button::RightTrigger);
    gamepad.process_current_potential_visit(
            SwitchClickPattern::ClickEnd(switch.clone()));
    gamepad.undo_last_layer_visit_with_switch(switch);

    gamepad.move_to_or_visit_layer(Switch::Button(Button::RightTrigger),LayerSpecifier {
        id: "second-layer-step-1".to_string(),
        index_in_gamepad: Some(4),
    });

    assert_eq!(gamepad.current_layer_index,4);
    assert_eq!(gamepad.layer_visits.len(), 0);
    assert_eq!(gamepad.potential_layer_visit.unwrap(),
        LayerVisit {
            trigger_switch: Switch::Button(Button::RightTrigger),
            from_index: 0,
            to_index: 4,
        });
}
