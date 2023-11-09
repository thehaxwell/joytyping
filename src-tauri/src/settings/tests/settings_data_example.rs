use crate::settings::data::*;

pub fn setup_settings_data_example() -> SettingsData {
    let ps4_controller_profile = Profile{
        name: "My PS3 Controller".to_string(),
        left_upper_is_d_pad: true,
        quick_lookup_window: QuickLookupWindow{
            theme: Some(QuickLookupWindowTheme::Light),
            inner_size: HeightAndWidth{
                height: 100.0,
                width: 100.0,
            },
            source_code: BrowserSourceCode{
                js_iife_bundle_file_path: "path/to/file/bundle.js".to_string(),
                css_file_path: None,
            }
        },
        stick_switches_click_thresholds: StickSwitchesClickThresholds {
            left_stick_up: 0.5,
            left_stick_down: 0.5,
            left_stick_left: 0.5,
            left_stick_right: 0.5,
            right_stick_up: 0.5,
            right_stick_down: 0.5,
            right_stick_left: 0.5,
            right_stick_right: 0.5
        },
        trigger_2_switches_click_thresholds: Trigger2SwitchesClickThresholds { 
            left_trigger_2: 0.3,
            right_trigger_2: 0.3,
        },
        switch_click_event_thresholds: SwitchClickEventThresholds {
            minimum_milliseconds_down_for_click_and_hold: 500,
            maximum_milliseconds_between_clicks_for_double_click: 500,
        },
        layers: Vec::new()
    };

    SettingsData {
        profiles: vec![ps4_controller_profile],
        global: Global{default_profile:"My PS3 Controller".to_string()},
        development: None,
    }
}
