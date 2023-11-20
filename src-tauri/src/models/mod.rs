use serde::Deserialize;
use self::err_message_builder::{ErrMessageBuilder, ErrMessageBuilderNode};

pub mod main_config;
pub mod layout;
pub mod err_message_builder;


#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct QuickLookupWindow {
    pub inner_size: HeightAndWidth,
    pub source_code: BrowserSourceCode,
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
        })
    }
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

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct BrowserSourceCode {
    pub js_iife_bundle_file_path: String,
    pub css_file_path: Option<String>,
}
