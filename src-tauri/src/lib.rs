// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::Manager;
pub mod audio;
pub mod commands;
pub mod config;
pub mod icon_gen;
mod tray;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec!["--hidden"]),
        ))
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, shortcut, event| {
                    if event.state() == tauri_plugin_global_shortcut::ShortcutState::Pressed {
                        let matched = {
                            let state =
                                app.state::<std::sync::Mutex<crate::config::ConfigManager>>();
                            let manager = state.lock().unwrap();
                            let config = manager.load();
                            if let Some(hotkey_str) = &config.hotkey {
                                use tauri_plugin_global_shortcut::Shortcut;
                                if let Ok(config_shortcut) = hotkey_str.parse::<Shortcut>() {
                                    config_shortcut == *shortcut
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        };

                        if matched {
                            let _ = crate::audio::cycle_next_device(app);
                        }
                    }
                })
                .build(),
        )
        .setup(|app| {
            let config_manager = config::ConfigManager::new(app.handle());
            let config = config_manager.load();
            app.manage(std::sync::Mutex::new(config_manager));

            // Register initial hotkey
            if let Some(hotkey_str) = &config.hotkey {
                use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};
                if let Ok(shortcut) = hotkey_str.parse::<Shortcut>() {
                    let _ = app.global_shortcut().register(shortcut);
                }
            }

            #[cfg(desktop)]
            {
                tray::create_tray(app.handle())?;
                let _ = crate::audio::update_tray_icon(app.handle());

                // Handle autostart hidden flag
                if std::env::args().any(|arg| arg == "--hidden") {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.hide();
                    }
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_audio_devices,
            commands::set_active_device,
            commands::get_config,
            commands::set_excluded_devices,
            commands::save_config
        ])
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                window.hide().unwrap();
                api.prevent_close();
            }
            tauri::WindowEvent::Focused(false) => {
                // Check if minimized and hide if so
                if window.is_minimized().unwrap_or(false) {
                    window.hide().unwrap();
                }
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
