use std::time::Instant;

use gilrs::Button;

// use crate::settings_data::KeyboardInput;
// use crate::settings_data::LayerSpecifier;
// use crate::settings_data::MouseInput;
// use crate::settings_data::SwitchEventAndReaction;
// use crate::settings_data::SwitchOnClickReaction;

use super::SwitchClickPatternDetector;
use super::SwitchClickPatternDetectorTrait;
use super::LatestSwitchEvent;
use super::SwitchClickPattern;
use super::Switch;
use super::SwitchEventType;

fn assert_latest_switch_events_are_equal(
    event1: LatestSwitchEvent, event2: LatestSwitchEvent){
    assert_eq!(event1.switch,event2.switch);
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

    switch_click_pattern_detector.button_pressed(
        Button::South);

    assert_latest_switch_events_are_equal(
        switch_click_pattern_detector.latest_switch_event.clone().unwrap(),
        LatestSwitchEvent {
            switch: Switch::Button(Button::South),
            event_type: SwitchEventType::KeyDownIntoClick,
            instant: Instant::now(),
        });

    assert_eq!(
        switch_click_pattern_detector.latest_switch_click_pattern.clone().unwrap(),
        SwitchClickPattern::Click(
            Switch::Button(Button::South),));

    assert_eq!(switch_click_pattern_detector.tick().unwrap(),
        SwitchClickPattern::Click(
            Switch::Button(Button::South),));

    // NEXT: don't call button_released to count as click-and-hold
    
    std::thread::sleep(std::time::Duration::from_millis(500));

    assert_latest_switch_events_are_equal(
        switch_click_pattern_detector.latest_switch_event.clone().unwrap(),
        LatestSwitchEvent {
            switch: Switch::Button(Button::South),
            event_type: SwitchEventType::KeyDownIntoClick,
            instant: Instant::now()
                .checked_sub(std::time::Duration::from_millis(500)).unwrap(),
        });

    assert_eq!(switch_click_pattern_detector.tick().unwrap(),
        SwitchClickPattern::ClickAndHold(
            Switch::Button(Button::South),));

    // NEXT: call button_released to fire key-up
    
    switch_click_pattern_detector.button_released(
        Button::South);

    assert_eq!(switch_click_pattern_detector.tick().unwrap(),
        SwitchClickPattern::ClickEnd(
            Switch::Button(Button::South)));
}

#[test]
fn double_click_works() {
    let mut switch_click_pattern_detector = SwitchClickPatternDetector{
        latest_switch_event: None,
        latest_switch_click_pattern: None,
    };

    switch_click_pattern_detector.button_pressed(
        Button::South);

    assert_latest_switch_events_are_equal(
        switch_click_pattern_detector.latest_switch_event.clone().unwrap(),
        LatestSwitchEvent {
            switch: Switch::Button(Button::South),
            event_type: SwitchEventType::KeyDownIntoClick,
            instant: Instant::now(),
        });

    assert_eq!(
        switch_click_pattern_detector.latest_switch_click_pattern.clone().unwrap(),
        SwitchClickPattern::Click(
            Switch::Button(Button::South)));

    assert_eq!(switch_click_pattern_detector.tick().unwrap(),
        SwitchClickPattern::Click(
            Switch::Button(Button::South)));

    // NEXT: release quickly enough to not count as click-and-hold

    switch_click_pattern_detector.button_released(
        Button::South);
    
    assert_latest_switch_events_are_equal(
        switch_click_pattern_detector.latest_switch_event.clone().unwrap(),
        LatestSwitchEvent {
            switch: Switch::Button(Button::South),
            event_type: SwitchEventType::KeyUpAfterClick,
            instant: Instant::now(),
        });

    assert_eq!(switch_click_pattern_detector
               .latest_switch_click_pattern.clone().unwrap(),
        SwitchClickPattern::ClickEnd(
            Switch::Button(Button::South)));
    assert_eq!(switch_click_pattern_detector.tick().unwrap(),
        SwitchClickPattern::ClickEnd(
            Switch::Button(Button::South)));


    // NEXT: press again to trigger double-click
    switch_click_pattern_detector.button_pressed(
        Button::South);

    assert_latest_switch_events_are_equal(
        switch_click_pattern_detector.latest_switch_event.clone().unwrap(),
        LatestSwitchEvent {
            switch: Switch::Button(Button::South),
            event_type: SwitchEventType::KeyDownIntoDoubleClick,
            instant: Instant::now(),
        });

    assert_eq!(
        switch_click_pattern_detector.latest_switch_click_pattern.clone().unwrap(),
        SwitchClickPattern::DoubleClick(
            Switch::Button(Button::South)));

    assert_eq!(switch_click_pattern_detector.tick().unwrap(),
        SwitchClickPattern::DoubleClick(
            Switch::Button(Button::South)));
    
    // NEXT: release quickly enough to not count as double-click-and-hold

    switch_click_pattern_detector.button_released(
        Button::South);
    
    assert_latest_switch_events_are_equal(
        switch_click_pattern_detector.latest_switch_event.clone().unwrap(),
        LatestSwitchEvent {
            switch: Switch::Button(Button::South),
            event_type: SwitchEventType::KeyUpAfterDoubleClick,
            instant: Instant::now(),
        });

    assert_eq!(switch_click_pattern_detector
               .latest_switch_click_pattern.clone().unwrap(),
        SwitchClickPattern::ClickEnd(
            Switch::Button(Button::South)));

    assert_eq!(switch_click_pattern_detector.tick().unwrap(),
        SwitchClickPattern::ClickEnd(
            Switch::Button(Button::South)));
}

#[test]
fn double_click_and_hold_works() {
    let mut switch_click_pattern_detector = SwitchClickPatternDetector{
        latest_switch_event: None,
        latest_switch_click_pattern: None,
    };

    switch_click_pattern_detector.button_pressed(
        Button::South);

    assert_latest_switch_events_are_equal(
        switch_click_pattern_detector.latest_switch_event.clone().unwrap(),
        LatestSwitchEvent {
            switch: Switch::Button(Button::South),
            event_type: SwitchEventType::KeyDownIntoClick,
            instant: Instant::now(),
        });

    assert_eq!(
        switch_click_pattern_detector.latest_switch_click_pattern.clone().unwrap(),
        SwitchClickPattern::Click(
            Switch::Button(Button::South)));

    assert_eq!(switch_click_pattern_detector.tick().unwrap(),
        SwitchClickPattern::Click(
            Switch::Button(Button::South)));

    // NEXT: release quickly enough to not count as click-and-hold

    switch_click_pattern_detector.button_released(
        Button::South);
    
    assert_latest_switch_events_are_equal(
        switch_click_pattern_detector.latest_switch_event.clone().unwrap(),
        LatestSwitchEvent {
            switch: Switch::Button(Button::South),
            event_type: SwitchEventType::KeyUpAfterClick,
            instant: Instant::now(),
        });

    assert_eq!(switch_click_pattern_detector
               .latest_switch_click_pattern.clone().unwrap(),
        SwitchClickPattern::ClickEnd(
            Switch::Button(Button::South)));

    assert_eq!(switch_click_pattern_detector.tick().clone().unwrap(),
        SwitchClickPattern::ClickEnd(
            Switch::Button(Button::South)));


    // NEXT: press again to trigger double-click
    switch_click_pattern_detector.button_pressed(
        Button::South);

    assert_latest_switch_events_are_equal(
        switch_click_pattern_detector.latest_switch_event.clone().unwrap(),
        LatestSwitchEvent {
            switch: Switch::Button(Button::South),
            event_type: SwitchEventType::KeyDownIntoDoubleClick,
            instant: Instant::now(),
        });

    assert_eq!(
        switch_click_pattern_detector.latest_switch_click_pattern.clone().unwrap(),
        SwitchClickPattern::DoubleClick(
            Switch::Button(Button::South)));

    assert_eq!(switch_click_pattern_detector.tick().unwrap(),
        SwitchClickPattern::DoubleClick(
            Switch::Button(Button::South)));
    
    // NEXT: don't call button_released to count as double-click-and-hold
    
    std::thread::sleep(std::time::Duration::from_millis(500));

    assert_latest_switch_events_are_equal(
        switch_click_pattern_detector.latest_switch_event.clone().unwrap(),
        LatestSwitchEvent {
            switch: Switch::Button(Button::South),
            event_type: SwitchEventType::KeyDownIntoDoubleClick,
            instant: Instant::now()
                .checked_sub(std::time::Duration::from_millis(500)).unwrap(),
        });

    assert_eq!(switch_click_pattern_detector.tick().unwrap(),
        SwitchClickPattern::DoubleClickAndHold(
            Switch::Button(Button::South)));
}

#[test]
fn tick_consumes_the_latest_switch_event() {
    let mut switch_click_pattern_detector = SwitchClickPatternDetector{
        latest_switch_event: None,
        latest_switch_click_pattern: Some(SwitchClickPattern::DoubleClickAndHold(
            Switch::Button(Button::East))),
    };

    assert_eq!(switch_click_pattern_detector.tick().unwrap(),
        SwitchClickPattern::DoubleClickAndHold(
        Switch::Button(Button::East)));
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

#[test]
fn double_clicks_are_mutually_exclusive() {
    assert!(true);
    let mut switch_click_pattern_detector = SwitchClickPatternDetector{
        latest_switch_event: None,
        latest_switch_click_pattern: None,
    };

    // this loop tests that only every second click 
    // counts as a double-click
    for idx in (1..10).into_iter() {
        switch_click_pattern_detector.button_pressed(
            Button::South);

        if idx % 2 == 0 {
            assert_latest_switch_events_are_equal(
                switch_click_pattern_detector.latest_switch_event.clone().unwrap(),
                LatestSwitchEvent {
                    switch: Switch::Button(Button::South),
                    event_type: SwitchEventType::KeyDownIntoDoubleClick,
                    instant: Instant::now(),
                });

            assert_eq!(
                switch_click_pattern_detector.latest_switch_click_pattern.clone().unwrap(),
                SwitchClickPattern::DoubleClick(
                    Switch::Button(Button::South)));

            assert_eq!(switch_click_pattern_detector.tick().unwrap(),
                SwitchClickPattern::DoubleClick(
                    Switch::Button(Button::South)));
        }
        else {
            assert_latest_switch_events_are_equal(
                switch_click_pattern_detector.latest_switch_event.clone().unwrap(),
                LatestSwitchEvent {
                    switch: Switch::Button(Button::South),
                    event_type: SwitchEventType::KeyDownIntoClick,
                    instant: Instant::now(),
                });

            assert_eq!(
                switch_click_pattern_detector.latest_switch_click_pattern.clone().unwrap(),
                SwitchClickPattern::Click(
                    Switch::Button(Button::South)));

            assert_eq!(switch_click_pattern_detector.tick().unwrap(),
                SwitchClickPattern::Click(
                    Switch::Button(Button::South)));
        }

        // NEXT: release quickly enough to not count as click-and-hold

        switch_click_pattern_detector.button_released(
            Button::South);
        
        if idx % 2 == 0 {
            assert_latest_switch_events_are_equal(
                switch_click_pattern_detector.latest_switch_event.clone().unwrap(),
                LatestSwitchEvent {
                    switch: Switch::Button(Button::South),
                    event_type: SwitchEventType::KeyUpAfterDoubleClick,
                    instant: Instant::now(),
                });
        }
        else {
            assert_latest_switch_events_are_equal(
                switch_click_pattern_detector.latest_switch_event.clone().unwrap(),
                LatestSwitchEvent {
                    switch: Switch::Button(Button::South),
                    event_type: SwitchEventType::KeyUpAfterClick,
                    instant: Instant::now(),
                });
        }

        assert_eq!(switch_click_pattern_detector
                   .latest_switch_click_pattern.clone().unwrap(),
            SwitchClickPattern::ClickEnd(
                Switch::Button(Button::South)));
        assert_eq!(switch_click_pattern_detector.tick().unwrap(),
            SwitchClickPattern::ClickEnd(
                Switch::Button(Button::South)));

    }
}
