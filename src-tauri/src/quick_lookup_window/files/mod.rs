
use crate::app_data_directory_manager::AppDataDirectoryManagerTrait;
use std::path::PathBuf;

use thiserror::Error;
#[cfg_attr(test, automock)]
pub trait FilesDependencies {
    fn read_to_string(&self, path: PathBuf) -> Result<String, std::io::Error>;
}

pub struct FilesDependenciesImpl;
impl FilesDependencies for FilesDependenciesImpl {
    fn read_to_string(&self, path: PathBuf) -> Result<String, std::io::Error> {
        std::fs::read_to_string(path)
    }
}

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg(test)]
mod tests;

#[cfg_attr(test, automock)]
pub trait FilesTrait {
    fn load_css(&self, css_file_path_opt: Option<String>) -> Result<Option<String>, StartupScriptLoadError>;
    fn load_js(&self, js_iife_bundle_file_path: String) -> Result<String, StartupScriptLoadError>;
}

pub struct Files {
   dependencies: Box<dyn FilesDependencies>,
   app_data_directory_manager: Box<dyn AppDataDirectoryManagerTrait>,
   // path prefix is to be used to change the
   // starting point for relative file paths
   path_prefix: Option<PathBuf>
}

impl Files {
    pub fn new(
       dependencies: Box<dyn FilesDependencies>,
       app_data_directory_manager: Box<dyn AppDataDirectoryManagerTrait>,
       path_prefix: Option<PathBuf>
       ) -> Self {
        Self { 
            dependencies,
            app_data_directory_manager,
            path_prefix,
        }
    }
}

impl FilesTrait for Files {
    fn load_css(&self, css_file_path_opt: Option<String>) -> Result<Option<String>, StartupScriptLoadError> {
        if let Some(css_file_path) = css_file_path_opt {
            self.app_data_directory_manager
                .get_app_data_directory()
                .map_err(|()|StartupScriptLoadError::FileNotFound("css_file_path".to_string()))
                .and_then(|mut path|{
                    if let Some(path_prefix) = &self.path_prefix {
                        path.push(path_prefix);
                    }
                    path.push(css_file_path);
                    self.dependencies.read_to_string(path)
                        .map_err(|e|get_file_load_error(e.kind(), "css_file_path".to_string()))
                })
                .and_then(|rollup_script_str| {
                    Ok(Some(format!("\
                        var styleSheet = document.createElement(\"style\");\n\
                        styleSheet.innerText = `{rollup_script_str}`;\n\
                        document.head.appendChild(styleSheet);")))
                })
        }
        else {
            Ok(None)
        }
    }

    fn load_js(&self, js_iife_bundle_file_path: String) -> Result<String, StartupScriptLoadError> {
        self.app_data_directory_manager
            .get_app_data_directory()
            .map_err(|()| StartupScriptLoadError::FileNotFound("js_iife_bundle_file_path".to_string()))
            .and_then(|mut path|{
                if let Some(path_prefix) = &self.path_prefix {
                    path.push(path_prefix);
                }
                path.push(js_iife_bundle_file_path);
                self.dependencies.read_to_string(path)
                    .map_err(|e|get_file_load_error(e.kind(), "js_iife_bundle_file_path".to_string()))
            }) 
    }
}


#[derive(Error, Debug, PartialEq)]
pub enum StartupScriptLoadError {
    #[error("{0} file not found")]
    FileNotFound(String),

    #[error("{0} file not readable")]
    FileNotReadable(String),

    #[error("OS denied access to {0} file")]
    PermissionDenied(String),
}

fn get_file_load_error(
    err_kind: std::io::ErrorKind,file_name: String)-> StartupScriptLoadError {
    match err_kind {
        std::io::ErrorKind::NotFound => {
            StartupScriptLoadError::FileNotFound(file_name)
        },
        std::io::ErrorKind::PermissionDenied => {
            StartupScriptLoadError::PermissionDenied(file_name)
        },
        _ => {
            StartupScriptLoadError::FileNotReadable(file_name)
        },
    }
}
