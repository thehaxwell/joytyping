// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use joytyping::joy_input::stepper::StepperButton;
use joytyping::run;
use joytyping::gamepad::gilrs_wrapper::GilrsWrapper;
use joytyping::gamepad::sticks_interpreter::SticksInterpreter;
use joytyping::joy_input::enigo_wrapper::EnigoWrapper;
use joytyping::quick_lookup_window::QuickLookupWindow;

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
        .setup(|app| {
            let handle = app.app_handle();

            std::thread::spawn( move || {
                let gamepad = joytyping::gamepad::Gamepad::new(
                    Box::new(GilrsWrapper::new()),
                    Box::new(SticksInterpreter::new()),
                );
                let joy_keyboard = joytyping::joy_input::JoyKeyboard::new(
                    Box::new(EnigoWrapper::new()),
                    Box::new(StepperButton::new()),
                    Box::new(StepperButton::new()),
                );
                run(gamepad,joy_keyboard,QuickLookupWindow::new(handle));
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}

