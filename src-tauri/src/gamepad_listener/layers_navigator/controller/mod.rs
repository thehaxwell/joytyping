use crate::{gamepad_listener::{switch_click_pattern_detector::SwitchClickPattern, Switch}, settings::models::layout::{SwitchOnClickReaction, CardinalLevers, SwitchEventAndReaction}};

use super::{LayersNavigatorTrait, LayerVisitTrigger};

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg(test)]
mod tests;

#[cfg_attr(test, automock)]
pub trait ControllerTrait {
    fn consumable_get_current_layer_index(&mut self) -> Option<usize>;
    fn get_cardinal_levers(
        &self) -> Option<CardinalLevers>;
    fn process_switch_event(
        &mut self,
        next_event_opt: Option<SwitchClickPattern>) -> Option<SwitchOnClickReaction>;
}

pub struct Controller {
   layers_navigator: Box<dyn LayersNavigatorTrait>,
}

impl Controller {
    pub fn new(
       layers_navigator: Box<dyn LayersNavigatorTrait>,
    )-> Self {
        Self {
            layers_navigator,
        }
    }
}

impl ControllerTrait for Controller {
    fn process_switch_event(
        &mut self,
        next_event_opt: Option<SwitchClickPattern>) -> Option<SwitchOnClickReaction> {

        if let Some(next_event) = next_event_opt.clone() {
            self.layers_navigator
                .process_current_potential_visit(
                next_event);
        }

        let is_double_click = match next_event_opt.clone() {
            Some(SwitchClickPattern::DoubleClick(_switch)) => { true }
            Some(SwitchClickPattern::DoubleClickAndHold(_switch)) => { true }
            _other => false
        };

        let reaction = match next_event_opt.clone() {
            Some(SwitchClickPattern::Click(switch)) 
            | Some(SwitchClickPattern::DoubleClick(switch))
            | Some(SwitchClickPattern::ClickAndHold(switch)) 
            | Some(SwitchClickPattern::DoubleClickAndHold(switch)) => {
                self.layers_navigator.get_switch_event_and_reaction(
                         switch.clone())
                       // For Click and DoubleClick
                       // this is useful to allow typing a letter twice fast,
                       // like "oo" in "look","book","too" etc.
                       // the first click will be Click and the second DoubleClick
                       //
                       // For ClickAndHold and DoubleClickAndHold
                       // if on_double_click (or fallback to on_click) is set to
                       // type out some key, then hold down that key
                       .and_then(|s_e_a_r| if is_double_click {
                           s_e_a_r.on_double_click
                                  .or_else(|| s_e_a_r.on_click )
                           }
                           else {
                            s_e_a_r.on_click
                        }) 
            }
            Some(SwitchClickPattern::ClickEnd(_switch)) => None,
            None => None,

        };

        match next_event_opt.clone() {
            Some(SwitchClickPattern::Click(switch)) 
                | Some(SwitchClickPattern::DoubleClick(switch)) => {
                let layer_visit_trigger 
                    = if is_double_click {
                        LayerVisitTrigger::DoubleClick(switch.clone())
                    } else {
                        LayerVisitTrigger::Click(switch.clone())
                    };

                match reaction.clone() {
                    Some(SwitchOnClickReaction::MoveToLayer(layer_specifier))
                    => self.layers_navigator.move_to_layer(layer_specifier),
                    Some(SwitchOnClickReaction::VisitLayer(layer_specifier))
                    => self.layers_navigator.visit_layer(layer_visit_trigger,layer_specifier),
                    Some(SwitchOnClickReaction::MoveToOrVisitLayer(layer_specifier))
                    => self.layers_navigator.move_to_or_visit_layer(layer_visit_trigger,layer_specifier),
                    _ => ()
                }

            }
            Some(SwitchClickPattern::ClickEnd(switch)) => {
                self.layers_navigator.undo_last_layer_visit_with_switch(switch.clone());
            }
            _other => ()
        };

        reaction
    }


    fn consumable_get_current_layer_index(&mut self) -> Option<usize>{
        self.layers_navigator.consumable_get_current_layer_index()
    }

    fn get_cardinal_levers(&self) -> Option<CardinalLevers>{
        self.layers_navigator.get_cardinal_levers()
    }
}
