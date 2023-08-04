// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn open_quick_lookup_window(handle: tauri::AppHandle) {
  tauri::WindowBuilder::new(
    &handle,
    "external", /* the unique window label */
            tauri::WindowUrl::App("#/quick-lookup".into())

  ).build().unwrap();
}

#[tauri::command]
async fn close_quick_lookup_window(handle: tauri::AppHandle) {
  let docs_window = handle.get_window("external").unwrap();
    docs_window.close().unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,open_quick_lookup_window,close_quick_lookup_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

