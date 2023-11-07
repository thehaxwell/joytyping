// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// use std::sync::mpsc;

use tauri::SystemTray;
use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem, SystemTrayEvent};

use tauri::Manager;
use joytyping::start_main_loop;

#[tauri::command]
fn start_main_loop_command(handle: tauri::AppHandle) {
    std::thread::spawn( move || {
        start_main_loop(handle);
    });
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
        .invoke_handler(tauri::generate_handler![start_main_loop_command])
        .system_tray(system_tray)
        .on_system_tray_event(move |app_handle, event| match event {
          SystemTrayEvent::MenuItemClick { id, .. } => {
            match id.as_str() {
              "quit" => {
                // end the main loop thread(s)
                app_handle.trigger_global("main-loop-interruption",Some("MainLoopInterruption::Terminate".to_string()));
                // end this thread
                // TODO: figure out how to make the 
                // following execute AFTER start_main_loop() returns
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

            std::thread::spawn( move || {
                start_main_loop(handle);
            });

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

