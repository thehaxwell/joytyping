use gilrs::Button;

use crate::settings_data::{LayerSpecifier, SwitchEventAndReaction, Layer, SwitchOnClickReaction};

use super::Switch;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg(test)]
mod tests;

#[cfg_attr(test, automock)]
pub trait LayersNavigatorTrait {
    fn move_to_layer(&mut self, layer_specifier: LayerSpecifier);
    fn visit_layer(&mut self, trigger_switch: Switch, layer_specifier: LayerSpecifier);
    fn move_to_or_visit_layer(&mut self, trigger_switch: Switch, layer_specifier: LayerSpecifier);
    fn get_current_layer_index(&self) -> usize;

    fn on_click(&mut self, switch: Switch);
    fn on_click_and_hold(&mut self, switch: Switch);
    fn on_double_click(&mut self, switch: Switch);
    fn on_double_click_and_hold(&mut self, switch: Switch);
    fn on_click_end(&mut self, switch: Switch);
}

pub struct LayersNavigator {
   current_layer_index: usize,
    // recording when to go back on layer visits
    layer_visits: Vec<LayerVisit>,
    // any interruption cause an emptying
    potential_layer_visit: Option<LayerVisit>,

    layer_visits_specified_for_each_layer: Vec<(usize,Vec<LayerVisit>)>
}

impl LayersNavigator {
    pub fn new(layers: Vec<Layer>) -> Self {
        Self {
            current_layer_index: 0,
            layer_visits: Vec::new(),
            potential_layer_visit: None,
            layer_visits_specified_for_each_layer: LayersNavigator::build_layer_visits(layers),
        }
    }

    fn build_layer_visits(layers: Vec<Layer>) -> Vec<(usize,Vec<LayerVisit>)> {
        layers
            .iter()
            .enumerate()
            .filter_map(|(index,layer)| 
                if let Some(switches) = &layer.switches {
                    let layer_visits: Vec<LayerVisit> = [
                            (Switch::Button(Button::South),&switches.south),
                        ]
                        .iter()
                        .filter_map(|(switch,event_and_reaction_opt)|
                            if let Some(SwitchEventAndReaction { on_click, on_double_click }) = event_and_reaction_opt {
                                     
                    
                                     if let Some(SwitchOnClickReaction::VisitLayer(layer_specifier)) 
                                         = on_click.clone().or(on_double_click.clone()) {
                                            Some(LayerVisit {
                                                trigger_switch: switch.clone(),
                                                from_index: layer_specifier.index_in_gamepad.unwrap(),
                                                to_index: layer_specifier.index_in_gamepad.unwrap(),
                                            })
                                     }
                                     else { None }

                            }
                            else { None }
                        )
                        .collect();
                    Some((index,layer_visits))
                }
                else { None }
            )
            .collect()
    }


    //  The conditions that are interpreted as a Visit
    //   click(MoveOrVisit) -> click-and-hold = Visit
    //   double-click(MoveOrVisit) -> double-click-and-hold = Visit
    //   click(MoveOrVisit) -> click-a-different-switch = Visit
    //   double-click(MoveOrVisit) -> click-a-different-switch = Visit
    //
    //  The conditions that are interpreted as a Move
    //   click(MoveOrVisit) -> end-click = Move
    //   double-click(MoveOrVisit) -> end-click = Move
    fn conditionally_commit_potential_visit_and_unset_it<F>(&mut self, condition: F)
        where F: FnOnce(Switch) -> bool {
        if let Some(potential_layer_visit) = &self.potential_layer_visit {
            if condition(potential_layer_visit.trigger_switch.clone()) {
                self.layer_visits.push(potential_layer_visit.clone());
            }
                self.potential_layer_visit = None;
        }
    }

    fn undo_last_layer_visit_with_switch(&mut self, switch: Switch) {
        let last_match_index_opt = self.layer_visits
            .iter()
            .enumerate()
            .rev()
            .find_map(|(index,vector)| 
                if vector.trigger_switch == switch.clone() {
                    Some(index) 
                } else {
                    None 
                });

        if let Some(to_remove_index) = last_match_index_opt {
            let start_from_index = self.layer_visits[0].from_index;
            self.layer_visits.remove(to_remove_index);

            // re-evaluate the current index
            self.layer_visits = self.layer_visits
                .iter()
                .enumerate()
                .filter_map(|(layer_visit_index, layer_visit)|
                    self.layer_visits_specified_for_each_layer
                        .iter()
                        .find_map(|layer|{ 
                            if layer.0 
                                != (if layer_visit_index == 0 {
                                        start_from_index
                                    } else {
                                        layer_visit.from_index
                                    }) { 
                                return None; 
                            }

                            layer.1.iter().find(|l| l.trigger_switch == layer_visit.trigger_switch.clone())
                                .map(|layer_visit|LayerVisit{
                                    trigger_switch: layer_visit.trigger_switch.clone(),
                                    from_index: layer.0, // this is the outer-scope from_index
                                    to_index: layer_visit.to_index.clone(),
                                })
                    })
                )
                .collect();


        }
    }
}

impl LayersNavigatorTrait for LayersNavigator {
    fn move_to_layer(&mut self, layer_specifier: LayerSpecifier) {
        self.current_layer_index = layer_specifier.index_in_gamepad.unwrap();
    }
    fn visit_layer(&mut self, trigger_switch: Switch, layer_specifier: LayerSpecifier) {
                self.layer_visits.push(LayerVisit {
                    trigger_switch,
                    to_index: layer_specifier.index_in_gamepad.unwrap(),
                    from_index: self.current_layer_index,
                });
                self.current_layer_index = layer_specifier.index_in_gamepad.unwrap();
    }
    fn move_to_or_visit_layer(&mut self, trigger_switch: Switch, layer_specifier: LayerSpecifier) {
                self.potential_layer_visit = Some(LayerVisit {
                    trigger_switch,
                    to_index: layer_specifier.index_in_gamepad.unwrap(),
                    from_index: self.current_layer_index,
                });
                self.current_layer_index = layer_specifier.index_in_gamepad.unwrap();
    }
    
    fn get_current_layer_index(&self) -> usize {
        self.current_layer_index
    }

    fn on_click(&mut self, switch: Switch){
        self.conditionally_commit_potential_visit_and_unset_it(
            |potential_visit_trigger_switch| potential_visit_trigger_switch != switch);
    }

    fn on_click_and_hold(&mut self, switch: Switch){
        self.conditionally_commit_potential_visit_and_unset_it(
            |potential_visit_trigger_switch| potential_visit_trigger_switch == switch);
    }

    fn on_double_click(&mut self, switch: Switch){
        self.conditionally_commit_potential_visit_and_unset_it(
            |potential_visit_trigger_switch| potential_visit_trigger_switch != switch);
    }

    fn on_double_click_and_hold(&mut self, switch: Switch){
        self.conditionally_commit_potential_visit_and_unset_it(
            |potential_visit_trigger_switch| potential_visit_trigger_switch == switch);
    }

    fn on_click_end(&mut self, switch: Switch){
        self.potential_layer_visit = None;
        self.undo_last_layer_visit_with_switch(switch);
    }
}

#[derive(Debug,Clone,PartialEq)]
struct LayerVisit {
    trigger_switch: Switch,
    to_index: usize,
    from_index: usize,
}
