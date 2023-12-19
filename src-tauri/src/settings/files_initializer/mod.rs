use crate::{zip_downloader::ZipDownloaderTrait, file_wrapper::FileWrapperTrait, app_data_directory_manager::AppDataDirectoryManagerTrait};

#[cfg(test)]
mod tests;

pub struct FilesInitializer {
    zip_downloader: Box<dyn ZipDownloaderTrait>,
    file_handler: Box<dyn FileWrapperTrait>,
    app_data_directory_manager: Box<dyn AppDataDirectoryManagerTrait>,
}

impl FilesInitializer {
    pub fn new(
        zip_downloader: Box<dyn ZipDownloaderTrait>,
        file_handler: Box<dyn FileWrapperTrait>,
        app_data_directory_manager: Box<dyn AppDataDirectoryManagerTrait>,
    ) -> Self {
        Self {
            zip_downloader,
            file_handler,
            app_data_directory_manager,
        }
    }

    pub fn initialize(&mut self) -> Result<(),String> {
        let file_name = "vallack-layout-0.0.0";
        let download_link = 
            "https://github.com/thehaxwell/vallack-layout/archive/refs/tags/v0.0.0.zip".to_string();
        let joytyping_path = self.app_data_directory_manager
            .get_app_data_directory()
            .map_err(|()|"couldn't retrieve the user directory".to_string())?;

        self.file_handler.create_dir_if_non_existant(&joytyping_path)
            .map_err(|e|e.to_string())?;

        let mut user_settings_path = joytyping_path.clone();
        user_settings_path.push("user");
        user_settings_path.set_extension("toml");

        self.file_handler
            .create_and_write_all(&user_settings_path,
        format!("[global]
default_profile= \"first profile\"

[[profiles]]
name=\"first profile\"
layout_settings_relative_file_path=\"{}/main.toml\"

# Set to true if your controller's d-pad is above the
# left-stick, like a Playstation controller.
# Set to false if the d-pad is below the left-stick,
# like a XBox controller.
left_upper_is_d_pad = true",
file_name).as_bytes())
            .map_err(|e|e.to_string())?;

        self.zip_downloader.download_and_unzip(
            download_link,
            file_name.to_string(),
            joytyping_path
        )
                .map_err(|e|e.to_string())?;
        Ok(())
    }
}
