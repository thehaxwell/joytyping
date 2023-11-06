// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::sync::mpsc;

use tauri::SystemTray;
use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem, SystemTrayEvent};

use tauri::Manager;
use joytyping::{start_main_loop, MainLoopInterruption};

fn main() {
    let (end_signal_mpsc_sender,end_signal_mpsc_receiver) = mpsc::sync_channel(0);
    let end_signal_mpsc_sender_1 = end_signal_mpsc_sender.clone();

    let reload = CustomMenuItem::new("reload".to_string(), "Reload Settings");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new()
      .add_item(reload)
      .add_native_item(SystemTrayMenuItem::Separator)
      .add_item(quit);

    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(move |_app, event| match event {
          SystemTrayEvent::MenuItemClick { id, .. } => {
            match id.as_str() {
              "quit" => {
                if let Ok(_) = end_signal_mpsc_sender_1.send(MainLoopInterruption::Terminate){
                    std::process::exit(0);
                }
                else {
                    panic!("Failed to quit!");
                }
              }
              "reload" => {
                if let Err(e) = end_signal_mpsc_sender_1.send(MainLoopInterruption::ReInitiailze){
                    panic!("Failed to reload settings: {}",e);
                }
              }
              _ => {}
            }
          }
          _ => {}
        })
        .setup(|app| {
            let handle = app.app_handle();

            std::thread::spawn( move || {
                start_main_loop(end_signal_mpsc_receiver, handle);
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

