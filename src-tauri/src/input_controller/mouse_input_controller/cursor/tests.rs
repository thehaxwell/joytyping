use mockall::predicate::eq;
use crate::input_controller::{mouse_input_controller::{MouseCardinalLeverInputControllerTrait, cursor::Cursor}, enigo_wrapper::MockEnigoTrait};

#[test]
fn mouse_cursor_x_and_y(){
    let mock_enigo = MockEnigoTrait::new();
   
    let mut scroll = Cursor {
        enigo: Box::new(mock_enigo),
        mouse_cursor_x_move: 0,
        mouse_cursor_y_move: 0,
    };
   
    scroll.trigger_input();


    let mut mock_enigo = MockEnigoTrait::new();
    mock_enigo
        .expect_mouse_move_relative()
        .with(eq(123),eq(0))
        .return_const(());
   
    let mut scroll = Cursor {
        enigo: Box::new(mock_enigo),
        mouse_cursor_x_move: 123,
        mouse_cursor_y_move: 0,
    };
   
    scroll.trigger_input();


    let mut mock_enigo = MockEnigoTrait::new();
    mock_enigo
        .expect_mouse_move_relative()
        .with(eq(1),eq(94818))
        .return_const(());
   
    let mut scroll = Cursor {
        enigo: Box::new(mock_enigo),
        mouse_cursor_x_move: 1,
        mouse_cursor_y_move: 94818,
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
    };
    
    scroll.set_x_and_y_axes(1,44);

    assert_eq!(scroll.mouse_cursor_x_move,1);
    assert_eq!(scroll.mouse_cursor_y_move,44);


    let mock_enigo = MockEnigoTrait::new();
    
    let mut scroll = Cursor {
        enigo: Box::new(mock_enigo),
        mouse_cursor_x_move: 1,
        mouse_cursor_y_move: 1,
    };
    
    scroll.set_x_and_y_axes(91,4);

    assert_eq!(scroll.mouse_cursor_x_move,91);
    assert_eq!(scroll.mouse_cursor_y_move,4);
}
