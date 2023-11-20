use crate::{app_data_directory_manager::AppDataDirectoryManagerTrait, models::{main_config::MainConfig, layout::Layout}};
use thiserror::Error;
use std::path::PathBuf;

#[cfg(test)]
use mockall::{automock, predicate::*};

//TODO: re-introduce tests
// #[cfg(test)]
// mod tests;


pub mod error_display_window;

#[cfg_attr(test, automock)]
pub trait SettingsDependencies {
    fn read_to_string(&self, path: PathBuf) -> Result<String, std::io::Error>;
    fn settings_from_str(&self, s: &str) -> Result<MainConfig, toml::de::Error>;
    fn layout_from_str(&self, s: &str) -> Result<Layout, toml::de::Error>;
    fn home_dir(&self) -> Option<PathBuf>;
}

pub struct SettingsDependenciesImpl;
impl SettingsDependencies for SettingsDependenciesImpl {
    fn read_to_string(&self, path: PathBuf) -> Result<String, std::io::Error> {
        std::fs::read_to_string(path)
    }

    fn settings_from_str(&self, s: &str) -> Result<MainConfig, toml::de::Error> {
        toml::from_str(s)
    }

    fn layout_from_str(&self, s: &str) -> Result<Layout, toml::de::Error> {
        toml::from_str(s)
    }

    fn home_dir(&self) -> Option<PathBuf> {
        home::home_dir()
    }
}

pub struct Settings {
    dependencies: Box<dyn SettingsDependencies>,
    app_data_directory_manager: Box<dyn AppDataDirectoryManagerTrait>,
}

impl Settings {
    pub fn new(
       deps: Box<dyn SettingsDependencies>,
       app_data_directory_manager: Box<dyn AppDataDirectoryManagerTrait>,
       ) -> Self{
        Self{
            dependencies:deps,
            app_data_directory_manager
        }
    }

    /// Load settings from the specified file.
    /// If reading or parsing the file fails, load the default settings.
    pub fn load_settings(&mut self) -> Result<MainConfig, SettingsLoadError> {
        self.app_data_directory_manager
            .get_app_data_directory()
            .map_err(|()|SettingsLoadError::FileNotFound)
            .and_then(|mut path|{
                path.push("start");
                path.set_extension("toml");
                self.dependencies.read_to_string(path)
                    .map_err(|e|{
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
            })
            .and_then(|settings_str| 
                      self.dependencies
                          .settings_from_str(&settings_str)
                          .map_err(|e|SettingsLoadError::FileNotParsable(e.to_string())))
            .and_then(|data|
                      data.validate_and_clone_and_set_layer_pointers()
                          .map_err(|e:String|SettingsLoadError::FileNotParsable(e)))
            
    }


    /// Load settings from the specified file.
    /// If reading or parsing the file fails, load the default settings.
    pub fn load_layout(&mut self, layout_file_path: PathBuf) -> Result<Layout, SettingsLoadError> {
        self.app_data_directory_manager
            .get_app_data_directory()
            .map_err(|()|SettingsLoadError::FileNotFound)
            .and_then(|mut path|{
                path.push(layout_file_path);
                self.dependencies.read_to_string(path)
                    .map_err(|e|{
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
            })
            .and_then(|settings_str| 
                      self.dependencies
                          .layout_from_str(&settings_str)
                          .map_err(|e|SettingsLoadError::FileNotParsable(e.to_string())))
            .and_then(|data|
                      data.validate_and_clone_and_set_layer_pointers()
                          .map_err(|e:String|SettingsLoadError::FileNotParsable(e)))
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


// a utility for layout_config::Layer
#[derive(Debug, PartialEq)]
pub struct LayerNodeRef{
    pub id: String,
    pub index: usize,
}
