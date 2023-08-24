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
    file_path: String,
}

impl Settings {
    pub fn new(deps: Box<dyn SettingsDependencies>, file_path: String) -> Self{
        Self{data: None,dependencies:deps,file_path}
    }

    /// Load settings from the specified file.
    /// If reading or parsing the file fails, load the default settings.
    pub fn load(&mut self) -> Result<(), SettingsLoadError> {
        match self.dependencies.read_to_string(&self.file_path) {
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
                match self.dependencies.from_str(&settings_str){
                    Err(e) => {
                        self.load_default();
                        Err(SettingsLoadError::FileNotParsable(e.to_string()))
                    },
                    Ok(deserialized) => {
                        if let Err(e) = Settings::validate_settings_data(&deserialized) {
                            self.load_default();
                            return Err(e);
                        }
                        else {
                            self.data = Some(deserialized);
                            Ok(())
                        }
                    }
                }
            },
        }
    }

    pub fn load_default(&mut self) {
        match self.dependencies.from_str(JOYTYPING_DEFAULT_SETTINGS){
            Err(e) => 
                panic!("Default settings file not parsable: {}", e.to_string())
            ,
            Ok(deserialized) => {
                if let Err(e) = Settings::validate_settings_data(&deserialized) {
                    panic!("Default settings file not parsable: {}", e.to_string());
                }
                self.data = Some(deserialized);
            }
        }
    }

    pub fn get_data(&self) -> Option<SettingsData> {
        self.data.clone()
    }

    fn validate_settings_data(data: &SettingsData)
        -> Result<(), SettingsLoadError> {
            let error: Option<SettingsLoadError> = data.profiles.iter().find_map(|p| {
                let thresholds_arr = [
                    p.left_stick.click_thresholds,
                    p.right_stick.click_thresholds];

                let mut idx = -1;
                thresholds_arr.iter().find_map(|t| {
                    idx += 1;
                    let err_at_direction = if t.up > 1.0 || t.up < 0.0 {
                            Some(String::from("Up"))
                    }
                    else if t.down > 1.0 || t.down < 0.0 {
                            Some(String::from("Down"))
                    }
                    else if t.left > 1.0 || t.left < 0.0 {
                            Some(String::from("Left"))
                    }
                    else if t.right > 1.0 || t.right < 0.0 {
                            Some(String::from("Right"))
                    }
                    else {
                        None
                    };

                    if let Some(direction) = err_at_direction {
                        Some(SettingsLoadError::FileNotParsable(format!(
                            "at \"{}\" profile > {} > click_thresholds: {} is out of bounds",
                            p.name,
                            if idx == 0 {"left_stick"} else {"right_stick"},
                            direction)))
                    }
                    else {
                        None
                    }
                })
            });
            if let Some(e) = error {
                Err(e)
            }
            else {
                Ok(())
            }
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
