use gilrs::Button;

use crate::gamepad::{on_release_reactions::{OnReleaseReactions,OnReleaseReactionsTrait, Reaction}, Switch, stick_switch_interpreter::StickSwitchButton, SwitchOnReleaseReaction};

#[test]
fn new_correctly_initializes() {
    assert_eq!(OnReleaseReactions::new().reactions.len(),0);
}

#[test]
fn add_consumable_works() {
    let switch = Switch::StickSwitchButton(StickSwitchButton::LeftStickUp);
    let reaction = SwitchOnReleaseReaction::MoveToLayerAtIndex(5);

    let mut reactions = OnReleaseReactions::new();
    reactions.add_consumable(switch.clone(), reaction.clone());

    assert_eq!(reactions.reactions.len(), 1);
    assert_eq!(reactions.reactions[0], Reaction{
        from_switch: switch.clone(),
        switch_on_release_reaction: reaction.clone(),
        delete_on_retrieval: true
    });

    let switch1 = Switch::Button(Button::C);
    let reaction1 = SwitchOnReleaseReaction::MoveToLayerAtIndex(8);

    reactions.add_consumable(switch1.clone(), reaction1.clone());
    assert_eq!(reactions.reactions.len(), 2);
    assert_eq!(reactions.reactions[0], Reaction{
        from_switch: switch.clone(),
        switch_on_release_reaction: reaction.clone(),
        delete_on_retrieval: true
    });
    assert_eq!(reactions.reactions[1], Reaction{
        from_switch: switch1.clone(),
        switch_on_release_reaction: reaction1.clone(),
        delete_on_retrieval: true
    });
}

#[test]
fn get_works() {
    let reactions_vec = vec![
        Reaction{
            from_switch: Switch::Button(Button::C),
            switch_on_release_reaction: SwitchOnReleaseReaction::MoveToLayerAtIndex(17),
            delete_on_retrieval: true
        },
        // note that this second Reaction has the same from_switch
        // which will be used as a key for get()
        Reaction{
            from_switch: Switch::Button(Button::C),
            switch_on_release_reaction: SwitchOnReleaseReaction::MoveToLayerAtIndex(7),
            delete_on_retrieval: false
        },
        Reaction{
            from_switch: Switch::StickSwitchButton(StickSwitchButton::LeftStickUp),
            switch_on_release_reaction: SwitchOnReleaseReaction::MoveToLayerAtIndex(17),
            delete_on_retrieval: true
        },
    ];

    let mut reactions = OnReleaseReactions {
        reactions: reactions_vec.clone(),
    };

    // get the first, it gets deleted
    assert_eq!(reactions.get(reactions_vec[0].from_switch.clone()).unwrap(),
        reactions_vec[0].switch_on_release_reaction.clone());

    assert_eq!(reactions.reactions.len(), 2);
    assert_eq!(reactions.reactions[0], reactions_vec[1].clone());
    assert_eq!(reactions.reactions[1], reactions_vec[2].clone());

    // get the second, it doesn't get deleted
    assert_eq!(reactions.get(reactions_vec[1].from_switch.clone()).unwrap(),
        reactions_vec[1].switch_on_release_reaction.clone());

    assert_eq!(reactions.reactions.len(), 2);
    assert_eq!(reactions.reactions[0], reactions_vec[1].clone());
    assert_eq!(reactions.reactions[1], reactions_vec[2].clone());

    // get the second, it gets deleted
    assert_eq!(reactions.get(reactions_vec[2].from_switch.clone()).unwrap(),
        reactions_vec[2].switch_on_release_reaction.clone());

    assert_eq!(reactions.reactions.len(), 1);
    assert_eq!(reactions.reactions[0], reactions_vec[1].clone());
}
