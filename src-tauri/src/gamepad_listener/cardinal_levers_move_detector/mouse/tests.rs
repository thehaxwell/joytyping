use mockall::predicate::eq;

use crate::{gamepad_listener::{cardinal_levers_move_detector::MockCardinalLeversMoveDetectorTrait, InputEvent}, settings::models::layout::{CardinalLevers, SingleCardinalLever}};

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
        .expect_activate_levers()
        .with(eq(false),eq(false))
        .times(1)
        .return_const(());
    mock_mouse_scroll_detector
        .expect_activate_levers()
        .with(eq(false),eq(false))
        .times(1)
        .return_const(());

    let mut mouse = Mouse {
       mouse_cursor_move_detector: Box::new(mock_mouse_cursor_move_detector),
       mouse_scroll_detector: Box::new(mock_mouse_scroll_detector),
    };

    mouse.set_mouse_controls(None);

    // only the left control is set:
    // that control is for for mouse_cursor_move_detector
    let mut mock_mouse_cursor_move_detector 
        = MockCardinalLeversMoveDetectorTrait::new();
    let mut mock_mouse_scroll_detector
        = MockCardinalLeversMoveDetectorTrait::new();

    mock_mouse_cursor_move_detector
        .expect_activate_levers()
        .with(eq(true),eq(false))
        .times(1)
        .return_const(());
    mock_mouse_scroll_detector
        .expect_activate_levers()
        .with(eq(false),eq(false))
        .times(1)
        .return_const(());

    let mut mouse = Mouse {
       mouse_cursor_move_detector: Box::new(mock_mouse_cursor_move_detector),
       mouse_scroll_detector: Box::new(mock_mouse_scroll_detector),
    };

    mouse.set_mouse_controls(Some(CardinalLevers{
        left_stick: Some(SingleCardinalLever
                ::ControlMouseCursor),
        right_stick: None,
    }));

    // only the left control is set:
    // that control is for for mock_mouse_scroll_detector
    let mut mock_mouse_cursor_move_detector 
        = MockCardinalLeversMoveDetectorTrait::new();
    let mut mock_mouse_scroll_detector
        = MockCardinalLeversMoveDetectorTrait::new();

    mock_mouse_cursor_move_detector
        .expect_activate_levers()
        .with(eq(false),eq(false))
        .times(1)
        .return_const(());
    mock_mouse_scroll_detector
        .expect_activate_levers()
        .with(eq(true),eq(false))
        .times(1)
        .return_const(());

    let mut mouse = Mouse {
       mouse_cursor_move_detector: Box::new(mock_mouse_cursor_move_detector),
       mouse_scroll_detector: Box::new(mock_mouse_scroll_detector),
    };

    mouse.set_mouse_controls(Some(CardinalLevers{
        left_stick: Some(SingleCardinalLever
                ::ControlMouseScrollwheel),
        right_stick: None,
    }));

    // both controls are set:
    // both are for mouse_cursor_move_detector
    let mut mock_mouse_cursor_move_detector 
        = MockCardinalLeversMoveDetectorTrait::new();
    let mut mock_mouse_scroll_detector
        = MockCardinalLeversMoveDetectorTrait::new();

    mock_mouse_cursor_move_detector
        .expect_activate_levers()
        .with(eq(true),eq(true))
        .times(1)
        .return_const(());
    mock_mouse_scroll_detector
        .expect_activate_levers()
        .with(eq(false),eq(false))
        .times(1)
        .return_const(());

    let mut mouse = Mouse {
       mouse_cursor_move_detector: Box::new(mock_mouse_cursor_move_detector),
       mouse_scroll_detector: Box::new(mock_mouse_scroll_detector),
    };

    mouse.set_mouse_controls(Some(CardinalLevers{
        left_stick: Some(SingleCardinalLever
                ::ControlMouseCursor),
        right_stick: Some(SingleCardinalLever
                ::ControlMouseCursor),
    }));


    // both controls are set:
    // both are for mouse_cursor_move_detector
    let mut mock_mouse_cursor_move_detector 
        = MockCardinalLeversMoveDetectorTrait::new();
    let mut mock_mouse_scroll_detector
        = MockCardinalLeversMoveDetectorTrait::new();

    mock_mouse_cursor_move_detector
        .expect_activate_levers()
        .with(eq(false),eq(false))
        .times(1)
        .return_const(());
    mock_mouse_scroll_detector
        .expect_activate_levers()
        .with(eq(true),eq(true))
        .times(1)
        .return_const(());

    let mut mouse = Mouse {
       mouse_cursor_move_detector: Box::new(mock_mouse_cursor_move_detector),
       mouse_scroll_detector: Box::new(mock_mouse_scroll_detector),
    };

    mouse.set_mouse_controls(Some(CardinalLevers{
        left_stick: Some(SingleCardinalLever
                ::ControlMouseScrollwheel),
        right_stick: Some(SingleCardinalLever
                ::ControlMouseScrollwheel),
    }));

    // both controls are set:
    // left for mouse_cursor_move_detector, and
    // right for mock_mouse_scroll_detector

    let mut mock_mouse_cursor_move_detector 
        = MockCardinalLeversMoveDetectorTrait::new();
    let mut mock_mouse_scroll_detector
        = MockCardinalLeversMoveDetectorTrait::new();

    mock_mouse_cursor_move_detector
        .expect_activate_levers()
        .with(eq(true),eq(false))
        .times(1)
        .return_const(());
    mock_mouse_scroll_detector
        .expect_activate_levers()
        .with(eq(false),eq(true))
        .times(1)
        .return_const(());

    let mut mouse = Mouse {
       mouse_cursor_move_detector: Box::new(mock_mouse_cursor_move_detector),
       mouse_scroll_detector: Box::new(mock_mouse_scroll_detector),
    };

    mouse.set_mouse_controls(Some(CardinalLevers{
        left_stick: Some(SingleCardinalLever
                ::ControlMouseCursor),
        right_stick: Some(SingleCardinalLever
                ::ControlMouseScrollwheel),
    }));

    // both controls are set:
    // left for mock_mouse_scroll_detector, and
    // right for mouse_cursor_move_detector

    let mut mock_mouse_cursor_move_detector 
        = MockCardinalLeversMoveDetectorTrait::new();
    let mut mock_mouse_scroll_detector
        = MockCardinalLeversMoveDetectorTrait::new();

    mock_mouse_cursor_move_detector
        .expect_activate_levers()
        .with(eq(false),eq(true))
        .times(1)
        .return_const(());
    mock_mouse_scroll_detector
        .expect_activate_levers()
        .with(eq(true),eq(false))
        .times(1)
        .return_const(());

    let mut mouse = Mouse {
       mouse_cursor_move_detector: Box::new(mock_mouse_cursor_move_detector),
       mouse_scroll_detector: Box::new(mock_mouse_scroll_detector),
    };

    mouse.set_mouse_controls(Some(CardinalLevers{
        left_stick: Some(SingleCardinalLever
                ::ControlMouseScrollwheel),
        right_stick: Some(SingleCardinalLever
                ::ControlMouseCursor),
    }));

}

