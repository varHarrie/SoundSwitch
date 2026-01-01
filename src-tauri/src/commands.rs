use crate::audio::{device, policy};
use tauri::command;

#[command]
pub fn get_audio_devices() -> Result<Vec<device::AudioDevice>, String> {
    device::enumerate_devices()
}

#[command]
pub fn set_active_device(app_handle: tauri::AppHandle, id: String) -> Result<(), String> {
    policy::set_default_device(&id)?;
    let _ = crate::audio::update_tray_icon(&app_handle);
    Ok(())
}

#[command]
pub fn get_config(
    state: tauri::State<'_, std::sync::Mutex<crate::config::ConfigManager>>,
) -> Result<crate::config::Config, String> {
    let manager = state
        .lock()
        .map_err(|_| "Failed to lock config manager".to_string())?;
    Ok(manager.load())
}

#[command]
pub fn set_excluded_devices(
    app_handle: tauri::AppHandle,
    ids: Vec<String>,
    state: tauri::State<'_, std::sync::Mutex<crate::config::ConfigManager>>,
) -> Result<(), String> {
    {
        let manager = state
            .lock()
            .map_err(|_| "Failed to lock config manager".to_string())?;
        let mut config = manager.load();
        config.excluded_device_ids = ids;
        manager.save(&config)?;
    }

    let _ = crate::audio::update_tray_icon(&app_handle);
    Ok(())
}
#[command]
pub fn save_config(
    app_handle: tauri::AppHandle,
    config: crate::config::Config,
    state: tauri::State<'_, std::sync::Mutex<crate::config::ConfigManager>>,
) -> Result<(), String> {
    {
        let manager = state
            .lock()
            .map_err(|_| "Failed to lock config manager".to_string())?;

        manager.save(&config)?;
    }

    // Re-register hotkey
    use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};
    let sc_manager = app_handle.global_shortcut();
    let _ = sc_manager.unregister_all();
    if let Some(hotkey_str) = &config.hotkey {
        match hotkey_str.parse::<Shortcut>() {
            Ok(shortcut) => {
                sc_manager.register(shortcut)
                    .map_err(|e| format!("Failed to register shortcut '{}': {}. It might be already in use by another application.", hotkey_str, e))?;
            }
            Err(e) => return Err(format!("Invalid shortcut format '{}': {}", hotkey_str, e)),
        }
    }

    let _ = crate::audio::update_tray_icon(&app_handle);
    Ok(())
}
