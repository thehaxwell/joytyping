use gilrs::Button;

use crate::gamepad::layers_navigator::LayerVisitTrigger;
use crate::models::layout::{Layer, SwitchesAdapter, SwitchEventAndReaction, SwitchOnClickReaction, LayerSpecifier};
use crate::gamepad::{layers_navigator::{AvailableLayerVisitsFromLayer, LayerVisit}, Switch};

pub fn setup_haxwell_layout_layers_with_only_visits() -> Vec<Layer> {
vec![
            Layer {
                id: "first-layer-step-1".to_string(),
                switches: Some(SwitchesAdapter {
                        right_upper_south: None,
                        right_upper_east: None,
                        right_upper_north: None,
                        right_upper_west: None,
                        left_upper_north: None,
                        left_upper_south: None,
                        left_upper_west: None,
                        left_upper_east: None,
                        left_lower_north: None,
                        left_lower_south: None,
                        left_lower_west: None,
                        left_lower_east: None,
                        right_lower_north: None,
                        right_lower_south: None,
                        right_lower_west: None,
                        right_lower_east: None,
                        left_trigger_2: None,
                        right_trigger_2: None,
                    right_trigger: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::VisitLayer(LayerSpecifier {
                            id: "first-layer-step-2".to_string(),
                            index_in_gamepad: Some(1),
                        })),
                        on_double_click: Some(SwitchOnClickReaction::MoveToOrVisitLayer(LayerSpecifier {
                            id: "second-layer-step-1".to_string(),
                            index_in_gamepad: Some(4),
                        })), 
                    }),
                    left_trigger: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::VisitLayer(LayerSpecifier {
                            id: "first-layer-step-3".to_string(),
                            index_in_gamepad: Some(2),
                        })),
                        on_double_click: Some(SwitchOnClickReaction::MoveToOrVisitLayer(LayerSpecifier {
                            id: "second-layer-step-1".to_string(),
                            index_in_gamepad: Some(4),
                        })), 
                    }),
                }),
                cardinal_levers: None,
            },
            Layer {
                id: "first-layer-step-2".to_string(),
                switches: Some(SwitchesAdapter {
                        right_upper_south: None,
                        right_upper_east: None,
                        right_upper_north: None,
                        right_upper_west: None,
                        left_upper_north: None,
                        left_upper_south: None,
                        left_upper_west: None,
                        left_upper_east: None,
                        left_lower_north: None,
                        left_lower_south: None,
                        left_lower_west: None,
                        left_lower_east: None,
                        right_lower_north: None,
                        right_lower_south: None,
                        right_lower_west: None,
                        right_lower_east: None,
                        left_trigger_2: None,
                        right_trigger_2: None,
                    right_trigger: None,
                    left_trigger: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::VisitLayer(LayerSpecifier {
                            id: "first-layer-step-4".to_string(),
                            index_in_gamepad: Some(3) 
                        })),
                        on_double_click: None, 
                    }),
                }),
                cardinal_levers: None,
            },
            Layer {
                id: "first-layer-step-3".to_string(),
                switches: Some(SwitchesAdapter {
                        right_upper_south: None,
                        right_upper_east: None,
                        right_upper_north: None,
                        right_upper_west: None,
                        left_upper_north: None,
                        left_upper_south: None,
                        left_upper_west: None,
                        left_upper_east: None,
                        left_lower_north: None,
                        left_lower_south: None,
                        left_lower_west: None,
                        left_lower_east: None,
                        right_lower_north: None,
                        right_lower_south: None,
                        right_lower_west: None,
                        right_lower_east: None,
                        left_trigger_2: None,
                        right_trigger_2: None,
                    right_trigger: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::VisitLayer(LayerSpecifier { 
                            id: "first-layer-step-4".to_string(),
                            index_in_gamepad: Some(3),
                        })),
                        on_double_click: None, 
                    }),
                    left_trigger: None,
                }),
                cardinal_levers: None,
            },
            Layer {
                id: "first-layer-step-4".to_string(),
                switches: Some(SwitchesAdapter {
                        right_upper_south: None,
                        right_upper_east: None,
                        right_upper_north: None,
                        right_upper_west: None,
                        left_upper_north: None,
                        left_upper_south: None,
                        left_upper_west: None,
                        left_upper_east: None,
                        left_lower_north: None,
                        left_lower_south: None,
                        left_lower_west: None,
                        left_lower_east: None,
                        right_lower_north: None,
                        right_lower_south: None,
                        right_lower_west: None,
                        right_lower_east: None,
                        right_trigger: None,
                        left_trigger: None,
                        left_trigger_2: None,
                        right_trigger_2: None,
                }),
                cardinal_levers: None,
            },
            Layer {
                id: "second-layer-step-1".to_string(),
                switches: Some(SwitchesAdapter {
                        right_upper_south: None,
                        right_upper_east: None,
                        right_upper_north: None,
                        right_upper_west: None,
                        left_upper_north: None,
                        left_upper_south: None,
                        left_upper_west: None,
                        left_upper_east: None,
                        left_lower_north: None,
                        left_lower_south: None,
                        left_lower_west: None,
                        left_lower_east: None,
                        right_lower_north: None,
                        right_lower_south: None,
                        right_lower_west: None,
                        right_lower_east: None,
                        left_trigger_2: None,
                        right_trigger_2: None,
                    right_trigger: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::VisitLayer(LayerSpecifier {
                            id: "second-layer-step-2".to_string(),
                            index_in_gamepad: Some(5),
                        })),
                        on_double_click: Some(SwitchOnClickReaction::MoveToOrVisitLayer(LayerSpecifier {
                            id: "fist-layer-step-1".to_string(),
                            index_in_gamepad: Some(0),
                        })), 
                    }),
                    left_trigger: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::VisitLayer(LayerSpecifier {
                            id: "second-layer-step-3".to_string(),
                            index_in_gamepad: Some(6),
                        })),
                        on_double_click: Some(SwitchOnClickReaction::MoveToOrVisitLayer(LayerSpecifier {
                            id: "fist-layer-step-1".to_string(),
                            index_in_gamepad: Some(0),
                        })), 
                    }),
                }),
                cardinal_levers: None,
            },
            Layer {
                id: "second-layer-step-2".to_string(),
                switches: Some(SwitchesAdapter {
                        right_upper_south: None,
                        right_upper_east: None,
                        right_upper_north: None,
                        right_upper_west: None,
                        left_upper_north: None,
                        left_upper_south: None,
                        left_upper_west: None,
                        left_upper_east: None,
                        left_lower_north: None,
                        left_lower_south: None,
                        left_lower_west: None,
                        left_lower_east: None,
                        right_lower_north: None,
                        right_lower_south: None,
                        right_lower_west: None,
                        right_lower_east: None,
                        left_trigger_2: None,
                        right_trigger_2: None,
                    right_trigger: None,
                    left_trigger: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::VisitLayer(LayerSpecifier {
                            id: "second-layer-step-4".to_string(),
                            index_in_gamepad: Some(7) })),
                        on_double_click: None, 
                    }),
                }),
                cardinal_levers: None,
            },
            Layer {
                id: "second-layer-step-3".to_string(),
                switches: Some(SwitchesAdapter {
                        right_upper_south: None,
                        right_upper_east: None,
                        right_upper_north: None,
                        right_upper_west: None,
                        left_upper_north: None,
                        left_upper_south: None,
                        left_upper_west: None,
                        left_upper_east: None,
                        left_lower_north: None,
                        left_lower_south: None,
                        left_lower_west: None,
                        left_lower_east: None,
                        right_lower_north: None,
                        right_lower_south: None,
                        right_lower_west: None,
                        right_lower_east: None,
                        left_trigger_2: None,
                        right_trigger_2: None,
                    right_trigger: Some(SwitchEventAndReaction {
                        on_click: Some(SwitchOnClickReaction::VisitLayer(LayerSpecifier{
                        id: "second-layer-step-4".to_string(),
                        index_in_gamepad: Some(7),
                        })),
                        on_double_click: None, 
                    }),
                    left_trigger: None,
                }),
                cardinal_levers: None,
            },
            Layer {
                id: "second-layer-step-4".to_string(),
                switches: Some(SwitchesAdapter {
                    right_upper_south: None,
                    right_upper_east: None,
                    right_upper_north: None,
                    right_upper_west: None,
                    left_upper_north: None,
                    left_upper_south: None,
                    left_upper_west: None,
                    left_upper_east: None,
                    left_lower_north: None,
                    left_lower_south: None,
                    left_lower_west: None,
                    left_lower_east: None,
                    right_lower_north: None,
                    right_lower_south: None,
                    right_lower_west: None,
                    right_lower_east: None,
                    right_trigger: None,
                    left_trigger: None,
                    left_trigger_2: None,
                    right_trigger_2: None,
                }),
                cardinal_levers: None,
            },
        ]
}

pub fn setup_haxwell_layout_layers_and_their_available_layer_visits() -> Vec<AvailableLayerVisitsFromLayer> {
    vec![
        AvailableLayerVisitsFromLayer {
            index_in_gamepad: 0,
            layer_visits: vec![
               LayerVisit{
                   trigger: LayerVisitTrigger::Click(Switch::Button(Button::RightTrigger)),
                   from_index: 0,
                   to_index: 1,
               },
               LayerVisit{
                   trigger: LayerVisitTrigger::DoubleClick(Switch::Button(Button::RightTrigger)),
                   from_index: 0,
                   to_index: 4,
               },
               LayerVisit{
                   trigger: LayerVisitTrigger::Click(Switch::Button(Button::LeftTrigger)),
                   from_index: 0,
                   to_index: 2,
               },
               LayerVisit{
                   trigger: LayerVisitTrigger::DoubleClick(Switch::Button(Button::LeftTrigger)),
                   from_index: 0,
                   to_index: 4,
               },
           ]
        },
        AvailableLayerVisitsFromLayer {
            index_in_gamepad: 1,
            layer_visits: vec![
               LayerVisit{
                   trigger: LayerVisitTrigger::Click(Switch::Button(Button::LeftTrigger)),
                   from_index: 1,
                   to_index: 3,
               },
           ]
        },
        AvailableLayerVisitsFromLayer {
            index_in_gamepad: 2,
            layer_visits: vec![
               LayerVisit{
                   trigger: LayerVisitTrigger::Click(Switch::Button(Button::RightTrigger)),
                   from_index: 2,
                   to_index: 3,
               },
           ]
        },
        AvailableLayerVisitsFromLayer {
            index_in_gamepad: 4,
            layer_visits: vec![
               LayerVisit{
                   trigger: LayerVisitTrigger::Click(Switch::Button(Button::RightTrigger)),
                   from_index: 4,
                   to_index: 5,
               },
               LayerVisit{
                   trigger: LayerVisitTrigger::DoubleClick(Switch::Button(Button::RightTrigger)),
                   from_index: 4,
                   to_index: 0,
               },
               LayerVisit{
                   trigger: LayerVisitTrigger::Click(Switch::Button(Button::LeftTrigger)),
                   from_index: 4,
                   to_index: 6,
               },
               LayerVisit{
                   trigger: LayerVisitTrigger::DoubleClick(Switch::Button(Button::LeftTrigger)),
                   from_index: 4,
                   to_index: 0,
               },
           ]
        },
        AvailableLayerVisitsFromLayer {
            index_in_gamepad: 5,
            layer_visits: vec![
               LayerVisit{
                   trigger: LayerVisitTrigger::Click(Switch::Button(Button::LeftTrigger)),
                   from_index: 5,
                   to_index: 7,
               },
           ]
        },
        AvailableLayerVisitsFromLayer {
            index_in_gamepad: 6,
            layer_visits: vec![
               LayerVisit{
                   trigger: LayerVisitTrigger::Click(Switch::Button(Button::RightTrigger)),
                   from_index: 6,
                   to_index: 7,
               },
           ]
        },
    ]
}
