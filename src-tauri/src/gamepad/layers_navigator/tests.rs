use gilrs::Button;

use crate::{gamepad::{layers_navigator::{LayersNavigator,LayersNavigatorTrait, LayerVisit}, Switch}, settings_data::LayerSpecifier};


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
       layer_visits_specified_for_each_layer: vec![
           (0, vec![
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
           ]),
           (1, vec![
               LayerVisit{
                   trigger_switch: Switch::Button(Button::LeftTrigger),
                   from_index: 1,
                   to_index: 3,
               },
           ])
       ],
    };

    gamepad.on_click_end(Switch::Button(Button::LeftTrigger));

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
       layer_visits_specified_for_each_layer: vec![
           (0, vec![
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
           ]),
           (1, vec![
               LayerVisit{
                   trigger_switch: Switch::Button(Button::LeftTrigger),
                   from_index: 1,
                   to_index: 3,
               },
           ])
       ],
    };

    gamepad.on_click_end(Switch::Button(Button::RightTrigger));
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
       layer_visits_specified_for_each_layer: Vec::new(),
    };

    gamepad.on_double_click(
        Switch::Button(Button::RightTrigger));

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