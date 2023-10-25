use mockall::predicate::eq;
use crate::input_controller::{mouse_input_controller::{MouseCardinalLeverInputControllerTrait, cursor::Cursor}, enigo_wrapper::MockEnigoTrait};

#[test]
fn mouse_cursor_x_and_y(){
    let mock_enigo = MockEnigoTrait::new();
   
    let mut scroll = Cursor {
        enigo: Box::new(mock_enigo),
        mouse_cursor_x_move: 0,
        mouse_cursor_y_move: 0,
        boost_multiplier: None,
    };
   
    scroll.trigger_input();


    // move x-axis with no boost
    let mut mock_enigo = MockEnigoTrait::new();
    mock_enigo
        .expect_mouse_move_relative()
        .with(eq(123),eq(0))
        .return_const(());
   
    let mut scroll = Cursor {
        enigo: Box::new(mock_enigo),
        mouse_cursor_x_move: 123,
        mouse_cursor_y_move: 0,
        boost_multiplier: None,
    };
   
    scroll.trigger_input();


    // move x-axis with boost
    let mut mock_enigo = MockEnigoTrait::new();
    mock_enigo
        .expect_mouse_move_relative()
        .with(eq(321*4),eq(0))
        .return_const(());
   
    let mut scroll = Cursor {
        enigo: Box::new(mock_enigo),
        mouse_cursor_x_move: 321,
        mouse_cursor_y_move: 0,
        boost_multiplier: Some(4),
    };
   
    scroll.trigger_input();


    // move both axis with no boost
    let mut mock_enigo = MockEnigoTrait::new();
    mock_enigo
        .expect_mouse_move_relative()
        .with(eq(1),eq(94818))
        .return_const(());
   
    let mut scroll = Cursor {
        enigo: Box::new(mock_enigo),
        mouse_cursor_x_move: 1,
        mouse_cursor_y_move: 94818,
        boost_multiplier: None,
    };
   
    scroll.trigger_input();


    // move both axis with boost
    let mut mock_enigo = MockEnigoTrait::new();
    mock_enigo
        .expect_mouse_move_relative()
        .with(eq(170),eq(94810))
        .return_const(());
   
    let mut scroll = Cursor {
        enigo: Box::new(mock_enigo),
        mouse_cursor_x_move: 17,
        mouse_cursor_y_move: 9481,
        boost_multiplier: Some(10),
    };
   
    scroll.trigger_input();


}

#[test]
fn set_x_and_y_axes(){
    let mock_enigo = MockEnigoTrait::new();
    
    let mut scroll = Cursor {
        enigo: Box::new(mock_enigo),
        mouse_cursor_x_move: 123,
        mouse_cursor_y_move: 0,
        boost_multiplier: None,
    };
    
    scroll.set_x_and_y_axes(1,44);

    assert_eq!(scroll.mouse_cursor_x_move,1);
    assert_eq!(scroll.mouse_cursor_y_move,44);


    let mock_enigo = MockEnigoTrait::new();
    
    let mut scroll = Cursor {
        enigo: Box::new(mock_enigo),
        mouse_cursor_x_move: 1,
        mouse_cursor_y_move: 1,
        boost_multiplier: None,
    };
    
    scroll.set_x_and_y_axes(91,4);

    assert_eq!(scroll.mouse_cursor_x_move,91);
    assert_eq!(scroll.mouse_cursor_y_move,4);
}


#[test]
fn start_boost(){
    let mock_enigo = MockEnigoTrait::new();
    let mut scroll = Cursor {
        enigo: Box::new(mock_enigo),
        mouse_cursor_x_move: 123,
        mouse_cursor_y_move: 0,
        boost_multiplier: None,
    };
    
    scroll.start_boost(5);

    assert_eq!(scroll.boost_multiplier.unwrap(), 5);
    assert_eq!(scroll.mouse_cursor_x_move, 123);
    assert_eq!(scroll.mouse_cursor_y_move, 0);
    
    scroll.start_boost(12);

    assert_eq!(scroll.boost_multiplier.unwrap(), 12);
    assert_eq!(scroll.mouse_cursor_x_move, 123);
    assert_eq!(scroll.mouse_cursor_y_move, 0);
}

#[test]
fn end_boost(){
    let mock_enigo = MockEnigoTrait::new();
    let mut scroll = Cursor {
        enigo: Box::new(mock_enigo),
        mouse_cursor_x_move: 39,
        mouse_cursor_y_move: 21,
        boost_multiplier: Some(11),
    };
    
    scroll.end_boost();

    assert!(scroll.boost_multiplier.is_none());
    assert_eq!(scroll.mouse_cursor_x_move, 39);
    assert_eq!(scroll.mouse_cursor_y_move, 21);

    let mock_enigo = MockEnigoTrait::new();
    let mut scroll = Cursor {
        enigo: Box::new(mock_enigo),
        mouse_cursor_x_move: 39,
        mouse_cursor_y_move: 21,
        boost_multiplier: Some(110),
    };
    
    scroll.end_boost();

    assert!(scroll.boost_multiplier.is_none());
    assert_eq!(scroll.mouse_cursor_x_move, 39);
    assert_eq!(scroll.mouse_cursor_y_move, 21);

}
