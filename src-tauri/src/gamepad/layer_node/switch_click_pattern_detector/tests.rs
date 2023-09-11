use std::time::Instant;

use gilrs::Button;

use crate::settings_data::KeyboardInput;
use crate::settings_data::LayerSpecifier;
use crate::settings_data::MouseInput;
use crate::settings_data::SwitchEventAndReaction;
use crate::settings_data::SwitchOnClickReaction;

use super::SwitchClickPatternDetector;
use super::SwitchClickPatternDetectorTrait;
use super::LatestSwitchEvent;
use super::SwitchClickPattern;
use super::Switch;
use super::SwitchEventType;

fn assert_latest_switch_events_are_equal(
    event1: LatestSwitchEvent, event2: LatestSwitchEvent){
    assert_eq!(event1.switch,event2.switch);
    assert_eq!(event1.event_and_reaction,event2.event_and_reaction);
    assert_eq!(event1.event_type,event2.event_type);
    assert!(event1.instant.duration_since(event2.instant).as_secs() < 1);
    assert!(event2.instant.duration_since(event1.instant).as_secs() < 1);
}

#[test]
fn click_and_hold_works() {
    let mut switch_click_pattern_detector = SwitchClickPatternDetector{
        latest_switch_event: None,
        latest_switch_click_pattern: None,
    };

    let on_click_reaction = SwitchOnClickReaction::Keyboard(
        KeyboardInput{
            key: enigo::Key::Layout('S'),
            modifiers: Vec::new(),
        });
    let on_click_and_hold_reaction = SwitchOnClickReaction::VisitLayer(LayerSpecifier{
        id: "some-id".to_string(),
        index_in_gamepad: None
    });

    let switch_event_and_reaction = SwitchEventAndReaction{
        on_click: Some(on_click_reaction.clone()),
        on_click_and_hold: Some(on_click_and_hold_reaction.clone()),
        on_double_click: None,
        on_double_click_and_hold: None,
    };

    switch_click_pattern_detector.button_pressed(
        Button::South,
        switch_event_and_reaction.clone());

    assert_latest_switch_events_are_equal(
        switch_click_pattern_detector.latest_switch_event.clone().unwrap(),
        LatestSwitchEvent {
            switch: Switch::Button(Button::South),
            event_and_reaction: switch_event_and_reaction.clone(),
            event_type: SwitchEventType::KeyDownIntoClick,
            instant: Instant::now(),
        });

    assert_eq!(
        switch_click_pattern_detector.latest_switch_click_pattern.clone().unwrap(),
        SwitchClickPattern::Click(
            on_click_reaction.clone()));

    assert_eq!(switch_click_pattern_detector.tick().unwrap(),
        SwitchClickPattern::Click(
            on_click_reaction.clone()));

    // NEXT: don't call button_released to count as click-and-hold
    
    std::thread::sleep(std::time::Duration::from_millis(500));

    assert_latest_switch_events_are_equal(
        switch_click_pattern_detector.latest_switch_event.clone().unwrap(),
        LatestSwitchEvent {
            switch: Switch::Button(Button::South),
            event_and_reaction: switch_event_and_reaction.clone(),
            event_type: SwitchEventType::KeyDownIntoClick,
            instant: Instant::now()
                .checked_sub(std::time::Duration::from_millis(500)).unwrap(),
        });

    assert_eq!(switch_click_pattern_detector.tick().unwrap(),
        SwitchClickPattern::ClickAndHold(
            on_click_and_hold_reaction.clone()));
}

#[test]
fn double_click_works() {
    let mut switch_click_pattern_detector = SwitchClickPatternDetector{
        latest_switch_event: None,
        latest_switch_click_pattern: None,
    };

    let on_click_reaction = SwitchOnClickReaction::Keyboard(
        KeyboardInput{
            key: enigo::Key::Layout('S'),
            modifiers: Vec::new(),
        });
    let on_double_click_reaction = SwitchOnClickReaction::VisitLayer(LayerSpecifier{
        id: "some-id".to_string(),
        index_in_gamepad: None
    });

    let switch_event_and_reaction = SwitchEventAndReaction{
        on_click: Some(on_click_reaction.clone()),
        on_click_and_hold: None,
        on_double_click: Some(on_double_click_reaction.clone()),
        on_double_click_and_hold: None,
    };

    switch_click_pattern_detector.button_pressed(
        Button::South,
        switch_event_and_reaction.clone());

    assert_latest_switch_events_are_equal(
        switch_click_pattern_detector.latest_switch_event.clone().unwrap(),
        LatestSwitchEvent {
            switch: Switch::Button(Button::South),
            event_and_reaction: switch_event_and_reaction.clone(),
            event_type: SwitchEventType::KeyDownIntoClick,
            instant: Instant::now(),
        });

    assert_eq!(
        switch_click_pattern_detector.latest_switch_click_pattern.clone().unwrap(),
        SwitchClickPattern::Click(
            on_click_reaction.clone()));

    assert_eq!(switch_click_pattern_detector.tick().unwrap(),
        SwitchClickPattern::Click(
            on_click_reaction.clone()));

    // NEXT: release quickly enough to not count as click-and-hold

    switch_click_pattern_detector.button_released(
        Button::South,
        switch_event_and_reaction.clone());
    
    assert_latest_switch_events_are_equal(
        switch_click_pattern_detector.latest_switch_event.clone().unwrap(),
        LatestSwitchEvent {
            switch: Switch::Button(Button::South),
            event_and_reaction: switch_event_and_reaction.clone(),
            event_type: SwitchEventType::KeyUpAfterClick,
            instant: Instant::now(),
        });

    assert!(switch_click_pattern_detector.latest_switch_click_pattern.is_none());
    assert!(switch_click_pattern_detector.tick().is_none());


    // NEXT: press again to trigger double-click
    switch_click_pattern_detector.button_pressed(
        Button::South,
        switch_event_and_reaction.clone());

    assert_latest_switch_events_are_equal(
        switch_click_pattern_detector.latest_switch_event.clone().unwrap(),
        LatestSwitchEvent {
            switch: Switch::Button(Button::South),
            event_and_reaction: switch_event_and_reaction.clone(),
            event_type: SwitchEventType::KeyDownIntoDoubleClick,
            instant: Instant::now(),
        });

    assert_eq!(
        switch_click_pattern_detector.latest_switch_click_pattern.clone().unwrap(),
        SwitchClickPattern::DoubleClick(
            on_double_click_reaction.clone()));

    assert_eq!(switch_click_pattern_detector.tick().unwrap(),
        SwitchClickPattern::DoubleClick(
            on_double_click_reaction.clone()));
    
    // NEXT: release quickly enough to not count as double-click-and-hold

    switch_click_pattern_detector.button_released(
        Button::South,
        switch_event_and_reaction.clone());
    
    assert_latest_switch_events_are_equal(
        switch_click_pattern_detector.latest_switch_event.clone().unwrap(),
        LatestSwitchEvent {
            switch: Switch::Button(Button::South),
            event_and_reaction: switch_event_and_reaction.clone(),
            event_type: SwitchEventType::KeyUpAfterDoubleClick,
            instant: Instant::now(),
        });

    assert!(switch_click_pattern_detector.latest_switch_click_pattern.is_none());
    assert!(switch_click_pattern_detector.tick().is_none());
}

#[test]
fn double_click_and_hold_works() {
    let mut switch_click_pattern_detector = SwitchClickPatternDetector{
        latest_switch_event: None,
        latest_switch_click_pattern: None,
    };

    let on_click_reaction = SwitchOnClickReaction::Keyboard(
        KeyboardInput{
            key: enigo::Key::Layout('S'),
            modifiers: Vec::new(),
        });
    let on_double_click_reaction = SwitchOnClickReaction::VisitLayer(LayerSpecifier{
        id: "some-id".to_string(),
        index_in_gamepad: None
    });
    let on_double_click_and_hold_reaction = SwitchOnClickReaction::Mouse(
        MouseInput{button: enigo::MouseButton::ScrollLeft});

    let switch_event_and_reaction = SwitchEventAndReaction{
        on_click: Some(on_click_reaction.clone()),
        on_click_and_hold: None,
        on_double_click: Some(on_double_click_reaction.clone()),
        on_double_click_and_hold: Some(on_double_click_and_hold_reaction.clone()),
    };

    switch_click_pattern_detector.button_pressed(
        Button::South,
        switch_event_and_reaction.clone());

    assert_latest_switch_events_are_equal(
        switch_click_pattern_detector.latest_switch_event.clone().unwrap(),
        LatestSwitchEvent {
            switch: Switch::Button(Button::South),
            event_and_reaction: switch_event_and_reaction.clone(),
            event_type: SwitchEventType::KeyDownIntoClick,
            instant: Instant::now(),
        });

    assert_eq!(
        switch_click_pattern_detector.latest_switch_click_pattern.clone().unwrap(),
        SwitchClickPattern::Click(
            on_click_reaction.clone()));

    assert_eq!(switch_click_pattern_detector.tick().unwrap(),
        SwitchClickPattern::Click(
            on_click_reaction.clone()));

    // NEXT: release quickly enough to not count as click-and-hold

    switch_click_pattern_detector.button_released(
        Button::South,
        switch_event_and_reaction.clone());
    
    assert_latest_switch_events_are_equal(
        switch_click_pattern_detector.latest_switch_event.clone().unwrap(),
        LatestSwitchEvent {
            switch: Switch::Button(Button::South),
            event_and_reaction: switch_event_and_reaction.clone(),
            event_type: SwitchEventType::KeyUpAfterClick,
            instant: Instant::now(),
        });

    assert!(switch_click_pattern_detector.latest_switch_click_pattern.is_none());
    assert!(switch_click_pattern_detector.tick().is_none());


    // NEXT: press again to trigger double-click
    switch_click_pattern_detector.button_pressed(
        Button::South,
        switch_event_and_reaction.clone());

    assert_latest_switch_events_are_equal(
        switch_click_pattern_detector.latest_switch_event.clone().unwrap(),
        LatestSwitchEvent {
            switch: Switch::Button(Button::South),
            event_and_reaction: switch_event_and_reaction.clone(),
            event_type: SwitchEventType::KeyDownIntoDoubleClick,
            instant: Instant::now(),
        });

    assert_eq!(
        switch_click_pattern_detector.latest_switch_click_pattern.clone().unwrap(),
        SwitchClickPattern::DoubleClick(
            on_double_click_reaction.clone()));

    assert_eq!(switch_click_pattern_detector.tick().unwrap(),
        SwitchClickPattern::DoubleClick(
            on_double_click_reaction.clone()));
    
    // NEXT: don't call button_released to count as double-click-and-hold
    
    std::thread::sleep(std::time::Duration::from_millis(500));

    assert_latest_switch_events_are_equal(
        switch_click_pattern_detector.latest_switch_event.clone().unwrap(),
        LatestSwitchEvent {
            switch: Switch::Button(Button::South),
            event_and_reaction: switch_event_and_reaction.clone(),
            event_type: SwitchEventType::KeyDownIntoDoubleClick,
            instant: Instant::now()
                .checked_sub(std::time::Duration::from_millis(500)).unwrap(),
        });

    assert_eq!(switch_click_pattern_detector.tick().unwrap(),
        SwitchClickPattern::DoubleClickAndHold(
            on_double_click_and_hold_reaction.clone()));
}

#[test]
fn tick_consumes_the_latest_switch_event() {
    let mut switch_click_pattern_detector = SwitchClickPatternDetector{
        latest_switch_event: None,
        latest_switch_click_pattern: Some(SwitchClickPattern::DoubleClickAndHold(
            SwitchOnClickReaction::Mouse(
            MouseInput{button: enigo::MouseButton::ScrollLeft}))),
    };

    assert_eq!(switch_click_pattern_detector.tick().unwrap(),
        SwitchClickPattern::DoubleClickAndHold(
        SwitchOnClickReaction::Mouse(
        MouseInput{button: enigo::MouseButton::ScrollLeft})));
    assert!(switch_click_pattern_detector.latest_switch_click_pattern.is_none());
    assert!(switch_click_pattern_detector.latest_switch_event.is_none());
}

#[test]
fn new_correcly_initializes_objects() {
    assert_eq!(SwitchClickPatternDetector::new(),
        SwitchClickPatternDetector{
            latest_switch_click_pattern: None,
            latest_switch_event: None,
        });
}
