use gilrs::Button;

use crate::{gamepad::{layers_navigator::{LayersNavigator,LayersNavigatorTrait, LayerVisit, AvailableLayerVisitsFromLayer}, Switch, switch_click_pattern_detector::SwitchClickPattern, gilrs_events::stick_switch_interpreter::StickSwitchButton}, settings_data::LayerSpecifier};

#[test]
fn visit_layer_works() {
    let mut layers_navigator = LayersNavigator {
       current_layer_index: 0,
       layer_visits: Vec::new(),
       potential_layer_visit: None,
       layers_and_their_available_layer_visits: Vec::new(),
    };

    [
        (Switch::Button(Button::South),1),
        (Switch::Button(Button::East),2),
        (Switch::Button(Button::North),3),
        (Switch::Button(Button::West),4),
        (Switch::Button(Button::DPadUp),6),
        (Switch::Button(Button::DPadDown),8),
        (Switch::Button(Button::DPadLeft),10),
        (Switch::Button(Button::DPadRight),15),
        (Switch::StickSwitchButton(StickSwitchButton::LeftStickUp),20),
        (Switch::StickSwitchButton(StickSwitchButton::LeftStickDown),25),
        (Switch::StickSwitchButton(StickSwitchButton::LeftStickLeft),30),
        (Switch::StickSwitchButton(StickSwitchButton::LeftStickRight),40),
        (Switch::StickSwitchButton(StickSwitchButton::RightStickUp),50),
        (Switch::StickSwitchButton(StickSwitchButton::RightStickDown),40),
        (Switch::StickSwitchButton(StickSwitchButton::RightStickLeft),30),
        (Switch::StickSwitchButton(StickSwitchButton::RightStickRight),25),
        (Switch::Button(Button::RightTrigger),2),
        (Switch::Button(Button::LeftTrigger),1),
    ]
    .iter()
    .enumerate()
    .for_each(|(idx,(switch,layer_index))|{
        let layer_index: usize = (*layer_index).try_into().unwrap();
        layers_navigator.visit_layer(
            switch.clone(),
            LayerSpecifier { id: String::new(), index_in_gamepad: Some(layer_index) });
        assert_eq!(layers_navigator.current_layer_index,layer_index);

        assert_eq!(layers_navigator.layer_visits.len(),idx+1);

        assert_eq!(layers_navigator.layer_visits[idx].trigger_switch,switch.clone());
        assert_eq!(layers_navigator.layer_visits[idx].to_index,layer_index);
        if idx == 0 {
            assert_eq!(layers_navigator.layer_visits[idx].from_index,0);
        } else {
            // current from_index equals to the prev to_index
            let prev_to_index = layers_navigator.layer_visits[idx-1].to_index;
            assert_eq!(layers_navigator.layer_visits[idx].from_index,prev_to_index);
        }

        // all other things remain the same
        assert!(layers_navigator.potential_layer_visit.is_none());
    });
}

#[test]
fn move_to_layer_works() {
    let mut layers_navigator = LayersNavigator {
       current_layer_index: 0,
       layer_visits: Vec::new(),
       potential_layer_visit: None,
       layers_and_their_available_layer_visits: Vec::new(),
    };

    [1,4,2,4,6,7,323,23,56,3,9]
        .iter()
        .for_each(|to_index|{
            let index_in_gamepad: usize = (*to_index).try_into().unwrap();
             layers_navigator.move_to_layer(
                 LayerSpecifier { 
                     id: String::new(),
                     index_in_gamepad: Some(index_in_gamepad) });

            assert_eq!(layers_navigator.current_layer_index,index_in_gamepad);

            // all other things remain the same
            assert_eq!(layers_navigator.layer_visits.len(),0);
            assert!(layers_navigator.potential_layer_visit.is_none());
        });
}

#[test]
fn visit_or_move_to_layer_works() {
    let mut layers_navigator = LayersNavigator {
       current_layer_index: 0,
       layer_visits: Vec::new(),
       potential_layer_visit: None,
       layers_and_their_available_layer_visits: Vec::new(),
    };


    let mut prev_current_layer_index = 0;
    [
        (Switch::Button(Button::South),1),
        (Switch::Button(Button::East),2),
        (Switch::Button(Button::North),3),
        (Switch::Button(Button::West),4),
        (Switch::Button(Button::DPadUp),6),
        (Switch::Button(Button::DPadDown),8),
        (Switch::Button(Button::DPadLeft),10),
        (Switch::Button(Button::DPadRight),15),
        (Switch::StickSwitchButton(StickSwitchButton::LeftStickUp),20),
        (Switch::StickSwitchButton(StickSwitchButton::LeftStickDown),25),
        (Switch::StickSwitchButton(StickSwitchButton::LeftStickLeft),30),
        (Switch::StickSwitchButton(StickSwitchButton::LeftStickRight),40),
        (Switch::StickSwitchButton(StickSwitchButton::RightStickUp),50),
        (Switch::StickSwitchButton(StickSwitchButton::RightStickDown),40),
        (Switch::StickSwitchButton(StickSwitchButton::RightStickLeft),30),
        (Switch::StickSwitchButton(StickSwitchButton::RightStickRight),25),
        (Switch::Button(Button::RightTrigger),2),
        (Switch::Button(Button::LeftTrigger),1),
    ]
    .iter()
    .for_each(|(switch,layer_index)|{
        let layer_index: usize = (*layer_index).try_into().unwrap();
        layers_navigator.move_to_or_visit_layer(
            switch.clone(),
            LayerSpecifier { id: String::new(), index_in_gamepad: Some(layer_index) });
        assert_eq!(layers_navigator.current_layer_index,layer_index);

        assert_eq!(
            layers_navigator.potential_layer_visit.as_ref().unwrap().to_index,
            layer_index);

        assert_eq!(
            layers_navigator.potential_layer_visit.as_ref().unwrap().from_index,
            prev_current_layer_index);
        prev_current_layer_index = layers_navigator.current_layer_index;

        // all other things remain the same
        assert_eq!(layers_navigator.layer_visits.len(),0);
    });
}

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

    let mut layers_navigator = LayersNavigator {
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
    layers_navigator.process_current_potential_visit(
            SwitchClickPattern::ClickEnd(switch.clone()));
    layers_navigator.undo_last_layer_visit_with_switch(switch);

    assert_eq!(layers_navigator.layer_visits.len(), 1);
    assert_eq!(layers_navigator.layer_visits[0], layer_visits[0].clone());

    assert!(layers_navigator.potential_layer_visit.is_none());
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

    let mut layers_navigator = LayersNavigator {
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
    layers_navigator.process_current_potential_visit(
            SwitchClickPattern::ClickEnd(switch.clone()));
    layers_navigator.undo_last_layer_visit_with_switch(switch);

    assert_eq!(layers_navigator.layer_visits.len(), 1);
    assert_eq!(layers_navigator.layer_visits[0], LayerVisit{
            trigger_switch: Switch::Button(Button::LeftTrigger),
            from_index: 0,
            to_index: 2,
       });

    assert!(layers_navigator.potential_layer_visit.is_none());
}


#[test]
fn tick_move_to_and_visit_layer_are_handled_correctly() {
    // Simulate DoubleClick RightTrigger to go (via MoveToOrVisitLayer)
    // to second-layer-step-1 (index 4)

    let mut layers_navigator = LayersNavigator {
       current_layer_index: 0,
       layer_visits: Vec::new(),
       potential_layer_visit: None,
       layers_and_their_available_layer_visits: Vec::new(),
    };

    let switch = Switch::Button(Button::RightTrigger);
    layers_navigator.process_current_potential_visit(
            SwitchClickPattern::ClickEnd(switch.clone()));
    layers_navigator.undo_last_layer_visit_with_switch(switch);

    layers_navigator.move_to_or_visit_layer(Switch::Button(Button::RightTrigger),LayerSpecifier {
        id: "second-layer-step-1".to_string(),
        index_in_gamepad: Some(4),
    });

    assert_eq!(layers_navigator.current_layer_index,4);
    assert_eq!(layers_navigator.layer_visits.len(), 0);
    assert_eq!(layers_navigator.potential_layer_visit.unwrap(),
        LayerVisit {
            trigger_switch: Switch::Button(Button::RightTrigger),
            from_index: 0,
            to_index: 4,
        });
}
