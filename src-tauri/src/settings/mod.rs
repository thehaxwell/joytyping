use crate::settings::data::SettingsData;
use thiserror::Error;

#[cfg(test)]
use mockall::{automock, predicate::*};

// #[cfg(test)]
// mod tests;

pub mod data;

pub mod error_display_window;

const JOYTYPING_DEFAULT_SETTINGS: &str = include_str!("default_settings.toml");

#[cfg_attr(test, automock)]
pub trait SettingsDependencies {
    fn read_to_string(&self, path: &str) -> Result<String, std::io::Error>;
    fn from_str(&self, s: &str) -> Result<SettingsData, toml::de::Error>;
}

pub struct SettingsDependenciesImpl;
impl SettingsDependencies for SettingsDependenciesImpl {
    fn read_to_string(&self, path: &str) -> Result<String, std::io::Error> {
        std::fs::read_to_string(path)
    }

    fn from_str(&self, s: &str) -> Result<SettingsData, toml::de::Error> {
        toml::from_str(s)
    }
}

pub struct Settings {
    data: Option<SettingsData>,
    dependencies: Box<dyn SettingsDependencies>,
    file_path: String,
}

impl Settings {
    pub fn new(deps: Box<dyn SettingsDependencies>, file_path: String) -> Self{
        Self{data: None,dependencies:deps,file_path}
    }

    /// Load settings from the specified file.
    /// If reading or parsing the file fails, load the default settings.
    pub fn load(&mut self) -> Result<(), SettingsLoadError> {
        match self.dependencies.read_to_string(&self.file_path)
            .map_err(|e|{
                self.load_default();
                match e.kind() {
                    std::io::ErrorKind::NotFound => {
                        SettingsLoadError::FileNotFound
                    },
                    std::io::ErrorKind::PermissionDenied => {
                        SettingsLoadError::PermissionDenied
                    },
                    _ => {
                        SettingsLoadError::FileNotReadable
                    },
                }
            
        })
        .and_then(|settings_str| 
                  self.dependencies
                      .from_str(&settings_str)
                      .map_err(|e|SettingsLoadError::FileNotParsable(e.to_string())))
        .and_then(|data:SettingsData|
                  data.validate_and_clone_and_set_layer_pointers()
                      .map_err(|e:String|SettingsLoadError::FileNotParsable(e)))
        {
            Ok(data) => {
                self.data = Some(data);
                Ok(())
            }
            Err(e) => {
                self.load_default();
                Err(e)
            }
        }
    }

    pub fn load_default(&mut self) {
        match self.dependencies.from_str(JOYTYPING_DEFAULT_SETTINGS){
            Err(e) => 
                panic!("Default settings file not parsable: {}", e.to_string())
            ,
            Ok(deserialized) => {
                match deserialized.validate_and_clone_and_set_layer_pointers() {
                    Ok(data) => self.data = Some(data),
                    Err(e) => panic!("Default settings file not parsable: {}", e.to_string()),
                };
                
            }
        }
    }

    pub fn get_data(&self) -> Option<SettingsData> {
        self.data.clone()
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum SettingsLoadError {
    #[error("Settings file not found")]
    FileNotFound,

    #[error("Settings file not readable")]
    FileNotReadable,

    #[error("Settings file not parsable: {0}")]
    FileNotParsable(String),

    #[error("OS denied access to settings file")]
    PermissionDenied
}


// a utility for settings_data::Layer
#[derive(Debug, PartialEq)]
pub struct LayerNodeRef{
    pub id: String,
    pub index: usize,
}
