#![allow(non_camel_case_types)]
use windows::core::{interface, HRESULT, PCWSTR};
use windows::Win32::Media::Audio::{eCommunications, eConsole, eMultimedia, ERole};
use windows::Win32::System::Com::{CoCreateInstance, CoInitializeEx, CLSCTX_ALL};

// Undocumented IPolicyConfig interface
// GUID for IPolicyConfig interface
#[interface("f8679f50-850a-41cf-9c72-430f290290c8")]
unsafe trait IPolicyConfig: windows::core::IUnknown {
    fn get_mix_format(&self, pcwstr: PCWSTR, waveformat: *mut *mut core::ffi::c_void) -> HRESULT;
    fn get_device_format(
        &self,
        pcwstr: PCWSTR,
        default: i32,
        waveformat: *mut *mut core::ffi::c_void,
    ) -> HRESULT;
    fn reset_device_format(&self, pcwstr: PCWSTR) -> HRESULT;
    fn set_device_format(
        &self,
        pcwstr: PCWSTR,
        waveformat: *mut core::ffi::c_void,
        waveformat2: *mut core::ffi::c_void,
    ) -> HRESULT;
    fn get_processing_period(
        &self,
        pcwstr: PCWSTR,
        input: i32,
        period: *mut i64,
        period2: *mut i64,
    ) -> HRESULT;
    fn set_processing_period(&self, pcwstr: PCWSTR, period: *mut i64, period2: *mut i64)
        -> HRESULT;
    fn get_share_mode(&self, pcwstr: PCWSTR, mode: *mut i32) -> HRESULT;
    fn set_share_mode(&self, pcwstr: PCWSTR, mode: *mut i32) -> HRESULT;
    fn get_property_value(
        &self,
        pcwstr: PCWSTR,
        key: *const core::ffi::c_void,
        value: *mut core::ffi::c_void,
    ) -> HRESULT;
    fn set_property_value(
        &self,
        pcwstr: PCWSTR,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
    ) -> HRESULT;
    fn set_default_endpoint(&self, pcwstr: PCWSTR, role: ERole) -> HRESULT;
    fn set_endpoint_visibility(&self, pcwstr: PCWSTR, visibility: i32) -> HRESULT;
}

// Class ID for PolicyConfigClient
// 0x870af99c_171d_4f9e_af0d_e63df40c2bc9
const CLSID_POLICY_CONFIG: windows::core::GUID =
    windows::core::GUID::from_u128(0x870af99c_171d_4f9e_af0d_e63df40c2bc9);

pub fn set_default_device(device_id: &str) -> Result<(), String> {
    unsafe {
        // Ensure COM is initialized
        // Use COINIT_APARTMENTTHREADED (STA) as strictly required by some Shell/UI COM objects.
        let _ = CoInitializeEx(None, windows::Win32::System::Com::COINIT_APARTMENTTHREADED);

        let policy_config: IPolicyConfig = CoCreateInstance(&CLSID_POLICY_CONFIG, None, CLSCTX_ALL)
            .map_err(|e| format!("Failed to create IPolicyConfig: {}", e))?;

        let mut id_wide: Vec<u16> = device_id.encode_utf16().collect();
        id_wide.push(0);
        let id_pcwstr = PCWSTR(id_wide.as_ptr());

        // Set for Console (Standard)
        policy_config
            .set_default_endpoint(id_pcwstr, eConsole)
            .ok()
            .map_err(|e| format!("Failed to set Console default: {}", e))?;

        // Set for Multimedia
        policy_config
            .set_default_endpoint(id_pcwstr, eMultimedia)
            .ok()
            .map_err(|e| format!("Failed to set Multimedia default: {}", e))?;

        // Set for Communications (Optional, but usually expected for "Default Device")
        policy_config
            .set_default_endpoint(id_pcwstr, eCommunications)
            .ok()
            .map_err(|e| format!("Failed to set Communications default: {}", e))?;

        Ok(())
    }
}
