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

        if let Some(dev) = &self.development {
            dev.validate(
                err_message_builder
                    .branch(ErrMessageBuilderNode::Single { field: "development".to_string() }))?;
        }

        Ok(SettingsData { 
            profiles: self.profiles
                .iter()
                .map(|profile|profile.validate_and_clone_and_set_layer_pointers(
                    err_message_builder
                        .branch(ErrMessageBuilderNode::OneOfMany {
                            field: "profiles".to_string(), specific_id: profile.name.clone() })))
                .collect::<Result<Vec<_>,_>>()?,
            global: self.global.clone(),
            development: self.development.clone(),
        })    
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Development {
    pub quick_lookup_window: Option<QuickLookupWindow>,
}

impl Development {
    pub fn validate(
        &self,
        err_message_builder: ErrMessageBuilder) -> Result<(),String> {
        if let Some(window) = &self.quick_lookup_window {
            window.validate(
                err_message_builder
                    .branch(ErrMessageBuilderNode::Single { field: "quick_lookup_window".to_string() }))?;
        }
        Ok(())
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
        self.quick_lookup_window
            .validate(err_message_builder
                .branch(ErrMessageBuilderNode::Single {
                    field: "quick_lookup_window".to_string()}))?;

        Ok(Profile {
            name: self.name.clone(),
            quick_lookup_window: self.quick_lookup_window.clone(),
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
        })
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct QuickLookupWindow {
    pub inner_size: HeightAndWidth,
    pub source_code: Option<BrowserSourceCode>,
}

impl QuickLookupWindow {
    pub fn validate(
        &self,
        err_message_builder: ErrMessageBuilder) -> Result<(),String> {
        self.inner_size.validate(
            err_message_builder
                .branch(ErrMessageBuilderNode::Single { field: "inner_size".to_string() }))?;
        Ok(())
    }
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
                                dbg!(threshold))))
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
    pub switches: Option<Switches>,
    // pub levers
    pub cardinal_levers: Option<CardinalLevers>,
}

impl Layer {
    pub fn validate_and_clone_and_set_layer_pointers(
        &self,
        pointers: &Vec<LayerNodeRef>,
        err_message_builder: ErrMessageBuilder) -> Result<Self,String> {
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

    pub fn get_switch_event_and_reaction(
        &self, switch: Switch) -> Option<SwitchEventAndReaction> {
        self.switches.clone().and_then(
            |switches| switches.get_switch_event_and_reaction(switch))
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct CardinalLevers {
    pub left_stick: Option<SingleCardinalLever>,
    pub right_stick: Option<SingleCardinalLever>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SingleCardinalLever {
    ControlMouseCursor(MouseControl),
    ControlMouseScrollwheel(MouseControl),
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct MouseControl {
    pub deadzone_upper_limit: f32,
    pub scale_factor: f32,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
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
    pub fn get_ids_pointing_to_layers(&self) -> Vec<String> {
        [
            &self.south,
            &self.east,
            &self.north,
            &self.west,
            &self.d_pad_up,
            &self.d_pad_down,
            &self.d_pad_left,
            &self.d_pad_right,
            &self.left_stick_up,
            &self.left_stick_down,
            &self.left_stick_left,
            &self.left_stick_right,
            &self.right_stick_up,
            &self.right_stick_down,
            &self.right_stick_left,
            &self.right_stick_right,
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
        Ok(Switches {
            south: if let Some(key) = &self.south { 
				Some(key.validate_and_clone_and_set_layer_pointers(
                    pointers,
                    err_message_builder
                        .branch(ErrMessageBuilderNode::Single { field: "south".to_string() })
                )?) } else { None },
            east: if let Some(key) = &self.east { 
				Some(key.validate_and_clone_and_set_layer_pointers(
					pointers,
                    err_message_builder
                        .branch(ErrMessageBuilderNode::Single { field: "east".to_string() })
                )?) } else { None },
            north: if let Some(key) = &self.north { 
				Some(key.validate_and_clone_and_set_layer_pointers(
					pointers,
                    err_message_builder
                        .branch(ErrMessageBuilderNode::Single { field: "north".to_string() })
                )?) } else { None },
            west: if let Some(key) = &self.west { 
				Some(key.validate_and_clone_and_set_layer_pointers(
					pointers,
                    err_message_builder
                        .branch(ErrMessageBuilderNode::Single { field: "west".to_string() })
                )?) } else { None },
            d_pad_up: if let Some(key) = &self.d_pad_up { 
				Some(key.validate_and_clone_and_set_layer_pointers(
					pointers,
                    err_message_builder
                        .branch(ErrMessageBuilderNode::Single { field: "d_pad_up".to_string() })
                )?) } else { None },
            d_pad_down: if let Some(key) = &self.d_pad_down { 
				Some(key.validate_and_clone_and_set_layer_pointers(
					pointers,
                    err_message_builder
                        .branch(ErrMessageBuilderNode::Single { field: "d_pad_down".to_string() })
                )?) } else { None },
            d_pad_left: if let Some(key) = &self.d_pad_left { 
				Some(key.validate_and_clone_and_set_layer_pointers(
					pointers,
                    err_message_builder
                        .branch(ErrMessageBuilderNode::Single { field: "d_pad_left".to_string() })
                )?) } else { None },
            d_pad_right: if let Some(key) = &self.d_pad_right { 
				Some(key.validate_and_clone_and_set_layer_pointers(
					pointers,
                    err_message_builder
                        .branch(ErrMessageBuilderNode::Single { field: "d_pad_right".to_string() })
                )?) } else { None },
            left_stick_up: if let Some(key) = &self.left_stick_up { 
				Some(key.validate_and_clone_and_set_layer_pointers(
					pointers,
                    err_message_builder
                        .branch(ErrMessageBuilderNode::Single { field: "left_stick_up".to_string() })
                )?) } else { None },
            left_stick_down: if let Some(key) = &self.left_stick_down { 
				Some(key.validate_and_clone_and_set_layer_pointers(
					pointers,
                    err_message_builder
                        .branch(ErrMessageBuilderNode::Single { field: "left_stick_down".to_string() })
                )?) } else { None },
            left_stick_left: if let Some(key) = &self.left_stick_left { 
				Some(key.validate_and_clone_and_set_layer_pointers(
					pointers,
                    err_message_builder
                        .branch(ErrMessageBuilderNode::Single { field: "left_stick_left".to_string() })
                )?) } else { None },
            left_stick_right: if let Some(key) = &self.left_stick_right { 
				Some(key.validate_and_clone_and_set_layer_pointers(
					pointers,
                    err_message_builder
                        .branch(ErrMessageBuilderNode::Single { field: "left_stick_right".to_string() })
                )?) } else { None },
            right_stick_up: if let Some(key) = &self.right_stick_up { 
				Some(key.validate_and_clone_and_set_layer_pointers(
					pointers,
                    err_message_builder
                        .branch(ErrMessageBuilderNode::Single { field: "right_stick_up".to_string() })
                )?) } else { None },
            right_stick_down: if let Some(key) = &self.right_stick_down { 
				Some(key.validate_and_clone_and_set_layer_pointers(
					pointers,
                    err_message_builder
                        .branch(ErrMessageBuilderNode::Single { field: "right_stick_down".to_string() })
                )?) } else { None },
            right_stick_left: if let Some(key) = &self.right_stick_left { 
				Some(key.validate_and_clone_and_set_layer_pointers(
					pointers,
                    err_message_builder
                        .branch(ErrMessageBuilderNode::Single { field: "right_stick_left".to_string() })
                )?) } else { None },
            right_stick_right: if let Some(key) = &self.right_stick_right { 
				Some(key.validate_and_clone_and_set_layer_pointers(
					pointers,
                    err_message_builder
                        .branch(ErrMessageBuilderNode::Single { field: "right_stick_right".to_string() })
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

    ShowQuickLookupWindow,
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

    fn to_enigo_key(key: String) -> Result<enigo::Key, KeyboardInputError> {
        match key.as_str() {
            "Alt" => Ok(enigo::Key::Alt),
            "Backspace" => Ok(enigo::Key::Backspace),
            // "Begin" => Ok(enigo::Key::Begin),
            // "Break" => Ok(enigo::Key::Break),
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
            "Hangul" => Ok(enigo::Key::Hangul),
            "Hanja" => Ok(enigo::Key::Hanja),
            "Help" => Ok(enigo::Key::Help),
            "Home" => Ok(enigo::Key::Home),
            "Insert" => Ok(enigo::Key::Insert),
            "Kanji" => Ok(enigo::Key::Kanji),
            "LControl" => Ok(enigo::Key::LControl),
            "LeftArrow" => Ok(enigo::Key::LeftArrow),
            // "Linefeed" => Ok(enigo::Key::Linefeed),
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
            // "Redo" => Ok(enigo::Key::Redo),
            "Return" => Ok(enigo::Key::Return),
            "RightArrow" => Ok(enigo::Key::RightArrow),
            "RShift" => Ok(enigo::Key::RShift),
            // "ScrollLock" => Ok(enigo::Key::ScrollLock),
            "Select" => Ok(enigo::Key::Select),
            // "ScriptSwitch" => Ok(enigo::Key::ScriptSwitch),
            "Shift" => Ok(enigo::Key::Shift),
            // "ShiftLock" => Ok(enigo::Key::ShiftLock),
            "Space" => Ok(enigo::Key::Space),
            // "SysReq" => Ok(enigo::Key::SysReq),
            "Tab" => Ok(enigo::Key::Tab),
            "UpArrow" => Ok(enigo::Key::UpArrow),
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
