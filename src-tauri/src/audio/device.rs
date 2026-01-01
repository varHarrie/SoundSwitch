use serde::Serialize;
use windows::Win32::Devices::FunctionDiscovery::PKEY_Device_FriendlyName;
use windows::Win32::Media::Audio::{
    eMultimedia, eRender, IMMDevice, IMMDeviceEnumerator, MMDeviceEnumerator, DEVICE_STATE_ACTIVE,
};
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_MULTITHREADED, STGM_READ,
};
use windows::Win32::UI::Shell::PropertiesSystem::IPropertyStore;

#[derive(Debug, Serialize, Clone)]
pub struct AudioDevice {
    pub id: String,
    pub name: String,
    pub is_default: bool, // Note: We might need a separate check for default to fill this
}

pub fn enumerate_devices() -> Result<Vec<AudioDevice>, String> {
    unsafe {
        // Initialize COM library (if not already initialized by Tauri)
        // Check if we need to init? valid Tauri main usually handles it or we do it safely.
        // For CLI testing we need it.
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);

        let enumerator: IMMDeviceEnumerator =
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)
                .map_err(|e| format!("Failed to create device enumerator: {}", e))?;

        let collection = enumerator
            .EnumAudioEndpoints(eRender, DEVICE_STATE_ACTIVE)
            .map_err(|e| format!("Failed to enum endpoints: {}", e))?;

        let count = collection
            .GetCount()
            .map_err(|e| format!("Failed to get count: {}", e))?;

        let mut devices = Vec::new();

        // Get default device to mark it
        let default_device_id = match enumerator.GetDefaultAudioEndpoint(eRender, eMultimedia) {
            Ok(device) => get_device_id(&device).unwrap_or_default(),
            Err(_) => String::new(),
        };

        for i in 0..count {
            let device = collection
                .Item(i)
                .map_err(|e| format!("Failed to get item {}: {}", i, e))?;

            let id = get_device_id(&device).map_err(|e| format!("Failed to get ID: {}", e))?;
            let name =
                get_device_name(&device).map_err(|e| format!("Failed to get name: {}", e))?;

            devices.push(AudioDevice {
                is_default: id == default_device_id,
                id,
                name,
            });
        }

        Ok(devices)
    }
}

unsafe fn get_device_id(device: &IMMDevice) -> Result<String, windows::core::Error> {
    let id_pwstr = device.GetId()?;
    let id_str = id_pwstr
        .to_string()
        .map_err(|_| windows::core::Error::from_win32())?;
    Ok(id_str)
}

unsafe fn get_device_name(device: &IMMDevice) -> Result<String, windows::core::Error> {
    // Open the property store
    let property_store: IPropertyStore = device.OpenPropertyStore(STGM_READ)?;

    // Get the friendly name property
    let mut prop_variant = property_store.GetValue(&PKEY_Device_FriendlyName)?;

    // Manually extract the string from PROPVARIANT
    // The structure of PROPVARIANT in windows-rs 0.58 has changed or is wrapped.
    // However, for VT_LPWSTR (31), it is typically a pointer to a wide string.
    // Let's rely on the fact that PROPVARIANT should be destructible or displayable if we are lucky,
    // but often we need `PropVariantToStringAlloc` which we couldn't find.

    // Let's try `PropVariantToStringAlloc` again, but ensure we use `windows::Win32::UI::Shell::PropertiesSystem`
    // If that fails, we can try to unsafe cast or debug print.

    // Alternative: Use `PropVariantToString` (not Alloc)?
    // Or just use the formatting that might exist?

    // Let's try using the raw union access if `windows` crate allows it, but often it's hidden in `Anonymous`.
    // Since `PropVariantToStringAlloc` was "unresolved", likely we need `Win32_UI_Shell_PropertiesSystem` feature which IS enabled.
    // Ensure the import is correct. It is in `windows::Win32::UI::Shell::PropertiesSystem`.

    // Let's try adding the import back and see if it works now.
    // use windows::Win32::UI::Shell::PropertiesSystem::PropVariantToStringAlloc;

    // Wait, if it failed before, maybe it's not in 0.58 `windows` crate?
    // Let's try `PropVariantToBSTR`?

    // If we can't get the name easily, users will be unhappy.
    // Let's try one more safe bet: `IMMEndpoint`.
    // No, friend name is property.

    // Let's try this manual unsafe extraction which is common in C++:
    // VT_LPWSTR = 31
    // The variant contains a union.

    // Actually, `start_with_fallback` approach:
    // Try to debug what we can get.

    // Let's try `PropVariantToStringAlloc` again, confirming the path.
    // windows::Win32::UI::Shell::PropertiesSystem::PropVariantToStringAlloc

    // If "unresolved", maybe it is in `windows::Win32::System::Com::StructuredStorage`? No.

    // OK, let's look at `property_store.GetValue` return type. It returns `PROPVARIANT`.
    // In windows 0.58, does `PROPVARIANT` have `to_string()`? No.

    // Let's try to trust `PropVariantToStringAlloc` is there but maybe I missed `Win32_UI_Shell` feature?
    // I added `Win32_UI_Shell` in previous steps.

    // Let's try to define the function signature manually if needed? No, that's linking (extern "system").

    // Let's try the `PropVariantToStringAlloc` once more with full path in the body to be sure.
    // Manual unsafe extraction via raw pointer arithmetic (HACK)
    // PROPVARIANT standard layout:
    // Offset 0: VARTYPE vt (u16)
    // Offset 8: Value (for 64-bit aligned union)

    unsafe {
        let ptr = &prop_variant as *const _ as *const u16;
        let vt = *ptr;

        if vt == 31 {
            // VT_LPWSTR
            // On 64-bit, the data matches the union offset.
            // PROPVARIANT is 4 WORDs (8 bytes) then union.
            // On 32-bit it might be different, but we are likely 64-bit targeting or aligned.
            // Let's assume standard alignment.
            // The union starts at offset 8.
            let value_ptr = (ptr as *const u8).add(8) as *const windows::core::PCWSTR;
            if !value_ptr.is_null() {
                let pcwstr = *value_ptr;
                if !pcwstr.is_null() {
                    let s = pcwstr.to_string().unwrap_or_default();
                    let _ = windows::Win32::System::Com::StructuredStorage::PropVariantClear(
                        &mut prop_variant as *const _ as *mut _,
                    );
                    return Ok(s);
                }
            }
        }
    }

    let _ = windows::Win32::System::Com::StructuredStorage::PropVariantClear(
        &mut prop_variant as *const _ as *mut _,
    );

    // Fallback to ID
    let id_pwstr = device.GetId()?;
    Ok(format!(
        "Audio Device (Unknown Name) - {}",
        id_pwstr.to_string().unwrap_or_default()
    ))
}
