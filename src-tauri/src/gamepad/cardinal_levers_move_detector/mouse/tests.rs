use mockall::predicate::eq;

use crate::{gamepad::{cardinal_levers_move_detector::MockCardinalLeversMoveDetectorTrait, InputEvent}, models::layout::{CardinalLevers, MouseControl, SingleCardinalLever}};

use {super::Mouse, super::MouseTrait};


#[test]
fn fn_tick_works() {
    let mut mock_mouse_cursor_move_detector 
        = MockCardinalLeversMoveDetectorTrait::new();
    let mut mock_mouse_scroll_detector
        = MockCardinalLeversMoveDetectorTrait::new();

    mock_mouse_cursor_move_detector
        .expect_tick()
        .times(1)
        .return_const(None);
    mock_mouse_scroll_detector
        .expect_tick()
        .times(1)
        .return_const(None);

    let mut mouse = Mouse {
       mouse_cursor_move_detector: Box::new(mock_mouse_cursor_move_detector),
       mouse_scroll_detector: Box::new(mock_mouse_scroll_detector),
    };

    assert!(mouse.tick().is_none());


    let mut mock_mouse_cursor_move_detector 
        = MockCardinalLeversMoveDetectorTrait::new();
    let mock_mouse_scroll_detector
        = MockCardinalLeversMoveDetectorTrait::new();

    mock_mouse_cursor_move_detector
        .expect_tick()
        .times(1)
        .return_const(Some((21,38)));

    let mut mouse = Mouse {
       mouse_cursor_move_detector: Box::new(mock_mouse_cursor_move_detector),
       mouse_scroll_detector: Box::new(mock_mouse_scroll_detector),
    };

    assert_eq!(mouse.tick().unwrap(),InputEvent::MoveMouseCursor(21,38));


    let mut mock_mouse_cursor_move_detector 
        = MockCardinalLeversMoveDetectorTrait::new();
    let mut mock_mouse_scroll_detector
        = MockCardinalLeversMoveDetectorTrait::new();

    mock_mouse_cursor_move_detector
        .expect_tick()
        .times(1)
        .return_const(None);
    mock_mouse_scroll_detector
        .expect_tick()
        .times(1)
        .return_const(Some((210,380)));

    let mut mouse = Mouse {
       mouse_cursor_move_detector: Box::new(mock_mouse_cursor_move_detector),
       mouse_scroll_detector: Box::new(mock_mouse_scroll_detector),
    };

    assert_eq!(mouse.tick().unwrap(),InputEvent::MouseScroll(210,380));
}

#[test]
fn fn_axis_changed_works() {
    let mut mock_mouse_cursor_move_detector 
        = MockCardinalLeversMoveDetectorTrait::new();
    let mut mock_mouse_scroll_detector
        = MockCardinalLeversMoveDetectorTrait::new();

    let axis = gilrs::ev::Axis::LeftStickX;
    let value = 0.23;
    mock_mouse_cursor_move_detector
        .expect_axis_changed()
        .with(eq(axis),eq(value))
        .times(1)
        .return_const(());
    mock_mouse_scroll_detector
        .expect_axis_changed()
        .with(eq(axis),eq(value))
        .times(1)
        .return_const(());

    let mut mouse = Mouse {
       mouse_cursor_move_detector: Box::new(mock_mouse_cursor_move_detector),
       mouse_scroll_detector: Box::new(mock_mouse_scroll_detector),
    };

    mouse.axis_changed(axis,value);
}

#[test]
fn fn_set_mouse_controls_works() {
    let mut mock_mouse_cursor_move_detector 
        = MockCardinalLeversMoveDetectorTrait::new();
    let mut mock_mouse_scroll_detector
        = MockCardinalLeversMoveDetectorTrait::new();

    mock_mouse_cursor_move_detector
        .expect_set_mouse_controls()
        .with(eq(None),eq(None))
        .times(1)
        .return_const(());
    mock_mouse_scroll_detector
        .expect_set_mouse_controls()
        .with(eq(None),eq(None))
        .times(1)
        .return_const(());

    let mut mouse = Mouse {
       mouse_cursor_move_detector: Box::new(mock_mouse_cursor_move_detector),
       mouse_scroll_detector: Box::new(mock_mouse_scroll_detector),
    };

    mouse.set_mouse_controls(None);

    // setup for the rest of the assertions in this test
    let left_mouse_mouse_control = MouseControl {
        deadzone_upper_limit: 0.2,
        scale_factor: 1.2,
    };
    let right_mouse_mouse_control = MouseControl {
        deadzone_upper_limit: 0.999,
        scale_factor: 20.0,
    };

    // only the left control is set:
    // that control is for for mouse_cursor_move_detector
    let mut mock_mouse_cursor_move_detector 
        = MockCardinalLeversMoveDetectorTrait::new();
    let mut mock_mouse_scroll_detector
        = MockCardinalLeversMoveDetectorTrait::new();

    mock_mouse_cursor_move_detector
        .expect_set_mouse_controls()
        .with(
            eq(Some(left_mouse_mouse_control.clone())),
            eq(None),
        )
        .times(1)
        .return_const(());
    mock_mouse_scroll_detector
        .expect_set_mouse_controls()
        .with(eq(None),eq(None))
        .times(1)
        .return_const(());

    let mut mouse = Mouse {
       mouse_cursor_move_detector: Box::new(mock_mouse_cursor_move_detector),
       mouse_scroll_detector: Box::new(mock_mouse_scroll_detector),
    };

    mouse.set_mouse_controls(Some(CardinalLevers{
        left_stick: Some(SingleCardinalLever
                ::ControlMouseCursor(left_mouse_mouse_control.clone())),
        right_stick: None,
    }));

    // only the left control is set:
    // that control is for for mock_mouse_scroll_detector
    let mut mock_mouse_cursor_move_detector 
        = MockCardinalLeversMoveDetectorTrait::new();
    let mut mock_mouse_scroll_detector
        = MockCardinalLeversMoveDetectorTrait::new();

    mock_mouse_cursor_move_detector
        .expect_set_mouse_controls()
        .with(
            eq(None),
            eq(None),
        )
        .times(1)
        .return_const(());
    mock_mouse_scroll_detector
        .expect_set_mouse_controls()
        .with(
            eq(Some(left_mouse_mouse_control.clone())),
            eq(None))
        .times(1)
        .return_const(());

    let mut mouse = Mouse {
       mouse_cursor_move_detector: Box::new(mock_mouse_cursor_move_detector),
       mouse_scroll_detector: Box::new(mock_mouse_scroll_detector),
    };

    mouse.set_mouse_controls(Some(CardinalLevers{
        left_stick: Some(SingleCardinalLever
                ::ControlMouseScrollwheel(left_mouse_mouse_control.clone())),
        right_stick: None,
    }));

    // both controls are set:
    // both are for mouse_cursor_move_detector
    let mut mock_mouse_cursor_move_detector 
        = MockCardinalLeversMoveDetectorTrait::new();
    let mut mock_mouse_scroll_detector
        = MockCardinalLeversMoveDetectorTrait::new();

    mock_mouse_cursor_move_detector
        .expect_set_mouse_controls()
        .with(
            eq(Some(left_mouse_mouse_control.clone())),
            eq(Some(right_mouse_mouse_control.clone())),
        )
        .times(1)
        .return_const(());
    mock_mouse_scroll_detector
        .expect_set_mouse_controls()
        .with(eq(None),eq(None))
        .times(1)
        .return_const(());

    let mut mouse = Mouse {
       mouse_cursor_move_detector: Box::new(mock_mouse_cursor_move_detector),
       mouse_scroll_detector: Box::new(mock_mouse_scroll_detector),
    };

    mouse.set_mouse_controls(Some(CardinalLevers{
        left_stick: Some(SingleCardinalLever
                ::ControlMouseCursor(left_mouse_mouse_control.clone())),
        right_stick: Some(SingleCardinalLever
                ::ControlMouseCursor(right_mouse_mouse_control.clone())),
    }));


    // both controls are set:
    // both are for mouse_cursor_move_detector
    let mut mock_mouse_cursor_move_detector 
        = MockCardinalLeversMoveDetectorTrait::new();
    let mut mock_mouse_scroll_detector
        = MockCardinalLeversMoveDetectorTrait::new();

    mock_mouse_cursor_move_detector
        .expect_set_mouse_controls()
        .with(eq(None),eq(None))
        .times(1)
        .return_const(());
    mock_mouse_scroll_detector
        .expect_set_mouse_controls()
        .with(
            eq(Some(left_mouse_mouse_control.clone())),
            eq(Some(right_mouse_mouse_control.clone())),
        )
        .times(1)
        .return_const(());

    let mut mouse = Mouse {
       mouse_cursor_move_detector: Box::new(mock_mouse_cursor_move_detector),
       mouse_scroll_detector: Box::new(mock_mouse_scroll_detector),
    };

    mouse.set_mouse_controls(Some(CardinalLevers{
        left_stick: Some(SingleCardinalLever
                ::ControlMouseScrollwheel(left_mouse_mouse_control.clone())),
        right_stick: Some(SingleCardinalLever
                ::ControlMouseScrollwheel(right_mouse_mouse_control.clone())),
    }));

    // both controls are set:
    // left for mouse_cursor_move_detector, and
    // right for mock_mouse_scroll_detector

    let mut mock_mouse_cursor_move_detector 
        = MockCardinalLeversMoveDetectorTrait::new();
    let mut mock_mouse_scroll_detector
        = MockCardinalLeversMoveDetectorTrait::new();

    mock_mouse_cursor_move_detector
        .expect_set_mouse_controls()
        .with(
            eq(Some(left_mouse_mouse_control.clone())),
            eq(None),
        )
        .times(1)
        .return_const(());
    mock_mouse_scroll_detector
        .expect_set_mouse_controls()
        .with(
            eq(None),
            eq(Some(right_mouse_mouse_control.clone()))
        )
        .times(1)
        .return_const(());

    let mut mouse = Mouse {
       mouse_cursor_move_detector: Box::new(mock_mouse_cursor_move_detector),
       mouse_scroll_detector: Box::new(mock_mouse_scroll_detector),
    };

    mouse.set_mouse_controls(Some(CardinalLevers{
        left_stick: Some(SingleCardinalLever
                ::ControlMouseCursor(left_mouse_mouse_control.clone())),
        right_stick: Some(SingleCardinalLever
                ::ControlMouseScrollwheel(right_mouse_mouse_control.clone())),
    }));

    // both controls are set:
    // left for mock_mouse_scroll_detector, and
    // right for mouse_cursor_move_detector

    let mut mock_mouse_cursor_move_detector 
        = MockCardinalLeversMoveDetectorTrait::new();
    let mut mock_mouse_scroll_detector
        = MockCardinalLeversMoveDetectorTrait::new();

    mock_mouse_cursor_move_detector
        .expect_set_mouse_controls()
        .with(
            eq(None),
            eq(Some(right_mouse_mouse_control.clone()))
        )
        .times(1)
        .return_const(());
    mock_mouse_scroll_detector
        .expect_set_mouse_controls()
        .with(
            eq(Some(left_mouse_mouse_control.clone())),
            eq(None),
        )
        .times(1)
        .return_const(());

    let mut mouse = Mouse {
       mouse_cursor_move_detector: Box::new(mock_mouse_cursor_move_detector),
       mouse_scroll_detector: Box::new(mock_mouse_scroll_detector),
    };

    mouse.set_mouse_controls(Some(CardinalLevers{
        left_stick: Some(SingleCardinalLever
                ::ControlMouseScrollwheel(left_mouse_mouse_control.clone())),
        right_stick: Some(SingleCardinalLever
                ::ControlMouseCursor(right_mouse_mouse_control.clone())),
    }));

}

