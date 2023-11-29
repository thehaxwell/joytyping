use serde::Deserialize;

use super::{QuickLookupWindow, err_message_builder::{ErrMessageBuilder, ErrMessageBuilderNode}};

#[cfg(test)]
mod tests;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct MainConfig {
	pub profiles: Vec<Profile>,
    pub global: Global,
    pub development: Option<Development>,
}

impl MainConfig {
    pub fn validate_and_clone_and_set_layer_pointers(&self) -> Result<Self,String> {
        let err_message_builder = ErrMessageBuilder::new();

        Ok(MainConfig { 
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
    pub stick_cardinal_levers: StickCardinalLevers,
    pub trigger_2_switches_click_thresholds: Trigger2SwitchesClickThresholds,
    pub left_upper_is_d_pad: bool,
    #[serde(default = "default_switch_click_event_thresholds")]
    pub switch_click_event_thresholds: SwitchClickEventThresholds,
    #[serde(default = "default_theme")]
    pub theme: Theme,
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
fn default_theme() -> Theme {
    // if the theme isn't specified then default to the
    // system setting
    match dark_light::detect() {
        dark_light::Mode::Dark => Theme::Dark,
        dark_light::Mode::Light => Theme::Light,
        dark_light::Mode::Default => Theme::Dark,
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

        self.stick_cardinal_levers
            .validate(err_message_builder
                .branch(ErrMessageBuilderNode::Single {
                    field: "stick_cardinal_levers".to_string()}))?;

        self.trigger_2_switches_click_thresholds
            .validate(err_message_builder
                .branch(ErrMessageBuilderNode::Single {
                    field: "trigger_2_switches_click_thresholds".to_string()}))?;

        Ok(Profile {
            name: self.name.clone(),
            layout_config_relative_file_path: 
                self.layout_config_relative_file_path.clone(),
            stick_switches_click_thresholds: self.stick_switches_click_thresholds,
            stick_cardinal_levers: self.stick_cardinal_levers,
            trigger_2_switches_click_thresholds: self.trigger_2_switches_click_thresholds,
            left_upper_is_d_pad: self.left_upper_is_d_pad,
            switch_click_event_thresholds: self.switch_click_event_thresholds.clone(),
            theme: self.theme.clone(),
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
pub struct StickCardinalLevers {
    pub deadzone_upper_limits: DeadzoneUpperLimits,
    pub mouse_controls: MouseControls,
}

impl StickCardinalLevers {
    pub fn validate(
        &self,
        err_message_builder: ErrMessageBuilder) -> Result<(),String> {
        self.deadzone_upper_limits
            .validate(err_message_builder
                        .branch(ErrMessageBuilderNode::Single {
                            field: "deadzone_upper_limits".to_string()}))?;
        Ok(())
    }
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct DeadzoneUpperLimits {
    pub left_stick: f32,
    pub right_stick: f32,
}

impl DeadzoneUpperLimits {
    pub fn validate(
        &self,
        err_message_builder: ErrMessageBuilder) -> Result<(),String> {
            let thresholds_arr = [
                (self.left_stick, "left_stick"),
                (self.right_stick, "right_stick"),
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
pub struct MouseControls {
    pub scroll_scale_factor: f32,
    pub cursor_move_scale_factor: f32,
}

impl MouseControls {
    pub fn validate(
        &self,
        err_message_builder: ErrMessageBuilder) -> Result<(),String> {
            [
                (self.scroll_scale_factor, "scroll_scale_factor"),
                (self.cursor_move_scale_factor, "cursor_move_scale_factor"),
            ]
            .iter()
            .map(|(threshold,label)|{
                if *threshold < 0.0 {
                    Err(err_message_builder
                        .branch(ErrMessageBuilderNode::Single { field: label.to_string() })
                        .build_message(format!(
                            "value ({}) is lower than the minimum acceptable 0.0",
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

