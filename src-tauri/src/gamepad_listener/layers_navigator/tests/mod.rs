use enigo::Key;
use gilrs::Button;
use mockall::predicate::eq;

use crate::{gamepad_listener::{layers_navigator::{LayersNavigator,LayersNavigatorTrait, LayerVisit, AvailableLayerVisitsFromLayer, tests::utils::{setup_haxwell_layout_layers_with_only_visits, setup_haxwell_layout_layers_and_their_available_layer_visits}, layers_wrapper::MockLayersWrapperTrait}, Switch, switch_click_pattern_detector::SwitchClickPattern, gilrs_events::stick_switch_interpreter::StickSwitchButton}, settings::models::layout::{LayerSpecifier, SwitchOnClickReaction, SwitchEventAndReaction, KeyboardInput, CardinalLevers, SingleCardinalLever}};

use super::LayerVisitTrigger;

mod utils;

#[test]
fn consumable_get_current_layer_index_works() {
    let mut layers_navigator = LayersNavigator {
       current_layer_index: 0,
       consumable_current_layer_index: Some(6),
       latest_move_to_index: 0,
       layer_visits: Vec::new(),
       potential_layer_visit: None,
       layers_and_their_available_layer_visits: Vec::new(),
       layers: Box::new(MockLayersWrapperTrait::new()),
    };
    assert_eq!(layers_navigator.consumable_get_current_layer_index().unwrap(),6);
    assert!(layers_navigator.consumable_get_current_layer_index().is_none());

    let mut layers_navigator = LayersNavigator {
       current_layer_index: 0,
       consumable_current_layer_index: None,
       latest_move_to_index: 0,
       layer_visits: Vec::new(),
       potential_layer_visit: None,
       layers_and_their_available_layer_visits: Vec::new(),
       layers: Box::new(MockLayersWrapperTrait::new()),
    };
    assert!(layers_navigator.consumable_get_current_layer_index().is_none());
}

fn setup_get_switch_event_and_reaction_works(
        current_layer_index: usize,
        switch: Switch,
        switch_event_and_reaction_opt: Option<SwitchEventAndReaction>,
    ) -> Option<SwitchEventAndReaction> {
    let mut mock_layers_wrapper = MockLayersWrapperTrait::new();

    mock_layers_wrapper
        .expect_get_switch_event_and_reaction()
        .with(eq(current_layer_index), eq(switch.clone()))
        .times(1)
        .return_const(switch_event_and_reaction_opt);

    let layers_navigator = LayersNavigator {
       current_layer_index,
       consumable_current_layer_index: Some(6),
       latest_move_to_index: 0,
       layer_visits: Vec::new(),
       potential_layer_visit: None,
       layers_and_their_available_layer_visits: Vec::new(),
       layers: Box::new(mock_layers_wrapper),
    };

    layers_navigator.get_switch_event_and_reaction(switch)
}

#[test]
fn get_switch_event_and_reaction_works() {
    let s_e_a_r = Some(SwitchEventAndReaction{
            on_click: None,
            on_double_click: None,
        });
    assert_eq!(
        setup_get_switch_event_and_reaction_works(
            0,
            Switch::Button(Button::C),
            s_e_a_r.clone()),s_e_a_r);

    let s_e_a_r = Some(SwitchEventAndReaction{
            on_click: Some(SwitchOnClickReaction::Keyboard(KeyboardInput{
                key: Key::Layout('I'),
                modifiers: vec![]
            })),
            on_double_click: None,
        });
    assert_eq!(
        setup_get_switch_event_and_reaction_works(
            12,
            Switch::Button(Button::South),
            s_e_a_r.clone()),s_e_a_r);

    let s_e_a_r = None;
    assert_eq!(
        setup_get_switch_event_and_reaction_works(
            120,
            Switch::StickSwitchButton(StickSwitchButton::LeftStickUp),
            s_e_a_r.clone()),s_e_a_r);
}


fn setup_get_cardinal_levers_works(
        current_layer_index: usize,
        cardinal_levers_opt: Option<CardinalLevers>,
    ) -> Option<CardinalLevers> {
    let mut mock_layers_wrapper = MockLayersWrapperTrait::new();

    mock_layers_wrapper
        .expect_get_cardinal_levers()
        .with(eq(current_layer_index))
        .times(1)
        .return_const(cardinal_levers_opt);

    let layers_navigator = LayersNavigator {
       current_layer_index,
       consumable_current_layer_index: Some(6),
       latest_move_to_index: 0,
       layer_visits: Vec::new(),
       potential_layer_visit: None,
       layers_and_their_available_layer_visits: Vec::new(),
       layers: Box::new(mock_layers_wrapper),
    };

    layers_navigator.get_cardinal_levers()
}
#[test]
fn get_cardinal_levers_works() {
    let cardinal_levers = Some(CardinalLevers{
        left_stick: Some(SingleCardinalLever::ControlMouseCursor),
        right_stick: None,
    });
    assert_eq!(
        setup_get_cardinal_levers_works(0,cardinal_levers.clone()),
        cardinal_levers);

    let cardinal_levers = Some(CardinalLevers{
        left_stick: Some(SingleCardinalLever::ControlMouseScrollwheel),
        right_stick: Some(SingleCardinalLever::ControlMouseCursor),
    });
    assert_eq!(
        setup_get_cardinal_levers_works(3,cardinal_levers.clone()),
        cardinal_levers);

    let cardinal_levers = None;
    assert_eq!(
        setup_get_cardinal_levers_works(500,cardinal_levers.clone()),
        cardinal_levers);
}

#[test]
fn visit_layer_works() {
    let mut layers_navigator = LayersNavigator {
       current_layer_index: 0,
       consumable_current_layer_index: None,
       latest_move_to_index: 0,
       layer_visits: Vec::new(),
       potential_layer_visit: None,
       layers_and_their_available_layer_visits: Vec::new(),
       layers: Box::new(MockLayersWrapperTrait::new()),
    };

    [
        (LayerVisitTrigger::Click(Switch::Button(Button::South)),1),
        (LayerVisitTrigger::DoubleClick(Switch::Button(Button::East)),2),
        (LayerVisitTrigger::Click(Switch::Button(Button::North)),3),
        (LayerVisitTrigger::DoubleClick(Switch::Button(Button::West)),4),
        (LayerVisitTrigger::Click(Switch::Button(Button::DPadUp)),6),
        (LayerVisitTrigger::DoubleClick(Switch::Button(Button::DPadDown)),8),
        (LayerVisitTrigger::Click(Switch::Button(Button::DPadLeft)),10),
        (LayerVisitTrigger::DoubleClick(Switch::Button(Button::DPadRight)),15),
        (LayerVisitTrigger::Click(Switch::StickSwitchButton(StickSwitchButton::LeftStickUp)),20),
        (LayerVisitTrigger::DoubleClick(Switch::StickSwitchButton(StickSwitchButton::LeftStickDown)),25),
        (LayerVisitTrigger::Click(Switch::StickSwitchButton(StickSwitchButton::LeftStickLeft)),30),
        (LayerVisitTrigger::Click(Switch::StickSwitchButton(StickSwitchButton::LeftStickRight)),40),
        (LayerVisitTrigger::DoubleClick(Switch::StickSwitchButton(StickSwitchButton::RightStickUp)),50),
        (LayerVisitTrigger::DoubleClick(Switch::StickSwitchButton(StickSwitchButton::RightStickDown)),40),
        (LayerVisitTrigger::Click(Switch::StickSwitchButton(StickSwitchButton::RightStickLeft)),30),
        (LayerVisitTrigger::Click(Switch::StickSwitchButton(StickSwitchButton::RightStickRight)),25),
        (LayerVisitTrigger::Click(Switch::Button(Button::RightTrigger)),4),
        (LayerVisitTrigger::DoubleClick(Switch::Button(Button::LeftTrigger)),3),
        (LayerVisitTrigger::DoubleClick(Switch::Button(Button::RightTrigger2)),2),
        (LayerVisitTrigger::DoubleClick(Switch::Button(Button::LeftTrigger2)),1),
    ]
    .iter()
    .enumerate()
    .for_each(|(idx,(switch,layer_index))|{
        let layer_index: usize = (*layer_index).try_into().unwrap();
        layers_navigator.visit_layer(
            switch.clone(),
            LayerSpecifier { id: String::new(), index_in_gamepad: Some(layer_index) });
        assert_eq!(layers_navigator.current_layer_index,layer_index);
        assert_eq!(layers_navigator.consumable_current_layer_index.unwrap(),layer_index);

        assert_eq!(layers_navigator.layer_visits.len(),idx+1);

        assert_eq!(layers_navigator.layer_visits[idx].trigger,switch.clone());
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
        assert_eq!(layers_navigator.latest_move_to_index,0);
    });
}

#[test]
fn move_to_layer_works() {
    let mut layers_navigator = LayersNavigator {
       current_layer_index: 0,
       consumable_current_layer_index: None,
       latest_move_to_index: 0,
       layer_visits: Vec::new(),
       potential_layer_visit: None,
       layers_and_their_available_layer_visits: Vec::new(),
       layers: Box::new(MockLayersWrapperTrait::new()),
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
            assert_eq!(layers_navigator.consumable_current_layer_index.unwrap(),index_in_gamepad);
            assert_eq!(layers_navigator.latest_move_to_index,index_in_gamepad);

            // all other things remain the same
            assert_eq!(layers_navigator.layer_visits.len(),0);
            assert!(layers_navigator.potential_layer_visit.is_none());
        });
}

#[test]
fn visit_or_move_to_layer_works() {
    let mut layers_navigator = LayersNavigator {
       current_layer_index: 0,
       consumable_current_layer_index: None,
       latest_move_to_index: 0,
       layer_visits: Vec::new(),
       potential_layer_visit: None,
       layers_and_their_available_layer_visits: Vec::new(),
       layers: Box::new(MockLayersWrapperTrait::new()),
    };


    let mut prev_current_layer_index = 0;
    [
        (LayerVisitTrigger::DoubleClick(Switch::Button(Button::South)),1),
        (LayerVisitTrigger::Click(Switch::Button(Button::East)),2),
        (LayerVisitTrigger::DoubleClick(Switch::Button(Button::North)),3),
        (LayerVisitTrigger::Click(Switch::Button(Button::West)),4),
        (LayerVisitTrigger::Click(Switch::Button(Button::DPadUp)),6),
        (LayerVisitTrigger::DoubleClick(Switch::Button(Button::DPadDown)),8),
        (LayerVisitTrigger::Click(Switch::Button(Button::DPadLeft)),10),
        (LayerVisitTrigger::Click(Switch::Button(Button::DPadRight)),15),
        (LayerVisitTrigger::Click(Switch::StickSwitchButton(StickSwitchButton::LeftStickUp)),20),
        (LayerVisitTrigger::DoubleClick(Switch::StickSwitchButton(StickSwitchButton::LeftStickDown)),25),
        (LayerVisitTrigger::Click(Switch::StickSwitchButton(StickSwitchButton::LeftStickLeft)),30),
        (LayerVisitTrigger::Click(Switch::StickSwitchButton(StickSwitchButton::LeftStickRight)),40),
        (LayerVisitTrigger::Click(Switch::StickSwitchButton(StickSwitchButton::RightStickUp)),50),
        (LayerVisitTrigger::Click(Switch::StickSwitchButton(StickSwitchButton::RightStickDown)),40),
        (LayerVisitTrigger::DoubleClick(Switch::StickSwitchButton(StickSwitchButton::RightStickLeft)),30),
        (LayerVisitTrigger::Click(Switch::StickSwitchButton(StickSwitchButton::RightStickRight)),25),
        (LayerVisitTrigger::Click(Switch::Button(Button::RightTrigger)),2),
        (LayerVisitTrigger::Click(Switch::Button(Button::LeftTrigger)),1),
    ]
    .iter()
    .for_each(|(trigger,layer_index)|{
        let layer_index: usize = (*layer_index).try_into().unwrap();
        layers_navigator.move_to_or_visit_layer(
            trigger.clone(),
            LayerSpecifier { id: String::new(), index_in_gamepad: Some(layer_index) });
        assert_eq!(layers_navigator.current_layer_index,layer_index);
        assert_eq!(layers_navigator.consumable_current_layer_index.unwrap(),layer_index);

        assert_eq!(
            layers_navigator.potential_layer_visit.as_ref().unwrap().to_index,
            layer_index);

        assert_eq!(
            layers_navigator.potential_layer_visit.as_ref().unwrap().from_index,
            prev_current_layer_index);
        prev_current_layer_index = layers_navigator.current_layer_index;

        // all other things remain the same
        assert_eq!(layers_navigator.layer_visits.len(),0);
        assert_eq!(layers_navigator.latest_move_to_index,0);
    });
}

#[test]
fn undo_last_layer_visit_with_switch_works() {
    let layer_visits = vec![
       LayerVisit{
            trigger: LayerVisitTrigger::Click(Switch::Button(Button::RightTrigger)),
            from_index: 0,
            to_index: 1,
       },
       LayerVisit{
            trigger: LayerVisitTrigger::DoubleClick(Switch::Button(Button::South)),
            from_index: 1,
            to_index: 3,
       },
       LayerVisit{
            trigger: LayerVisitTrigger::Click(Switch::Button(Button::LeftTrigger)),
            from_index: 3,
            to_index: 4,
       },
       LayerVisit{
            trigger: LayerVisitTrigger::DoubleClick(Switch::StickSwitchButton(StickSwitchButton::LeftStickUp)),
            from_index: 4,
            to_index: 6,
       },
       LayerVisit{
            trigger: LayerVisitTrigger::Click(Switch::StickSwitchButton(StickSwitchButton::RightStickRight)),
            from_index: 6,
            to_index: 7,
       },
   ];

    let mut layers_navigator = LayersNavigator {
       current_layer_index: 6,
       consumable_current_layer_index: None,
       latest_move_to_index: 0,
       layer_visits: layer_visits.clone(),
       potential_layer_visit: None,
       // these layers_and_their_available_layer_visits,
       // are minimally to make the recorded layer_visits
       // possible (expect where it's a "SPECIAL CASE")
       layers_and_their_available_layer_visits: vec![
            AvailableLayerVisitsFromLayer {
                index_in_gamepad: 0,
                layer_visits: vec![
                   LayerVisit{
                       trigger: LayerVisitTrigger::Click(Switch::Button(Button::RightTrigger)),
                       from_index: 0,
                       to_index: 1,
                   },
                   // SPECIAL CASE, allows the use of layer_visits[1]
                   // to go from 0 to 3 (as opposed to the original 1 to 3)
                   LayerVisit{
                       trigger: LayerVisitTrigger::DoubleClick(Switch::Button(Button::South)),
                       from_index: 0,
                       to_index: 3,
                   },
                ]
            },
            AvailableLayerVisitsFromLayer {
                index_in_gamepad: 1,
                layer_visits: vec![
                   LayerVisit{
                       trigger: LayerVisitTrigger::DoubleClick(Switch::Button(Button::South)),
                       from_index: 1,
                       to_index: 3,
                   },
                ]
            },
            AvailableLayerVisitsFromLayer {
                index_in_gamepad: 3,
                layer_visits: vec![
                   LayerVisit{
                       trigger: LayerVisitTrigger::Click(Switch::Button(Button::LeftTrigger)),
                       from_index: 3,
                       to_index: 4,
                   },
                ]
            },
            AvailableLayerVisitsFromLayer {
                index_in_gamepad: 4,
                layer_visits: vec![
                   LayerVisit{
                       trigger: LayerVisitTrigger::DoubleClick(Switch::StickSwitchButton(StickSwitchButton::LeftStickUp)),
                       from_index: 4,
                       to_index: 6,
                   },
                ]
            },
            AvailableLayerVisitsFromLayer {
                index_in_gamepad: 6,
                layer_visits: vec![
                   LayerVisit{
                       trigger: LayerVisitTrigger::Click(Switch::StickSwitchButton(StickSwitchButton::RightStickRight)),
                       from_index: 6,
                       to_index: 7,
                   },
                ]
            },
       ],
       layers: Box::new(MockLayersWrapperTrait::new()),
    };

    // undo the last visit (the 5th)
    layers_navigator.undo_last_layer_visit_with_switch(
        Switch::StickSwitchButton(StickSwitchButton::RightStickRight));

    assert_eq!(layers_navigator.current_layer_index,6);
    assert_eq!(layers_navigator.consumable_current_layer_index.unwrap(),6);

    assert_eq!(layers_navigator.layer_visits.len(),4);
    assert_eq!(layers_navigator.layer_visits[0],layer_visits[0]);
    assert_eq!(layers_navigator.layer_visits[1],layer_visits[1]);
    assert_eq!(layers_navigator.layer_visits[2],layer_visits[2]);
    assert_eq!(layers_navigator.layer_visits[3],layer_visits[3]);

    // There are now 4 visits
    // undo the third visit, which cuts off the fourth(4th)
    // since it's no longer reachable, leaving only 2 visits
    layers_navigator.undo_last_layer_visit_with_switch(
        Switch::Button(Button::LeftTrigger));
    
    assert_eq!(layers_navigator.current_layer_index,3);
    assert_eq!(layers_navigator.consumable_current_layer_index.unwrap(),3);
    
    assert_eq!(layers_navigator.layer_visits.len(),2);
    assert_eq!(layers_navigator.layer_visits[0],layer_visits[0]);
    assert_eq!(layers_navigator.layer_visits[1],layer_visits[1]);

    // There are now 4 visits
    // Undo the first visit but still go to the second visit
    // via a different path, leaving the second visit still there
    layers_navigator.undo_last_layer_visit_with_switch(
        Switch::Button(Button::RightTrigger));
    
    assert_eq!(layers_navigator.current_layer_index,3);
    assert_eq!(layers_navigator.consumable_current_layer_index.unwrap(),3);
    
    assert_eq!(layers_navigator.layer_visits.len(),1);
    assert_eq!(layers_navigator.layer_visits[0],LayerVisit{
        trigger: layer_visits[1].trigger.clone(),
        from_index: 0,
        to_index: layer_visits[1].to_index});

}

fn setup_layers_navigator_with_potential_layer_visit(
    potential_layer_visit_init_opt: Option<LayerVisit>,
    process_current_potential_visit_arg: SwitchClickPattern) -> LayersNavigator {
    let mut layers_navigator = LayersNavigator {
       current_layer_index: 0,
       consumable_current_layer_index: None,
       latest_move_to_index: 0,
       layer_visits: Vec::new(),
       potential_layer_visit: 
           potential_layer_visit_init_opt.clone(),
       layers_and_their_available_layer_visits: Vec::new(),
       layers: Box::new(MockLayersWrapperTrait::new()),
    };

    layers_navigator.process_current_potential_visit(
        process_current_potential_visit_arg.clone());

    // all else remains the same
    assert!(layers_navigator.potential_layer_visit.is_none());
    assert_eq!(layers_navigator.current_layer_index,0);
    assert!(layers_navigator.consumable_current_layer_index.is_none());
    assert_eq!(layers_navigator.latest_move_to_index,0);

    layers_navigator
}

#[test]
fn process_current_potential_visit_works() {
    // Cases where the potential visit is not commited

    // 1. If a the same Switch is clicked
    //
    let layers_navigator = setup_layers_navigator_with_potential_layer_visit(
        // potential_layer_visit_init_opt
        Some(LayerVisit{
           trigger: LayerVisitTrigger::Click(Switch::Button(Button::RightTrigger)),
           from_index: 0,
           to_index: 1,
       }),
       // process_current_potential_visit_arg
       SwitchClickPattern::Click(Switch::Button(Button::RightTrigger)));

    assert!(layers_navigator.layer_visits.is_empty());
    assert_eq!(layers_navigator.current_layer_index,0);
    assert_eq!(layers_navigator.latest_move_to_index,0);

    // 2. If the same Switch is double-clicked
    //
    let layers_navigator = setup_layers_navigator_with_potential_layer_visit(
        // potential_layer_visit_init_opt
        Some(LayerVisit{
           trigger: LayerVisitTrigger::Click(Switch::Button(Button::DPadLeft)),
           from_index: 0,
           to_index: 10,
       }),
       // process_current_potential_visit_arg
       SwitchClickPattern::DoubleClick(Switch::Button(Button::DPadLeft)));

    assert!(layers_navigator.layer_visits.is_empty());
    assert_eq!(layers_navigator.current_layer_index,0);
    assert_eq!(layers_navigator.latest_move_to_index,0);

    // 3. If a different Switch is held (ClickAndHold)
    //
    let layers_navigator = setup_layers_navigator_with_potential_layer_visit(
        // potential_layer_visit_init_opt
        Some(LayerVisit{
           trigger: LayerVisitTrigger::Click(Switch::Button(Button::DPadLeft)),
           from_index: 0,
           to_index: 10,
       }),
       // process_current_potential_visit_arg
       SwitchClickPattern::ClickAndHold(Switch::Button(Button::DPadRight)));

    assert!(layers_navigator.layer_visits.is_empty());
    assert_eq!(layers_navigator.current_layer_index,0);
    assert_eq!(layers_navigator.latest_move_to_index,0);

    // 4. If a different Switch is held (DoubleClickAndHold)
    //
    let layers_navigator = setup_layers_navigator_with_potential_layer_visit(
        // potential_layer_visit_init_opt
        Some(LayerVisit{
           trigger: LayerVisitTrigger::Click(Switch::StickSwitchButton(StickSwitchButton::LeftStickLeft)),
           from_index: 0,
           to_index: 10,
       }),
       // process_current_potential_visit_arg
       SwitchClickPattern::DoubleClickAndHold(Switch::StickSwitchButton(StickSwitchButton::LeftStickDown)));

    assert!(layers_navigator.layer_visits.is_empty());
    assert_eq!(layers_navigator.current_layer_index,0);
    assert_eq!(layers_navigator.latest_move_to_index,0);

    // 5. If any Switch is released (ClickEnd)
    //
    let layers_navigator = setup_layers_navigator_with_potential_layer_visit(
        // potential_layer_visit_init_opt
        Some(LayerVisit{
           trigger: LayerVisitTrigger::Click(Switch::Button(Button::DPadDown)),
           from_index: 0,
           to_index: 10,
       }),
       // process_current_potential_visit_arg
       SwitchClickPattern::ClickEnd(Switch::StickSwitchButton(StickSwitchButton::LeftStickLeft)));

    assert!(layers_navigator.layer_visits.is_empty());
    assert_eq!(layers_navigator.current_layer_index,0);
    assert_eq!(layers_navigator.latest_move_to_index,0);






    // Cases where the potential visit is commited

    // 1. If a different Switch is clicked
    //
    let potential_layer_visit_init = LayerVisit{
           trigger: LayerVisitTrigger::Click(Switch::Button(Button::RightTrigger)),
           from_index: 3,
           to_index: 11,
       };

    let layers_navigator = setup_layers_navigator_with_potential_layer_visit(
        // potential_layer_visit_init_opt
        Some(potential_layer_visit_init.clone()),
       // process_current_potential_visit_arg
       SwitchClickPattern::Click(Switch::Button(Button::South)));

    assert_eq!(layers_navigator.layer_visits.len(), 1);
    assert_eq!(layers_navigator.layer_visits[0], potential_layer_visit_init);
    assert_eq!(layers_navigator.current_layer_index,0);
    assert_eq!(layers_navigator.latest_move_to_index,0);

    // 2. If a different Switch is double-clicked
    //
    let potential_layer_visit_init = LayerVisit{
           trigger: LayerVisitTrigger::Click(Switch::Button(Button::DPadDown)),
           from_index: 3,
           to_index: 11,
       };

    let layers_navigator = setup_layers_navigator_with_potential_layer_visit(
        // potential_layer_visit_init_opt
        Some(potential_layer_visit_init.clone()),
       // process_current_potential_visit_arg
       SwitchClickPattern::DoubleClick(Switch::Button(Button::RightTrigger)));

    assert_eq!(layers_navigator.layer_visits.len(), 1);
    assert_eq!(layers_navigator.layer_visits[0], potential_layer_visit_init);
    assert_eq!(layers_navigator.current_layer_index,0);
    assert_eq!(layers_navigator.latest_move_to_index,0);

    // 3. If the same switch is held (ClickAndHold)
    //
    let potential_layer_visit_init = LayerVisit{
           trigger: LayerVisitTrigger::Click(Switch::Button(Button::RightTrigger)),
           from_index: 3,
           to_index: 11,
       };

    let layers_navigator = setup_layers_navigator_with_potential_layer_visit(
        // potential_layer_visit_init_opt
        Some(potential_layer_visit_init.clone()),
       // process_current_potential_visit_arg
       SwitchClickPattern::ClickAndHold(Switch::Button(Button::RightTrigger)));

    assert_eq!(layers_navigator.layer_visits.len(), 1);
    assert_eq!(layers_navigator.layer_visits[0], potential_layer_visit_init);
    assert_eq!(layers_navigator.current_layer_index,0);
    assert_eq!(layers_navigator.latest_move_to_index,0);

    // 4. If the same switch is held (DoubleClickAndHold)
    //
    let potential_layer_visit_init = LayerVisit{
           trigger: LayerVisitTrigger::Click(Switch::Button(Button::DPadLeft)),
           from_index: 3,
           to_index: 1,
       };

    let layers_navigator = setup_layers_navigator_with_potential_layer_visit(
        // potential_layer_visit_init_opt
        Some(potential_layer_visit_init.clone()),
       // process_current_potential_visit_arg
       SwitchClickPattern::ClickAndHold(Switch::Button(Button::DPadLeft)));

    assert_eq!(layers_navigator.layer_visits.len(), 1);
    assert_eq!(layers_navigator.layer_visits[0], potential_layer_visit_init);
    assert_eq!(layers_navigator.current_layer_index,0);
    assert_eq!(layers_navigator.latest_move_to_index,0);
}

#[test]
fn build_layer_visits_works() {
    let layers_and_their_available_layer_visits 
        = LayersNavigator::build_layer_visits(setup_haxwell_layout_layers_with_only_visits(),true);

    let expected = setup_haxwell_layout_layers_and_their_available_layer_visits();
    assert_eq!(layers_and_their_available_layer_visits[0], expected[0]);
    assert_eq!(layers_and_their_available_layer_visits[1], expected[1]);
    assert_eq!(layers_and_their_available_layer_visits[2], expected[2]);
    assert_eq!(layers_and_their_available_layer_visits[3], expected[3]);
    assert_eq!(layers_and_their_available_layer_visits[4], expected[4]);
    assert_eq!(layers_and_their_available_layer_visits[5], expected[5]);
    assert_eq!(layers_and_their_available_layer_visits.len(),expected.len());
    assert_eq!(layers_and_their_available_layer_visits.len(),6);
}

// this driver should use LayersNavigator 
// the way gamepad_listener::GamepadListener would
struct LayersNavigatorDriver {
    layers_navigator: LayersNavigator,
}

impl LayersNavigatorDriver {
    fn new(
        layers_navigator: LayersNavigator,
        ) -> Self {
        Self {
            layers_navigator
        }
    }

    fn process_event(
        &mut self,
        switch_click_pattern: SwitchClickPattern,
        switch_on_click_reaction: Option<SwitchOnClickReaction>) {
        self.layers_navigator.process_current_potential_visit(
            switch_click_pattern.clone());
        match switch_click_pattern {
            SwitchClickPattern::Click(switch) =>
                match switch_on_click_reaction {
                    Some(SwitchOnClickReaction::MoveToLayer(layer_specifier))
                    => self.layers_navigator.move_to_layer(layer_specifier),
                    Some(SwitchOnClickReaction::VisitLayer(layer_specifier))
                    => self.layers_navigator.visit_layer(LayerVisitTrigger::Click(switch),layer_specifier),
                    Some(SwitchOnClickReaction::MoveToOrVisitLayer(layer_specifier))
                    => self.layers_navigator.move_to_or_visit_layer(LayerVisitTrigger::Click(switch),layer_specifier),
                    _ => ()
                }
            SwitchClickPattern::DoubleClick(switch) => 
                match switch_on_click_reaction {
                    Some(SwitchOnClickReaction::MoveToLayer(layer_specifier))
                    => self.layers_navigator.move_to_layer(layer_specifier),
                    Some(SwitchOnClickReaction::VisitLayer(layer_specifier))
                    => self.layers_navigator.visit_layer(LayerVisitTrigger::DoubleClick(switch),layer_specifier),
                    Some(SwitchOnClickReaction::MoveToOrVisitLayer(layer_specifier))
                    => self.layers_navigator.move_to_or_visit_layer(LayerVisitTrigger::DoubleClick(switch),layer_specifier),
                    _ => ()
                }
            SwitchClickPattern::ClickAndHold(_switch)
            | SwitchClickPattern::DoubleClickAndHold(_switch)=> (),
            SwitchClickPattern::ClickEnd(switch)
                => self.layers_navigator.undo_last_layer_visit_with_switch(switch),

        }
    }
}

#[test]
fn scenario_double_click_to_go_from_layer_0_to_4() {
    let mut layers_navigator_driver = LayersNavigatorDriver::new(LayersNavigator {
       current_layer_index: 0,
       consumable_current_layer_index: None,
       latest_move_to_index: 0,
       layer_visits: Vec::new(),
       potential_layer_visit: None,
       layers_and_their_available_layer_visits:
           LayersNavigator::build_layer_visits(setup_haxwell_layout_layers_with_only_visits(),true),
       layers: Box::new(MockLayersWrapperTrait::new()),
    });

    let switch = Switch::Button(Button::RightTrigger);

    // Click
    layers_navigator_driver.process_event(
        SwitchClickPattern::Click(switch.clone()),
        Some(SwitchOnClickReaction::VisitLayer(
            LayerSpecifier {
                id: "first-layer-step-2".to_string(),
                index_in_gamepad: Some(1),
            })),
        );

    assert_eq!(layers_navigator_driver.layers_navigator.layer_visits.len(),1);
    assert_eq!(layers_navigator_driver.layers_navigator.layer_visits[0],
       LayerVisit{
           trigger: LayerVisitTrigger::Click(Switch::Button(Button::RightTrigger)),
           from_index: 0,
           to_index: 1,
       });
    assert_eq!(layers_navigator_driver.layers_navigator.current_layer_index,1);
    assert_eq!(layers_navigator_driver.layers_navigator.consumable_current_layer_index.unwrap(),1);
    assert_eq!(layers_navigator_driver.layers_navigator.latest_move_to_index,0);

    // ClickEnd
    layers_navigator_driver.process_event(
        SwitchClickPattern::ClickEnd(switch.clone()),None);
   
    assert_eq!(layers_navigator_driver.layers_navigator.layer_visits.len(),0);
    assert_eq!(layers_navigator_driver.layers_navigator.current_layer_index,0);
    assert_eq!(layers_navigator_driver.layers_navigator.consumable_current_layer_index.unwrap(),0);
    assert_eq!(layers_navigator_driver.layers_navigator.latest_move_to_index,0);
   
    // DoubleClick
    layers_navigator_driver.process_event(
        SwitchClickPattern::DoubleClick(switch.clone()),
        Some(SwitchOnClickReaction::MoveToOrVisitLayer(LayerSpecifier {
            id: "second-layer-step-1".to_string(),
            index_in_gamepad: Some(4),
        })));
   
    assert_eq!(
       layers_navigator_driver.layers_navigator.potential_layer_visit.clone().unwrap(),
       LayerVisit{
           trigger: LayerVisitTrigger::DoubleClick(Switch::Button(Button::RightTrigger)),
           from_index: 0,
           to_index: 4,
       });
   
    assert_eq!(layers_navigator_driver.layers_navigator.layer_visits.len(),0);
    assert_eq!(layers_navigator_driver.layers_navigator.current_layer_index,4);
    assert_eq!(layers_navigator_driver.layers_navigator.consumable_current_layer_index.unwrap(),4);
    assert_eq!(layers_navigator_driver.layers_navigator.latest_move_to_index,0);
   
    // ClickEnd
    layers_navigator_driver.process_event(
        SwitchClickPattern::ClickEnd(switch.clone()), None);
   
    assert_eq!(layers_navigator_driver.layers_navigator.layer_visits.len(),0);
    assert_eq!(layers_navigator_driver.layers_navigator.current_layer_index,4);
    assert_eq!(layers_navigator_driver.layers_navigator.consumable_current_layer_index.unwrap(),4);

}

#[test]
fn scenario_double_click_to_go_from_layer_4_to_0() {
    let mut layers_navigator_driver = LayersNavigatorDriver::new(LayersNavigator {
       current_layer_index: 4,
       consumable_current_layer_index: None,
       latest_move_to_index: 4,
       layer_visits: Vec::new(),
       potential_layer_visit: None,
       layers_and_their_available_layer_visits:
           LayersNavigator::build_layer_visits(setup_haxwell_layout_layers_with_only_visits(),true),
       layers: Box::new(MockLayersWrapperTrait::new()),
    });

    let switch = Switch::Button(Button::RightTrigger);

    // Click
    layers_navigator_driver.process_event(
        SwitchClickPattern::Click(switch.clone()),
        Some(SwitchOnClickReaction::VisitLayer(
            LayerSpecifier {
                id: "second-layer-step-2".to_string(),
                index_in_gamepad: Some(5),
            })),
        );

    assert_eq!(layers_navigator_driver.layers_navigator.layer_visits.len(),1);
    assert_eq!(layers_navigator_driver.layers_navigator.layer_visits[0],
       LayerVisit{
           trigger: LayerVisitTrigger::Click(Switch::Button(Button::RightTrigger)),
           from_index: 4,
           to_index: 5,
       });
    assert_eq!(layers_navigator_driver.layers_navigator.current_layer_index,5);
    assert_eq!(layers_navigator_driver.layers_navigator.consumable_current_layer_index.unwrap(),5);
    assert_eq!(layers_navigator_driver.layers_navigator.latest_move_to_index,4);

    // ClickEnd
    layers_navigator_driver.process_event(
        SwitchClickPattern::ClickEnd(switch.clone()),None);
   
    assert_eq!(layers_navigator_driver.layers_navigator.layer_visits.len(),0);
    assert_eq!(layers_navigator_driver.layers_navigator.current_layer_index,4);
    assert_eq!(layers_navigator_driver.layers_navigator.consumable_current_layer_index.unwrap(),4);
    assert_eq!(layers_navigator_driver.layers_navigator.latest_move_to_index,4);
   
    // DoubleClick
    layers_navigator_driver.process_event(
        SwitchClickPattern::DoubleClick(switch.clone()),
        Some(SwitchOnClickReaction::MoveToOrVisitLayer(LayerSpecifier {
            id: "first-layer-step-1".to_string(),
            index_in_gamepad: Some(0),
        })));
   
    assert_eq!(
       layers_navigator_driver.layers_navigator.potential_layer_visit.clone().unwrap(),
       LayerVisit{
           trigger: LayerVisitTrigger::DoubleClick(Switch::Button(Button::RightTrigger)),
           from_index: 4,
           to_index: 0,
       });
   
    assert_eq!(layers_navigator_driver.layers_navigator.layer_visits.len(),0);
    assert_eq!(layers_navigator_driver.layers_navigator.current_layer_index,0);
    assert_eq!(layers_navigator_driver.layers_navigator.consumable_current_layer_index.unwrap(),0);
    assert_eq!(layers_navigator_driver.layers_navigator.latest_move_to_index,4);
   
    // ClickEnd
    layers_navigator_driver.process_event(
        SwitchClickPattern::ClickEnd(switch.clone()), None);
   
    assert_eq!(layers_navigator_driver.layers_navigator.layer_visits.len(),0);
    assert_eq!(layers_navigator_driver.layers_navigator.current_layer_index,0);
    assert_eq!(layers_navigator_driver.layers_navigator.consumable_current_layer_index.unwrap(),0);
    assert_eq!(layers_navigator_driver.layers_navigator.latest_move_to_index,0);

}

