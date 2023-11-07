use enigo::MouseButton;
use gilrs::Button;
use serde::Deserialize;
use std::fmt;
use serde::de::{self, Deserializer, Visitor, MapAccess};

use crate::{settings::LayerNodeRef, gamepad::{Switch, gilrs_events::stick_switch_interpreter::StickSwitchButton}};

use self::err_message_builder::{ErrMessageBuilderNode, ErrMessageBuilder};

#[cfg(test)]
mod tests;

mod err_message_builder;

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
        // if let Some(window) = &self.quick_lookup_window {
        //     window.validate(
        //         err_message_builder
        //             .branch(ErrMessageBuilderNode::Single { field: "quick_lookup_window".to_string() }))?;
        // }
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
    pub quick_lookup_window: QuickLookupWindow,
    pub layers: Vec<Layer>,
    pub stick_switches_click_thresholds: StickSwitchesClickThresholds,
    pub trigger_2_switches_click_thresholds: Trigger2SwitchesClickThresholds,
    pub left_upper_is_d_pad: bool,
    #[serde(default = "default_switch_click_event_thresholds")]
    pub switch_click_event_thresholds: SwitchClickEventThresholds,
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
        let pointers: Vec<LayerNodeRef> = self.layers
            .iter()
            .enumerate()
            .map(|(idx,layer)|{
                let res = LayerNodeRef{
                    id: layer.id.to_string(),
                    index: idx.try_into().unwrap(),
                };
                res
            })
            .collect();


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
            quick_lookup_window:
                self.quick_lookup_window
                    .validate_and_clone(err_message_builder
                        .branch(ErrMessageBuilderNode::Single {
                            field: "quick_lookup_window".to_string()}))?,
            layers: self.layers
                .iter()
                .map(|layer| 
                    layer.validate_and_clone_and_set_layer_pointers(
                        &pointers,
                        err_message_builder
                            .branch(ErrMessageBuilderNode::OneOfMany {
                                field: "layers".to_string(), specific_id: layer.id.clone() })
                        ))
                .collect::<Result<Vec<_>,_>>()?,
            stick_switches_click_thresholds: self.stick_switches_click_thresholds,
            trigger_2_switches_click_thresholds: self.trigger_2_switches_click_thresholds,
            left_upper_is_d_pad: self.left_upper_is_d_pad,
            switch_click_event_thresholds: self.switch_click_event_thresholds.clone(),
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
pub struct QuickLookupWindow {
    pub inner_size: HeightAndWidth,
    pub source_code: BrowserSourceCode,
    pub theme: Option<QuickLookupWindowTheme>,
}

impl QuickLookupWindow {
    pub fn validate_and_clone(
        &self,
        err_message_builder: ErrMessageBuilder) -> Result<Self,String> {
        self.inner_size.validate(
            err_message_builder
                .branch(ErrMessageBuilderNode::Single { field: "inner_size".to_string() }))?;
        Ok(QuickLookupWindow { 
            inner_size: self.inner_size.clone(),
            source_code: self.source_code.clone(),
            // if the theme isn't specified then default to the
            // system setting
            theme: if let Some(th) = &self.theme {
                    Some(th.clone())
                }
                else {
                    Some(match dark_light::detect() {
                        dark_light::Mode::Dark => QuickLookupWindowTheme::Dark,
                        dark_light::Mode::Light => QuickLookupWindowTheme::Light,
                        dark_light::Mode::Default => QuickLookupWindowTheme::Dark,
                    })
                }
        })
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QuickLookupWindowTheme {
    Light,
    Dark,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct BrowserSourceCode {
    pub js_iife_bundle_file_path: String,
    pub css_file_path: Option<String>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct HeightAndWidth {
    pub width: f64,
    pub height: f64,
}

impl HeightAndWidth {
    pub fn validate(
        &self,
        err_message_builder: ErrMessageBuilder) -> Result<(),String> {
            let thresholds_arr = [
                (self.height, "height"),
                (self.width, "width"),
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
                    else {
                        Ok(())
                    }
                })
                .collect()
    }
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

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Layer {
    pub id: String,
    pub switches: Option<SwitchesAdapter>,
    // pub levers
    pub cardinal_levers: Option<CardinalLevers>,
}

impl Layer {
    pub fn validate_and_clone_and_set_layer_pointers(
        &self,
        pointers: &Vec<LayerNodeRef>,
        err_message_builder: ErrMessageBuilder) -> Result<Self,String> {

        if let Some(cardinal_levers) = &self.cardinal_levers {
            cardinal_levers.validate(
              err_message_builder
                 .branch(ErrMessageBuilderNode::Single {
                     field: "cardinal_levers".to_string() }))?
        }

        Ok(Self {
            id: self.id.clone(),
            switches: if let Some(key) = &self.switches { 
				Some(key.validate_and_clone_and_set_layer_pointers(
                        pointers,
                        err_message_builder
                            .branch(ErrMessageBuilderNode::Single {
                                field: "switches".to_string() }),
                        )?) } else { None },
            cardinal_levers: self.cardinal_levers.clone(),
        })
    }

    pub fn get_switches(&self,left_upper_is_d_pad: bool) -> Option<Switches> {
        if let Some(switches) = self.switches.clone() {
            Some(Switches {
                south: switches.right_upper_south,
                east: switches.right_upper_east,
                north: switches.right_upper_north,
                west: switches.right_upper_west,
                d_pad_up: if left_upper_is_d_pad {
					 switches.left_upper_north.clone() } else { switches.left_lower_north.clone() },
                d_pad_down: if left_upper_is_d_pad {
					 switches.left_upper_south.clone() } else { switches.left_lower_south.clone() },
                d_pad_left: if left_upper_is_d_pad {
					 switches.left_upper_west.clone() } else { switches.left_lower_west.clone() },
                d_pad_right: if left_upper_is_d_pad {
					 switches.left_upper_east.clone() } else { switches.left_lower_east.clone() },
                left_stick_up: if left_upper_is_d_pad {
					 switches.left_lower_north.clone() } else { switches.left_upper_north.clone() },
                left_stick_down: if left_upper_is_d_pad {
					 switches.left_lower_south.clone() } else { switches.left_upper_south.clone() },
                left_stick_left: if left_upper_is_d_pad {
					 switches.left_lower_west.clone() } else { switches.left_upper_west.clone() },
                left_stick_right: if left_upper_is_d_pad {
					 switches.left_lower_east.clone() } else { switches.left_upper_east.clone() },
                right_stick_up: switches.right_lower_north,
                right_stick_down: switches.right_lower_south,
                right_stick_left: switches.right_lower_west,
                right_stick_right: switches.right_lower_east,
                right_trigger: switches.right_trigger,
                left_trigger: switches.left_trigger,
                right_trigger_2: switches.right_trigger_2,
                left_trigger_2: switches.left_trigger_2,
            })
        }
        else {
            None
        }
    }
}

#[derive(Debug,PartialEq)]
pub struct Switches {
	pub south: Option<SwitchEventAndReaction>,
	pub east: Option<SwitchEventAndReaction>,
	pub north: Option<SwitchEventAndReaction>,
	pub west: Option<SwitchEventAndReaction>,
	pub d_pad_up: Option<SwitchEventAndReaction>,
	pub d_pad_down: Option<SwitchEventAndReaction>,
	pub d_pad_left: Option<SwitchEventAndReaction>,
	pub d_pad_right: Option<SwitchEventAndReaction>,
	pub left_stick_up: Option<SwitchEventAndReaction>,
	pub left_stick_down: Option<SwitchEventAndReaction>,
	pub left_stick_left: Option<SwitchEventAndReaction>,
	pub left_stick_right: Option<SwitchEventAndReaction>,
	pub right_stick_up: Option<SwitchEventAndReaction>,
	pub right_stick_down: Option<SwitchEventAndReaction>,
	pub right_stick_left: Option<SwitchEventAndReaction>,
	pub right_stick_right: Option<SwitchEventAndReaction>,
	pub right_trigger: Option<SwitchEventAndReaction>,
	pub left_trigger: Option<SwitchEventAndReaction>,
	pub right_trigger_2: Option<SwitchEventAndReaction>,
	pub left_trigger_2: Option<SwitchEventAndReaction>,
}

impl Switches {
    pub fn get_switch_event_and_reaction(
        &self, switch: Switch) -> Option<SwitchEventAndReaction> {
        match switch {
            Switch::Button(button) => match button {
                Button::North => self.north.clone(),
                Button::South => self.south.clone(),
                Button::East => self.east.clone(),
                Button::West => self.west.clone(),
                Button::DPadUp => self.d_pad_up.clone(),
                Button::DPadDown => self.d_pad_down.clone(),
                Button::DPadRight => self.d_pad_right.clone(),
                Button::DPadLeft => self.d_pad_left.clone(),
                Button::LeftTrigger => self.left_trigger.clone(),
                Button::RightTrigger => self.right_trigger.clone(),
                Button::LeftTrigger2 => self.left_trigger_2.clone(),
                Button::RightTrigger2 => self.right_trigger_2.clone(),
                _ => None
            },
            Switch::StickSwitchButton(button) => match button {
                StickSwitchButton::LeftStickUp => self.left_stick_up.clone(),
                StickSwitchButton::LeftStickDown => self.left_stick_down.clone(),
                StickSwitchButton::LeftStickRight => self.left_stick_right.clone(),
                StickSwitchButton::LeftStickLeft => self.left_stick_left.clone(),
                StickSwitchButton::RightStickUp => self.right_stick_up.clone(),
                StickSwitchButton::RightStickDown => self.right_stick_down.clone(),
                StickSwitchButton::RightStickRight => self.right_stick_right.clone(),
                StickSwitchButton::RightStickLeft => self.right_stick_left.clone(),
            }
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct CardinalLevers {
    pub left_stick: Option<SingleCardinalLever>,
    pub right_stick: Option<SingleCardinalLever>,
}

impl CardinalLevers {
    pub fn validate(
        &self,
        err_message_builder: ErrMessageBuilder) -> Result<(),String> {
        if let Some(left_stick) = &self.left_stick {
            left_stick.validate(
              err_message_builder
                 .branch(ErrMessageBuilderNode::Single {
                     field: "left_stick".to_string() }))?
        }
        if let Some(right_stick) = &self.right_stick {
            right_stick.validate(
              err_message_builder
                 .branch(ErrMessageBuilderNode::Single {
                     field: "right_stick".to_string() }))?
        }
        Ok(())
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SingleCardinalLever {
    ControlMouseCursor(MouseControl),
    ControlMouseScrollwheel(MouseControl),
}

impl SingleCardinalLever {
    pub fn validate(
        &self,
        err_message_builder: ErrMessageBuilder) -> Result<(),String> {
        match self {
            SingleCardinalLever::ControlMouseCursor(mouse_control)
                => mouse_control.validate(
                      err_message_builder
                         .branch(ErrMessageBuilderNode::Single {
                             field: "control_mouse_cursor".to_string() }))?,
            SingleCardinalLever::ControlMouseScrollwheel(mouse_control)
                => mouse_control.validate(
                      err_message_builder
                         .branch(ErrMessageBuilderNode::Single {
                             field: "control_mouse_scrollwheel".to_string() }))?,
        };
        Ok(())
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct MouseControl {
    pub deadzone_upper_limit: f32,
    pub scale_factor: f32,
}

impl MouseControl {
    pub fn validate(
        &self,
        err_message_builder: ErrMessageBuilder) -> Result<(),String> {
            if self.deadzone_upper_limit > 1.0 {
                return Err(err_message_builder
                    .branch(ErrMessageBuilderNode::Single { field: "deadzone_upper_limit".to_string() })
                    .build_message(format!(
                        "value ({}) is higher than the maximum acceptable 1.0",
                        self.deadzone_upper_limit)));
            }

            [
                (self.deadzone_upper_limit, "deadzone_upper_limit"),
                (self.scale_factor, "scale_factor"),
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

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct SwitchesAdapter {
	pub right_upper_south: Option<SwitchEventAndReaction>,
	pub right_upper_east: Option<SwitchEventAndReaction>,
	pub right_upper_north: Option<SwitchEventAndReaction>,
	pub right_upper_west: Option<SwitchEventAndReaction>,
	pub left_upper_north: Option<SwitchEventAndReaction>,
	pub left_upper_south: Option<SwitchEventAndReaction>,
	pub left_upper_west: Option<SwitchEventAndReaction>,
	pub left_upper_east: Option<SwitchEventAndReaction>,
	pub left_lower_north: Option<SwitchEventAndReaction>,
	pub left_lower_south: Option<SwitchEventAndReaction>,
	pub left_lower_west: Option<SwitchEventAndReaction>,
	pub left_lower_east: Option<SwitchEventAndReaction>,
	pub right_lower_north: Option<SwitchEventAndReaction>,
	pub right_lower_south: Option<SwitchEventAndReaction>,
	pub right_lower_west: Option<SwitchEventAndReaction>,
	pub right_lower_east: Option<SwitchEventAndReaction>,
	pub right_trigger: Option<SwitchEventAndReaction>,
	pub left_trigger: Option<SwitchEventAndReaction>,
	pub right_trigger_2: Option<SwitchEventAndReaction>,
	pub left_trigger_2: Option<SwitchEventAndReaction>,
}

impl SwitchesAdapter {
    pub fn get_ids_pointing_to_layers(&self) -> Vec<String> {
        [
            &self.right_upper_south,
            &self.right_upper_east,
            &self.right_upper_north,
            &self.right_upper_west,
            &self.left_upper_north,
            &self.left_upper_south,
            &self.left_upper_west,
            &self.left_upper_east,
            &self.left_lower_north,
            &self.left_lower_south,
            &self.left_lower_west,
            &self.left_lower_east,
            &self.right_lower_north,
            &self.right_lower_south,
            &self.right_lower_west,
            &self.right_lower_east,
            &self.right_trigger,
            &self.left_trigger,
            &self.right_trigger_2,
            &self.left_trigger_2,
        ]
            .iter()
            .filter_map(|key_opt| if let Some(key) = key_opt {Some(key.get_ids_pointing_to_layers())} else { None })
            .flatten()
            .collect()
    }

    pub fn validate_and_clone_and_set_layer_pointers(
        &self,
        pointers: &Vec<LayerNodeRef>,
        err_message_builder: ErrMessageBuilder) -> Result<Self,String> {
        Ok(SwitchesAdapter {
            right_upper_south: if let Some(key) = &self.right_upper_south { 
				Some(key.validate_and_clone_and_set_layer_pointers(
                    pointers,
                    err_message_builder
                        .branch(ErrMessageBuilderNode::Single { field: "right_upper_south".to_string() })
                )?) } else { None },
            right_upper_east: if let Some(key) = &self.right_upper_east { 
				Some(key.validate_and_clone_and_set_layer_pointers(
					pointers,
                    err_message_builder
                        .branch(ErrMessageBuilderNode::Single { field: "right_upper_east".to_string() })
                )?) } else { None },
            right_upper_north: if let Some(key) = &self.right_upper_north { 
				Some(key.validate_and_clone_and_set_layer_pointers(
					pointers,
                    err_message_builder
                        .branch(ErrMessageBuilderNode::Single { field: "right_upper_north".to_string() })
                )?) } else { None },
            right_upper_west: if let Some(key) = &self.right_upper_west { 
				Some(key.validate_and_clone_and_set_layer_pointers(
					pointers,
                    err_message_builder
                        .branch(ErrMessageBuilderNode::Single { field: "right_upper_west".to_string() })
                )?) } else { None },
            left_upper_north: if let Some(key) = &self.left_upper_north { 
				Some(key.validate_and_clone_and_set_layer_pointers(
					pointers,
                    err_message_builder
                        .branch(ErrMessageBuilderNode::Single { field: "left_upper_north".to_string() })
                )?) } else { None },
            left_upper_south: if let Some(key) = &self.left_upper_south { 
				Some(key.validate_and_clone_and_set_layer_pointers(
					pointers,
                    err_message_builder
                        .branch(ErrMessageBuilderNode::Single { field: "left_upper_south".to_string() })
                )?) } else { None },
            left_upper_west: if let Some(key) = &self.left_upper_west { 
				Some(key.validate_and_clone_and_set_layer_pointers(
					pointers,
                    err_message_builder
                        .branch(ErrMessageBuilderNode::Single { field: "left_upper_west".to_string() })
                )?) } else { None },
            left_upper_east: if let Some(key) = &self.left_upper_east { 
				Some(key.validate_and_clone_and_set_layer_pointers(
					pointers,
                    err_message_builder
                        .branch(ErrMessageBuilderNode::Single { field: "left_upper_east".to_string() })
                )?) } else { None },
            left_lower_north: if let Some(key) = &self.left_lower_north { 
				Some(key.validate_and_clone_and_set_layer_pointers(
					pointers,
                    err_message_builder
                        .branch(ErrMessageBuilderNode::Single { field: "left_lower_north".to_string() })
                )?) } else { None },
            left_lower_south: if let Some(key) = &self.left_lower_south { 
				Some(key.validate_and_clone_and_set_layer_pointers(
					pointers,
                    err_message_builder
                        .branch(ErrMessageBuilderNode::Single { field: "left_lower_south".to_string() })
                )?) } else { None },
            left_lower_west: if let Some(key) = &self.left_lower_west { 
				Some(key.validate_and_clone_and_set_layer_pointers(
					pointers,
                    err_message_builder
                        .branch(ErrMessageBuilderNode::Single { field: "left_lower_west".to_string() })
                )?) } else { None },
            left_lower_east: if let Some(key) = &self.left_lower_east { 
				Some(key.validate_and_clone_and_set_layer_pointers(
					pointers,
                    err_message_builder
                        .branch(ErrMessageBuilderNode::Single { field: "left_lower_east".to_string() })
                )?) } else { None },
            right_lower_north: if let Some(key) = &self.right_lower_north { 
				Some(key.validate_and_clone_and_set_layer_pointers(
					pointers,
                    err_message_builder
                        .branch(ErrMessageBuilderNode::Single { field: "right_lower_north".to_string() })
                )?) } else { None },
            right_lower_south: if let Some(key) = &self.right_lower_south { 
				Some(key.validate_and_clone_and_set_layer_pointers(
					pointers,
                    err_message_builder
                        .branch(ErrMessageBuilderNode::Single { field: "right_lower_south".to_string() })
                )?) } else { None },
            right_lower_west: if let Some(key) = &self.right_lower_west { 
				Some(key.validate_and_clone_and_set_layer_pointers(
					pointers,
                    err_message_builder
                        .branch(ErrMessageBuilderNode::Single { field: "right_lower_west".to_string() })
                )?) } else { None },
            right_lower_east: if let Some(key) = &self.right_lower_east { 
				Some(key.validate_and_clone_and_set_layer_pointers(
					pointers,
                    err_message_builder
                        .branch(ErrMessageBuilderNode::Single { field: "right_lower_east".to_string() })
                )?) } else { None },
            right_trigger: if let Some(key) = &self.right_trigger { 
				Some(key.validate_and_clone_and_set_layer_pointers(
					pointers,
                    err_message_builder
                        .branch(ErrMessageBuilderNode::Single { field: "right_trigger".to_string() })
                )?) } else { None },
            left_trigger: if let Some(key) = &self.left_trigger { 
				Some(key.validate_and_clone_and_set_layer_pointers(
					pointers,
                    err_message_builder
                        .branch(ErrMessageBuilderNode::Single { field: "left_trigger".to_string() })
                )?) } else { None },
            right_trigger_2: if let Some(key) = &self.right_trigger_2 { 
				Some(key.validate_and_clone_and_set_layer_pointers(
					pointers,
                    err_message_builder
                        .branch(ErrMessageBuilderNode::Single { field: "right_trigger_2".to_string() })
                )?) } else { None },
            left_trigger_2: if let Some(key) = &self.left_trigger_2 { 
				Some(key.validate_and_clone_and_set_layer_pointers(
					pointers,
                    err_message_builder
                        .branch(ErrMessageBuilderNode::Single { field: "left_trigger_2".to_string() })
                )?) } else { None },
        })
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct SwitchEventAndReaction {
    pub on_click: Option<SwitchOnClickReaction>,
    pub on_double_click: Option<SwitchOnClickReaction>,
}

impl SwitchEventAndReaction {
    pub fn get_ids_pointing_to_layers(&self) -> Vec<String> {
        [
            &self.on_click,
            &self.on_double_click,
        ]
            .iter()
            .filter_map(|key_opt| match key_opt {
                Some(SwitchOnClickReaction::VisitLayer(layer_specifier))
                    => Some(layer_specifier.id.clone()),
                Some(SwitchOnClickReaction::MoveToLayer(layer_specifier))
                    => Some(layer_specifier.id.clone()),
                Some(SwitchOnClickReaction::MoveToOrVisitLayer(layer_specifier))
                    => Some(layer_specifier.id.clone()),
                _other => None,
            })
            .collect()
    }

    pub fn validate_and_clone_and_set_layer_pointers(
        &self, pointers: &Vec<LayerNodeRef>, err_message_builder: ErrMessageBuilder) -> Result<Self,String>{
        Ok(SwitchEventAndReaction {
            on_click: SwitchEventAndReaction
                ::clone_event_with_layer_pointers_possibly_set(
                    &self.on_click,pointers,
                    err_message_builder
                        .branch(ErrMessageBuilderNode::Single { field: "on_click".to_string() }))?,
            on_double_click: SwitchEventAndReaction
                ::clone_event_with_layer_pointers_possibly_set(
                    &self.on_double_click,pointers,
                    err_message_builder
                        .branch(ErrMessageBuilderNode::Single { field: "on_double_click".to_string() }))?,
        })

    }

    fn clone_event_with_layer_pointers_possibly_set(
        event: &Option<SwitchOnClickReaction>,
        pointers: &Vec<LayerNodeRef>,
        err_message_builder: ErrMessageBuilder) -> Result<Option<SwitchOnClickReaction>,String>{
        match event {
            Some(SwitchOnClickReaction::VisitLayer(layer_specifier))
                => Ok(Some(SwitchOnClickReaction::VisitLayer(
                    layer_specifier.validate_and_clone_and_set_layer_pointer(
                        pointers,
                        err_message_builder
                            .branch(ErrMessageBuilderNode::Single { field: "visit_layer".to_string() })
                        )?))),
            Some(SwitchOnClickReaction::MoveToLayer(layer_specifier))
                => Ok(Some(SwitchOnClickReaction::MoveToLayer(
                    layer_specifier.validate_and_clone_and_set_layer_pointer(
                        pointers,
                        err_message_builder
                            .branch(ErrMessageBuilderNode::Single { field: "move_to_layer".to_string() })
                        )?))),
            Some(SwitchOnClickReaction::MoveToOrVisitLayer(layer_specifier))
                => Ok(Some(SwitchOnClickReaction::MoveToOrVisitLayer(
                    layer_specifier.validate_and_clone_and_set_layer_pointer(
                        pointers,
                        err_message_builder
                            .branch(ErrMessageBuilderNode::Single { field: "move_to_or_visit_layer".to_string() })
                        )?))),
            other => Ok(other.clone()),
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SwitchOnClickReaction {
    Keyboard(KeyboardInput),
    Mouse(MouseInput),

    // visit_layer goes to the layer until the click ends
    VisitLayer(LayerSpecifier),
    // move_to_layer goes to the layer
    MoveToLayer(LayerSpecifier),
    // move_to_or_visit_layer goes to the layer,
    // but if the click is active for long enough then 
    // come back to this layer when the click ends
    MoveToOrVisitLayer(LayerSpecifier),

    ShowQuickLookupWindowOnHold,

    BoostMouseCursorByMultiplier(u32),
}

// this struct should allow serde to always accept a string
// and assign it to name. The pointer will be assigned elsewhere
#[derive(Debug, Clone, PartialEq)]
pub struct LayerSpecifier {
    pub id: String,
    pub index_in_gamepad: Option<usize>,
}

impl LayerSpecifier {
    pub fn new(id: String) -> Self {
        Self {
            id, index_in_gamepad: None
        }
    }

    pub fn validate_and_clone_and_set_layer_pointer(
        &self,
        pointers: &Vec<LayerNodeRef>,
        err_message_builder: ErrMessageBuilder) -> Result<Self,String> {
        let ptr = pointers
            .iter()
            .find(|pointer| pointer.id == self.id);
        if let Some(ptr) = ptr {
            Ok(LayerSpecifier {
                id: self.id.clone(),
                index_in_gamepad: Some(ptr.index),
            })           
        }
        else {
            Err(err_message_builder
                .build_message(format!("No layer found having the id \"{}\"",self.id)))
        }
    }
}

// this impl should allow the following fields to look like this
// to serde:
//
// SwitchOnClickReaction::visit_layer: Option<String>,
// SwitchOnClickReaction::move_to_layer: Option<String>,
// 
impl<'de> Deserialize<'de> for LayerSpecifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LayerSpecifierVisitor;

        impl<'de> Visitor<'de> for LayerSpecifierVisitor {
            type Value = LayerSpecifier;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("string")
            }

            fn visit_str<E>(self, value: &str) -> Result<LayerSpecifier, E>
            where
                E: de::Error,
            {
                Ok(LayerSpecifier::new(value.to_string()))
            }
        }

        deserializer.deserialize_string(LayerSpecifierVisitor)
    }
}

#[derive(Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct MouseInput {
    #[serde(with = "EnigoMouseButtonDef")]
    pub button: enigo::MouseButton,
}

#[derive(Deserialize, Debug, Copy, Clone, PartialEq)]
#[serde(remote = "MouseButton")]
pub enum EnigoMouseButtonDef {
    Left,
    Middle,
    Right,
    Back,
    Forward,
    ScrollUp,
    ScrollDown,
    ScrollLeft,
    ScrollRight,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KeyboardInput {
    pub key: enigo::Key,
    pub modifiers: Vec<enigo::Key>,
}

#[derive(Debug)]
enum KeyboardInputError {
    UnknownVariant(String)
}

impl KeyboardInput {
    fn new(
        key_arg: String,
        modifiers_arg_option: Option<Vec<String>>,) -> Result<KeyboardInput, KeyboardInputError> {

        let key = 
            if key_arg.chars().count() == 1 {
                enigo::Key::Layout(key_arg.chars().next().unwrap())}
            else {
                KeyboardInput::to_enigo_key(key_arg.clone())?
            };

        let mut modifiers = Vec::new();
        if let Some(modifiers_arg) = modifiers_arg_option {
            for modifier in modifiers_arg {
                match KeyboardInput::to_enigo_key(modifier) {
                    Ok(key) => modifiers.push(key),
                    Err(err) => return Err(err)
                }
            }
        }
        Ok(Self{key, modifiers})
    }

    #[cfg(not(target_os="windows"))]
    fn to_enigo_key(key: String) -> Result<enigo::Key, KeyboardInputError> {
        match key.as_str() {
            "Alt" => Ok(enigo::Key::Alt),
            "Backspace" => Ok(enigo::Key::Backspace),
            "Cancel" => Ok(enigo::Key::Cancel),
            "CapsLock" => Ok(enigo::Key::CapsLock),
            "Clear" => Ok(enigo::Key::Clear),
            "Control" => Ok(enigo::Key::Control),
            "Delete" => Ok(enigo::Key::Delete),
            "DownArrow" => Ok(enigo::Key::DownArrow),
            "End" => Ok(enigo::Key::End),
            "Escape" => Ok(enigo::Key::Escape),
            "Execute" => Ok(enigo::Key::Execute),
            "F1" => Ok(enigo::Key::F1),
            "F2" => Ok(enigo::Key::F2),
            "F3" => Ok(enigo::Key::F3),
            "F4" => Ok(enigo::Key::F4),
            "F5" => Ok(enigo::Key::F5),
            "F6" => Ok(enigo::Key::F6),
            "F7" => Ok(enigo::Key::F7),
            "F8" => Ok(enigo::Key::F8),
            "F9" => Ok(enigo::Key::F9),
            "F10" => Ok(enigo::Key::F10),
            "F11" => Ok(enigo::Key::F11),
            "F12" => Ok(enigo::Key::F12),
            "F13" => Ok(enigo::Key::F13),
            "F14" => Ok(enigo::Key::F14),
            "F15" => Ok(enigo::Key::F15),
            "F16" => Ok(enigo::Key::F16),
            "F17" => Ok(enigo::Key::F17),
            "F18" => Ok(enigo::Key::F18),
            "F19" => Ok(enigo::Key::F19),
            "F20" => Ok(enigo::Key::F20),
            "F21" => Ok(enigo::Key::F21),
            "F22" => Ok(enigo::Key::F22),
            "F23" => Ok(enigo::Key::F23),
            "F24" => Ok(enigo::Key::F24),
            "Hangul" => Ok(enigo::Key::Hangul),
            "Hanja" => Ok(enigo::Key::Hanja),
            "Help" => Ok(enigo::Key::Help),
            "Home" => Ok(enigo::Key::Home),
            "Insert" => Ok(enigo::Key::Insert),
            "Kanji" => Ok(enigo::Key::Kanji),
            "LControl" => Ok(enigo::Key::LControl),
            "LeftArrow" => Ok(enigo::Key::LeftArrow),
            "LMenu" => Ok(enigo::Key::LMenu),
            "LShift" => Ok(enigo::Key::LShift),
            "Meta" => Ok(enigo::Key::Meta),
            "ModeChange" => Ok(enigo::Key::ModeChange),
            "Numlock" => Ok(enigo::Key::Numlock),
            "Option" => Ok(enigo::Key::Option),
            "PageDown" => Ok(enigo::Key::PageDown),
            "PageUp" => Ok(enigo::Key::PageUp),
            "Pause" => Ok(enigo::Key::Pause),
            "Print" => Ok(enigo::Key::Print),
            "RControl" => Ok(enigo::Key::RControl),
            "Return" => Ok(enigo::Key::Return),
            "RightArrow" => Ok(enigo::Key::RightArrow),
            "RShift" => Ok(enigo::Key::RShift),
            "Select" => Ok(enigo::Key::Select),
            "Shift" => Ok(enigo::Key::Shift),
            "Space" => Ok(enigo::Key::Space),
            "Tab" => Ok(enigo::Key::Tab),
            "UpArrow" => Ok(enigo::Key::UpArrow),
            _other => return Err(KeyboardInputError::UnknownVariant(key)),
        }
    }

    #[cfg(target_os="windows")]
    fn to_enigo_key(key: String) -> Result<enigo::Key, KeyboardInputError> {
        match key.as_str() {
            "Alt" => Ok(enigo::Key::Alt),
            "Backspace" => Ok(enigo::Key::Backspace),
            "Cancel" => Ok(enigo::Key::Cancel),
            "CapsLock" => Ok(enigo::Key::CapsLock),
            "Clear" => Ok(enigo::Key::Clear),
            "Control" => Ok(enigo::Key::Control),
            "Delete" => Ok(enigo::Key::Delete),
            "DownArrow" => Ok(enigo::Key::DownArrow),
            "End" => Ok(enigo::Key::End),
            "Escape" => Ok(enigo::Key::Escape),
            "Execute" => Ok(enigo::Key::Execute),
            "F1" => Ok(enigo::Key::F1),
            "F2" => Ok(enigo::Key::F2),
            "F3" => Ok(enigo::Key::F3),
            "F4" => Ok(enigo::Key::F4),
            "F5" => Ok(enigo::Key::F5),
            "F6" => Ok(enigo::Key::F6),
            "F7" => Ok(enigo::Key::F7),
            "F8" => Ok(enigo::Key::F8),
            "F9" => Ok(enigo::Key::F9),
            "F10" => Ok(enigo::Key::F10),
            "F11" => Ok(enigo::Key::F11),
            "F12" => Ok(enigo::Key::F12),
            "F13" => Ok(enigo::Key::F13),
            "F14" => Ok(enigo::Key::F14),
            "F15" => Ok(enigo::Key::F15),
            "F16" => Ok(enigo::Key::F16),
            "F17" => Ok(enigo::Key::F17),
            "F18" => Ok(enigo::Key::F18),
            "F19" => Ok(enigo::Key::F19),
            "F20" => Ok(enigo::Key::F20),
            "F21" => Ok(enigo::Key::F21),
            "F22" => Ok(enigo::Key::F22),
            "F23" => Ok(enigo::Key::F23),
            "F24" => Ok(enigo::Key::F24),
            "Hangul" => Ok(enigo::Key::Hangul),
            "Hanja" => Ok(enigo::Key::Hanja),
            "Help" => Ok(enigo::Key::Help),
            "Home" => Ok(enigo::Key::Home),
            "Insert" => Ok(enigo::Key::Insert),
            "Kanji" => Ok(enigo::Key::Kanji),
            "LControl" => Ok(enigo::Key::LControl),
            "LeftArrow" => Ok(enigo::Key::LeftArrow),
            "LMenu" => Ok(enigo::Key::LMenu),
            "LShift" => Ok(enigo::Key::LShift),
            "Meta" => Ok(enigo::Key::Meta),
            "ModeChange" => Ok(enigo::Key::ModeChange),
            "Numlock" => Ok(enigo::Key::Numlock),
            "Option" => Ok(enigo::Key::Option),
            "PageDown" => Ok(enigo::Key::PageDown),
            "PageUp" => Ok(enigo::Key::PageUp),
            "Pause" => Ok(enigo::Key::Pause),
            "Print" => Ok(enigo::Key::Print),
            "RControl" => Ok(enigo::Key::RControl),
            "Return" => Ok(enigo::Key::Return),
            "RightArrow" => Ok(enigo::Key::RightArrow),
            "RShift" => Ok(enigo::Key::RShift),
            "Select" => Ok(enigo::Key::Select),
            "Shift" => Ok(enigo::Key::Shift),
            "Space" => Ok(enigo::Key::Space),
            "Tab" => Ok(enigo::Key::Tab),
            "UpArrow" => Ok(enigo::Key::UpArrow),
            // "Begin" => Ok(enigo::Key::Begin),
            // "Break" => Ok(enigo::Key::Break),
            // "F25" => Ok(enigo::Key::F25),
            // "F26" => Ok(enigo::Key::F26),
            // "F27" => Ok(enigo::Key::F27),
            // "F28" => Ok(enigo::Key::F28),
            // "F29" => Ok(enigo::Key::F29),
            // "F30" => Ok(enigo::Key::F30),
            // "F31" => Ok(enigo::Key::F31),
            // "F32" => Ok(enigo::Key::F32),
            // "F33" => Ok(enigo::Key::F33),
            // "F34" => Ok(enigo::Key::F34),
            // "F35" => Ok(enigo::Key::F35),
            // "Find" => Ok(enigo::Key::Find),
            // "Linefeed" => Ok(enigo::Key::Linefeed),
            // "Redo" => Ok(enigo::Key::Redo),
            // "ScrollLock" => Ok(enigo::Key::ScrollLock),
            // "ScriptSwitch" => Ok(enigo::Key::ScriptSwitch),
            // "ShiftLock" => Ok(enigo::Key::ShiftLock),
            // "SysReq" => Ok(enigo::Key::SysReq),
            // "Undo" => Ok(enigo::Key::Undo),
            _other => return Err(KeyboardInputError::UnknownVariant(key)),
        }
    }
}

impl<'de> Deserialize<'de> for KeyboardInput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field { Key, Modifiers }


        struct KeyboardInputVisitor;

        impl<'de> Visitor<'de> for KeyboardInputVisitor {
            type Value = KeyboardInput;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct KeyboardInput")
            }

            // NOTE: I didn't implemented visit_seq because I only plan on importing toml
            // fn visit_seq<V>(self, mut seq: V) -> Result<KeyboardInput, V::Error>

            fn visit_map<V>(self, mut map: V) -> Result<KeyboardInput, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut key = None;
                let mut modifiers = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        Field::Key => {
                            if key.is_some() {
                                return Err(de::Error::duplicate_field("key"));
                            }
                            key = Some(map.next_value()?);
                        }
                        Field::Modifiers => {
                            if modifiers.is_some() {
                                return Err(de::Error::duplicate_field("modifiers"));
                            }
                            modifiers = Some(map.next_value()?);
                        }
                    }
                }
                let key = key.ok_or_else(|| de::Error::missing_field("key"))?;
                // let modifiers = modifiers.ok_or_else(|| de::Error::missing_field("modifiers"))?;

                let result = KeyboardInput::new(key, modifiers).or_else(|err| match err {
                    KeyboardInputError::UnknownVariant(var)
                        => return Err(de::Error::unknown_variant(&var, 
                            // Hard coded some valid values
                            &["Meta","Alt", "Control", "..."])),
                })?;
                Ok(result)

            }
        }

        const FIELDS: &'static [&'static str] = &["key", "modifiers"];
        deserializer.deserialize_struct("KeyboardInput", FIELDS, KeyboardInputVisitor)
    }
}
