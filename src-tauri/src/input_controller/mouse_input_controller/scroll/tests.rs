use mockall::predicate::eq;
use crate::input_controller::{mouse_input_controller::{scroll::Scroll, MouseInputControllerTrait}, enigo_wrapper::MockEnigoTrait};

#[test]
fn mouse_scroll_x_and_y(){
    let mut mock_enigo = MockEnigoTrait::new();
    mock_enigo
        .expect_mouse_scroll_x()
        .with(eq(123))
        .return_const(());
    
    let mut scroll = Scroll {
        enigo: Box::new(mock_enigo),
        mouse_scroll_x_move: 123,
        mouse_scroll_y_move: 0,
    };
    
    scroll.trigger_input();


    let mut mock_enigo = MockEnigoTrait::new();
    mock_enigo
        .expect_mouse_scroll_x()
        .with(eq(1203))
        .return_const(());
    mock_enigo
        .expect_mouse_scroll_y()
        .with(eq(2))
        .return_const(());
    
    let mut scroll = Scroll {
        enigo: Box::new(mock_enigo),
        mouse_scroll_x_move: 1203,
        mouse_scroll_y_move: 2,
    };
    
    scroll.trigger_input();


    let mut mock_enigo = MockEnigoTrait::new();
    mock_enigo
        .expect_mouse_scroll_y()
        .with(eq(20))
        .return_const(());
    
    let mut scroll = Scroll {
        enigo: Box::new(mock_enigo),
        mouse_scroll_x_move: 0,
        mouse_scroll_y_move: 20,
    };
    
    scroll.trigger_input();
}

#[test]
fn set_x_and_y_axes(){
    let mock_enigo = MockEnigoTrait::new();
    
    let mut scroll = Scroll {
        enigo: Box::new(mock_enigo),
        mouse_scroll_x_move: 123,
        mouse_scroll_y_move: 0,
    };
    
    scroll.set_x_and_y_axes(1,44);

    assert_eq!(scroll.mouse_scroll_x_move,1);
    assert_eq!(scroll.mouse_scroll_y_move,44);


    let mock_enigo = MockEnigoTrait::new();
    
    let mut scroll = Scroll {
        enigo: Box::new(mock_enigo),
        mouse_scroll_x_move: 1,
        mouse_scroll_y_move: 1,
    };
    
    scroll.set_x_and_y_axes(91,4);

    assert_eq!(scroll.mouse_scroll_x_move,91);
    assert_eq!(scroll.mouse_scroll_y_move,4);
}
