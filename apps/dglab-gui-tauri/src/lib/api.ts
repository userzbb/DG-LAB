/**
 * Tauri API wrappers
 */

import { invoke } from "@tauri-apps/api/core";
import type {
  DeviceInfo,
  DeviceState,
  ScannedDevice,
  SessionInfo,
} from "../types";

/** 扫描 BLE 设备 */
export async function scanBleDevices(timeoutSecs?: number): Promise<ScannedDevice[]> {
  return await invoke<ScannedDevice[]>("scan_ble_devices", { timeoutSecs });
}

/** 连接设备 */
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
