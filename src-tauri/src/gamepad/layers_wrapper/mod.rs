#[cfg(test)]
use mockall::{automock, predicate::*};

use crate::settings::data::{Layer, SwitchEventAndReaction, CardinalLevers, Switches};

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
   // we store the Switches so that we can avoid
   // calling Layer::get_switches() repeatedly during typing
   layers_switches: Vec<Option<Switches>>,
}

impl LayersWrapper {
    pub fn new(
       layers: Vec<Layer>,left_upper_is_d_pad: bool
    ) -> Self {
        Self{
            layers_switches: layers
                .iter()
                .map(|layer| layer.get_switches(left_upper_is_d_pad))
                .collect(),
            layers,
        }
    }
}

impl LayersWrapperTrait for LayersWrapper {
    fn get_switch_event_and_reaction(
        &self, layer_index: usize, switch: Switch) -> Option<SwitchEventAndReaction> {
        if let Some(switches) = &self.layers_switches[layer_index] {
            return switches.get_switch_event_and_reaction(switch.clone());
        }
        None
    }

    fn get_cardinal_levers(
        &self, layer_index: usize) -> Option<CardinalLevers> {
        self.layers[layer_index].cardinal_levers.clone()
    }
}
