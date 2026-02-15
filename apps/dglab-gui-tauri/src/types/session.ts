/**
 * Session types matching Rust backend
 */

import type { DeviceState } from "./device";

/** 会话事件 */
export type SessionEvent =
  | { type: "DeviceAdded"; device_id: string }
  | { type: "DeviceRemoved"; device_id: string }
  | { type: "DeviceStateChanged"; device_id: string; state: DeviceState }
  | { type: "Error"; message: string };

/** 会话信息 */
export interface SessionInfo {
  /** 会话 ID */
  id: string;
  /** 会话创建时间 */
  created_at: string;
  /** 活动设备数量 */
  active_devices: number;
  /** 总设备数量 */
  total_devices: number;
}
