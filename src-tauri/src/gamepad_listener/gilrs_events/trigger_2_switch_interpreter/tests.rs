use gilrs::Button;

use crate::{gamepad_listener::gilrs_events::trigger_2_switch_interpreter::Trigger2SwitchInterpreter, LeftOrRight, settings::models::main_config::Trigger2SwitchesClickThresholds};
use super::Trigger2SwitchInterpreterTrait;

use super::Trigger2SwitchEvent;


#[test]
fn new_initializes_correctly() {
    let trigger_2_switch_interpreter = Trigger2SwitchInterpreter::new(
        Trigger2SwitchesClickThresholds {
            left_trigger_2: 0.123, right_trigger_2: 0.212 },
        LeftOrRight::Left,
    );
    assert_eq!(trigger_2_switch_interpreter.is_pressed, false);
    assert_eq!(trigger_2_switch_interpreter.trigger_2_click_thresholds,
       Trigger2SwitchesClickThresholds {
            left_trigger_2: 0.123, right_trigger_2: 0.212 });
    assert_eq!(trigger_2_switch_interpreter.alignment, LeftOrRight::Left);
}

#[test]
fn interpret_left_trigger_move_works() {
    let mut trigger_2_switch_interpreter = Trigger2SwitchInterpreter {
        trigger_2_click_thresholds: Trigger2SwitchesClickThresholds {
            left_trigger_2: 0.123, right_trigger_2: 0.212 },
        alignment: LeftOrRight::Left,
        is_pressed: false,
    };

    assert!(trigger_2_switch_interpreter.interpret_move(0.122).is_none());
    assert_eq!(trigger_2_switch_interpreter.is_pressed,false);

    if let Trigger2SwitchEvent::ButtonPressed(button) = 
        trigger_2_switch_interpreter.interpret_move(0.123).unwrap() {
        assert_eq!(button,Button::LeftTrigger2);
    }
    else {
        assert!(false);
    }
    assert_eq!(trigger_2_switch_interpreter.is_pressed,true);

    assert!(trigger_2_switch_interpreter.interpret_move(0.124).is_none());
    assert!(trigger_2_switch_interpreter.interpret_move(0.2).is_none());
    assert!(trigger_2_switch_interpreter.interpret_move(0.123).is_none());
    assert_eq!(trigger_2_switch_interpreter.is_pressed,true);

    if let Trigger2SwitchEvent::ButtonReleased(button) = 
        trigger_2_switch_interpreter.interpret_move(0.122).unwrap() {
        assert_eq!(button,Button::LeftTrigger2);
    }
    else {
        assert!(false);
    }
    assert_eq!(trigger_2_switch_interpreter.is_pressed,false);

    assert!(trigger_2_switch_interpreter.interpret_move(0.1).is_none());
    assert_eq!(trigger_2_switch_interpreter.is_pressed,false);
}

#[test]
fn interpret_right_trigger_move_works() {
    let mut trigger_2_switch_interpreter = Trigger2SwitchInterpreter {
        trigger_2_click_thresholds: Trigger2SwitchesClickThresholds {
            left_trigger_2: 0.123, right_trigger_2: 0.212 },
        alignment: LeftOrRight::Right,
        is_pressed: false,
    };

    assert!(trigger_2_switch_interpreter.interpret_move(0.211).is_none());
    assert_eq!(trigger_2_switch_interpreter.is_pressed,false);

    if let Trigger2SwitchEvent::ButtonPressed(button) = 
        trigger_2_switch_interpreter.interpret_move(0.212).unwrap() {
        assert_eq!(button,Button::RightTrigger2);
    }
    else {
        assert!(false);
    }
    assert_eq!(trigger_2_switch_interpreter.is_pressed,true);

    assert!(trigger_2_switch_interpreter.interpret_move(0.213).is_none());
    assert!(trigger_2_switch_interpreter.interpret_move(0.3).is_none());
    assert!(trigger_2_switch_interpreter.interpret_move(0.212).is_none());
    assert_eq!(trigger_2_switch_interpreter.is_pressed,true);

    if let Trigger2SwitchEvent::ButtonReleased(button) = 
        trigger_2_switch_interpreter.interpret_move(0.211).unwrap() {
        assert_eq!(button,Button::RightTrigger2);
    }
    else {
        assert!(false);
    }
    assert_eq!(trigger_2_switch_interpreter.is_pressed,false);

    assert!(trigger_2_switch_interpreter.interpret_move(0.1).is_none());
    assert_eq!(trigger_2_switch_interpreter.is_pressed,false);
}

