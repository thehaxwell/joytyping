use crate::models::{main_config::*, HeightAndWidth, BrowserSourceCode};

pub struct SetupConfig {
    pub my_ps3_controller_stick_switches_click_thresholds_left_stick_left: f32,
    pub my_ps3_controller_stick_switches_click_thresholds_right_trigger_2: f32,
    pub development_quick_lookup_window_inner_size_height: f64,
}

pub fn setup_example(config: SetupConfig) -> MainConfig {
    let ps4_controller_profile = Profile{
        name: "My PS3 Controller".to_string(),
        left_upper_is_d_pad: true,
        theme: Theme::Light,
        layout_config_relative_file_path: "layout-name/main.toml".to_string(),
        stick_switches_click_thresholds: StickSwitchesClickThresholds {
            left_stick_up: 0.5,
            left_stick_down: 0.5,
            left_stick_left: config.my_ps3_controller_stick_switches_click_thresholds_left_stick_left,
            left_stick_right: 0.5,
            right_stick_up: 0.5,
            right_stick_down: 0.5,
            right_stick_left: 0.5,
            right_stick_right: 0.5
        },
        stick_cardinal_levers: StickCardinalLevers {
            deadzone_upper_limits: DeadzoneUpperLimits { left_stick: 0.12, right_stick: 0.21 },
            mouse_controls: MouseControls { scroll_scale_factor: 2.0, cursor_move_scale_factor: 5.3 },
        },
        trigger_2_switches_click_thresholds: Trigger2SwitchesClickThresholds { 
            left_trigger_2: 0.3,
            right_trigger_2: config.my_ps3_controller_stick_switches_click_thresholds_right_trigger_2, 
        },
        switch_click_event_thresholds: SwitchClickEventThresholds {
            minimum_milliseconds_down_for_click_and_hold: 500,
            maximum_milliseconds_between_clicks_for_double_click: 500,
        },
    };

    MainConfig {
        profiles: vec![ps4_controller_profile],
        global: Global{default_profile:"My PS3 Controller".to_string()},
        development: Some(Development {
            quick_lookup_window: Some(QuickLookupWindow{
                inner_size: HeightAndWidth{
                    height: config.development_quick_lookup_window_inner_size_height,
                    width: 30.0,
                },
                source_code: BrowserSourceCode{
                    js_iife_bundle_file_path: "path/to/file/bundle.js".to_string(),
                    css_file_path: None,
                }
            }) 
        }),
    }
}
