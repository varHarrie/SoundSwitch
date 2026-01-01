<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { listen } from "@tauri-apps/api/event";
import {
  getAudioDevices,
  setActiveDevice,
  getConfig,
  saveConfig,
  type AudioDevice,
  type Config,
} from "../services/invoke";
import { enable, disable, isEnabled } from "@tauri-apps/plugin-autostart";
import { getVersion } from "@tauri-apps/api/app";

const version = ref("");

const view = ref<"list" | "settings">("list");
const devices = ref<AudioDevice[]>([]);
const excludedIds = ref<Set<string>>(new Set());
const currentConfig = ref<Config | null>(null);
const isRecording = ref(false);
const hotkeyDisplay = ref("");
const error = ref<string>("");
const loading = ref(false);
const autoStart = ref(false);

async function checkAutoStart() {
  try {
    autoStart.value = await isEnabled();
  } catch (e) {
    console.error("Failed to check autostart status", e);
  }
}

async function toggleAutoStart() {
  try {
    if (autoStart.value) {
      await disable();
    } else {
      await enable();
    }
    autoStart.value = await isEnabled();
  } catch (e: any) {
    error.value = `Failed to update autostart: ${e.toString()}`;
  }
}

async function loadData() {
  loading.value = true;
  error.value = "";
  try {
    const [devs, config] = await Promise.all([getAudioDevices(), getConfig()]);
    devices.value = devs;
    currentConfig.value = config;
    excludedIds.value = new Set(config.excluded_device_ids);
    hotkeyDisplay.value = config.hotkey || "None";
  } catch (e: any) {
    error.value = e.toString();
  } finally {
    loading.value = false;
  }
}

function startRecording() {
  isRecording.value = true;
  hotkeyDisplay.value = "Press keys...";
  window.addEventListener("keydown", handleKeydown);
}

function handleKeydown(e: KeyboardEvent) {
  e.preventDefault();
  e.stopPropagation();

  const modifiers = [];
  if (e.ctrlKey) modifiers.push("Control");
  if (e.shiftKey) modifiers.push("Shift");
  if (e.altKey) modifiers.push("Alt");
  if (e.metaKey) modifiers.push("Command");

  const key = e.key;
  if (
    key === "Control" ||
    key === "Shift" ||
    key === "Alt" ||
    key === "Meta" ||
    key === "OS"
  ) {
    return;
  }

  // Map JS keys to Tauri Shortcut keys if needed, but standard characters usually work.
  // Tauri uses 'CommandOrControl', 'Alt', 'Shift', etc.
  // We'll use a simple mapping.
  let tauriKey = key.toUpperCase();
  if (tauriKey === " ") tauriKey = "SPACE";

  const modifierStr = modifiers
    .map((m) => (m === "Control" ? "CommandOrControl" : m))
    .join("+");
  const finalHotkey = modifierStr ? `${modifierStr}+${tauriKey}` : tauriKey;

  hotkeyDisplay.value = finalHotkey;
  stopRecording(finalHotkey);
}

function stopRecording(hotkey?: string) {
  isRecording.value = false;
  window.removeEventListener("keydown", handleKeydown);

  if (hotkey && currentConfig.value) {
    currentConfig.value.hotkey = hotkey;
    saveCurrentConfig();
  }
}

async function saveCurrentConfig() {
  if (!currentConfig.value) return;
  try {
    await saveConfig(currentConfig.value);
  } catch (e: any) {
    error.value = `Failed to save config: ${e.toString()}`;
  }
}

async function switchDevice(id: string) {
  try {
    await setActiveDevice(id);
    await loadData(); // Refresh list to update active status
  } catch (e: any) {
    error.value = `Failed to switch: ${e.toString()}`;
  }
}

async function toggleCycleInclusion(deviceId: string, included: boolean) {
  if (included) {
    excludedIds.value.delete(deviceId);
  } else {
    excludedIds.value.add(deviceId);
  }

  if (currentConfig.value) {
    currentConfig.value.excluded_device_ids = Array.from(excludedIds.value);
    await saveCurrentConfig();
  }
}

const unlisten = ref<() => void>();

onMounted(async () => {
  loadData();
  checkAutoStart();
  getVersion().then((v) => (version.value = v));
  unlisten.value = await listen("device-changed", () => {
    loadData();
  });
});

function getDisplayIndex(deviceId: string): string {
  // Filter excluded
  const included = devices.value.filter((d) => !excludedIds.value.has(d.id));
  const idx = included.findIndex((d) => d.id === deviceId);
  return idx !== -1 ? (idx + 1).toString() : "-";
}

onUnmounted(() => {
  if (unlisten.value) {
    unlisten.value();
  }
});
</script>

<template>
  <div
    class="h-screen bg-[#f3f3f3] text-gray-900 font-sans select-none flex flex-col"
  >
    <!-- Header / Toolbar -->
    <div
      class="bg-white px-6 py-4 border-b border-gray-200 flex items-center justify-between sticky top-0 z-10 shrink-0"
    >
      <div class="flex items-center gap-3">
        <span class="icon-[tabler--headphones] text-xl text-indigo-600"></span>
        <h1 class="text-lg font-bold tracking-tight text-gray-800">
          SoundSwitch
        </h1>
      </div>

      <div class="flex items-center gap-1">
        <button
          v-if="view === 'list'"
          @click="loadData"
          :disabled="loading"
          class="w-9 h-9 flex items-center justify-center rounded-full hover:bg-gray-100 transition-colors text-gray-400 hover:text-indigo-600 cursor-pointer disabled:opacity-50"
          title="Refresh"
        >
          <span
            class="icon-[tabler--refresh] text-lg"
            :class="{ 'animate-spin': loading }"
          ></span>
        </button>

        <button
          @click="view = view === 'list' ? 'settings' : 'list'"
          class="w-9 h-9 flex items-center justify-center rounded-full hover:bg-gray-100 transition-colors text-gray-400 hover:text-indigo-600 cursor-pointer"
          :title="view === 'list' ? 'Settings' : 'Back to List'"
        >
          <span
            :class="
              view === 'list'
                ? 'icon-[tabler--settings]'
                : 'icon-[tabler--list]'
            "
            class="text-lg"
          ></span>
        </button>
      </div>
    </div>

    <!-- Content -->
    <div class="flex-1 overflow-y-auto">
      <div v-if="view === 'list'" class="p-4 sm:p-6 pb-20">
        <!-- Error Banner -->
        <div
          v-if="error"
          class="bg-red-50 text-red-700 px-4 py-3 rounded-md mb-4 flex items-center gap-2 border border-red-100 text-sm animate-in fade-in slide-in-from-top-1"
        >
          <span class="icon-[tabler--alert-circle] text-lg shrink-0"></span>
          <p>{{ error }}</p>
        </div>

        <!-- Device List -->
        <div v-if="devices.length" class="space-y-3">
          <div
            class="text-xs font-bold text-gray-400 uppercase tracking-widest mb-3 px-1"
          >
            Output Devices
          </div>

          <div
            v-for="device in devices"
            :key="device.id"
            class="group bg-white border border-gray-100 hover:border-indigo-200 rounded-xl p-4 flex items-center gap-4 transition-all duration-200 shadow-sm hover:shadow-md cursor-default"
            :class="{
              'ring-2 ring-indigo-500 ring-offset-2 border-transparent!':
                device.is_default,
            }"
          >
            <!-- Inclusion Checkbox -->
            <div
              class="shrink-0 flex items-center justify-center"
              title="Include in Cycle"
            >
              <input
                type="checkbox"
                class="hidden-checkbox peer"
                :id="'chk-' + device.id"
                :checked="!excludedIds.has(device.id)"
                @change="(e) => toggleCycleInclusion(device.id, (e.target as HTMLInputElement).checked)"
              />
              <label
                :for="'chk-' + device.id"
                class="w-5 h-5 border-2 border-gray-300 rounded cursor-pointer flex items-center justify-center peer-checked:bg-indigo-600 peer-checked:border-indigo-600 transition-all hover:border-indigo-400"
              >
                <span
                  class="icon-[tabler--check] text-white text-[10px] opacity-0 peer-checked:opacity-100 pointer-events-none"
                ></span>
              </label>
            </div>

            <!-- Index Badge -->
            <div class="shrink-0">
              <div
                class="w-10 h-10 rounded-full flex items-center justify-center font-bold text-lg transition-all"
                :class="[
                  device.is_default
                    ? 'bg-indigo-600 text-white'
                    : 'bg-indigo-50 text-indigo-400 group-hover:bg-indigo-100',
                ]"
              >
                {{ getDisplayIndex(device.id) }}
              </div>
            </div>

            <!-- Device Info -->
            <div class="flex-1 min-w-0">
              <p
                class="text-sm font-semibold truncate text-gray-800"
                :class="{ 'text-indigo-900': device.is_default }"
              >
                {{ device.name }}
              </p>
              <p
                class="text-xs text-gray-400 font-medium h-4"
                :class="{ 'text-indigo-400': device.is_default }"
              >
                {{ device.is_default ? "ACTIVE DEVICE" : "" }}
              </p>
            </div>

            <!-- Action -->
            <div class="shrink-0">
              <button
                v-if="!device.is_default"
                @click="switchDevice(device.id)"
                class="bg-gray-50 hover:bg-indigo-600 text-gray-400 hover:text-white px-4 py-1.5 rounded-lg text-xs font-bold transition-all active:scale-95 cursor-pointer shadow-sm"
              >
                ACTIVATE
              </button>
              <div
                v-else
                class="w-8 h-8 rounded-full bg-green-50 flex items-center justify-center text-green-500"
              >
                <span class="icon-[tabler--circle-check-filled] text-xl"></span>
              </div>
            </div>
          </div>
        </div>

        <div
          v-else-if="!loading"
          class="h-64 flex flex-col items-center justify-center text-gray-400"
        >
          <span
            class="icon-[tabler--device-speaker-off] text-5xl mb-4 opacity-20"
          ></span>
          <p class="text-sm font-medium">No audio devices detected</p>
        </div>
      </div>

      <!-- Settings View -->
      <div v-else class="p-4 sm:p-6 animate-in fade-in slide-in-from-right-4">
        <div class="max-w-md mx-auto space-y-8">
          <header>
            <h2 class="text-2xl font-black text-gray-800 tracking-tight">
              Settings
            </h2>
            <p class="text-sm text-gray-400 font-medium">
              Configure your experience
            </p>
          </header>

          <!-- Group: Startup -->
          <section class="space-y-4">
            <h3
              class="text-xs font-bold text-gray-400 uppercase tracking-widest px-1"
            >
              General
            </h3>
            <div
              class="bg-white rounded-2xl p-4 shadow-sm border border-gray-100 flex items-center justify-between"
            >
              <div class="flex items-center gap-3">
                <div
                  class="w-10 h-10 rounded-xl bg-orange-50 text-orange-500 flex items-center justify-center"
                >
                  <span class="icon-[tabler--rocket] text-xl"></span>
                </div>
                <div>
                  <p class="text-sm font-bold text-gray-800">Launch at Login</p>
                  <p class="text-[10px] text-gray-400 font-medium leading-none">
                    Start automatically with Windows
                  </p>
                </div>
              </div>

              <button
                @click="toggleAutoStart"
                class="w-12 h-6 rounded-full relative transition-all duration-300 ease-in-out cursor-pointer"
                :class="autoStart ? 'bg-indigo-600' : 'bg-gray-200'"
              >
                <div
                  class="absolute top-1 left-1 w-4 h-4 bg-white rounded-full transition-transform duration-300 shadow-sm"
                  :class="autoStart ? 'translate-x-6' : 'translate-x-0'"
                ></div>
              </button>
            </div>
          </section>

          <!-- Group: Hotkey -->
          <section class="space-y-4">
            <h3
              class="text-xs font-bold text-gray-400 uppercase tracking-widest px-1"
            >
              Shortcuts
            </h3>
            <div
              class="bg-white rounded-2xl p-4 shadow-sm border border-gray-100"
            >
              <div class="flex items-center justify-between mb-4">
                <div class="flex items-center gap-3">
                  <div
                    class="w-10 h-10 rounded-xl bg-indigo-50 text-indigo-500 flex items-center justify-center"
                  >
                    <span class="icon-[tabler--keyboard] text-xl"></span>
                  </div>
                  <div>
                    <p class="text-sm font-bold text-gray-800">Switch Device</p>
                    <p
                      class="text-[10px] text-gray-400 font-medium leading-none"
                    >
                      Global hotkey to cycle audio
                    </p>
                  </div>
                </div>
              </div>

              <div
                @click="startRecording"
                class="relative h-12 bg-gray-50 border-2 border-dashed border-gray-200 hover:border-indigo-400 rounded-xl flex items-center justify-center cursor-pointer transition-all group"
                :class="{ 'border-indigo-600! bg-indigo-50!': isRecording }"
              >
                <span
                  class="text-sm font-black tracking-widest transition-colors"
                  :class="isRecording ? 'text-indigo-600' : 'text-gray-400'"
                >
                  {{ hotkeyDisplay }}
                </span>
                <span
                  v-if="!isRecording"
                  class="absolute right-4 text-[10px] font-bold text-gray-300 opacity-0 group-hover:opacity-100 transition-opacity"
                >
                  CLICK TO CHANGE
                </span>
              </div>
            </div>
          </section>

          <!-- Footer in Settings -->
          <div class="pt-8 text-center space-y-1">
            <p class="text-[10px] font-bold text-gray-300 tracking-tighter">
              MADE WITH ❤️ BY ANTIGRAVITY
            </p>
            <p class="text-[10px] font-bold text-gray-300 tracking-tighter">
              VERSION {{ version }}
            </p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style>
.hidden-checkbox {
  position: absolute;
  opacity: 0;
  width: 0;
  height: 0;
}

/* Animations */
.animate-in {
  animation: animate-in 0.3s ease-out;
}

@keyframes animate-in {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

/* Support for peer-checked inside child elements if standard tailwind doesn't pick it up */
.peer:checked ~ label .icon-\[tabler--check\] {
  opacity: 1;
}
</style>
