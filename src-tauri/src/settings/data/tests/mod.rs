use crate::settings::data::tests::settings_data_example::setup_settings_data_example;

use self::settings_data_example::SetupConfig;

use super::SettingsData;

mod settings_data_example;

fn setup_settings_data_toggle_index_in_gamepad(include_index_in_gamepad: bool) -> SettingsData {
    setup_settings_data_example(SetupConfig {
       include_index_in_gamepad,
       my_ps3_controller_first_layer_step_1_left_trigger_on_double_click_visit_layer_name:
           "second-layer-step-1".to_string()
    })
}

#[test]
fn settings_data_clone_and_set_layer_pointers_works() {
    assert_eq!(
        setup_settings_data_toggle_index_in_gamepad(false)
            .clone_and_set_layer_pointers().unwrap(), 
            setup_settings_data_toggle_index_in_gamepad(true));
}

#[test]
fn settings_data_clone_and_set_layer_pointers_gives_the_right_error() {
    let res = setup_settings_data_example(SetupConfig {
       include_index_in_gamepad: false,
       my_ps3_controller_first_layer_step_1_left_trigger_on_double_click_visit_layer_name:
           "seconda-layer-step-1".to_string()
    }).clone_and_set_layer_pointers();

    assert_eq!(
        res.unwrap_err(),
        "Error in profile \"My PS3 Controller\" > layer with id, \"first-layer-step-1\" > left_trigger > on_double_click > move_to_or_visit_layer: No layer found having the id \"seconda-layer-step-1\"".to_string());

    // TODO: consider changing the format to
    // Error in
    // > profile: "My PS3 Controller"
    //   > layer: "first-layer-step-1"
    //     > left_trigger
    //       > on_double_click
    //         > move_to_or_visit_layer
    // No layer found having the id "seconda-layer-step-1"
}
