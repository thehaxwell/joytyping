use std::sync::mpsc;

use settings::error_display_window::ErrorDisplayWindow;
use settings::{Settings,SettingsDependenciesImpl};

pub mod settings;
pub mod settings_data;

pub fn start_main_loop(
    end_signal_mpsc_receiver: mpsc::Receiver<MainLoopInterruption>,
    handle: tauri::AppHandle
    ){
    let mut settings_error_display_window = ErrorDisplayWindow::new(handle);

    'main_loop_initializer_loop: loop {
        // close any open window first while there's time
        let _ = settings_error_display_window.close();

        let mut settings = Settings::new(
            Box::new(SettingsDependenciesImpl),
            "/home/haxwell/.config/joytyping/joytyping1.toml".to_string());

        if let Err(e) = settings.load() {
           let _ = settings_error_display_window.open_and_show(e);
        }

        let mut settings_data = settings.get_data().unwrap();

        let active_profile_index_option = settings_data.profiles.iter()
            .position(|profile| profile.name == settings_data.global.default_profile);
                
        let active_profile = settings_data.profiles.remove(match active_profile_index_option {
            Some(idx) => idx,
            None => 0
        });

        'main_loop: loop {
            //TODO: check if this actually eases the load on the CPU
            std::thread::sleep(std::time::Duration::from_millis(10));
            match end_signal_mpsc_receiver.try_recv() {
                Ok(MainLoopInterruption::ReInitiailze) => {
                    println!("Restarting");
                    break 'main_loop;
                }
                Ok(MainLoopInterruption::Terminate) | Err(mpsc::TryRecvError::Disconnected) => {
                    println!("Terminating.");
                    break 'main_loop_initializer_loop;
                }
                Err(mpsc::TryRecvError::Empty) => {}
            }

        }
    }
}

#[derive(Debug,PartialEq,Clone)]
pub enum LeftOrRight {
    Left,
    Right,
}

#[derive(PartialEq,Clone)]
pub enum MainLoopInterruption {
    Terminate,
    ReInitiailze,
}
