use mockall::predicate::*;
use settings_data_example::setup_settings_data_example;
use crate::settings::{MockSettingsDependencies, Settings, SettingsLoadError, JOYTYPING_DEFAULT_SETTINGS};

mod settings_data_example;

#[test]
fn load_works() {
    let settings_file_path = "/home/haxwell/.config/joytyping/joytyping.toml";
    let mut mock_deps = MockSettingsDependencies::new();
    mock_deps
        .expect_read_to_string()
        .with(eq(settings_file_path))
        .returning(|_| Ok(String::from("settings_string_example")));
    mock_deps
        .expect_from_str()
        .with(eq("settings_string_example"))
        .returning(|_| Ok(setup_settings_data_example()));

    let mut settings = Settings::new(Box::new(mock_deps),settings_file_path.to_string());

    assert!(settings.load().is_ok());
    assert_eq!(settings.data.unwrap(), setup_settings_data_example());
}

fn setup_load_returns_correct_errors_from_read_to_string(io_error: std::io::Error, expected_error: SettingsLoadError) {
    let settings_file_path = "/home/haxwell/.config/joytyping/joytyping.toml";
    let mut mock_deps = MockSettingsDependencies::new();
    mock_deps
        .expect_read_to_string()
        .with(eq(settings_file_path))
        .return_once(|_| Err(io_error));

    // this will be called from Settings::load_default()
    mock_deps
        .expect_from_str()
        .with(eq(JOYTYPING_DEFAULT_SETTINGS))
        .returning(|_| Ok(setup_settings_data_example()));

    let mut settings = Settings::new(Box::new(mock_deps),settings_file_path.to_string());

    assert!(settings.load().is_err_and(|e| e == expected_error));
    assert_eq!(settings.data.unwrap(), setup_settings_data_example());
}

#[test]
fn load_returns_correct_errors_from_read_to_string() {
    setup_load_returns_correct_errors_from_read_to_string(
        std::io::Error::new(std::io::ErrorKind::NotFound,"file not found"),
        SettingsLoadError::FileNotFound,
    );
    setup_load_returns_correct_errors_from_read_to_string(
        std::io::Error::new(std::io::ErrorKind::PermissionDenied,"this text doesn't matter"),
        SettingsLoadError::PermissionDenied,
    );
    setup_load_returns_correct_errors_from_read_to_string(
        std::io::Error::new(std::io::ErrorKind::TimedOut,"this text also doesn't matter"),
        SettingsLoadError::FileNotReadable,
    );
    setup_load_returns_correct_errors_from_read_to_string(
        std::io::Error::new(std::io::ErrorKind::Interrupted,"even this text matters not"),
        SettingsLoadError::FileNotReadable,
    );
}

#[test]
fn load_returns_correct_errors_from_from_str() {
    // I can't test this because I can't initialize
    // a toml::de::Error TODO: fix this
    // fn from_str(&self, s: &str) -> Result<SettingsData, toml::de::Error> {
}

// -----------
// validate_settings_data
// -----------
#[test]
fn validate_settings_data_works_with_valid_data() {
    assert!(Settings::validate_settings_data(&setup_settings_data_example()).is_ok());
    // TODO: test different cases
}

#[test]
fn validate_settings_data_finds_stick_threshold_errors() {
    let mut settings_data = setup_settings_data_example();
    settings_data.profiles[0].stick_switches_click_thresholds.left_stick_up = 1.1;
    assert!(Settings::validate_settings_data(&settings_data)
        .is_err_and(|e| e == SettingsLoadError::FileNotParsable(
            "at \"My PS3 Controller\" profile > stick_switches_click_thresholds: left_stick_up is out of bounds"
            .to_string()),

    ));


    let mut settings_data = setup_settings_data_example();
    settings_data.profiles[0].stick_switches_click_thresholds.left_stick_left = -0.1;
    assert!(Settings::validate_settings_data(&settings_data)
        .is_err_and(|e| e == SettingsLoadError::FileNotParsable(
            "at \"My PS3 Controller\" profile > stick_switches_click_thresholds: left_stick_left is out of bounds"
            .to_string()),
    ));

    let mut settings_data = setup_settings_data_example();
    settings_data.profiles[0].stick_switches_click_thresholds.right_stick_down = -1.1;
    assert!(Settings::validate_settings_data(&settings_data)
        .is_err_and(|e| e == SettingsLoadError::FileNotParsable(
            "at \"My PS3 Controller\" profile > stick_switches_click_thresholds: right_stick_down is out of bounds"
            .to_string()),
    ));
}
