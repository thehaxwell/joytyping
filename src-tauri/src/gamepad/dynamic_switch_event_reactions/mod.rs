use crate::settings_data::SwitchOnClickReaction;

use super::EventReactionOrNesting;
use super::Switch;
use super::DynamicSwitchEventReaction;
use super::Handle;

#[cfg(test)]
mod tests;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait DynamicSwitchEventReactionsTrait {
    fn add(
        &mut self,
        switch: Switch,switch_on_release_reaction: DynamicSwitchEventReaction);
    fn get(
        &mut self,
        switch: Switch, handle: Handle) -> Option<SwitchOnClickReaction>;
}

pub struct DynamicSwitchEventReactions {
    reactions: Vec<(Switch,DynamicSwitchEventReaction)>,
}

impl DynamicSwitchEventReactions {
    pub fn new() -> Self {
        Self{ 
            reactions: Vec::new(),
        }
    }
}

impl DynamicSwitchEventReactionsTrait for DynamicSwitchEventReactions {
    fn add(
        &mut self,
        switch: Switch,
        switch_on_release_reaction: DynamicSwitchEventReaction) {
        // enforcing unique keys
        for (index, reaction) in self.reactions.clone().iter().enumerate() {
            if reaction.0 == switch {
                self.reactions.remove(index);
            }
        }
        self.reactions.push((switch,switch_on_release_reaction))
    }

    fn get(&mut self,
        switch: Switch, handle: Handle) -> Option<SwitchOnClickReaction> {
        for (index, reaction) in self.reactions.clone().iter().enumerate() {
            if reaction.0 == switch {
                let reaction_or_nesting = match handle {
                    Handle::Click 
                        => if let DynamicSwitchEventReaction::Click(react_or_nest) 
                            = &reaction.1 { Some(react_or_nest) } else { None },
                    Handle::ClickAndHold
                        => if let DynamicSwitchEventReaction::ClickAndHold(react_or_nest) 
                            = &reaction.1 { Some(react_or_nest) } else { None },
                    Handle::DoubleClick
                        => if let DynamicSwitchEventReaction::DoubleClick(react_or_nest) 
                            = &reaction.1 { Some(react_or_nest) } else { None },
                    Handle::DoubleClickAndHold
                        => if let DynamicSwitchEventReaction::DoubleClickAndHold(react_or_nest) 
                            = &reaction.1 { Some(react_or_nest) } else { None },
                    Handle::ClickEnd
                        => if let DynamicSwitchEventReaction::ClickEnd(react_or_nest) 
                            = &reaction.1 { Some(react_or_nest) } else { None },
                };

                match reaction_or_nesting {
                    Some(EventReactionOrNesting::Reaction(switch_on_click_reaction))
                    => {
                        self.reactions.remove(index);
                        return Some(switch_on_click_reaction.clone());
                    }
                    Some(EventReactionOrNesting::Nesting(dynamic_switch_event_reaction_box))
                    => {
                        self.reactions.remove(index);
                        self.add(switch, *dynamic_switch_event_reaction_box.clone());
                        return None;
                    }
                    None => (),
                }
            }
        }

        None
    }
}
