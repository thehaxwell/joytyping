use std::path::PathBuf;

#[cfg(test)]
use mockall::{automock, predicate::*};

// #[cfg(test)]
// mod tests;

#[cfg_attr(test, automock)]
pub trait AppDataDirectoryDependencies {
    fn home_dir(&self) -> Option<PathBuf>;
}

pub struct AppDataDirectoryDependenciesImpl;
impl AppDataDirectoryDependencies for AppDataDirectoryDependenciesImpl {
    fn home_dir(&self) -> Option<PathBuf> {
        home::home_dir()
    }
}

// ---------------

#[cfg_attr(test, automock)]
pub trait AppDataDirectoryManagerTrait {
    fn get_app_data_directory(&self) -> Result<PathBuf,()>;
}

pub struct AppDataDirectoryManager {
   dependencies: Box<dyn AppDataDirectoryDependencies>,
}

impl AppDataDirectoryManager {
    pub fn new(
       dependencies: Box<dyn AppDataDirectoryDependencies>,
    ) -> Self {
        Self {
            dependencies,
        }
    }
}

impl AppDataDirectoryManager {
    // see https://softwareengineering.stackexchange.com/questions/3956/best-way-to-save-application-settings

    #[cfg(target_os="windows")]
    fn get_app_data_path() -> String {
        "AppData".to_string()
    }

    #[cfg(not(target_os="windows"))]
    fn get_app_data_path() -> String {
        ".config".to_string()
    }
}

impl AppDataDirectoryManagerTrait for AppDataDirectoryManager {
    fn get_app_data_directory(&self) -> Result<PathBuf,()> {
        match self.dependencies.home_dir() {
            Some(mut path) => {
                path.push(Self::get_app_data_path());
                path.push("joytyping");
                Ok(path)
            },
            None => {
                println!("home_dir() didn't work!");
                Err(())
            }
        }
    }
}
