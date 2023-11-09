use crate::{settings::{Settings, MockSettingsDependencies, tests::settings_data_example::setup_settings_data_example}, app_data_directory_manager::MockAppDataDirectoryManagerTrait};

mod settings_data_example;

mod fn_load;

#[test]
fn get_data_works() {
    let mock_app_data_directory_manager = MockAppDataDirectoryManagerTrait::new();
    let mock_deps = MockSettingsDependencies::new();

    let settings = Settings {
        data: None,
        dependencies: Box::new(mock_deps),
        app_data_directory_manager: Box::new(mock_app_data_directory_manager),
    };

    assert!(settings.get_data().is_none());


    let mock_app_data_directory_manager = MockAppDataDirectoryManagerTrait::new();
    let mock_deps = MockSettingsDependencies::new();

    let settings = Settings {
        data: Some(setup_settings_data_example()),
        dependencies: Box::new(mock_deps),
        app_data_directory_manager: Box::new(mock_app_data_directory_manager),
    };

    assert_eq!(settings.get_data().unwrap(),setup_settings_data_example());
}

