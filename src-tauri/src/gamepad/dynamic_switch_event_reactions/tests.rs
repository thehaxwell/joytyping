use crate::gamepad::DynamicSwitchEventReaction;
use crate::gamepad::EventReactionOrNesting;
use crate::gamepad::Handle;
use crate::gamepad::Switch;
use crate::settings_data::KeyboardInput;
use crate::settings_data::LayerSpecifier;
use crate::settings_data::MouseInput;
use crate::settings_data::SwitchOnClickReaction;

use super::DynamicSwitchEventReactionsTrait;
use super::DynamicSwitchEventReactions;

fn setup_dynamic_event_reactions() -> (DynamicSwitchEventReactions,
               Vec<(Switch,DynamicSwitchEventReaction)>) {
    let reactions = vec![
        (
            Switch::Button(gilrs::Button::East),
            DynamicSwitchEventReaction::ClickEnd(
                EventReactionOrNesting::Reaction(
                    SwitchOnClickReaction::MoveToLayer(
                        LayerSpecifier{
                            id: String::new(),
                            index_in_gamepad: Some(12),
                        })))
        ),
        (
            Switch::Button(gilrs::Button::Select),
            DynamicSwitchEventReaction::DoubleClick(
                EventReactionOrNesting::Nesting(
                    Box::new(DynamicSwitchEventReaction::ClickEnd(
                        EventReactionOrNesting::Reaction(
                            SwitchOnClickReaction::MoveToLayer(
                                LayerSpecifier {
                                    id: String::new(),
                                    index_in_gamepad: Some(1),
                                }))))))
        ),
        (
            Switch::Button(gilrs::Button::Select),
            DynamicSwitchEventReaction::ClickEnd(
                EventReactionOrNesting::Reaction(
                    SwitchOnClickReaction::Keyboard(
                        KeyboardInput {
                            key: enigo::Key::Layout('I'),
                            modifiers: vec![enigo::Key::Alt] 
                        })))
        ),
    ];
    let obj = DynamicSwitchEventReactions {
        reactions: reactions.clone()
    };
    (obj,reactions)
}

#[test]
fn get_works() {
    let (mut dynamic_event_reactions,reactions) 
        = setup_dynamic_event_reactions();

    assert_eq!(
        dynamic_event_reactions.get(reactions[2].clone().0,Handle::ClickEnd).unwrap(),
            SwitchOnClickReaction::Keyboard(
                KeyboardInput {
                    key: enigo::Key::Layout('I'),
                    modifiers: vec![enigo::Key::Alt] 
                }));

    assert_eq!(dynamic_event_reactions.reactions.len(),2);
    assert_eq!(dynamic_event_reactions.reactions[0],reactions[0]);
    assert_eq!(dynamic_event_reactions.reactions[1],reactions[1]);


    assert!(
        dynamic_event_reactions.get(reactions[2].clone().0,Handle::ClickEnd).is_none(),
        "get-ing a non-existant item");


    // first call to get a reaction with a nesting gives
    // None but adds an unwrapped version of the reaction
    // internally 
    assert!(
        dynamic_event_reactions.get(reactions[1].clone().0,Handle::DoubleClick).is_none());
    assert_eq!(dynamic_event_reactions.reactions.len(),2);
    assert_eq!(dynamic_event_reactions.reactions[0],reactions[0]);

    assert_eq!(dynamic_event_reactions.reactions[1].0,reactions[1].0);
    assert_eq!(dynamic_event_reactions.reactions[1].1,
       DynamicSwitchEventReaction::ClickEnd(
            EventReactionOrNesting::Reaction(
                SwitchOnClickReaction::MoveToLayer(
                    LayerSpecifier {
                        id: String::new(),
                        index_in_gamepad: Some(1),
                    }))));


    assert_eq!(
        dynamic_event_reactions.get(reactions[1].clone().0,Handle::ClickEnd).unwrap(),
            SwitchOnClickReaction::MoveToLayer(
                LayerSpecifier {
                    id: String::new(),
                    index_in_gamepad: Some(1),
                }));

    assert_eq!(dynamic_event_reactions.reactions.len(),1);
    assert_eq!(dynamic_event_reactions.reactions[0],reactions[0]);
}

#[test]
fn add_works() {
    let (mut dynamic_event_reactions,reactions) 
        = setup_dynamic_event_reactions();

    assert_eq!(dynamic_event_reactions.reactions.len(),3);
    assert_eq!(dynamic_event_reactions.reactions[0],reactions[0]);
    assert_eq!(dynamic_event_reactions.reactions[1],reactions[1]);
    assert_eq!(dynamic_event_reactions.reactions[2],reactions[2]);

    let new_entry = (
        Switch::Button(gilrs::Button::South),
            DynamicSwitchEventReaction::ClickAndHold(
                EventReactionOrNesting::Reaction(
                    SwitchOnClickReaction::Mouse(
                        MouseInput {
                            button: enigo::MouseButton::Left }))));

    dynamic_event_reactions.add(new_entry.0.clone(),new_entry.1.clone());

    assert_eq!(dynamic_event_reactions.reactions.len(),4);
    assert_eq!(dynamic_event_reactions.reactions[0],reactions[0]);
    assert_eq!(dynamic_event_reactions.reactions[1],reactions[1]);
    assert_eq!(dynamic_event_reactions.reactions[2],reactions[2]);
    assert_eq!(dynamic_event_reactions.reactions[3],new_entry);
}

#[test]
fn add_enforces_unique_keys() {
    let (mut dynamic_event_reactions,reactions) 
        = setup_dynamic_event_reactions();

    // unique keys are enforced by overriding
    // the existing entry that had the same key (switch)
    let new_entry = (
        reactions[0].0.clone(),
            DynamicSwitchEventReaction::ClickEnd(
                EventReactionOrNesting::Reaction(
                    SwitchOnClickReaction::Keyboard(
                        KeyboardInput {
                            key: enigo::Key::Tab,
                            modifiers: vec![enigo::Key::Shift] 
                        }))));


    dynamic_event_reactions.add(
        new_entry.0.clone(),
        new_entry.1.clone());

    assert_eq!(dynamic_event_reactions.reactions.len(),3);
    assert_eq!(dynamic_event_reactions.reactions[0],reactions[1]);
    assert_eq!(dynamic_event_reactions.reactions[1],reactions[2]);
    assert_eq!(dynamic_event_reactions.reactions[2],new_entry);
}

