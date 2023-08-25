// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::SystemTray;
use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem, SystemTrayEvent};



use joytyping::joy_keyboard::input_controller::InputController;
use joytyping::joy_keyboard::keys_config::KeysConfig;
use joytyping::settings::{Settings,SettingsLoadError,SettingsDependenciesImpl};
use tauri::Manager;
use joytyping::joy_keyboard::stepper::StepperButton;
use joytyping::{run, LeftOrRight};
use joytyping::gamepad::gilrs_wrapper::GilrsWrapper;
use joytyping::gamepad::sticks_interpreter::{SticksInterpreter, AxisClickThresholds};
use joytyping::joy_keyboard::input_controller::enigo_wrapper::EnigoWrapper;
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
    // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let reload = CustomMenuItem::new("reload".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
      .add_item(quit)
      .add_native_item(SystemTrayMenuItem::Separator)
      .add_item(reload);

    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|_app, event| match event {
          SystemTrayEvent::MenuItemClick { id, .. } => {
            match id.as_str() {
              "quit" => {
                // std::process::exit(0);
                println!("Quitting...");
              }
              "reload" => {
                // let window = app.get_window("main").unwrap();
                // window.hide().unwrap();
                println!("Reloading...");
              }
              _ => {}
            }
          }
          _ => {}
        })
        .invoke_handler(tauri::generate_handler![greet,open_quick_lookup_window,close_quick_lookup_window])
        .setup(|app| {
            let handle = app.app_handle();

            std::thread::spawn( move || {
                let mut settings = Settings::new(Box::new(SettingsDependenciesImpl),
                    "/home/haxwell/.config/joytyping/joytyping.toml".to_string());
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

                let active_profile_index_option = settings_data.profiles.iter()
                    .position(|profile| profile.name == settings_data.global.default_profile);
                        
                let active_profile = settings_data.profiles.remove(match active_profile_index_option {
                    Some(idx) => idx,
                    None => 0
                });

                let gamepad = joytyping::gamepad::Gamepad::new(
                    Box::new(GilrsWrapper::new()),
                    Box::new(SticksInterpreter::new(
                    AxisClickThresholds::get_from_setting(
                        active_profile.left_stick.click_thresholds,
                        LeftOrRight::Left),
                    AxisClickThresholds::get_from_setting(
                        active_profile.right_stick.click_thresholds,
                        LeftOrRight::Right),
                        
                    )),
                );
                let joy_keyboard = joytyping::joy_keyboard::JoyKeyboard::new(
                    Box::new(InputController::new(Box::new(EnigoWrapper::new()))),
                    Box::new(StepperButton::new()),
                    Box::new(StepperButton::new()),
                    KeysConfig::from(
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
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
              // the app must run in the background
              api.prevent_exit();
            }
            _ => {}
        });




}

