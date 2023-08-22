// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use joytyping::joy_input::joy_keyboard_keys_config::JoyKeyboardKeysConfig;
use joytyping::settings::{Settings,SettingsLoadError,SettingsDependenciesImpl};
use tauri::Manager;
use joytyping::joy_input::stepper::StepperButton;
use joytyping::{run, Alignment};
use joytyping::gamepad::gilrs_wrapper::GilrsWrapper;
use joytyping::gamepad::sticks_interpreter::{SticksInterpreter, AxisClickThresholds};
use joytyping::joy_input::enigo_wrapper::EnigoWrapper;
use joytyping::quick_lookup_window::{QuickLookupWindow, QuickLookupWindowDependenciesImpl};

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
                let mut settings = Settings::new(Box::new(SettingsDependenciesImpl));
                match settings.load() {
                    Err(e) => {
                        match e {
                            SettingsLoadError::FileNotParsable(msg) => {
                                println!("Error: {}", msg);
                            },
                            _ => {
                                println!("Error!");
                            }
                        }
                    },
                    Ok(_) => {
                        println!("Settings loaded");
                    }
                }
                let mut settings_data = settings.get_data().unwrap();

                let gamepad = joytyping::gamepad::Gamepad::new(
                    Box::new(GilrsWrapper::new()),
                    Box::new(SticksInterpreter::new(
                    AxisClickThresholds {
                        up: 0.5,
                        down: 0.5,
                        left: 0.5,
                        right: 0.5,
                        alignment: Alignment::Left,
                    },
                    AxisClickThresholds {
                        up: 0.5,
                        down: 0.5,
                        left: 0.5,
                        right: 0.5,
                        alignment: Alignment::Right,
                    },)),
                );
                let active_profile_index_option = settings_data.profiles.iter()
                    .position(|profile| profile.name == settings_data.global.default_profile);
                        
                let active_profile = settings_data.profiles.remove(match active_profile_index_option {
                    Some(idx) => idx,
                    None => 0
                });
                let joy_keyboard = joytyping::joy_input::JoyKeyboard::new(
                    Box::new(EnigoWrapper::new()),
                    Box::new(StepperButton::new()),
                    Box::new(StepperButton::new()),
                    JoyKeyboardKeysConfig::from(
                        active_profile.keyboard_mode.key_mappings)
                );

                let mut quick_lookup_window = QuickLookupWindow::new(
                    handle,
                    Box::new(QuickLookupWindowDependenciesImpl),
                );

                quick_lookup_window.set_window_settings(active_profile.quick_lookup_window);
                match quick_lookup_window.load_startup_script() {
                    Err(e) => {
                        match e {
                            _ => {
                                println!("Error!");
                            }
                        }
                    },
                    Ok(_) => {
                        println!("quick lookup window external script");
                    }
                }

                run(gamepad,joy_keyboard,quick_lookup_window);
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}

