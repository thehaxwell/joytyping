mod settings_data_example;

mod fn_load;


// #[test]
// fn load_works() {
//     let settings_file_path = "/home/haxwell/.config/joytyping/joytyping.toml";
//     let mut mock_deps = MockSettingsDependencies::new();
//     mock_deps
//         .expect_read_to_string()
//         .with(eq(settings_file_path))
//         .returning(|_| Ok(String::from("settings_string_example")));
//     mock_deps
//         .expect_from_str()
//         .with(eq("settings_string_example"))
//         .returning(|_| Ok(setup_settings_data_example()));
//
//     let mut settings = Settings::new(Box::new(mock_deps),settings_file_path.to_string());
//
//     assert!(settings.load().is_ok());
//     assert_eq!(settings.data.unwrap(), setup_settings_data_example());
// }
//
// fn setup_load_returns_correct_errors_from_read_to_string(io_error: std::io::Error, expected_error: SettingsLoadError) {
//     let settings_file_path = "/home/haxwell/.config/joytyping/joytyping.toml";
//     let mut mock_deps = MockSettingsDependencies::new();
//     mock_deps
//         .expect_read_to_string()
//         .with(eq(settings_file_path))
//         .return_once(|_| Err(io_error));
//
//     // this will be called from Settings::load_default()
//     mock_deps
//         .expect_from_str()
//         .with(eq(JOYTYPING_DEFAULT_SETTINGS))
//         .returning(|_| Ok(setup_settings_data_example()));
//
//     let mut settings = Settings::new(Box::new(mock_deps),settings_file_path.to_string());
//
//     assert!(settings.load().is_err_and(|e| e == expected_error));
//     assert_eq!(settings.data.unwrap(), setup_settings_data_example());
// }
//
// #[test]
// fn load_returns_correct_errors_from_read_to_string() {
//     setup_load_returns_correct_errors_from_read_to_string(
//         std::io::Error::new(std::io::ErrorKind::NotFound,"file not found"),
//         SettingsLoadError::FileNotFound,
//     );
//     setup_load_returns_correct_errors_from_read_to_string(
//         std::io::Error::new(std::io::ErrorKind::PermissionDenied,"this text doesn't matter"),
//         SettingsLoadError::PermissionDenied,
//     );
//     setup_load_returns_correct_errors_from_read_to_string(
//         std::io::Error::new(std::io::ErrorKind::TimedOut,"this text also doesn't matter"),
//         SettingsLoadError::FileNotReadable,
//     );
//     setup_load_returns_correct_errors_from_read_to_string(
//         std::io::Error::new(std::io::ErrorKind::Interrupted,"even this text matters not"),
//         SettingsLoadError::FileNotReadable,
//     );
// }
//
