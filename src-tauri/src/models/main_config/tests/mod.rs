use self::setup_example::setup_example;

use self::setup_example::SetupConfig;

use super::*;

mod setup_example;

fn setup_settings_data_toggle_index_in_gamepad() -> MainConfig {
    setup_example(SetupConfig {
        my_ps3_controller_stick_switches_click_thresholds_left_stick_left: 0.5,
        my_ps3_controller_stick_switches_click_thresholds_right_trigger_2: 0.2,
        development_quick_lookup_window_inner_size_height: 100.0,
    })
}

#[test]
fn settings_data_validate_and_clone_and_set_layer_pointers_works() {
    assert_eq!(
        setup_settings_data_toggle_index_in_gamepad()
            .validate_and_clone_and_set_layer_pointers().unwrap(), 
            setup_settings_data_toggle_index_in_gamepad());
}

#[test]
fn settings_data_validate_and_clone_and_set_layer_pointers_gives_the_right_error() {

    let res = setup_example(SetupConfig {
       my_ps3_controller_stick_switches_click_thresholds_left_stick_left: 1.5,
       my_ps3_controller_stick_switches_click_thresholds_right_trigger_2: 0.2,
       development_quick_lookup_window_inner_size_height: 100.0,
    }).validate_and_clone_and_set_layer_pointers();

    assert_eq!(
        res.unwrap_err(),
        format!("{}\n{}\n{}\n{}\n{}",
        "Error in",
        "> profiles: \"My PS3 Controller\"",
        "   > stick_switches_click_thresholds",
        "      > left_stick_left",
        "value (1.5) is higher than the maximum acceptable 1.0",));


    let res = setup_example(SetupConfig {
       my_ps3_controller_stick_switches_click_thresholds_left_stick_left: 1.21,
       my_ps3_controller_stick_switches_click_thresholds_right_trigger_2: 0.2,
       development_quick_lookup_window_inner_size_height: 100.0,
    }).validate_and_clone_and_set_layer_pointers();

    assert_eq!(
        res.unwrap_err(),
        format!("{}\n{}\n{}\n{}\n{}",
        "Error in",
        "> profiles: \"My PS3 Controller\"",
        "   > stick_switches_click_thresholds",
        "      > left_stick_left",
        "value (1.21) is higher than the maximum acceptable 1.0",));


    let res = setup_example(SetupConfig {
       my_ps3_controller_stick_switches_click_thresholds_left_stick_left: -0.1,
       my_ps3_controller_stick_switches_click_thresholds_right_trigger_2: 0.2,
       development_quick_lookup_window_inner_size_height: 100.0,
    }).validate_and_clone_and_set_layer_pointers();

    assert_eq!(
        res.unwrap_err(),
        format!("{}\n{}\n{}\n{}\n{}",
        "Error in",
        "> profiles: \"My PS3 Controller\"",
        "   > stick_switches_click_thresholds",
        "      > left_stick_left",
        "value (-0.1) is lower than the minimum acceptable 0.0",));


    let res = setup_example(SetupConfig {
       my_ps3_controller_stick_switches_click_thresholds_left_stick_left: -1.07,
       my_ps3_controller_stick_switches_click_thresholds_right_trigger_2: 0.2,
       development_quick_lookup_window_inner_size_height: 100.0,
    }).validate_and_clone_and_set_layer_pointers();

    assert_eq!(
        res.unwrap_err(),
        format!("{}\n{}\n{}\n{}\n{}",
        "Error in",
        "> profiles: \"My PS3 Controller\"",
        "   > stick_switches_click_thresholds",
        "      > left_stick_left",
        "value (-1.07) is lower than the minimum acceptable 0.0",));


    let res = setup_example(SetupConfig {
       my_ps3_controller_stick_switches_click_thresholds_left_stick_left: 0.07,
       my_ps3_controller_stick_switches_click_thresholds_right_trigger_2: 1.2,
       development_quick_lookup_window_inner_size_height: 100.0,
    }).validate_and_clone_and_set_layer_pointers();

    assert_eq!(
        res.unwrap_err(),
        format!("{}\n{}\n{}\n{}\n{}",
        "Error in",
        "> profiles: \"My PS3 Controller\"",
        "   > trigger_2_switches_click_thresholds",
        "      > right_trigger_2",
        "value (1.2) is higher than the maximum acceptable 1.0",));


    let res = setup_example(SetupConfig {
       my_ps3_controller_stick_switches_click_thresholds_left_stick_left: 0.07,
       my_ps3_controller_stick_switches_click_thresholds_right_trigger_2: -0.2,
       development_quick_lookup_window_inner_size_height: 100.0,
    }).validate_and_clone_and_set_layer_pointers();

    assert_eq!(
        res.unwrap_err(),
        format!("{}\n{}\n{}\n{}\n{}",
        "Error in",
        "> profiles: \"My PS3 Controller\"",
        "   > trigger_2_switches_click_thresholds",
        "      > right_trigger_2",
        "value (-0.2) is lower than the minimum acceptable 0.0",));

}


