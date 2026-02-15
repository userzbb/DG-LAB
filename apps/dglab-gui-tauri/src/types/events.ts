/**
 * Tauri event types and names
 */

import { DeviceInfo, DeviceState } from "./device";

/** 设备状态变更事件 */
export interface DeviceStateChangedEvent {
  device_id: string;
  state: DeviceState;
}

/** 设备功率变更事件 */
export interface DevicePowerChangedEvent {
  device_id: string;
  power_a: number;
  power_b: number;
}

/** 设备信息更新事件 */
export interface DeviceInfoUpdatedEvent {
  device_id: string;
  info: DeviceInfo;
}

/** 设备电池电量更新事件 */
export interface DeviceBatteryUpdatedEvent {
  device_id: string;
  battery: number;
}

/** 设备错误事件 */
export interface DeviceErrorEvent {
  device_id: string;
  error: string;
}

/** 事件名称常量 */
export const EVENT_NAMES = {
  DEVICE_STATE_CHANGED: "device:state_changed",
  DEVICE_POWER_CHANGED: "device:power_changed",
  DEVICE_INFO_UPDATED: "device:info_updated",
  DEVICE_BATTERY_UPDATED: "device:battery_updated",
  DEVICE_ERROR: "device:error",
} as const;
