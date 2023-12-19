use std::{path::Path, io};

use mockall::predicate::eq;

use crate::{zip_downloader::{MockZipDownloaderTrait, ZipDownloaderError, ZipDownloaderStage}, file_wrapper::MockFileWrapperTrait, app_data_directory_manager::MockAppDataDirectoryManagerTrait};

use super::FilesInitializer;

#[test]
fn initialize_works() {
    let mut mock_zip_downloader = MockZipDownloaderTrait::new();
    let mut mock_file_wrapper = MockFileWrapperTrait::new();
    let mut mock_app_data_directory_manager = MockAppDataDirectoryManagerTrait::new();

    let joytyping_path = Path::new("an/example/path").to_path_buf();
    mock_app_data_directory_manager
        .expect_get_app_data_directory()
        .times(1)
        .return_const(Ok(joytyping_path.clone()));

    mock_file_wrapper
        .expect_create_dir_if_non_existant()
        .times(1)
        .with(eq(joytyping_path.clone()))
        .returning(|_| Ok(true));

    mock_file_wrapper
        .expect_create_and_write_all()
        .times(1)
        .with(eq(Path::new("an/example/path/user.toml").to_path_buf()),
        eq("[global]
default_profile= \"first profile\"

[[profiles]]
name=\"first profile\"
layout_settings_relative_file_path=\"vallack-layout-0.0.0/main.toml\"

# Set to true if your controller's d-pad is above the
# left-stick, like a Playstation controller.
# Set to false if the d-pad is below the left-stick,
# like a XBox controller.
left_upper_is_d_pad = true".as_bytes()))
        .returning(|_,_| Ok(()));

    mock_zip_downloader
        .expect_download_and_unzip()
        .times(1)
        .with(
            eq("https://github.com/thehaxwell/vallack-layout/archive/refs/tags/v0.0.0.zip".to_string()),
            eq("vallack-layout-0.0.0".to_string()),
            eq(joytyping_path),
        )
        .returning(|_,_,_| Ok(()));

    let mut files_initializer = FilesInitializer::new(
        Box::new(mock_zip_downloader),
        Box::new(mock_file_wrapper),
        Box::new(mock_app_data_directory_manager),
    );

    assert_eq!(files_initializer.initialize().unwrap(),());
}

#[test]
fn initialize_handles_get_app_data_directory_error() {
    let mock_zip_downloader = MockZipDownloaderTrait::new();
    let mock_file_wrapper = MockFileWrapperTrait::new();
    let mut mock_app_data_directory_manager = MockAppDataDirectoryManagerTrait::new();

    mock_app_data_directory_manager
        .expect_get_app_data_directory()
        .times(1)
        .return_const(Err(()));

    let mut files_initializer = FilesInitializer::new(
        Box::new(mock_zip_downloader),
        Box::new(mock_file_wrapper),
        Box::new(mock_app_data_directory_manager),
    );

    assert_eq!(files_initializer.initialize().unwrap_err(),"couldn't retrieve the user directory".to_string());
}

#[test]
fn initialize_handles_create_dir_if_non_existant_error() {
    let mock_zip_downloader = MockZipDownloaderTrait::new();
    let mut mock_file_wrapper = MockFileWrapperTrait::new();
    let mut mock_app_data_directory_manager = MockAppDataDirectoryManagerTrait::new();

    let joytyping_path = Path::new("an/example/path").to_path_buf();
    mock_app_data_directory_manager
        .expect_get_app_data_directory()
        .times(1)
        .return_const(Ok(joytyping_path.clone()));

    mock_file_wrapper
        .expect_create_dir_if_non_existant()
        .times(1)
        .with(eq(joytyping_path.clone()))
        .returning(|_| Err(io::Error::new(io::ErrorKind::TimedOut, "Oh no! The shop is closed")));

    let mut files_initializer = FilesInitializer::new(
        Box::new(mock_zip_downloader),
        Box::new(mock_file_wrapper),
        Box::new(mock_app_data_directory_manager),
    );

    assert_eq!(files_initializer.initialize().unwrap_err(),"Oh no! The shop is closed".to_string());
}

#[test]
fn initialize_handles_create_and_write_all_error() {
    let mock_zip_downloader = MockZipDownloaderTrait::new();
    let mut mock_file_wrapper = MockFileWrapperTrait::new();
    let mut mock_app_data_directory_manager = MockAppDataDirectoryManagerTrait::new();

    let joytyping_path = Path::new("an/example/path").to_path_buf();
    mock_app_data_directory_manager
        .expect_get_app_data_directory()
        .times(1)
        .return_const(Ok(joytyping_path.clone()));

    mock_file_wrapper
        .expect_create_dir_if_non_existant()
        .times(1)
        .with(eq(joytyping_path.clone()))
        .returning(|_| Ok(true));

    mock_file_wrapper
        .expect_create_and_write_all()
        .times(1)
        .with(eq(Path::new("an/example/path/user.toml").to_path_buf()),
        eq("[global]
default_profile= \"first profile\"

[[profiles]]
name=\"first profile\"
layout_settings_relative_file_path=\"vallack-layout-0.0.0/main.toml\"

# Set to true if your controller's d-pad is above the
# left-stick, like a Playstation controller.
# Set to false if the d-pad is below the left-stick,
# like a XBox controller.
left_upper_is_d_pad = true".as_bytes()))
        .returning(|_,_| Err(io::Error::new(io::ErrorKind::TimedOut, "Something very specific went wrong :(")));


    let mut files_initializer = FilesInitializer::new(
        Box::new(mock_zip_downloader),
        Box::new(mock_file_wrapper),
        Box::new(mock_app_data_directory_manager),
    );

    assert_eq!(files_initializer.initialize().unwrap_err(),"Something very specific went wrong :(".to_string());
}

#[test]
fn initialize_handles_download_and_unzip_error() {
    let mut mock_zip_downloader = MockZipDownloaderTrait::new();
    let mut mock_file_wrapper = MockFileWrapperTrait::new();
    let mut mock_app_data_directory_manager = MockAppDataDirectoryManagerTrait::new();

    let joytyping_path = Path::new("an/example/path").to_path_buf();
    mock_app_data_directory_manager
        .expect_get_app_data_directory()
        .times(1)
        .return_const(Ok(joytyping_path.clone()));

    mock_file_wrapper
        .expect_create_dir_if_non_existant()
        .times(1)
        .with(eq(joytyping_path.clone()))
        .returning(|_| Ok(true));

    mock_file_wrapper
        .expect_create_and_write_all()
        .times(1)
        .with(eq(Path::new("an/example/path/user.toml").to_path_buf()),
        eq("[global]
default_profile= \"first profile\"

[[profiles]]
name=\"first profile\"
layout_settings_relative_file_path=\"vallack-layout-0.0.0/main.toml\"

# Set to true if your controller's d-pad is above the
# left-stick, like a Playstation controller.
# Set to false if the d-pad is below the left-stick,
# like a XBox controller.
left_upper_is_d_pad = true".as_bytes()))
        .returning(|_,_| Ok(()));

    mock_zip_downloader
        .expect_download_and_unzip()
        .times(1)
        .with(
            eq("https://github.com/thehaxwell/vallack-layout/archive/refs/tags/v0.0.0.zip".to_string()),
            eq("vallack-layout-0.0.0".to_string()),
            eq(joytyping_path),
        )
        .returning(|_,_,_| 
        Err(ZipDownloaderError::IO(
                io::Error::new(io::ErrorKind::TimedOut, "Oh no! The shop is closed"),
                ZipDownloaderStage::DownloadZip)));

    let mut files_initializer = FilesInitializer::new(
        Box::new(mock_zip_downloader),
        Box::new(mock_file_wrapper),
        Box::new(mock_app_data_directory_manager),
    );

    assert_eq!(files_initializer.initialize().unwrap_err(),"IO error in DownloadZip step: Oh no! The shop is closed".to_string());
}
