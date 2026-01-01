import { invoke } from "@tauri-apps/api/core";

export interface AudioDevice {
  id: string;
  name: string;
  is_default: boolean;
}

export interface Config {
  excluded_device_ids: string[];
  hotkey: string | null;
}

export async function getAudioDevices(): Promise<AudioDevice[]> {
  return invoke("get_audio_devices");
}

export async function setActiveDevice(id: string): Promise<void> {
  return invoke("set_active_device", { id });
}

export async function getConfig(): Promise<Config> {
  return invoke("get_config");
}

export async function setExcludedDevices(ids: string[]): Promise<void> {
  return invoke("set_excluded_devices", { ids });
}

export async function saveConfig(config: Config): Promise<void> {
  return invoke("save_config", { config });
}
