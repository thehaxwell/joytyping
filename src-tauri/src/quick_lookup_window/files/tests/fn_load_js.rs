use std::path::Path;

use mockall::predicate::eq;

use crate::{app_data_directory_manager::MockAppDataDirectoryManagerTrait, quick_lookup_window::files::{MockFilesDependencies, Files, FilesTrait, StartupScriptLoadError}};


#[test]
fn works() {
    let mut mock_app_data_directory_manager = MockAppDataDirectoryManagerTrait::new();
    let mut mock_deps = MockFilesDependencies::new();

    mock_app_data_directory_manager
        .expect_get_app_data_directory()
        .times(1)
        .return_const(Ok(Path::new("an/example/path").to_path_buf()));

    mock_deps
        .expect_read_to_string()
        .with(eq(Path::new("an/example/path/build/bundle.js").to_path_buf()))
        .times(1)
        .returning(|_|Ok("js code placeholder".to_string()));

    let files = Files {
        dependencies: Box::new(mock_deps),
        app_data_directory_manager: Box::new(mock_app_data_directory_manager),
    };

    assert_eq!(
        files.load_js("build/bundle.js".to_string()).unwrap(),
        "js code placeholder".to_string());
}

#[test]
fn interprets_app_data_directory_manager_error() {
    let mut mock_app_data_directory_manager = MockAppDataDirectoryManagerTrait::new();
    let mock_deps = MockFilesDependencies::new();

    mock_app_data_directory_manager
        .expect_get_app_data_directory()
        .times(1)
        .return_const(Err(()));

    let files = Files {
        dependencies: Box::new(mock_deps),
        app_data_directory_manager: Box::new(mock_app_data_directory_manager),
    };

    assert_eq!(files.load_js("build/bundle.js".to_string()),
        Err(StartupScriptLoadError::FileNotFound("js_iife_bundle_file_path".to_string())));
}


fn setup_interprets_dependencies_read_to_string_error(
    err_kind: std::io::ErrorKind) -> Files {
    let mut mock_app_data_directory_manager = MockAppDataDirectoryManagerTrait::new();
    let mut mock_deps = MockFilesDependencies::new();

    mock_app_data_directory_manager
        .expect_get_app_data_directory()
        .times(1)
        .return_const(Ok(Path::new("an/example/path").to_path_buf()));

    mock_deps
        .expect_read_to_string()
        .times(1)
        .returning(move |_|Err(std::io::Error::new(err_kind,"Oh no!")));

    Files {
        dependencies: Box::new(mock_deps),
        app_data_directory_manager: Box::new(mock_app_data_directory_manager),
    }
}

#[test]
fn interprets_dependencies_read_to_string_error() {
    let files = setup_interprets_dependencies_read_to_string_error(
        std::io::ErrorKind::NotFound);
    assert_eq!(files.load_js("build/bundle.js".to_string()),
		Err(StartupScriptLoadError::FileNotFound("js_iife_bundle_file_path".to_string())));

    let files = setup_interprets_dependencies_read_to_string_error(
        std::io::ErrorKind::PermissionDenied);
    assert_eq!(files.load_js("build/bundle.js".to_string()),
		Err(StartupScriptLoadError::PermissionDenied("js_iife_bundle_file_path".to_string())));

    // the rest all give FileNotReadable. We'll just test a few
    let files = setup_interprets_dependencies_read_to_string_error(
        std::io::ErrorKind::ConnectionReset);
    assert_eq!(files.load_js("build/bundle.js".to_string()),
		Err(StartupScriptLoadError::FileNotReadable("js_iife_bundle_file_path".to_string())));

    let files = setup_interprets_dependencies_read_to_string_error(
        std::io::ErrorKind::BrokenPipe);
    assert_eq!(files.load_js("build/bundle.js".to_string()),
		Err(StartupScriptLoadError::FileNotReadable("js_iife_bundle_file_path".to_string())));

    let files = setup_interprets_dependencies_read_to_string_error(
        std::io::ErrorKind::AddrInUse);
    assert_eq!(files.load_js("build/bundle.js".to_string()),
		Err(StartupScriptLoadError::FileNotReadable("js_iife_bundle_file_path".to_string())));

    let files = setup_interprets_dependencies_read_to_string_error(
        std::io::ErrorKind::ConnectionAborted);
    assert_eq!(files.load_js("build/bundle.js".to_string()),
		Err(StartupScriptLoadError::FileNotReadable("js_iife_bundle_file_path".to_string())));

    let files = setup_interprets_dependencies_read_to_string_error(
        std::io::ErrorKind::NotConnected);
    assert_eq!(files.load_js("build/bundle.js".to_string()),
		Err(StartupScriptLoadError::FileNotReadable("js_iife_bundle_file_path".to_string())));

    let files = setup_interprets_dependencies_read_to_string_error(
        std::io::ErrorKind::AlreadyExists);
    assert_eq!(files.load_js("build/bundle.js".to_string()),
		Err(StartupScriptLoadError::FileNotReadable("js_iife_bundle_file_path".to_string())));

    let files = setup_interprets_dependencies_read_to_string_error(
        std::io::ErrorKind::InvalidData);
    assert_eq!(files.load_js("build/bundle.js".to_string()),
		Err(StartupScriptLoadError::FileNotReadable("js_iife_bundle_file_path".to_string())));

    let files = setup_interprets_dependencies_read_to_string_error(
        std::io::ErrorKind::TimedOut);
    assert_eq!(files.load_js("build/bundle.js".to_string()),
		Err(StartupScriptLoadError::FileNotReadable("js_iife_bundle_file_path".to_string())));
}
