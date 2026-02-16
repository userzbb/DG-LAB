/**
 * Tauri API wrappers
 */

import { invoke } from "@tauri-apps/api/core";
import type {
  DeviceInfo,
  DeviceState,
  ScannedDevice,
  SessionInfo,
  WifiConnectRequest,
  WifiConnectResponse,
} from "../types";

/** 扫描 BLE 设备 */
export async function scanBleDevices(timeoutSecs?: number): Promise<ScannedDevice[]> {
  return await invoke<ScannedDevice[]>("scan_ble_devices", { timeoutSecs });
}

/** 连接 BLE 设备（扫描后首次连接） */
export async function connectBleDevice(deviceId: string, deviceName: string): Promise<DeviceInfo> {
  return await invoke<DeviceInfo>("connect_ble_device", { deviceId, deviceName });
}

/** 连接设备（重新连接已存在的设备） */
export async function connectDevice(deviceId: string): Promise<DeviceInfo> {
  return await invoke<DeviceInfo>("connect_device", { deviceId });
}

/** 断开设备连接 */
export async function disconnectDevice(deviceId: string): Promise<void> {
  return await invoke<void>("disconnect_device", { deviceId });
}

/** 获取设备信息 */
export async function getDeviceInfo(deviceId: string): Promise<DeviceInfo> {
  return await invoke<DeviceInfo>("get_device_info", { deviceId });
}

/** 获取设备状态 */
export async function getDeviceState(deviceId: string): Promise<DeviceState> {
  return await invoke<DeviceState>("get_device_state", { deviceId });
}

/** 设置设备功率 */
export async function setPower(
  deviceId: string,
  channel: "A" | "B" | number,
  power: number
): Promise<void> {
  const channelNum = typeof channel === "string" 
    ? (channel === "A" ? 0 : 1)
    : channel;
  return await invoke<void>("set_power", { deviceId, channel: channelNum, power });
}

/** 开始设备输出 */
export async function startDevice(deviceId: string): Promise<void> {
  return await invoke<void>("start_device", { deviceId });
}

/** 停止设备输出 */
export async function stopDevice(deviceId: string): Promise<void> {
  return await invoke<void>("stop_device", { deviceId });
}

/** 紧急停止 */
export async function emergencyStop(deviceId: string): Promise<void> {
  return await invoke<void>("emergency_stop", { deviceId });
}

/** 获取会话信息 */
export async function getSessionInfo(): Promise<SessionInfo> {
  return await invoke<SessionInfo>("get_session_info");
}

/** 列出所有设备 */
export async function listDevices(): Promise<string[]> {
  return await invoke<string[]>("list_devices");
}

// ========== WiFi API ==========

/** 连接 WiFi 设备 */
export async function wifiConnect(request: WifiConnectRequest): Promise<WifiConnectResponse> {
  return await invoke<WifiConnectResponse>("wifi_connect", { request });
}

/** 检查 WiFi 设备绑定状态 */
export async function wifiCheckBinding(deviceId: string): Promise<boolean> {
  return await invoke<boolean>("wifi_check_binding", { deviceId });
}

/** 取消 WiFi 连接 */
export async function wifiCancel(deviceId: string): Promise<void> {
  return await invoke<void>("wifi_cancel", { deviceId });
}
