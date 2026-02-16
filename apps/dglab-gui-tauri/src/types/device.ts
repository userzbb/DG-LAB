/**
 * Device types matching Rust backend
 */

/** 设备状态 */
export enum DeviceState {
  /** 已断开 */
  Disconnected = "Disconnected",
  /** 连接中 */
  Connecting = "Connecting",
  /** 已连接 */
  Connected = "Connected",
  /** 运行中 */
  Running = "Running",
  /** 错误 */
  Error = "Error",
}

/** 设备信息 */
export interface DeviceInfo {
  /** 设备 ID */
  id: string;
  /** 设备名称 */
  name: string;
  /** 设备类型 */
  device_type: string;
  /** 固件版本 */
  firmware_version: string;
  /** 硬件版本 */
  hardware_version: string;
  /** 电池电量 (0-100) */
  battery_level: number;
  /** 通道 A 当前强度 */
  power_a: number;
  /** 通道 B 当前强度 */
  power_b: number;
  /** 通道 A 最大强度 */
  max_power_a: number;
  /** 通道 B 最大强度 */
  max_power_b: number;
}

/** 设备配置 */
export interface DeviceConfig {
  /** 设备 ID */
  id: string;
  /** 设备名称 */
  name: string;
  /** 连接类型 (ble/wifi) */
  connection_type: "ble" | "wifi";
  /** 连接地址 */
  address?: string;
  /** 自动重连 */
  auto_reconnect: boolean;
  /** 安全限制（最大强度） */
  safety_limit?: number;
}

/** 设备事件 */
export type DeviceEvent =
  | { type: "StateChanged"; state: DeviceState }
  | { type: "PowerChanged"; power_a: number; power_b: number }
  | { type: "InfoUpdated"; info: DeviceInfo }
  | { type: "BatteryUpdated"; battery_level: number }
  | { type: "Error"; message: string };

/** 扫描到的设备 */
export interface ScannedDevice {
  /** 设备 ID */
  id: string;
  /** 设备名称 */
  name: string;
  /** 信号强度 (RSSI) */
  rssi?: number;
  /** 设备地址 */
  address: string;
}

/** WiFi 连接请求 */
export interface WifiConnectRequest {
  /** 自定义服务器地址（可选） */
  server_url?: string;
}

/** WiFi 连接响应 */
export interface WifiConnectResponse {
  /** 设备 ID */
  device_id: string;
  /** 设备名称 */
  device_name: string;
  /** 二维码 URL */
  qr_url: string;
}
