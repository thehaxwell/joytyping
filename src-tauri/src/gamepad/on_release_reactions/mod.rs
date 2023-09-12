use crate::gamepad::SwitchOnReleaseReaction;

use super::Switch;

#[cfg(test)]
mod tests;

pub trait OnReleaseReactionsTrait {
    fn add_consumable(
        &mut self,
        from_switch: Switch,switch_on_release_reaction: SwitchOnReleaseReaction);
    fn get(
        &mut self,
        from_switch: Switch) -> Option<SwitchOnReleaseReaction>;
}

pub struct OnReleaseReactions {
    reactions: Vec<Reaction>,
}

impl OnReleaseReactions {
    pub fn new() -> Self {
        Self{ 
            reactions: Vec::new(),
        }
    }
}

impl OnReleaseReactionsTrait for OnReleaseReactions {
    fn add_consumable(
        &mut self,
        from_switch: Switch,
        switch_on_release_reaction: SwitchOnReleaseReaction) {
        self.reactions.push(Reaction {
            from_switch,
            switch_on_release_reaction,
            delete_on_retrieval: true });
    }

    fn get(&mut self,
        from_switch: Switch) -> Option<SwitchOnReleaseReaction> {
        for (index, reaction) in self.reactions.clone().iter().enumerate() {
            if reaction.from_switch == from_switch {
                if reaction.delete_on_retrieval {
                    self.reactions.remove(index);
                }
                return Some(reaction.switch_on_release_reaction.clone());
            }
        }

        None
    }
}

#[derive(Debug,Clone,PartialEq)]
struct Reaction {
    from_switch: Switch,
    switch_on_release_reaction: SwitchOnReleaseReaction,
    delete_on_retrieval: bool,
}
