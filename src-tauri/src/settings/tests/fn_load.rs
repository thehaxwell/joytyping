use std::path::Path;

use mockall::predicate::*;
use crate::{settings::{MockSettingsDependencies, Settings, SettingsLoadError, tests::settings_data_example::setup_settings_data_example_with_error}, app_data_directory_manager::MockAppDataDirectoryManagerTrait};

use super::settings_data_example::setup_settings_data_example;

#[test]
fn works() {
    let mut mock_app_data_directory_manager = MockAppDataDirectoryManagerTrait::new();
    let mut mock_deps = MockSettingsDependencies::new();

    mock_app_data_directory_manager
        .expect_get_app_data_directory()
        .times(1)
        .return_const(Ok(Path::new("an/example/path").to_path_buf()));

    mock_deps
        .expect_read_to_string()
        .times(1)
        .returning(|_|Ok("an example of the raw toml code".to_string()));

    mock_deps
        .expect_from_str()
        .with(eq("an example of the raw toml code".to_string()))
        .times(1)
        .return_const(Ok(setup_settings_data_example()));

    let mut settings = Settings {
        data: None,
        dependencies: Box::new(mock_deps),
        app_data_directory_manager: Box::new(mock_app_data_directory_manager),
    };

    assert!(settings.load().is_ok());
    assert_eq!(settings.data.unwrap(),setup_settings_data_example());
}

#[test]
fn interprets_app_data_directory_manager_error() {
    let mut mock_app_data_directory_manager = MockAppDataDirectoryManagerTrait::new();
    let mock_deps = MockSettingsDependencies::new();

    mock_app_data_directory_manager
        .expect_get_app_data_directory()
        .times(1)
        .return_const(Err(()));

    let mut settings = Settings {
        data: None,
        dependencies: Box::new(mock_deps),
        app_data_directory_manager: Box::new(mock_app_data_directory_manager),
    };

    assert_eq!(settings.load(),Err(SettingsLoadError::FileNotFound));
    assert!(settings.data.is_none());
}


fn setup_interprets_dependencies_read_to_string_error(
    err_kind: std::io::ErrorKind) -> Settings {
    let mut mock_app_data_directory_manager = MockAppDataDirectoryManagerTrait::new();
    let mut mock_deps = MockSettingsDependencies::new();

    mock_app_data_directory_manager
        .expect_get_app_data_directory()
        .times(1)
        .return_const(Ok(Path::new("an/example/path").to_path_buf()));

    mock_deps
        .expect_read_to_string()
        .times(1)
        .returning(move |_|Err(std::io::Error::new(err_kind,"Oh no!")));

    Settings {
        data: None,
        dependencies: Box::new(mock_deps),
        app_data_directory_manager: Box::new(mock_app_data_directory_manager),
    }
}

#[test]
fn interprets_dependencies_read_to_string_error() {
    let mut settings = setup_interprets_dependencies_read_to_string_error(
        std::io::ErrorKind::NotFound);
    assert_eq!(settings.load(),Err(SettingsLoadError::FileNotFound));
    assert!(settings.data.is_none());

    let mut settings = setup_interprets_dependencies_read_to_string_error(
        std::io::ErrorKind::PermissionDenied);
    assert_eq!(settings.load(),Err(SettingsLoadError::PermissionDenied));
    assert!(settings.data.is_none());

    // the rest all give FileNotReadable. We'll just test a few
    let mut settings = setup_interprets_dependencies_read_to_string_error(
        std::io::ErrorKind::ConnectionReset);
    assert_eq!(settings.load(),Err(SettingsLoadError::FileNotReadable));
    assert!(settings.data.is_none());

    let mut settings = setup_interprets_dependencies_read_to_string_error(
        std::io::ErrorKind::BrokenPipe);
    assert_eq!(settings.load(),Err(SettingsLoadError::FileNotReadable));
    assert!(settings.data.is_none());

    let mut settings = setup_interprets_dependencies_read_to_string_error(
        std::io::ErrorKind::AddrInUse);
    assert_eq!(settings.load(),Err(SettingsLoadError::FileNotReadable));
    assert!(settings.data.is_none());

    let mut settings = setup_interprets_dependencies_read_to_string_error(
        std::io::ErrorKind::ConnectionAborted);
    assert_eq!(settings.load(),Err(SettingsLoadError::FileNotReadable));
    assert!(settings.data.is_none());

    let mut settings = setup_interprets_dependencies_read_to_string_error(
        std::io::ErrorKind::NotConnected);
    assert_eq!(settings.load(),Err(SettingsLoadError::FileNotReadable));
    assert!(settings.data.is_none());

    let mut settings = setup_interprets_dependencies_read_to_string_error(
        std::io::ErrorKind::AlreadyExists);
    assert_eq!(settings.load(),Err(SettingsLoadError::FileNotReadable));
    assert!(settings.data.is_none());

    let mut settings = setup_interprets_dependencies_read_to_string_error(
        std::io::ErrorKind::InvalidData);
    assert_eq!(settings.load(),Err(SettingsLoadError::FileNotReadable));
    assert!(settings.data.is_none());

    let mut settings = setup_interprets_dependencies_read_to_string_error(
        std::io::ErrorKind::TimedOut);
    assert_eq!(settings.load(),Err(SettingsLoadError::FileNotReadable));
    assert!(settings.data.is_none());
}

// #[test]
// fn interprets_dependencies_from_str_error() {
//     let mut mock_app_data_directory_manager = MockAppDataDirectoryManagerTrait::new();
//     let mut mock_deps = MockSettingsDependencies::new();
//
//     mock_app_data_directory_manager
//         .expect_get_app_data_directory()
//         .times(1)
//         .return_const(Ok(Path::new("an/example/path").to_path_buf()));
//
//     mock_deps
//         .expect_read_to_string()
//         .times(1)
//         .returning(|_|Ok("an example of the raw toml code".to_string()));
//
//     mock_deps
//         .expect_from_str()
//         .with(eq("an example of the raw toml code".to_string()))
//         .times(1)
//         .return_const(Err(toml::de::Error::message("something went wrong")));
//
//     let mut settings = Settings {
//         data: None,
//         dependencies: Box::new(mock_deps),
//         app_data_directory_manager: Box::new(mock_app_data_directory_manager),
//     };
//
//     assert_eq!(settings.load(),
//         Err(SettingsLoadError::FileNotParsable("something went wrong".to_string())));
//     assert!(settings.data.is_none());
// }

#[test]
fn interprets_validate_and_clone_and_set_layer_pointers_error() {
    let mut mock_app_data_directory_manager = MockAppDataDirectoryManagerTrait::new();
    let mut mock_deps = MockSettingsDependencies::new();

    mock_app_data_directory_manager
        .expect_get_app_data_directory()
        .times(1)
        .return_const(Ok(Path::new("an/example/path").to_path_buf()));

    mock_deps
        .expect_read_to_string()
        .times(1)
        .returning(|_|Ok("an example of the raw toml code".to_string()));

    mock_deps
        .expect_from_str()
        .with(eq("an example of the raw toml code".to_string()))
        .times(1)
        .return_const(Ok(setup_settings_data_example_with_error()));

    let mut settings = Settings {
        data: None,
        dependencies: Box::new(mock_deps),
        app_data_directory_manager: Box::new(mock_app_data_directory_manager),
    };

    assert_eq!(settings.load(),
        Err(SettingsLoadError::FileNotParsable(format!("{}\n{}\n{}\n{}\n{}",
            "Error in",
            "> profiles: \"My PS3 Controller\"",
            "   > stick_switches_click_thresholds",
            "      > left_stick_left",
            "value (1.5) is higher than the maximum acceptable 1.0",))));

    assert!(settings.data.is_none());
}

