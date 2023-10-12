
#[cfg(test)]
use mockall::{automock, predicate::*};

use crate::settings::data::{Layer, SwitchEventAndReaction, CardinalLevers};

use super::Switch;

#[cfg(test)]
mod tests;

#[cfg_attr(test, automock)]
pub trait LayersWrapperTrait {
    fn get_switch_event_and_reaction(
        &self, layer_index: usize, switch: Switch) -> Option<SwitchEventAndReaction>;
    fn get_cardinal_levers(
        &self, layer_index: usize) -> Option<CardinalLevers>;
}

pub struct LayersWrapper {
   layers: Vec<Layer>,
}

impl LayersWrapper {
    pub fn new(
       layers: Vec<Layer>,
    ) -> Self {
        Self{
            layers,
        }
    }
}

impl LayersWrapperTrait for LayersWrapper {
    fn get_switch_event_and_reaction(
        &self, layer_index: usize, switch: Switch) -> Option<SwitchEventAndReaction> {
        self.layers[layer_index].get_switch_event_and_reaction(switch.clone())
    }

    fn get_cardinal_levers(
        &self, layer_index: usize) -> Option<CardinalLevers> {
        self.layers[layer_index].cardinal_levers.clone()
    }
}
