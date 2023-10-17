use mockall::predicate::eq;

use crate::input_controller::{mouse_input_controller::button::Button, enigo_wrapper::MockEnigoTrait};


#[test]
fn key_down_works(){
    [
        enigo::MouseButton::Left,
        enigo::MouseButton::Right,
        enigo::MouseButton::ScrollUp,
        enigo::MouseButton::ScrollDown,
        enigo::MouseButton::Forward,
    ]
    .iter()
    .for_each(|key|{
        let mut mock_enigo = MockEnigoTrait::new();

        mock_enigo
            .expect_mouse_down()
            .times(1)
            .with(eq(key))
            .return_const(());

        let mut button = Button {
            enigo: Box::new(mock_enigo),
            active_button: None,
        };

        button.key_down(*key);
        assert_eq!(button.active_button.unwrap(),*key);
    });
}

#[test]
fn key_up_works(){
    [
        enigo::MouseButton::Left,
        enigo::MouseButton::Right,
        enigo::MouseButton::ScrollUp,
        enigo::MouseButton::ScrollDown,
        enigo::MouseButton::Forward,
    ]
    .iter()
    .for_each(|key|{
        let mut mock_enigo = MockEnigoTrait::new();

        mock_enigo
            .expect_mouse_up()
            .times(1)
            .with(eq(key))
            .return_const(());

        let mut button = Button {
            enigo: Box::new(mock_enigo),
            active_button: Some(*key),
        };

        button.key_up();
        assert!(button.active_button.is_none());
    });

    let mock_enigo = MockEnigoTrait::new();

    let mut button = Button {
        enigo: Box::new(mock_enigo),
        active_button: None,
    };

    button.key_up();
    assert!(button.active_button.is_none());
}
