use serde::Deserialize;

use super::{QuickLookupWindow, err_message_builder::{ErrMessageBuilder, ErrMessageBuilderNode}};

#[cfg(test)]
mod tests;

//TODO: rename SettingsData to Settings
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct SettingsData {
	pub profiles: Vec<Profile>,
    pub global: Global,
    pub development: Option<Development>,
}

impl SettingsData {
    pub fn validate_and_clone_and_set_layer_pointers(&self) -> Result<Self,String> {
        let err_message_builder = ErrMessageBuilder::new();

        Ok(SettingsData { 
            profiles: self.profiles
                .iter()
                .map(|profile|profile.validate_and_clone_and_set_layer_pointers(
                    err_message_builder
                        .branch(ErrMessageBuilderNode::OneOfMany {
                            field: "profiles".to_string(), specific_id: profile.name.clone() })))
                .collect::<Result<Vec<_>,_>>()?,
            global: self.global.clone(),
            development:
                if let Some(dev) = &self.development {
                    Some(dev.validate_and_clone(
                        err_message_builder
                            .branch(ErrMessageBuilderNode::Single { field: "development".to_string() }))?)
                }
                else {
                    None
                },
        })    
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Development {
    pub quick_lookup_window: Option<QuickLookupWindow>,
}

impl Development {
    pub fn validate_and_clone(
        &self,
        err_message_builder: ErrMessageBuilder) -> Result<Self,String> {
        Ok(Development {
            quick_lookup_window: 
                if let Some(window) = &self.quick_lookup_window {
                    Some(window.validate_and_clone(
                        err_message_builder
                            .branch(ErrMessageBuilderNode::Single {
                                field: "quick_lookup_window".to_string() }))?)
                }
                else {
                    None
                },
        })
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Global {
    pub default_profile: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Profile {
	pub name: String,
    pub stick_switches_click_thresholds: StickSwitchesClickThresholds,
    pub trigger_2_switches_click_thresholds: Trigger2SwitchesClickThresholds,
    pub left_upper_is_d_pad: bool,
    #[serde(default = "default_switch_click_event_thresholds")]
    pub switch_click_event_thresholds: SwitchClickEventThresholds,
    pub theme: Option<Theme>,
    pub layout_config_relative_file_path: String,
}
fn default_switch_click_event_thresholds() -> SwitchClickEventThresholds {
    SwitchClickEventThresholds {
        minimum_milliseconds_down_for_click_and_hold:
            default_switch_click_event_threshold_milliseconds(),
        maximum_milliseconds_between_clicks_for_double_click: 
            default_switch_click_event_threshold_milliseconds(),
    }
}

impl Profile {
    fn validate_and_clone_and_set_layer_pointers(
        &self,
        err_message_builder: ErrMessageBuilder) -> Result<Self,String> {

        self.stick_switches_click_thresholds
            .validate(err_message_builder
                .branch(ErrMessageBuilderNode::Single {
                    field: "stick_switches_click_thresholds".to_string()}))?;
        self.trigger_2_switches_click_thresholds
            .validate(err_message_builder
                .branch(ErrMessageBuilderNode::Single {
                    field: "trigger_2_switches_click_thresholds".to_string()}))?;

        Ok(Profile {
            name: self.name.clone(),
            layout_config_relative_file_path: 
                self.layout_config_relative_file_path.clone(),
            stick_switches_click_thresholds: self.stick_switches_click_thresholds,
            trigger_2_switches_click_thresholds: self.trigger_2_switches_click_thresholds,
            left_upper_is_d_pad: self.left_upper_is_d_pad,
            switch_click_event_thresholds: self.switch_click_event_thresholds.clone(),
            theme: if let Some(th) = &self.theme {
                    Some(th.clone())
                }
                else {
                    // if the theme isn't specified then default to the
                    // system setting
                    Some(match dark_light::detect() {
                        dark_light::Mode::Dark => Theme::Dark,
                        dark_light::Mode::Light => Theme::Light,
                        dark_light::Mode::Default => Theme::Dark,
                    })
                }
        })
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct SwitchClickEventThresholds {
    #[serde(default = "default_switch_click_event_threshold_milliseconds")]
    pub minimum_milliseconds_down_for_click_and_hold: u64,
    #[serde(default = "default_switch_click_event_threshold_milliseconds")]
    pub maximum_milliseconds_between_clicks_for_double_click: u64,
}
fn default_switch_click_event_threshold_milliseconds() -> u64 {500}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Theme {
    Light,
    Dark,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct StickSwitchesClickThresholds {
    pub left_stick_up: f32,
    pub left_stick_down: f32,
    pub left_stick_left: f32,
    pub left_stick_right: f32,
    pub right_stick_up: f32,
    pub right_stick_down: f32,
    pub right_stick_left: f32,
    pub right_stick_right: f32,
}

impl StickSwitchesClickThresholds {
    pub fn validate(
        &self,
        err_message_builder: ErrMessageBuilder) -> Result<(),String> {
            let thresholds_arr = [
                (self.left_stick_up, "left_stick_up"),
                (self.left_stick_down, "left_stick_down"),
                (self.left_stick_left, "left_stick_left"),
                (self.left_stick_right, "left_stick_right"),
                (self.right_stick_up, "right_stick_up"),
                (self.right_stick_down, "right_stick_down"),
                (self.right_stick_left, "right_stick_left"),
                (self.right_stick_right, "right_stick_right"),
            ];
            thresholds_arr
                .iter()
                .map(|(threshold,label)|{
                    if *threshold < 0.0 {
                        Err(err_message_builder
                            .branch(ErrMessageBuilderNode::Single { field: label.to_string() })
                            .build_message(format!(
                                "value ({}) is lower than the minimum acceptable 0.0",
                                threshold)))
                    }
                    else if *threshold > 1.0 {
                        Err(err_message_builder
                            .branch(ErrMessageBuilderNode::Single { field: label.to_string() })
                            .build_message(format!(
                                "value ({}) is higher than the maximum acceptable 1.0",
                                threshold)))
                    }
                    else {
                        Ok(())
                    }
                })
                .collect()
    }
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Trigger2SwitchesClickThresholds {
    pub left_trigger_2: f32,
    pub right_trigger_2: f32,
}

impl Trigger2SwitchesClickThresholds {
    pub fn validate(
        &self,
        err_message_builder: ErrMessageBuilder) -> Result<(),String> {
            let thresholds_arr = [
                (self.left_trigger_2, "left_trigger_2"),
                (self.right_trigger_2, "right_trigger_2"),
            ];
            thresholds_arr
                .iter()
                .map(|(threshold,label)|{
                    if *threshold < 0.0 {
                        Err(err_message_builder
                            .branch(ErrMessageBuilderNode::Single { field: label.to_string() })
                            .build_message(format!(
                                "value ({}) is lower than the minimum acceptable 0.0",
                                threshold)))
                    }
                    else if *threshold > 1.0 {
                        Err(err_message_builder
                            .branch(ErrMessageBuilderNode::Single { field: label.to_string() })
                            .build_message(format!(
                                "value ({}) is higher than the maximum acceptable 1.0",
                                threshold)))
                    }
                    else {
                        Ok(())
                    }
                })
                .collect()
    }
}

