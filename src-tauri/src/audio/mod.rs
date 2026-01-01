use tauri::{Emitter, Manager};

pub mod device;
pub mod policy;

pub fn cycle_next_device(app_handle: &tauri::AppHandle) -> Result<String, String> {
    let devices = device::enumerate_devices()?;
    if devices.is_empty() {
        return Err("No audio devices found".to_string());
    }

    // Load config
    let state = app_handle.state::<std::sync::Mutex<crate::config::ConfigManager>>();
    let manager = state
        .lock()
        .map_err(|_| "Failed to lock config manager".to_string())?;
    // Reload config to ensure we have latest exclude list
    let config = manager.load();

    let included_devices: Vec<&device::AudioDevice> = devices
        .iter()
        .filter(|d| !config.excluded_device_ids.contains(&d.id))
        .collect();

    if included_devices.is_empty() {
        // Fallback to all devices if filtering removed everything or allowed nothing
        return cycle_next_device_internal(&devices);
    }

    let current_index = included_devices.iter().position(|d| d.is_default);

    let next_device = match current_index {
        Some(idx) => included_devices[(idx + 1) % included_devices.len()],
        None => included_devices[0],
    };

    policy::set_default_device(&next_device.id)?;

    // Update Tray Icon
    // Calculate 1-based index for the NEXT device in the included list
    let next_index_in_list = match current_index {
        Some(idx) => (idx + 1) % included_devices.len(),
        None => 0,
    };
    // Index + 1 for display
    let display_number = next_index_in_list + 1;

    if let Some(tray) = app_handle.tray_by_id("tray") {
        if let Some(icon) = crate::icon_gen::generate_number_icon(display_number) {
            let _ = tray.set_icon(Some(icon));
        }
    }

    // Emit event to update frontend
    let _ = app_handle.emit("device-changed", ());
    Ok(next_device.name.clone())
}

fn cycle_next_device_internal(devices: &[device::AudioDevice]) -> Result<String, String> {
    let current_index = devices.iter().position(|d| d.is_default);

    let next_index = match current_index {
        Some(idx) => (idx + 1) % devices.len(),
        None => 0,
    };

    let next_device = &devices[next_index];
    policy::set_default_device(&next_device.id)?;

    Ok(next_device.name.clone())
}

pub fn update_tray_icon(app_handle: &tauri::AppHandle) -> Result<(), String> {
    let devices = device::enumerate_devices()?;
    if devices.is_empty() {
        return Ok(());
    }

    // Load config
    let state = app_handle.state::<std::sync::Mutex<crate::config::ConfigManager>>();
    let manager = state
        .lock()
        .map_err(|_| "Failed to lock config manager".to_string())?;
    // Reload config to ensure we have latest exclude list
    let config = manager.load();

    let included_devices: Vec<&device::AudioDevice> = devices
        .iter()
        .filter(|d| !config.excluded_device_ids.contains(&d.id))
        .collect();

    if included_devices.is_empty() {
        return Ok(());
    }

    let current_index = included_devices.iter().position(|d| d.is_default);

    if let Some(idx) = current_index {
        let display_number = idx + 1;
        if let Some(tray) = app_handle.tray_by_id("tray") {
            if let Some(icon) = crate::icon_gen::generate_number_icon(display_number) {
                let _ = tray.set_icon(Some(icon));
            }
        }
    }

    Ok(())
}
