use crate::settings::data::tests::settings_data_example::setup_settings_data_example;

use self::settings_data_example::SetupConfig;

use super::SettingsData;

mod settings_data_example;

fn setup_settings_data_toggle_index_in_gamepad(include_index_in_gamepad: bool) -> SettingsData {
    setup_settings_data_example(SetupConfig {
       include_index_in_gamepad,
       my_ps3_controller_first_layer_step_1_left_trigger_on_double_click_visit_layer_name:
           "second-layer-step-1".to_string(),
        my_ps3_controller_stick_switches_click_thresholds_left_stick_left: 0.5,
        my_ps3_controller_stick_switches_click_thresholds_right_trigger_2: 0.2,
        my_ps3_controller_quick_lookup_window_inner_size_height: 100.0,
        my_ps3_controller_quick_lookup_window_inner_size_width: 100.0,
        development_quick_lookup_window_inner_size_height: 100.0,
    })
}

#[test]
fn settings_data_validate_and_clone_and_set_layer_pointers_works() {
    assert_eq!(
        setup_settings_data_toggle_index_in_gamepad(false)
            .validate_and_clone_and_set_layer_pointers().unwrap(), 
            setup_settings_data_toggle_index_in_gamepad(true));
}

#[test]
fn settings_data_validate_and_clone_and_set_layer_pointers_gives_the_right_error() {
    let res = setup_settings_data_example(SetupConfig {
       include_index_in_gamepad: false,
       my_ps3_controller_first_layer_step_1_left_trigger_on_double_click_visit_layer_name:
           "seconda-layer-step-1".to_string(),
       my_ps3_controller_stick_switches_click_thresholds_left_stick_left: 0.5,
       my_ps3_controller_stick_switches_click_thresholds_right_trigger_2: 0.2,
       my_ps3_controller_quick_lookup_window_inner_size_height: 100.0,
       my_ps3_controller_quick_lookup_window_inner_size_width: 100.0,
       development_quick_lookup_window_inner_size_height: 100.0,
    }).validate_and_clone_and_set_layer_pointers();

    assert_eq!(
        res.unwrap_err(),
        format!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
        "Error in",
        "> profiles: \"My PS3 Controller\"",
        "   > layers: \"first-layer-step-1\"",
        "      > switches",
        "         > left_trigger",
        "            > on_double_click",
        "               > move_to_or_visit_layer",
        "No layer found having the id \"seconda-layer-step-1\"",));


    let res = setup_settings_data_example(SetupConfig {
       include_index_in_gamepad: false,
       my_ps3_controller_first_layer_step_1_left_trigger_on_double_click_visit_layer_name:
           "second-layer-step-1".to_string(),
       my_ps3_controller_stick_switches_click_thresholds_left_stick_left: 1.5,
       my_ps3_controller_stick_switches_click_thresholds_right_trigger_2: 0.2,
       my_ps3_controller_quick_lookup_window_inner_size_height: 100.0,
       my_ps3_controller_quick_lookup_window_inner_size_width: 100.0,
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


    let res = setup_settings_data_example(SetupConfig {
       include_index_in_gamepad: false,
       my_ps3_controller_first_layer_step_1_left_trigger_on_double_click_visit_layer_name:
           "second-layer-step-1".to_string(),
       my_ps3_controller_stick_switches_click_thresholds_left_stick_left: 1.21,
       my_ps3_controller_stick_switches_click_thresholds_right_trigger_2: 0.2,
       my_ps3_controller_quick_lookup_window_inner_size_height: 100.0,
       my_ps3_controller_quick_lookup_window_inner_size_width: 100.0,
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


    let res = setup_settings_data_example(SetupConfig {
       include_index_in_gamepad: false,
       my_ps3_controller_first_layer_step_1_left_trigger_on_double_click_visit_layer_name:
           "second-layer-step-1".to_string(),
       my_ps3_controller_stick_switches_click_thresholds_left_stick_left: -0.1,
       my_ps3_controller_stick_switches_click_thresholds_right_trigger_2: 0.2,
       my_ps3_controller_quick_lookup_window_inner_size_height: 100.0,
       my_ps3_controller_quick_lookup_window_inner_size_width: 100.0,
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


    let res = setup_settings_data_example(SetupConfig {
       include_index_in_gamepad: false,
       my_ps3_controller_first_layer_step_1_left_trigger_on_double_click_visit_layer_name:
           "second-layer-step-1".to_string(),
       my_ps3_controller_stick_switches_click_thresholds_left_stick_left: -1.07,
       my_ps3_controller_stick_switches_click_thresholds_right_trigger_2: 0.2,
       my_ps3_controller_quick_lookup_window_inner_size_height: 100.0,
       my_ps3_controller_quick_lookup_window_inner_size_width: 100.0,
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


    let res = setup_settings_data_example(SetupConfig {
       include_index_in_gamepad: false,
       my_ps3_controller_first_layer_step_1_left_trigger_on_double_click_visit_layer_name:
           "second-layer-step-1".to_string(),
       my_ps3_controller_stick_switches_click_thresholds_left_stick_left: 0.07,
       my_ps3_controller_stick_switches_click_thresholds_right_trigger_2: 1.2,
       my_ps3_controller_quick_lookup_window_inner_size_height: 100.0,
       my_ps3_controller_quick_lookup_window_inner_size_width: 100.0,
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


    let res = setup_settings_data_example(SetupConfig {
       include_index_in_gamepad: false,
       my_ps3_controller_first_layer_step_1_left_trigger_on_double_click_visit_layer_name:
           "second-layer-step-1".to_string(),
       my_ps3_controller_stick_switches_click_thresholds_left_stick_left: 0.07,
       my_ps3_controller_stick_switches_click_thresholds_right_trigger_2: -0.2,
       my_ps3_controller_quick_lookup_window_inner_size_height: 100.0,
       my_ps3_controller_quick_lookup_window_inner_size_width: 100.0,
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


    let res = setup_settings_data_example(SetupConfig {
       include_index_in_gamepad: false,
       my_ps3_controller_first_layer_step_1_left_trigger_on_double_click_visit_layer_name:
           "second-layer-step-1".to_string(),
       my_ps3_controller_stick_switches_click_thresholds_left_stick_left: 0.5,
       my_ps3_controller_stick_switches_click_thresholds_right_trigger_2: 0.2,
       my_ps3_controller_quick_lookup_window_inner_size_height: 100.0,
       my_ps3_controller_quick_lookup_window_inner_size_width: -100.0,
       development_quick_lookup_window_inner_size_height: 100.0,
    }).validate_and_clone_and_set_layer_pointers();

    assert_eq!(
        res.unwrap_err(),
        format!("{}\n{}\n{}\n{}\n{}\n{}",
        "Error in",
        "> profiles: \"My PS3 Controller\"",
        "   > quick_lookup_window",
        "      > inner_size",
        "         > width",
        "value (-100) is lower than the minimum acceptable 0.0",));


    let res = setup_settings_data_example(SetupConfig {
       include_index_in_gamepad: false,
       my_ps3_controller_first_layer_step_1_left_trigger_on_double_click_visit_layer_name:
           "second-layer-step-1".to_string(),
       my_ps3_controller_stick_switches_click_thresholds_left_stick_left: 0.5,
       my_ps3_controller_stick_switches_click_thresholds_right_trigger_2: 0.2,
       my_ps3_controller_quick_lookup_window_inner_size_height: -1.002,
       my_ps3_controller_quick_lookup_window_inner_size_width: 100.0,
       development_quick_lookup_window_inner_size_height: 100.0,
    }).validate_and_clone_and_set_layer_pointers();

    assert_eq!(
        res.unwrap_err(),
        format!("{}\n{}\n{}\n{}\n{}\n{}",
        "Error in",
        "> profiles: \"My PS3 Controller\"",
        "   > quick_lookup_window",
        "      > inner_size",
        "         > height",
        "value (-1.002) is lower than the minimum acceptable 0.0",));


    let res = setup_settings_data_example(SetupConfig {
       include_index_in_gamepad: false,
       my_ps3_controller_first_layer_step_1_left_trigger_on_double_click_visit_layer_name:
           "second-layer-step-1".to_string(),
       my_ps3_controller_stick_switches_click_thresholds_left_stick_left: 0.5,
       my_ps3_controller_stick_switches_click_thresholds_right_trigger_2: 0.2,
       my_ps3_controller_quick_lookup_window_inner_size_height: 100.0,
       my_ps3_controller_quick_lookup_window_inner_size_width: 100.0,
       development_quick_lookup_window_inner_size_height: -10.12,
    }).validate_and_clone_and_set_layer_pointers();

    assert_eq!(
        res.unwrap_err(),
        format!("{}\n{}\n{}\n{}\n{}\n{}",
        "Error in",
        "> development",
        "   > quick_lookup_window",
        "      > inner_size",
        "         > height",
        "value (-10.12) is lower than the minimum acceptable 0.0",));
}

