use crate::settings_data::SettingsData;
use thiserror::Error;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg(test)]
mod tests;

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
}

impl Settings {
    pub fn new(deps: Box<dyn SettingsDependencies>,) -> Self{
        Self{data: None,dependencies:deps}
    }

    /// Load settings from the specified file.
    /// If reading or parsing the file fails, load the default settings.
    pub fn load(&mut self) -> Result<(), SettingsLoadError> {
        match self.dependencies.read_to_string(
            "/home/haxwell/.config/joytyping/joytyping.toml") {
            Err(e) => {
                self.load_default();
                match e.kind() {
                    std::io::ErrorKind::NotFound => {
                        Err(SettingsLoadError::FileNotFound)
                    },
                    std::io::ErrorKind::PermissionDenied => {
                        Err(SettingsLoadError::PermissionDenied)
                    },
                    _ => {
                        Err(SettingsLoadError::FileNotReadable)
                    },
                }
            },
            Ok(settings_str) => {
                match self.dependencies.from_str(&settings_str) {
                    Err(e) => {
                        self.load_default();
                        Err(SettingsLoadError::FileNotParsable(e.to_string()))
                    },
                    Ok(deserialized) => {
                        self.data = Some(deserialized);
                        Ok(())
                    }
                }
            },
        }
    }

    pub fn load_default(&mut self) {
        match self.dependencies.from_str(JOYTYPING_DEFAULT_SETTINGS) {
            Err(e) => 
                panic!("Default settings file not parsable: {}", e.to_string())
            ,
            Ok(deserialized) => {
                self.data = Some(deserialized);
            }
        }
    }

    pub fn get_data(&self) -> Option<SettingsData> {
        self.data.clone()
    }
}

#[derive(Error, Debug)]
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
