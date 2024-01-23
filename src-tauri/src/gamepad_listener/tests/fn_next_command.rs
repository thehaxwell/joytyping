use enigo::{Key, MouseButton};
use gilrs::Button;
use mockall::predicate::*;

use crate::{gamepad_listener::{switch_click_pattern_detector::{MockSwitchClickPatternDetectorTrait, SwitchClickPattern}, gilrs_events::{MockGilrsEventsTrait, stick_switch_interpreter::StickSwitchButton}, layers_navigator, cardinal_levers_move_detector::{self, mouse::MouseEvent}, Switch, GamepadListener}, settings::models::layout::{SwitchOnClickReaction, KeyboardInput, MouseInput, CardinalLevers, SingleCardinalLever}, input_controller::MockInputControllerTrait, quick_lookup_window};

fn setup_and_test_switch_input(
    reaction: SwitchOnClickReaction,
    pattern: SwitchClickPattern,
    mut mock_input_controller: MockInputControllerTrait,
    ) {
        let mut mock_switch_click_pattern_detector = MockSwitchClickPatternDetectorTrait::new();
        let mock_gilrs_events = MockGilrsEventsTrait::new();
        let mut mock_layers_navigator = layers_navigator::controller::MockControllerTrait::new();
        let mut mock_mouse_cardinal_levers_move_detector 
            = cardinal_levers_move_detector::mouse::MockMouseTrait::new();

        let mock_quick_lookup_window_controller = quick_lookup_window::controller::MockControllerTrait::new();

        mock_switch_click_pattern_detector
            .expect_tick()
            .times(1)
            .return_const(Some(pattern.clone()));

        mock_layers_navigator
            .expect_process_switch_event()
            .times(1)
            .with(eq(Some(pattern.clone())))
            .return_const(
                Some(reaction));

        mock_layers_navigator
            .expect_consumable_get_current_layer_index()
            .times(1)
            .return_const(None);

        mock_mouse_cardinal_levers_move_detector
            .expect_tick()
            .times(1)
            .return_const(None);

        mock_input_controller
            .expect_trigger_input()
            .times(1)
            .return_const(());

        let mut gamepad_listener = GamepadListener {
           gilrs_events: Box::new(mock_gilrs_events),
           layers_navigator: Box::new(mock_layers_navigator),
           switch_click_pattern_detector: Box::new(mock_switch_click_pattern_detector),
           mouse_cardinal_levers_move_detector: 
               Box::new(mock_mouse_cardinal_levers_move_detector),
           input: Box::new(mock_input_controller),
           quick_lookup_window: Box::new(mock_quick_lookup_window_controller),
        };

        gamepad_listener.next_command()
}

#[test]
fn handles_keyboard_input_events(){
   let keyboard_input = KeyboardInput{
       key: Key::Layout('I'),
       modifiers: vec![]
   };
   setup_and_test_switch_input(
       SwitchOnClickReaction::Keyboard(keyboard_input.clone()),
       SwitchClickPattern::Click(Switch::Button(Button::South)),
       (||{
           let mut mock_input_controller = MockInputControllerTrait::new();
           mock_input_controller
               .expect_key_click()
               .times(1)
               .with(eq(keyboard_input.clone()))
               .return_const(());
           mock_input_controller
       })(),);
   
   // ----------------
   // different SwitchClickPattern::Click triggering switches
   // ----------------
    let keyboard_input = KeyboardInput{
        key: Key::Layout('*'),
        modifiers: vec![]
    };
   
   setup_and_test_switch_input(
       SwitchOnClickReaction::Keyboard(keyboard_input.clone()),
       SwitchClickPattern::Click(Switch::StickSwitchButton(StickSwitchButton::RightStickDown)),
       (||{
           let mut mock_input_controller = MockInputControllerTrait::new();
           mock_input_controller
               .expect_key_click()
               .times(1)
               .with(eq(keyboard_input.clone()))
               .return_const(());
           mock_input_controller
       })(),);
   
    // -------------
    // DoubleClick triggers on_double_click
    // -------------
    let keyboard_input = KeyboardInput{
        key: Key::Layout('I'),
        modifiers: vec![]
    };
   setup_and_test_switch_input(
       SwitchOnClickReaction::Keyboard(keyboard_input.clone()),
       SwitchClickPattern::DoubleClick(Switch::Button(Button::LeftTrigger2)),
       (||{
           let mut mock_input_controller = MockInputControllerTrait::new();
           mock_input_controller
               .expect_key_click()
               .times(1)
               .with(eq(keyboard_input.clone()))
               .return_const(());
           mock_input_controller
       })(),);
   
    // -------------
    // DoubleClick count as Click if on_double_click wasn't set
    // -------------
    let keyboard_input = KeyboardInput{
        key: Key::Tab,
        modifiers: vec![Key::Shift]
    };
   
   setup_and_test_switch_input(
       SwitchOnClickReaction::Keyboard(keyboard_input.clone()),
       SwitchClickPattern::DoubleClick(Switch::Button(Button::Start)),
       (||{
           let mut mock_input_controller = MockInputControllerTrait::new();
           mock_input_controller
               .expect_key_click()
               .times(1)
               .with(eq(keyboard_input.clone()))
               .return_const(());
           mock_input_controller
       })(),);
   
   // ----------------
   // SwitchClickPattern::ClickAndHold triggers Keydown
   // ----------------
    let keyboard_input = KeyboardInput{
        key: Key::Layout('I'),
        modifiers: vec![]
    };
   
   setup_and_test_switch_input(
       SwitchOnClickReaction::Keyboard(keyboard_input.clone()),
       SwitchClickPattern::ClickAndHold(Switch::Button(Button::LeftTrigger2)),
       (||{
           let mut mock_input_controller = MockInputControllerTrait::new();
           mock_input_controller
               .expect_key_down()
               .times(1)
               .with(eq(keyboard_input.clone()))
               .return_const(());
           mock_input_controller
       })(),);
   
   // ----------------
   // SwitchClickPattern::DoubleClickAndHold triggers Keydown
   // ----------------
    let keyboard_input = KeyboardInput{
        key: Key::Layout('*'),
        modifiers: vec![]
    };
   
   setup_and_test_switch_input(
       SwitchOnClickReaction::Keyboard(keyboard_input.clone()),
       SwitchClickPattern::DoubleClickAndHold(Switch::StickSwitchButton(StickSwitchButton::RightStickDown)),
       (||{
           let mut mock_input_controller = MockInputControllerTrait::new();
           mock_input_controller
               .expect_key_down()
               .times(1)
               .with(eq(keyboard_input.clone()))
               .return_const(());
           mock_input_controller
       })(),);
   
}


#[test]
fn handles_mouse_input_events(){
   let mouse_input = MouseInput {
       button: enigo::MouseButton::Right,
   };
   setup_and_test_switch_input(
       SwitchOnClickReaction::Mouse(mouse_input.clone()),
       SwitchClickPattern::Click(Switch::Button(Button::South)),
       (||{
           let mut mock_input_controller = MockInputControllerTrait::new();
           mock_input_controller
               .expect_mouse_down()
               .times(1)
               .with(eq(mouse_input.button))
               .return_const(());
           mock_input_controller
       })(),);
    
   // ----------------
   // different SwitchClickPattern::Click triggering switches
   // ----------------
   let mouse_input = MouseInput {
       button: enigo::MouseButton::Right,
   };
   setup_and_test_switch_input(
       SwitchOnClickReaction::Mouse(mouse_input.clone()),
       SwitchClickPattern::Click(Switch::StickSwitchButton(StickSwitchButton::RightStickDown)),
       (||{
           let mut mock_input_controller = MockInputControllerTrait::new();
           mock_input_controller
               .expect_mouse_down()
               .times(1)
               .with(eq(mouse_input.button))
               .return_const(());
           mock_input_controller
       })(),);
    
    // -------------
    // DoubleClick triggers on_double_click
    // -------------
   let mouse_input = MouseInput {
       button: enigo::MouseButton::Left,
   };
   setup_and_test_switch_input(
       SwitchOnClickReaction::Mouse(mouse_input.clone()),
       SwitchClickPattern::DoubleClick(Switch::StickSwitchButton(StickSwitchButton::RightStickDown)),
       (||{
           let mut mock_input_controller = MockInputControllerTrait::new();
           mock_input_controller
               .expect_mouse_down()
               .times(1)
               .with(eq(mouse_input.button))
               .return_const(());
           mock_input_controller
       })(),);
    
   // -------------
   // DoubleClick count as Click if on_double_click wasn't set
   // -------------
   let mouse_input = MouseInput {
       button: enigo::MouseButton::Left,
   };
   setup_and_test_switch_input(
       SwitchOnClickReaction::Mouse(mouse_input.clone()),
       SwitchClickPattern::DoubleClick(Switch::Button(Button::Start)),
       (||{
           let mut mock_input_controller = MockInputControllerTrait::new();
           mock_input_controller
               .expect_mouse_down()
               .times(1)
               .with(eq(mouse_input.button))
               .return_const(());
           mock_input_controller
       })(),);
    
   // ----------------
   // SwitchClickPattern::ClickAndHold doesn't trigger mouse
   // ----------------
   let mouse_input = MouseInput {
       button: enigo::MouseButton::ScrollUp,
   };
   setup_and_test_switch_input(
       SwitchOnClickReaction::Mouse(mouse_input),
       SwitchClickPattern::ClickAndHold(Switch::Button(Button::LeftTrigger2)),
       (||{
           let mock_input_controller = MockInputControllerTrait::new();
           mock_input_controller
       })(),);
    
   // ----------------
   // SwitchClickPattern::DoubleClickAndHold doesn't trigger mouse
   // ----------------
   let mouse_input = MouseInput {
       button: enigo::MouseButton::ScrollRight,
   };
   setup_and_test_switch_input(
       SwitchOnClickReaction::Mouse(mouse_input),
       SwitchClickPattern::DoubleClickAndHold(Switch::StickSwitchButton(StickSwitchButton::RightStickDown)),
       (||{
           let mock_input_controller = MockInputControllerTrait::new();
           mock_input_controller
       })(),);
}

#[test]
fn handles_mouse_cursor_boost_events(){
   let mul = 32;
   setup_and_test_switch_input(
       SwitchOnClickReaction::BoostMouseCursorByMultiplier(mul),
       SwitchClickPattern::Click(Switch::Button(Button::South)),
       (||{
           let mut mock_input_controller = MockInputControllerTrait::new();
           mock_input_controller
               .expect_boost_mouse_cursor()
               .times(1)
               .with(eq(mul))
               .return_const(());
           mock_input_controller
       })(),);
   
   // ----------------
   // different SwitchClickPattern::Click triggering switches
   // ----------------
   let mul = 2;
   setup_and_test_switch_input(
       SwitchOnClickReaction::BoostMouseCursorByMultiplier(mul),
       SwitchClickPattern::Click(Switch::StickSwitchButton(StickSwitchButton::RightStickDown)),
       (||{
           let mut mock_input_controller = MockInputControllerTrait::new();
           mock_input_controller
               .expect_boost_mouse_cursor()
               .times(1)
               .with(eq(mul))
               .return_const(());
           mock_input_controller
       })(),);
   
    // -------------
    // DoubleClick triggers on_double_click
    // -------------
   let mul = 11;
   setup_and_test_switch_input(
       SwitchOnClickReaction::BoostMouseCursorByMultiplier(mul),
       SwitchClickPattern::DoubleClick(Switch::StickSwitchButton(StickSwitchButton::RightStickDown)),
       (||{
           let mut mock_input_controller = MockInputControllerTrait::new();
           mock_input_controller
               .expect_boost_mouse_cursor()
               .times(1)
               .with(eq(mul))
               .return_const(());
           mock_input_controller
       })(),);
   
   // -------------
   // DoubleClick count as Click if on_double_click wasn't set
   // -------------
   let mul = 10;
   setup_and_test_switch_input(
       SwitchOnClickReaction::BoostMouseCursorByMultiplier(mul),
       SwitchClickPattern::DoubleClick(Switch::Button(Button::Start)),
       (||{
           let mut mock_input_controller = MockInputControllerTrait::new();
           mock_input_controller
               .expect_boost_mouse_cursor()
               .times(1)
               .with(eq(mul))
               .return_const(());
           mock_input_controller
       })(),);
   
   // ----------------
   // SwitchClickPattern::ClickAndHold doesn't trigger mouse
   // ----------------
   let mul = 7;
   setup_and_test_switch_input(
       SwitchOnClickReaction::BoostMouseCursorByMultiplier(mul),
       SwitchClickPattern::ClickAndHold(Switch::Button(Button::LeftTrigger2)),
       (||{
           let mock_input_controller = MockInputControllerTrait::new();
           mock_input_controller
       })(),);
   
   // ----------------
   // SwitchClickPattern::DoubleClickAndHold doesn't trigger mouse
   // ----------------
   let mul = 3;
   setup_and_test_switch_input(
       SwitchOnClickReaction::BoostMouseCursorByMultiplier(mul),
       SwitchClickPattern::DoubleClickAndHold(Switch::StickSwitchButton(StickSwitchButton::RightStickDown)),
       (||{
           let mock_input_controller = MockInputControllerTrait::new();
           mock_input_controller
       })(),);
}


struct SetupHandlesQuickLookupWindowEventsArgs{
    reaction: SwitchOnClickReaction,
    pattern: SwitchClickPattern,
    mock_quick_lookup_window_controller: quick_lookup_window::controller::MockControllerTrait,
}

fn setup_handles_quick_lookup_window(
    args: SetupHandlesQuickLookupWindowEventsArgs
    ) {
        let mut mock_switch_click_pattern_detector = MockSwitchClickPatternDetectorTrait::new();
        let mock_gilrs_events = MockGilrsEventsTrait::new();
        let mut mock_layers_navigator = layers_navigator::controller::MockControllerTrait::new();
        let mut mock_mouse_cardinal_levers_move_detector 
            = cardinal_levers_move_detector::mouse::MockMouseTrait::new();

        let mut mock_input_controller = MockInputControllerTrait::new();

        mock_switch_click_pattern_detector
            .expect_tick()
            .times(1)
            .return_const(Some(args.pattern.clone()));

        mock_layers_navigator
            .expect_process_switch_event()
            .times(1)
            .with(eq(Some(args.pattern.clone())))
            .return_const(
                Some(args.reaction));

        mock_layers_navigator
            .expect_consumable_get_current_layer_index()
            .times(1)
            .return_const(None);

        mock_mouse_cardinal_levers_move_detector
            .expect_tick()
            .times(1)
            .return_const(None);

        mock_input_controller
            .expect_trigger_input()
            .times(1)
            .return_const(());


        let mut gamepad_listener = GamepadListener {
           gilrs_events: Box::new(mock_gilrs_events),
           switch_click_pattern_detector: Box::new(mock_switch_click_pattern_detector),
           layers_navigator: Box::new(mock_layers_navigator),
           mouse_cardinal_levers_move_detector: 
               Box::new(mock_mouse_cardinal_levers_move_detector),
           input: Box::new(mock_input_controller),
           quick_lookup_window: Box::new(args.mock_quick_lookup_window_controller),
        };

        gamepad_listener.next_command();
}

#[test]
fn handles_show_quick_lookup_window(){
    let switch = Switch::Button(Button::DPadRight);
    setup_handles_quick_lookup_window(
        SetupHandlesQuickLookupWindowEventsArgs {
            reaction: SwitchOnClickReaction::ShowQuickLookupWindowOnHold,
            pattern: SwitchClickPattern::Click(switch.clone()),
            mock_quick_lookup_window_controller: (||{
                let mut mock_quick_lookup_window_controller 
                    = quick_lookup_window::controller::MockControllerTrait::new();
                mock_quick_lookup_window_controller
                    .expect_show_until_switch_keyup()
                    .times(1)
                    .with(eq(switch))
                    .returning(|_| Ok(()));
                mock_quick_lookup_window_controller
            })()
        }
    );

    let switch = Switch::Button(Button::South);
    setup_handles_quick_lookup_window(
        SetupHandlesQuickLookupWindowEventsArgs {
            reaction: SwitchOnClickReaction::ShowQuickLookupWindowOnHold,
            pattern: SwitchClickPattern::DoubleClick(switch.clone()),
            mock_quick_lookup_window_controller: (||{
                let mut mock_quick_lookup_window_controller 
                    = quick_lookup_window::controller::MockControllerTrait::new();
                mock_quick_lookup_window_controller
                    .expect_show_until_switch_keyup()
                    .times(1)
                    .with(eq(switch))
                    .returning(|_| Ok(()));
                mock_quick_lookup_window_controller
            })()
        }
    );
}

#[test]
fn handles_toggle_quick_lookup_window(){
    let switch = Switch::Button(Button::DPadRight);
    setup_handles_quick_lookup_window(
        SetupHandlesQuickLookupWindowEventsArgs {
            reaction: SwitchOnClickReaction::ToggleQuickLookupWindow,
            pattern: SwitchClickPattern::Click(switch.clone()),
            mock_quick_lookup_window_controller: (||{
                let mut mock_quick_lookup_window_controller 
                    = quick_lookup_window::controller::MockControllerTrait::new();
                mock_quick_lookup_window_controller
                    .expect_toggle_by_switch()
                    .times(1)
                    .with(eq(switch))
                    .returning(|_| Ok(()));
                mock_quick_lookup_window_controller
            })()
        }
    );

    let switch = Switch::Button(Button::South);
    setup_handles_quick_lookup_window(
        SetupHandlesQuickLookupWindowEventsArgs {
            reaction: SwitchOnClickReaction::ToggleQuickLookupWindow,
            pattern: SwitchClickPattern::DoubleClick(switch.clone()),
            mock_quick_lookup_window_controller: (||{
                let mut mock_quick_lookup_window_controller 
                    = quick_lookup_window::controller::MockControllerTrait::new();
                mock_quick_lookup_window_controller
                    .expect_toggle_by_switch()
                    .times(1)
                    .with(eq(switch))
                    .returning(|_| Ok(()));
                mock_quick_lookup_window_controller
            })()
        }
    );
}


fn setup_processes_click_end_switch_pattern(
    reaction: SwitchOnClickReaction,
    pattern: SwitchClickPattern,
    ) {
        let mut mock_switch_click_pattern_detector = MockSwitchClickPatternDetectorTrait::new();
        let mock_gilrs_events = MockGilrsEventsTrait::new();
        let mut mock_layers_navigator = layers_navigator::controller::MockControllerTrait::new();
        let mut mock_mouse_cardinal_levers_move_detector 
            = cardinal_levers_move_detector::mouse::MockMouseTrait::new();

        let mut mock_quick_lookup_window_controller = quick_lookup_window::controller::MockControllerTrait::new();
       let mut mock_input_controller = MockInputControllerTrait::new();

        mock_switch_click_pattern_detector
            .expect_tick()
            .times(1)
            .return_const(Some(pattern.clone()));

        mock_layers_navigator
            .expect_process_switch_event()
            .times(1)
            .with(eq(Some(pattern.clone())))
            .return_const(
                Some(reaction));

        let switch = match pattern.clone() {
            SwitchClickPattern::Click(sw) => sw,
            SwitchClickPattern::ClickAndHold(sw) => sw,
            SwitchClickPattern::DoubleClick(sw) => sw,
            SwitchClickPattern::DoubleClickAndHold(sw) => sw,
            SwitchClickPattern::ClickEnd(sw) => sw,
        };
       mock_quick_lookup_window_controller
           .expect_react_to_keyup()
           .times(1)
           .with(eq(switch))
           .returning(|_| Ok(()));

       mock_input_controller
           .expect_key_up()
           .times(1)
           .with()
           .return_const(());

        mock_layers_navigator
            .expect_consumable_get_current_layer_index()
            .times(1)
            .return_const(None);

        mock_mouse_cardinal_levers_move_detector
            .expect_tick()
            .times(1)
            .return_const(None);

        mock_input_controller
            .expect_trigger_input()
            .times(1)
            .return_const(());

        let mut gamepad_listener = GamepadListener {
           gilrs_events: Box::new(mock_gilrs_events),
           layers_navigator: Box::new(mock_layers_navigator),
           switch_click_pattern_detector: Box::new(mock_switch_click_pattern_detector),
           mouse_cardinal_levers_move_detector: 
               Box::new(mock_mouse_cardinal_levers_move_detector),
           input: Box::new(mock_input_controller),
           quick_lookup_window: Box::new(mock_quick_lookup_window_controller),
        };

        gamepad_listener.next_command()
}

#[test]
fn processes_click_end_switch_pattern(){
    setup_processes_click_end_switch_pattern(
       SwitchOnClickReaction::Keyboard(KeyboardInput{
           key: Key::Layout('I'),
           modifiers: vec![]
       }),
       SwitchClickPattern::ClickEnd(Switch::Button(Button::DPadRight)),
    );

    setup_processes_click_end_switch_pattern(
       SwitchOnClickReaction::ShowQuickLookupWindowOnHold,
       SwitchClickPattern::ClickEnd(Switch::Button(Button::DPadRight)),
    );
    
    setup_processes_click_end_switch_pattern(
       SwitchOnClickReaction::Mouse(MouseInput { button: MouseButton::Back }),
       SwitchClickPattern::ClickEnd(Switch::Button(Button::South)),
    );
}


fn setup_handles_mouse_move_events(
    cardinal_lever_move: Option<MouseEvent>,
    mut mock_input_controller: MockInputControllerTrait,) {

        let mut mock_switch_click_pattern_detector = MockSwitchClickPatternDetectorTrait::new();
        let mock_gilrs_events = MockGilrsEventsTrait::new();
        let mut mock_layers_navigator = layers_navigator::controller::MockControllerTrait::new();
        let mut mock_mouse_cardinal_levers_move_detector 
            = cardinal_levers_move_detector::mouse::MockMouseTrait::new();

        let mock_quick_lookup_window_controller = quick_lookup_window::controller::MockControllerTrait::new();

        mock_switch_click_pattern_detector
            .expect_tick()
            .times(1)
            .return_const(None);

        mock_layers_navigator
            .expect_process_switch_event()
            .times(1)
            .with(eq(None))
            .return_const(None);

        mock_layers_navigator
            .expect_consumable_get_current_layer_index()
            .times(1)
            .return_const(None);

        mock_mouse_cardinal_levers_move_detector
            .expect_tick()
            .times(1)
            .return_const(cardinal_lever_move);

        mock_input_controller
            .expect_trigger_input()
            .times(1)
            .return_const(());

        let mut gamepad_listener = GamepadListener {
           gilrs_events: Box::new(mock_gilrs_events),
           layers_navigator: Box::new(mock_layers_navigator),
           switch_click_pattern_detector: Box::new(mock_switch_click_pattern_detector),
           mouse_cardinal_levers_move_detector: 
               Box::new(mock_mouse_cardinal_levers_move_detector),
           input: Box::new(mock_input_controller),
           quick_lookup_window: Box::new(mock_quick_lookup_window_controller),
        };

        gamepad_listener.next_command()
}

#[test]
fn handles_mouse_move_events(){
    setup_handles_mouse_move_events(
        Some(MouseEvent::MoveCursor(2,13)),
        (||{
           let mut mock_input_controller = MockInputControllerTrait::new();
           mock_input_controller
               .expect_move_mouse_cursor()
               .times(1)
               .with(eq(2),eq(13))
               .return_const(());
           mock_input_controller
       })(),
    );


    setup_handles_mouse_move_events(
        Some(MouseEvent::MoveCursor(5123,9)),
        (||{
           let mut mock_input_controller = MockInputControllerTrait::new();
           mock_input_controller
               .expect_move_mouse_cursor()
               .times(1)
               .with(eq(5123),eq(9))
               .return_const(());
           mock_input_controller
       })(),
    );

    setup_handles_mouse_move_events(
        Some(MouseEvent::Scroll(31,0)),
        (||{
           let mut mock_input_controller = MockInputControllerTrait::new();
           mock_input_controller
               .expect_mouse_scroll()
               .times(1)
               .with(eq(31),eq(0))
               .return_const(());
           mock_input_controller
       })(),
    );

    setup_handles_mouse_move_events(
        Some(MouseEvent::Scroll(10,110)),
        (||{
           let mut mock_input_controller = MockInputControllerTrait::new();
           mock_input_controller
               .expect_mouse_scroll()
               .times(1)
               .with(eq(10),eq(110))
               .return_const(());
           mock_input_controller
       })(),
    );

    setup_handles_mouse_move_events(
        None,
        (||{
           let mock_input_controller = MockInputControllerTrait::new();
           mock_input_controller
       })(),
   );
}
   

fn setup_handles_layer_changed(
    new_layer_index: usize,
    new_layer_cardinal_levers_settings_opt: Option<CardinalLevers>,
) {

        let mut mock_switch_click_pattern_detector = MockSwitchClickPatternDetectorTrait::new();
        let mock_gilrs_events = MockGilrsEventsTrait::new();
        let mut mock_layers_navigator = layers_navigator::controller::MockControllerTrait::new();
        let mut mock_mouse_cardinal_levers_move_detector 
            = cardinal_levers_move_detector::mouse::MockMouseTrait::new();
           let mut mock_input_controller = MockInputControllerTrait::new();

        let mut mock_quick_lookup_window_controller = quick_lookup_window::controller::MockControllerTrait::new();

        mock_switch_click_pattern_detector
            .expect_tick()
            .times(1)
            .return_const(None);

        mock_layers_navigator
            .expect_process_switch_event()
            .times(1)
            .with(eq(None))
            .return_const(None);

        mock_layers_navigator
            .expect_consumable_get_current_layer_index()
            .times(1)
            .return_const(Some(new_layer_index));

        mock_layers_navigator
            .expect_get_cardinal_levers()
            .times(1)
            .return_const(new_layer_cardinal_levers_settings_opt.clone());

        mock_mouse_cardinal_levers_move_detector
            .expect_set_mouse_controls()
            .times(1)
            .with(eq(new_layer_cardinal_levers_settings_opt))
            .return_const(());

        mock_quick_lookup_window_controller
            .expect_emit_current_layer_notification()
            .times(1)
            .with(eq(new_layer_index))
            .returning(|_|Ok(()));

        mock_mouse_cardinal_levers_move_detector
            .expect_tick()
            .times(1)
            .return_const(None);

        mock_input_controller
            .expect_trigger_input()
            .times(1)
            .return_const(());

        let mut gamepad_listener = GamepadListener {
           gilrs_events: Box::new(mock_gilrs_events),
           layers_navigator: Box::new(mock_layers_navigator),
           switch_click_pattern_detector: Box::new(mock_switch_click_pattern_detector),
           mouse_cardinal_levers_move_detector: 
               Box::new(mock_mouse_cardinal_levers_move_detector),
           input: Box::new(mock_input_controller),
           quick_lookup_window: Box::new(mock_quick_lookup_window_controller),
        };

        gamepad_listener.next_command()
}

#[test]
fn handles_layer_changed(){
    setup_handles_layer_changed(
        4,
        Some(CardinalLevers { 
            left_stick: Some(SingleCardinalLever::ControlMouseCursor),
            right_stick: Some(SingleCardinalLever::ControlMouseScrollwheel), 
        }),
    );

    setup_handles_layer_changed(
        101,
        Some(CardinalLevers { 
            left_stick: None,
            right_stick: Some(SingleCardinalLever::ControlMouseCursor), 
        }),
    );

    setup_handles_layer_changed(
        0,
        None,
    );
}
