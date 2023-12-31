// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// use std::sync::mpsc;

use joytyping::app_data_directory_manager::{AppDataDirectoryManager, AppDataDirectoryDependenciesImpl};
use joytyping::file_wrapper::FileWrapper;
use joytyping::settings::files_initializer::FilesInitializer;
use joytyping::zip_downloader::ZipDownloader;
use joytyping::zip_downloader::reqwest_wrapper::ReqwestWrapper;
use joytyping::zip_downloader::zip_archive_wrapper::ZipArchiveWrapper;
use tauri::SystemTray;
use tauri::api::notification::Notification;
use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem, SystemTrayEvent};

use tauri::Manager;
use joytyping::start_main_loop;

#[tauri::command]
fn start_main_loop_command(app_handle: tauri::AppHandle) {
    app_handle.trigger_global("main-loop-interruption",Some("MainLoopInterruption::ReInitiailze".to_string()))
}

#[tauri::command]
fn setup_settings_command(app_handle: tauri::AppHandle) {
    println!("downloading layout...");
    //TODO: return the result of initialize back to the caller(window) of this command
    FilesInitializer::new(
        Box::new(ZipDownloader::new(
            Box::new(ZipArchiveWrapper),
            Box::new(ReqwestWrapper),
            Box::new(FileWrapper),)
        ),
        Box::new(FileWrapper),
        Box::new(AppDataDirectoryManager::new(
            Box::new(AppDataDirectoryDependenciesImpl))),
    ).initialize().unwrap();

    app_handle.trigger_global("main-loop-interruption",Some("MainLoopInterruption::ReInitiailze".to_string()))
}

fn main() {
    let reload = CustomMenuItem::new("reload".to_string(), "Reload Settings");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new()
      .add_item(reload)
      .add_native_item(SystemTrayMenuItem::Separator)
      .add_item(quit);

    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_main_loop_command,setup_settings_command])
        .system_tray(system_tray)
        .on_system_tray_event(move |app_handle, event| match event {
          SystemTrayEvent::MenuItemClick { id, .. } => {
            match id.as_str() {
              "quit" => {
                // end the main loop thread(s)
                app_handle.trigger_global("main-loop-interruption",Some("MainLoopInterruption::Terminate".to_string()));
                // sleep for .5secs to let start_main_loop() return
                std::thread::sleep(std::time::Duration::from_millis(500));
                // end this thread
                std::process::exit(0); 
              }
              "reload" => {
                app_handle.trigger_global("main-loop-interruption",Some("MainLoopInterruption::ReInitiailze".to_string()))
              }
              _ => {}
            }
          }
          _ => {}
        })
        .setup(|app| {
            let handle = app.app_handle();
            let id = app.config().tauri.bundle.identifier.clone();
            std::thread::spawn( move || {
                start_main_loop(handle,id);
            });
            let _ = Notification::new(app.config().tauri.bundle.identifier.clone())
                .title("Joytyping is running")
                .body(format!("Joytyping is running in the background. {}",access_taskbar_options_instruction()))
                .show();

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
              // the app must keep running in the background
              // when no windows are open
              api.prevent_exit();
            }
            _ => {}
        });
}


#[cfg(not(target_os="windows"))]
fn access_taskbar_options_instruction() -> String {
    "Click on the taskbar item to access options.".to_string()
}

#[cfg(target_os="windows")]
fn access_taskbar_options_instruction() -> String {
    "Right-click on the taskbar item to access options.".to_string()
}
