use gilrs::Button;

use crate::models::layout::{LayerSpecifier, SwitchEventAndReaction, Layer, SwitchOnClickReaction};

use super::{Switch, switch_click_pattern_detector::SwitchClickPattern, gilrs_events::stick_switch_interpreter::StickSwitchButton};

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg(test)]
mod tests;

#[cfg_attr(test, automock)]
pub trait LayersNavigatorTrait {
    fn move_to_layer(&mut self, layer_specifier: LayerSpecifier);
    fn visit_layer(&mut self, trigger: LayerVisitTrigger, layer_specifier: LayerSpecifier);
    fn move_to_or_visit_layer(&mut self, trigger: LayerVisitTrigger, layer_specifier: LayerSpecifier);
    fn get_current_layer_index(&self) -> usize;
    fn undo_last_layer_visit_with_switch(&mut self, switch: Switch);
    fn process_current_potential_visit(&mut self,pattern: SwitchClickPattern);
    fn consumable_get_current_layer_index(&mut self) -> Option<usize>;
}

pub struct LayersNavigator {
    current_layer_index: usize,

    // will be used to only send updates to
    // the quick_lookup_window when the
    // current_layer_index has been changed
    consumable_current_layer_index: Option<usize>,

    // recording when to go back on layer visits
    layer_visits: Vec<LayerVisit>,
    // any interruption cause an emptying
    potential_layer_visit: Option<LayerVisit>,

    layers_and_their_available_layer_visits: Vec<AvailableLayerVisitsFromLayer>,
    // this keeps track of the latest index that
    // was moved to, it doesn't count a MoveToOrVisit destination
    // until the -Visit is discarded, making it a definate
    // "MoveTo-"
    latest_move_to_index: usize,

}

impl LayersNavigator {
    pub fn new(layers: Vec<Layer>,left_upper_is_d_pad: bool) -> Self {
        Self {
            current_layer_index: 0,
            consumable_current_layer_index: None,
            latest_move_to_index: 0,
            layer_visits: Vec::new(),
            potential_layer_visit: None,
            layers_and_their_available_layer_visits: LayersNavigator::build_layer_visits(layers,left_upper_is_d_pad),
        }
    }

    fn build_layer_visits(layers: Vec<Layer>,left_upper_is_d_pad: bool) -> Vec<AvailableLayerVisitsFromLayer> {
        layers
            .iter()
            .enumerate()
            .filter_map(|(index,layer)| 
                if let Some(switches) = &layer.get_switches(left_upper_is_d_pad) {
                    let layer_visits: Vec<LayerVisit> = [
                            (Switch::Button(Button::South),&switches.south),
                            (Switch::Button(Button::East),&switches.east),
                            (Switch::Button(Button::North),&switches.north),
                            (Switch::Button(Button::West),&switches.west),
                            (Switch::Button(Button::DPadUp),&switches.d_pad_up),
                            (Switch::Button(Button::DPadDown),&switches.d_pad_down),
                            (Switch::Button(Button::DPadLeft),&switches.d_pad_left),
                            (Switch::Button(Button::DPadRight),&switches.d_pad_right),
                            (Switch::StickSwitchButton(StickSwitchButton::LeftStickUp),&switches.left_stick_up),
                            (Switch::StickSwitchButton(StickSwitchButton::LeftStickDown),&switches.left_stick_down),
                            (Switch::StickSwitchButton(StickSwitchButton::LeftStickLeft),&switches.left_stick_left),
                            (Switch::StickSwitchButton(StickSwitchButton::LeftStickRight),&switches.left_stick_right),
                            (Switch::StickSwitchButton(StickSwitchButton::RightStickUp),&switches.right_stick_up),
                            (Switch::StickSwitchButton(StickSwitchButton::RightStickDown),&switches.right_stick_down),
                            (Switch::StickSwitchButton(StickSwitchButton::RightStickLeft),&switches.right_stick_left),
                            (Switch::StickSwitchButton(StickSwitchButton::RightStickRight),&switches.right_stick_right),
                            (Switch::Button(Button::RightTrigger),&switches.right_trigger),
                            (Switch::Button(Button::LeftTrigger),&switches.left_trigger),
                            (Switch::Button(Button::RightTrigger2),&switches.right_trigger_2),
                            (Switch::Button(Button::LeftTrigger2),&switches.left_trigger_2),
                        ]
                        .iter()
                        .filter_map(|(switch,event_and_reaction_opt)|
                            if let Some(SwitchEventAndReaction { on_click, on_double_click }) = event_and_reaction_opt {
                                 // if let Some(SwitchOnClickReaction::VisitLayer(layer_specifier)) 
                                 //     = on_click.clone().or(on_double_click.clone()) {
                                 //        Some(LayerVisit {
                                 //            trigger_switch: switch.clone(),
                                 //            from_index: index,
                                 //            to_index: layer_specifier.index_in_gamepad.unwrap(),
                                 //        })
                                 // }
                                 // else { None }

                                let mut layer_visits: Vec<LayerVisit> = Vec::new();

                                // get layer_visits from on_click
                                 let to_index_opt = match on_click.clone() {
                                    Some(SwitchOnClickReaction::VisitLayer(layer_specifier))
                                        => Some(layer_specifier.index_in_gamepad.unwrap()),
                                    Some(SwitchOnClickReaction::MoveToOrVisitLayer(layer_specifier))
                                        => Some(layer_specifier.index_in_gamepad.unwrap()),
                                    _ => None,
                                 };

                                 if let Some(to_index) = to_index_opt {
                                    layer_visits.push(LayerVisit {
                                        trigger: LayerVisitTrigger::Click(switch.clone()),
                                        from_index: index,
                                        to_index,
                                    })
                                 }

                                // get layer_visits from on_double_click
                                 let to_index_opt = match on_double_click.clone() {
                                    Some(SwitchOnClickReaction::VisitLayer(layer_specifier))
                                        => Some(layer_specifier.index_in_gamepad.unwrap()),
                                    Some(SwitchOnClickReaction::MoveToOrVisitLayer(layer_specifier))
                                        => Some(layer_specifier.index_in_gamepad.unwrap()),
                                    _ => None,
                                 };

                                 if let Some(to_index) = to_index_opt {
                                    layer_visits.push(LayerVisit {
                                        trigger: LayerVisitTrigger::DoubleClick(switch.clone()),
                                        from_index: index,
                                        to_index,
                                    })
                                 }


                                 if layer_visits.is_empty() {
                                     None
                                 }
                                 else { Some(layer_visits) }
                            }
                            else { None }
                        )
                        .flatten()
                        .collect();

                    if layer_visits.is_empty() {
                        None
                    }
                    else {
                        Some(AvailableLayerVisitsFromLayer{
                            index_in_gamepad: index,
                            layer_visits,
                        })
                    }
                }
                else { None }
            )
            .collect()
    }

    fn set_current_layer_index(&mut self, index: usize) {
        self.current_layer_index = index;
        self.consumable_current_layer_index = Some(index);
    }
}

impl LayersNavigatorTrait for LayersNavigator {
    fn move_to_layer(&mut self, layer_specifier: LayerSpecifier) {
        println!(">>>>> from {}, move_to_layer: {:?}",self.current_layer_index ,layer_specifier);
        self.set_current_layer_index(layer_specifier.index_in_gamepad.unwrap());
        self.latest_move_to_index = self.current_layer_index;
    }

    fn visit_layer(&mut self, trigger: LayerVisitTrigger, layer_specifier: LayerSpecifier) {
        println!(">>>>> from {}, visit_layer: {:?}",self.current_layer_index ,layer_specifier);
        self.layer_visits.push(LayerVisit {
            trigger,
            to_index: layer_specifier.index_in_gamepad.unwrap(),
            from_index: self.current_layer_index,
        });
        self.set_current_layer_index(layer_specifier.index_in_gamepad.unwrap());
    }

    fn move_to_or_visit_layer(&mut self, trigger: LayerVisitTrigger, layer_specifier: LayerSpecifier) {
        println!(">>>>> from {}, move_to_or_visit_layer: {:?}",self.current_layer_index,layer_specifier);
        self.potential_layer_visit = Some(LayerVisit {
            trigger,
            to_index: layer_specifier.index_in_gamepad.unwrap(),
            from_index: self.current_layer_index,
        });
        self.set_current_layer_index(layer_specifier.index_in_gamepad.unwrap());
    }
    
    fn get_current_layer_index(&self) -> usize {
        self.current_layer_index
    }

    fn process_current_potential_visit(&mut self,pattern: SwitchClickPattern) {
        //  The conditions that are interpreted as a Visit
        //   click(MoveOrVisit) -> click-and-hold = Visit
        //   double-click(MoveOrVisit) -> double-click-and-hold = Visit
        //   click(MoveOrVisit) -> click-a-different-switch = Visit
        //   double-click(MoveOrVisit) -> click-a-different-switch = Visit
        //
        //  The conditions that are interpreted as a Move
        //   click(MoveOrVisit) -> end-click = Move
        //   double-click(MoveOrVisit) -> end-click = Move
        if let Some(potential_layer_visit) = &self.potential_layer_visit {
            let should_commit = match &potential_layer_visit.trigger {
                LayerVisitTrigger::Click(trigger_switch)
                    => match pattern {
                        SwitchClickPattern::Click(switch)
                            => *trigger_switch != switch,
                        SwitchClickPattern::ClickAndHold(switch)
                            => *trigger_switch == switch,
                        SwitchClickPattern::DoubleClick(switch)
                            => *trigger_switch != switch,
                        SwitchClickPattern::DoubleClickAndHold(switch)
                            => *trigger_switch == switch,
                        SwitchClickPattern::ClickEnd(_switch)
                            => false
                    },
                LayerVisitTrigger::DoubleClick(trigger_switch)
                    => match pattern {
                        SwitchClickPattern::Click(switch)
                            => *trigger_switch != switch,
                        SwitchClickPattern::ClickAndHold(switch)
                            => *trigger_switch == switch,
                        SwitchClickPattern::DoubleClick(switch)
                            => *trigger_switch != switch,
                        SwitchClickPattern::DoubleClickAndHold(switch)
                            => *trigger_switch == switch,
                        SwitchClickPattern::ClickEnd(_switch)
                            => false
                    },
            };

            if should_commit {
                self.layer_visits.push(potential_layer_visit.clone());
            }
            else {
                // the move now cannot be undone as a visit
                self.latest_move_to_index = self.current_layer_index;
            }

            self.potential_layer_visit = None;
        }
    }

    fn undo_last_layer_visit_with_switch(&mut self, switch: Switch) {
        let last_match_index_opt = self.layer_visits
            .iter()
            .enumerate()
            .rev()
            .find_map(|(index,vector)|{
                    let trigger_switch = match &vector.trigger {
                        LayerVisitTrigger::Click(trigger_switch) 
                            => trigger_switch,
                        LayerVisitTrigger::DoubleClick(trigger_switch) 
                            => trigger_switch,
                    };

                    if *trigger_switch == switch.clone() {
                        return Some(index);
                    }

                    None 
                });

        if let Some(to_remove_index) = last_match_index_opt {
            let start_from_index = self.layer_visits[0].from_index;
            self.layer_visits.remove(to_remove_index);

            // re-evaluate the layer_visits and current index
            let mut new_layer_visits: Vec<LayerVisit> = Vec::new();
            for (layer_visit_index, layer_visit) in self.layer_visits.iter().enumerate() {
                let next_doable_visit_opt = self.layers_and_their_available_layer_visits
                        .iter()
                        .find_map(|layer|{ 
                            if layer.index_in_gamepad 
                                != (if layer_visit_index == 0 {
                                        start_from_index
                                    } else {
                                        layer_visit.from_index
                                    }) { 
                                // ignore this iteration if this layers_and_their_available_layer_visits
                                // is not the layer at the index we want
                                return None; 
                            }

                            let index_to_go_from 
                                = if let Some(new_layer_visits_last) = new_layer_visits.last() {
                                new_layer_visits_last.to_index
                            } else {
                                self.latest_move_to_index
                            };

                            layer.layer_visits
                                .iter()
                                .find(|l|{
                                    let from_indexes_match = l.from_index == index_to_go_from;
                                    if let LayerVisitTrigger::Click(switch) = &l.trigger {
                                        if let LayerVisitTrigger::Click(switch2) = &layer_visit.trigger {
                                           return *switch == *switch2
                                              && from_indexes_match;
                                        }
                                    }
                                    if let LayerVisitTrigger::DoubleClick(switch) = &l.trigger {
                                        if let LayerVisitTrigger::DoubleClick(switch2) = &layer_visit.trigger {
                                           return *switch == *switch2
                                              && from_indexes_match;
                                        }
                                    }
                                    false
                                })
                                .map(|layer_visit|LayerVisit{
                                    trigger: layer_visit.trigger.clone(),
                                    from_index: layer.index_in_gamepad, // this is the outer-scope from_index
                                    to_index: layer_visit.to_index.clone(),
                                })
                        });

                if let Some(next_doable_visit) = next_doable_visit_opt {
                    new_layer_visits.push(next_doable_visit);
                }
                else {break}
            }
            self.layer_visits = new_layer_visits;

            if let Some(layer_visit) = self.layer_visits.last() {
                self.set_current_layer_index(layer_visit.to_index);
            }
            else {
                self.set_current_layer_index(self.latest_move_to_index);
            }
        }
    }

    fn consumable_get_current_layer_index(&mut self) -> Option<usize> {
        let index = self.consumable_current_layer_index;
        self.consumable_current_layer_index = None;
        index
    }
}

#[derive(Debug,Clone,PartialEq)]
struct LayerVisit {
    trigger: LayerVisitTrigger,
    to_index: usize,
    from_index: usize,
}

#[derive(Debug,Clone,PartialEq)]
pub enum LayerVisitTrigger {
    Click(Switch),
    DoubleClick(Switch),
}

#[derive(Debug,PartialEq)]
pub struct AvailableLayerVisitsFromLayer {
    index_in_gamepad: usize,
    layer_visits: Vec<LayerVisit>,
}
