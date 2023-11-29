use crate::{gamepad::cardinal_levers_move_detector::{CardinalLeversMoveDetector, calc}, models::{layout::MouseControl, main_config::DeadzoneUpperLimits}};

use super::CardinalLeversMoveDetectorTrait;

#[test]
fn initializer_works() {
    let deadzone_upper_limits = DeadzoneUpperLimits {
        left_stick: 0.123,
        right_stick: 0.3221,
    };
    let cardinal_levers_move_detector = CardinalLeversMoveDetector::new(deadzone_upper_limits);
    assert!(cardinal_levers_move_detector.left_mouse_control.is_none());
    assert!(cardinal_levers_move_detector.right_mouse_control.is_none());
    assert!(cardinal_levers_move_detector.current_left_stick_x.is_none());
    assert!(cardinal_levers_move_detector.current_left_stick_y.is_none());
    assert!(cardinal_levers_move_detector.current_right_stick_x.is_none());
    assert!(cardinal_levers_move_detector.current_right_stick_y.is_none());
    assert_eq!(cardinal_levers_move_detector.left_stick_at_rest_counter, 0);
    assert_eq!(cardinal_levers_move_detector.right_stick_at_rest_counter, 0);
}

// meant to mimic CardinalLeversMoveDetector::new()
fn setup_new_cardinal_levers_move_detector()
    -> CardinalLeversMoveDetector {
    CardinalLeversMoveDetector {
        deadzone_upper_limits: DeadzoneUpperLimits {
            left_stick: 0.123,
            right_stick: 0.3221,
        },
        left_mouse_control: None,
        right_mouse_control: None,
        current_left_stick_x: None,
        current_left_stick_y: None,
        current_right_stick_x: None,
        current_right_stick_y: None,
        left_stick_at_rest_counter: 0,
        right_stick_at_rest_counter: 0,
    }
}

#[test]
fn set_mouse_controls_works() {
    let mut cardinal_levers_move_detector 
        = setup_new_cardinal_levers_move_detector();

    cardinal_levers_move_detector.set_mouse_controls(
        None, None);

    assert!(cardinal_levers_move_detector.left_mouse_control.is_none());
    assert!(cardinal_levers_move_detector.right_mouse_control.is_none());
    assert!(cardinal_levers_move_detector.current_left_stick_x.is_none());
    assert!(cardinal_levers_move_detector.current_left_stick_y.is_none());
    assert!(cardinal_levers_move_detector.current_right_stick_x.is_none());
    assert!(cardinal_levers_move_detector.current_right_stick_y.is_none());


    let mut cardinal_levers_move_detector 
        = CardinalLeversMoveDetector {
        deadzone_upper_limits: DeadzoneUpperLimits {
            left_stick: 0.2,
            right_stick: 0.3221,
        },
        left_mouse_control: None,
        right_mouse_control: None,
        current_left_stick_x: Some(1),
        current_left_stick_y: Some(31),
        current_right_stick_x: Some(11),
        current_right_stick_y: Some(65),
        left_stick_at_rest_counter: 0,
        right_stick_at_rest_counter: 0,
    };

    let left_mouse_control = MouseControl {
        scale_factor: 5.0 };

    cardinal_levers_move_detector.set_mouse_controls(
        Some(left_mouse_control.clone()), None);

    assert_eq!(cardinal_levers_move_detector.left_mouse_control.unwrap(), left_mouse_control);
    assert!(cardinal_levers_move_detector.right_mouse_control.is_none());
    assert!(cardinal_levers_move_detector.current_left_stick_x.is_none());
    assert!(cardinal_levers_move_detector.current_left_stick_y.is_none());
    assert!(cardinal_levers_move_detector.current_right_stick_x.is_none());
    assert!(cardinal_levers_move_detector.current_right_stick_y.is_none());


    let mut cardinal_levers_move_detector 
        = CardinalLeversMoveDetector {
        deadzone_upper_limits: DeadzoneUpperLimits {
            left_stick: 0.12,
            right_stick: 1.201,
        },
        left_mouse_control: None,
        right_mouse_control: None,
        current_left_stick_x: Some(0),
        current_left_stick_y: Some(12),
        current_right_stick_x: None,
        current_right_stick_y: None,
        left_stick_at_rest_counter: 0,
        right_stick_at_rest_counter: 0,
    };

    let left_mouse_control = MouseControl {
        scale_factor: 5.12 };
    let right_mouse_control = MouseControl {
        scale_factor: 1.23 };

    cardinal_levers_move_detector.set_mouse_controls(
        Some(left_mouse_control.clone()), Some(right_mouse_control.clone()));

    assert_eq!(cardinal_levers_move_detector.left_mouse_control.unwrap(),
        left_mouse_control);
    assert_eq!(cardinal_levers_move_detector.right_mouse_control.unwrap(),
        right_mouse_control);
    assert!(cardinal_levers_move_detector.current_left_stick_x.is_none());
    assert!(cardinal_levers_move_detector.current_left_stick_y.is_none());
    assert!(cardinal_levers_move_detector.current_right_stick_x.is_none());
    assert!(cardinal_levers_move_detector.current_right_stick_y.is_none());
}

#[test]
fn calc_utility_fn_works() {
    // not passing the deadzone
    [
        // the second must be less than the
        // second. The third is random, it's not used anyway.
        (0.002, 0.001,        1.23),
        (0.10, 0.0,           2.3),
        (0.10, 0.09,          3.3),
        (0.4, 0.1,            1.1),
        (0.4, 0.2,            0.02),
        (0.4, 0.3,            0.0101),
        (0.4, 0.35,           10.0),
        (0.4, 0.39,           6.321),

        (0.002, -0.001,        1.23),
        (0.10, -0.0,           2.3),
        (0.10, -0.09,          3.3),
        (0.4, -0.1,            1.1),
        (0.4, -0.2,            0.02),
        (0.4, -0.3,            0.0101),
        (0.4, -0.35,           10.0),
        (0.4, -0.39,           6.321),
    ]
    .iter()
    .for_each(|(deadzone_upper_limit,value,scale_factor)|{
        assert_eq!(
            calc(&MouseControl {
                scale_factor: *scale_factor,
            },*deadzone_upper_limit,*value),
        0);
    });

    // passing the deadzone
    assert_eq!(calc(&MouseControl { scale_factor: 5.12 },
        0.0, 1.0), 5);
    assert_eq!(calc(&MouseControl { scale_factor: 5.12 },
        0.12, 1.0), 5);

    assert_eq!(calc(&MouseControl { scale_factor: 10.0 },
        0.0, 0.1), 1);
    assert_eq!(calc(&MouseControl { scale_factor: 10.0 },
        0.0, 0.3), 3);
    assert_eq!(calc(&MouseControl { scale_factor: 10.0 },
        0.0, 0.7), 7);
    assert_eq!(calc(&MouseControl { scale_factor: 10.0 },
        0.0, 1.0), 10);

    assert_eq!(calc(&MouseControl { scale_factor: 2.0 },
        0.0, 0.5), 1);
    assert_eq!(calc(&MouseControl { scale_factor: 2.4 },
        0.0, 0.5), 1);
    assert_eq!(calc(&MouseControl { scale_factor: 2.5 },
        0.0, 0.5), 1);
    assert_eq!(calc(&MouseControl { scale_factor: 2.9 },
        0.0, 0.5), 1);
    assert_eq!(calc(&MouseControl { scale_factor: 3.0 },
        0.0, 0.5), 2);

}

#[test]
fn axis_changed_works_for_left_stick_x() {
    [
        (Option::<i32>::None,0),
        (Some(5),5),
        (Some(100),100),
    ].iter().for_each(|(current_left_stick_y,result_current_left_stick_y)|{
        let mut cardinal_levers_move_detector 
            = CardinalLeversMoveDetector {
                deadzone_upper_limits: DeadzoneUpperLimits {
                    left_stick: 0.12,
                    right_stick: 0.2,
                },
                left_mouse_control: 
                    Some(MouseControl {
                        scale_factor: 5.12 }),
                right_mouse_control: None,
                current_left_stick_x: None,
                current_left_stick_y: *current_left_stick_y,
                current_right_stick_x: None,
                current_right_stick_y: None,
                left_stick_at_rest_counter: 0,
                right_stick_at_rest_counter: 0,
            };

        cardinal_levers_move_detector
            .axis_changed(gilrs::ev::Axis::LeftStickX,-1.0);

        assert_eq!(cardinal_levers_move_detector.current_left_stick_x.unwrap(),-5);
        assert_eq!(cardinal_levers_move_detector.current_left_stick_y.unwrap(),
            *result_current_left_stick_y);

        assert!(cardinal_levers_move_detector.current_right_stick_x.is_none());
        assert!(cardinal_levers_move_detector.current_right_stick_y.is_none());
        assert_eq!(cardinal_levers_move_detector.left_stick_at_rest_counter, 0);
        assert_eq!(cardinal_levers_move_detector.right_stick_at_rest_counter, 0);
    });
}

#[test]
fn axis_changed_works_for_left_stick_y() {
    [
        (Option::<i32>::None,0),
        (Some(5),5),
        (Some(100),100),
    ].iter().for_each(|(current_left_stick_x,result_current_left_stick_x)|{
        let mut cardinal_levers_move_detector 
            = CardinalLeversMoveDetector {
                deadzone_upper_limits: DeadzoneUpperLimits {
                    left_stick: 0.12,
                    right_stick: 0.2,
                },
                left_mouse_control: 
                    Some(MouseControl {
                        scale_factor: 5.12 }),
                right_mouse_control: None,
                current_left_stick_x: *current_left_stick_x,
                current_left_stick_y: None,
                current_right_stick_x: None,
                current_right_stick_y: None,
                left_stick_at_rest_counter: 0,
                right_stick_at_rest_counter: 0,
            };

        cardinal_levers_move_detector
            .axis_changed(gilrs::ev::Axis::LeftStickY,-1.0);

        assert_eq!(cardinal_levers_move_detector.current_left_stick_x.unwrap(),
            *result_current_left_stick_x);
        assert_eq!(cardinal_levers_move_detector.current_left_stick_y.unwrap(),5);

        assert!(cardinal_levers_move_detector.current_right_stick_x.is_none());
        assert!(cardinal_levers_move_detector.current_right_stick_y.is_none());
        assert_eq!(cardinal_levers_move_detector.left_stick_at_rest_counter, 0);
        assert_eq!(cardinal_levers_move_detector.right_stick_at_rest_counter, 0);
    });
}


#[test]
fn axis_changed_works_for_right_stick_x() {
    [
        (Option::<i32>::None,0),
        (Some(5),5),
        (Some(100),100),
    ].iter().for_each(|(current_right_stick_y,result_current_right_stick_y)|{
        let mut cardinal_levers_move_detector 
            = CardinalLeversMoveDetector {
                deadzone_upper_limits: DeadzoneUpperLimits {
                    left_stick: 0.1,
                    right_stick: 0.12,
                },
                left_mouse_control: None,
                right_mouse_control: 
                    Some(MouseControl {
                        scale_factor: 5.12 }),
                current_left_stick_x: None,
                current_left_stick_y: None,
                current_right_stick_x: None,
                current_right_stick_y: *current_right_stick_y,
                left_stick_at_rest_counter: 0,
                right_stick_at_rest_counter: 0,
            };

        cardinal_levers_move_detector
            .axis_changed(gilrs::ev::Axis::RightStickX,-1.0);


        assert!(cardinal_levers_move_detector.current_left_stick_x.is_none());
        assert!(cardinal_levers_move_detector.current_left_stick_y.is_none());

        assert_eq!(cardinal_levers_move_detector.current_right_stick_x.unwrap(),-5);
        assert_eq!(cardinal_levers_move_detector.current_right_stick_y.unwrap(),
            *result_current_right_stick_y);

        assert_eq!(cardinal_levers_move_detector.left_stick_at_rest_counter, 0);
        assert_eq!(cardinal_levers_move_detector.right_stick_at_rest_counter, 0);
    });
}

#[test]
fn axis_changed_works_for_right_stick_y() {
    [
        (Option::<i32>::None,0),
        (Some(5),5),
        (Some(100),100),
    ].iter().for_each(|(current_right_stick_x,result_current_right_stick_x)|{
        let mut cardinal_levers_move_detector 
            = CardinalLeversMoveDetector {
                deadzone_upper_limits: DeadzoneUpperLimits {
                    left_stick: 0.1,
                    right_stick: 0.12,
                },
                left_mouse_control: None,
                right_mouse_control: Some(MouseControl {
                        scale_factor: 5.12 }),
                current_left_stick_x: None,
                current_left_stick_y: None,
                current_right_stick_x: *current_right_stick_x,
                current_right_stick_y: None,
                left_stick_at_rest_counter: 0,
                right_stick_at_rest_counter: 0,
            };

        cardinal_levers_move_detector
            .axis_changed(gilrs::ev::Axis::RightStickY,-1.0);

        assert!(cardinal_levers_move_detector.current_left_stick_x.is_none());
        assert!(cardinal_levers_move_detector.current_left_stick_y.is_none());
        assert_eq!(cardinal_levers_move_detector.current_right_stick_x.unwrap(),
            *result_current_right_stick_x);
        assert_eq!(cardinal_levers_move_detector.current_right_stick_y.unwrap(),5);

        assert_eq!(cardinal_levers_move_detector.left_stick_at_rest_counter, 0);
        assert_eq!(cardinal_levers_move_detector.right_stick_at_rest_counter, 0);
    });
}

#[test]
fn tick_works_left_stick() {
    [
        (0,1),
        (1,0),
        (2,2),
        (12,30),
    ].iter().for_each(|(x,y)|{
        let x = *x;
        let y = *y;

        let mut cardinal_levers_move_detector 
            = CardinalLeversMoveDetector {
            deadzone_upper_limits: DeadzoneUpperLimits {
                left_stick: 0.12,
                right_stick: 0.21,
            },
            left_mouse_control: None,
            right_mouse_control: None,
            current_left_stick_x: Some(x),
            current_left_stick_y: Some(y),
            current_right_stick_x: None,
            current_right_stick_y: None,
            left_stick_at_rest_counter: 0,
            right_stick_at_rest_counter: 0,
        };

        assert_eq!(
            cardinal_levers_move_detector.tick().unwrap(),(x,y));

        assert!(cardinal_levers_move_detector.left_mouse_control.is_none());
        assert!(cardinal_levers_move_detector.right_mouse_control.is_none());
        assert_eq!(cardinal_levers_move_detector.current_left_stick_x.unwrap(),x);
        assert_eq!(cardinal_levers_move_detector.current_left_stick_y.unwrap(),y);
        assert!(cardinal_levers_move_detector.current_right_stick_x.is_none());
        assert!(cardinal_levers_move_detector.current_right_stick_y.is_none());
        assert_eq!(cardinal_levers_move_detector.left_stick_at_rest_counter, 0);
        assert_eq!(cardinal_levers_move_detector.right_stick_at_rest_counter, 0);

    });
}

#[test]
fn tick_left_stick_resting_position_wont_emit_forever() {
    let mut cardinal_levers_move_detector 
        = CardinalLeversMoveDetector {
        deadzone_upper_limits: DeadzoneUpperLimits {
            left_stick: 0.12,
            right_stick: 0.21,
        },
        left_mouse_control: None,
        right_mouse_control: None,
        current_left_stick_x: Some(0),
        current_left_stick_y: Some(0),
        current_right_stick_x: None,
        current_right_stick_y: None,
        left_stick_at_rest_counter: 0,
        right_stick_at_rest_counter: 0,
    };

    for i in (1..10).into_iter() {
        if i < 5 {
            assert_eq!(
                cardinal_levers_move_detector.tick().unwrap(),(0,0));
        }
        else {
            assert!(cardinal_levers_move_detector.tick().is_none());
        }

        assert!(cardinal_levers_move_detector.left_mouse_control.is_none());
        assert!(cardinal_levers_move_detector.right_mouse_control.is_none());
        assert_eq!(cardinal_levers_move_detector.current_left_stick_x.unwrap(),0);
        assert_eq!(cardinal_levers_move_detector.current_left_stick_y.unwrap(),0);
        assert!(cardinal_levers_move_detector.current_right_stick_x.is_none());
        assert!(cardinal_levers_move_detector.current_right_stick_y.is_none());

        if i < 5 {
            assert_eq!(cardinal_levers_move_detector.left_stick_at_rest_counter, i);
        }
        else {
            assert_eq!(cardinal_levers_move_detector.left_stick_at_rest_counter, 5);
        }

        assert_eq!(cardinal_levers_move_detector.right_stick_at_rest_counter, 0);
    }
}

#[test]
fn tick_works_right_stick() {
    [
        (0,1),
        (1,0),
        (2,2),
        (12,30),
    ].iter().for_each(|(x,y)|{
        let x = *x;
        let y = *y;

        let mut cardinal_levers_move_detector 
            = CardinalLeversMoveDetector {
            deadzone_upper_limits: DeadzoneUpperLimits {
                left_stick: 0.12,
                right_stick: 0.21,
            },
            left_mouse_control: None,
            right_mouse_control: None,
            current_left_stick_x: None,
            current_left_stick_y: None,
            current_right_stick_x: Some(x),
            current_right_stick_y: Some(y),
            left_stick_at_rest_counter: 0,
            right_stick_at_rest_counter: 0,
        };

        assert_eq!(
            cardinal_levers_move_detector.tick().unwrap(),(x,y));

        assert!(cardinal_levers_move_detector.left_mouse_control.is_none());
        assert!(cardinal_levers_move_detector.right_mouse_control.is_none());
        assert_eq!(cardinal_levers_move_detector.current_right_stick_x.unwrap(),x);
        assert_eq!(cardinal_levers_move_detector.current_right_stick_y.unwrap(),y);
        assert!(cardinal_levers_move_detector.current_left_stick_x.is_none());
        assert!(cardinal_levers_move_detector.current_left_stick_y.is_none());
        assert_eq!(cardinal_levers_move_detector.left_stick_at_rest_counter, 0);
        assert_eq!(cardinal_levers_move_detector.right_stick_at_rest_counter, 0);

    });
}

#[test]
fn tick_right_stick_resting_position_wont_emit_forever() {
    let mut cardinal_levers_move_detector 
        = CardinalLeversMoveDetector {
        deadzone_upper_limits: DeadzoneUpperLimits {
            left_stick: 0.12,
            right_stick: 0.21,
        },
        left_mouse_control: None,
        right_mouse_control: None,
        current_left_stick_x: None,
        current_left_stick_y: None,
        current_right_stick_x: Some(0),
        current_right_stick_y: Some(0),
        left_stick_at_rest_counter: 0,
        right_stick_at_rest_counter: 0,
    };

    for i in (1..10).into_iter() {
        if i < 5 {
            assert_eq!(
                cardinal_levers_move_detector.tick().unwrap(),(0,0));
        }
        else {
            assert!(cardinal_levers_move_detector.tick().is_none());
        }

        assert!(cardinal_levers_move_detector.left_mouse_control.is_none());
        assert!(cardinal_levers_move_detector.right_mouse_control.is_none());
        assert!(cardinal_levers_move_detector.current_left_stick_x.is_none());
        assert!(cardinal_levers_move_detector.current_left_stick_y.is_none());
        assert_eq!(cardinal_levers_move_detector.current_right_stick_x.unwrap(),0);
        assert_eq!(cardinal_levers_move_detector.current_right_stick_y.unwrap(),0);

        if i < 5 {
            assert_eq!(cardinal_levers_move_detector.right_stick_at_rest_counter, i);
        }
        else {
            assert_eq!(cardinal_levers_move_detector.right_stick_at_rest_counter, 5);
        }

        assert_eq!(cardinal_levers_move_detector.left_stick_at_rest_counter, 0);
    }
}

#[test]
fn tick_left_stick_takes_precidence_over_right() {
    [
        (0,1),
        (1,0),
        (2,2),
        (12,30),
    ].iter().for_each(|(x,y)|{
        let x = *x;
        let y = *y;

        let mut cardinal_levers_move_detector 
            = CardinalLeversMoveDetector {
            deadzone_upper_limits: DeadzoneUpperLimits {
                left_stick: 0.12,
                right_stick: 0.21,
            },
            left_mouse_control: None,
            right_mouse_control: None,
            current_left_stick_x: Some(x),
            current_left_stick_y: Some(y),
            current_right_stick_x: Some(12), //random value
            current_right_stick_y: Some(1), //random value
            left_stick_at_rest_counter: 0,
            right_stick_at_rest_counter: 0,
        };

        assert_eq!(
            cardinal_levers_move_detector.tick().unwrap(),(x,y));

        assert!(cardinal_levers_move_detector.left_mouse_control.is_none());
        assert!(cardinal_levers_move_detector.right_mouse_control.is_none());
        assert_eq!(cardinal_levers_move_detector.current_left_stick_x.unwrap(),x);
        assert_eq!(cardinal_levers_move_detector.current_left_stick_y.unwrap(),y);
        assert_eq!(cardinal_levers_move_detector.current_right_stick_x.unwrap(),12);
        assert_eq!(cardinal_levers_move_detector.current_right_stick_y.unwrap(),1);
        assert_eq!(cardinal_levers_move_detector.left_stick_at_rest_counter, 0);
        assert_eq!(cardinal_levers_move_detector.right_stick_at_rest_counter, 0);

    });
}
