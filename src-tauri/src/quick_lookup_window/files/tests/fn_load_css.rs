use std::path::Path;

use crate::{quick_lookup_window::files::{MockFilesDependencies, Files, FilesTrait, StartupScriptLoadError}, app_data_directory_manager::MockAppDataDirectoryManagerTrait};

#[test]
fn works() {
    let mut mock_files_dependencies = MockFilesDependencies::new();
    let mut mock_app_data_directory_manager = MockAppDataDirectoryManagerTrait::new();
    
    mock_app_data_directory_manager
        .expect_get_app_data_directory()
        .times(1)
        .return_const(Ok(Path::new("path/to/css").to_path_buf()));

    mock_files_dependencies
        .expect_read_to_string()
        .times(1)
        .returning(|_|Ok("css code placeholder".to_string()));


    let files = Files {
       dependencies: Box::new(mock_files_dependencies),
       app_data_directory_manager: Box::new(mock_app_data_directory_manager),
    };
    
    assert_eq!(
        files.load_css(Some("path/to/css".to_string())).unwrap().unwrap(),"\
        var styleSheet = document.createElement(\"style\");\n\
        styleSheet.innerText = `css code placeholder`;\n\
        document.head.appendChild(styleSheet);");
}

#[test]
fn interprets_app_data_directory_manager_error() {
    let mut mock_app_data_directory_manager = MockAppDataDirectoryManagerTrait::new();
    let mock_files_dependencies = MockFilesDependencies::new();

    mock_app_data_directory_manager
        .expect_get_app_data_directory()
        .times(1)
        .return_const(Err(()));

    let files = Files {
       dependencies: Box::new(mock_files_dependencies),
       app_data_directory_manager: Box::new(mock_app_data_directory_manager),
    };

    assert_eq!(files.load_css(Some("path/to/css".to_string())).unwrap_err(),
        StartupScriptLoadError::FileNotFound("css_file_path".to_string()));
}


fn setup_interprets_dependencies_read_to_string_error(
    err_kind: std::io::ErrorKind) -> Files {
    let mut mock_app_data_directory_manager = MockAppDataDirectoryManagerTrait::new();
    let mut mock_files_dependencies = MockFilesDependencies::new();

    mock_app_data_directory_manager
        .expect_get_app_data_directory()
        .times(1)
        .return_const(Ok(Path::new("an/example/path").to_path_buf()));

    mock_files_dependencies
        .expect_read_to_string()
        .times(1)
        .returning(move |_|Err(std::io::Error::new(err_kind,"Oh no!")));

    Files {
       dependencies: Box::new(mock_files_dependencies),
       app_data_directory_manager: Box::new(mock_app_data_directory_manager),
    }
}

#[test]
fn interprets_dependencies_read_to_string_error() {
    let settings = setup_interprets_dependencies_read_to_string_error(
        std::io::ErrorKind::NotFound);
    assert_eq!(settings.load_css(Some("path/to/css".to_string())),
		Err(StartupScriptLoadError::FileNotFound("css_file_path".to_string())));

    let settings = setup_interprets_dependencies_read_to_string_error(
        std::io::ErrorKind::PermissionDenied);
    assert_eq!(settings.load_css(Some("path/to/css".to_string())),
		Err(StartupScriptLoadError::PermissionDenied("css_file_path".to_string())));

    // the rest all give FileNotReadable. We'll just test a few
    let settings = setup_interprets_dependencies_read_to_string_error(
        std::io::ErrorKind::ConnectionReset);
    assert_eq!(settings.load_css(Some("path/to/css".to_string())),
		Err(StartupScriptLoadError::FileNotReadable("css_file_path".to_string())));

    let settings = setup_interprets_dependencies_read_to_string_error(
        std::io::ErrorKind::BrokenPipe);
    assert_eq!(settings.load_css(Some("path/to/css".to_string())),
		Err(StartupScriptLoadError::FileNotReadable("css_file_path".to_string())));

    let settings = setup_interprets_dependencies_read_to_string_error(
        std::io::ErrorKind::AddrInUse);
    assert_eq!(settings.load_css(Some("path/to/css".to_string())),
		Err(StartupScriptLoadError::FileNotReadable("css_file_path".to_string())));

    let settings = setup_interprets_dependencies_read_to_string_error(
        std::io::ErrorKind::ConnectionAborted);
    assert_eq!(settings.load_css(Some("path/to/css".to_string())),
		Err(StartupScriptLoadError::FileNotReadable("css_file_path".to_string())));

    let settings = setup_interprets_dependencies_read_to_string_error(
        std::io::ErrorKind::NotConnected);
    assert_eq!(settings.load_css(Some("path/to/css".to_string())),
		Err(StartupScriptLoadError::FileNotReadable("css_file_path".to_string())));

    let settings = setup_interprets_dependencies_read_to_string_error(
        std::io::ErrorKind::AlreadyExists);
    assert_eq!(settings.load_css(Some("path/to/css".to_string())),
		Err(StartupScriptLoadError::FileNotReadable("css_file_path".to_string())));

    let settings = setup_interprets_dependencies_read_to_string_error(
        std::io::ErrorKind::InvalidData);
    assert_eq!(settings.load_css(Some("path/to/css".to_string())),
		Err(StartupScriptLoadError::FileNotReadable("css_file_path".to_string())));

    let settings = setup_interprets_dependencies_read_to_string_error(
        std::io::ErrorKind::TimedOut);
    assert_eq!(settings.load_css(Some("path/to/css".to_string())),
		Err(StartupScriptLoadError::FileNotReadable("css_file_path".to_string())));
}
