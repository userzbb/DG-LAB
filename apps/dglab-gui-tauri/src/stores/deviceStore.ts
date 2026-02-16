/**
 * Device store - manages device state and operations
 */

import { create } from "zustand";
import {
  DeviceInfo,
  DeviceState,
  ScannedDevice,
} from "../types/device";
import * as api from "../lib/api";
import { toast } from "../lib/toast";

interface DeviceStore {
  // State
  /** 当前连接的设备 */
  currentDevice: DeviceInfo | null;
  /** 扫描到的设备列表 */
  scannedDevices: ScannedDevice[];
  /** 是否正在扫描 */
  isScanning: boolean;
  /** 设备状态 */
  deviceState: DeviceState;
  /** 通道 A 强度 */
  powerA: number;
  /** 通道 B 强度 */
  powerB: number;
  /** 是否已连接 */
  isConnected: boolean;

  // Actions
  /** 设置当前设备 */
  setCurrentDevice: (device: DeviceInfo | null) => void;
  /** 设置扫描到的设备列表 */
  setScannedDevices: (devices: ScannedDevice[]) => void;
  /** 添加扫描到的设备 */
  addScannedDevice: (device: ScannedDevice) => void;
  /** 清空扫描到的设备 */
  clearScannedDevices: () => void;
  /** 设置扫描状态 */
  setScanning: (scanning: boolean) => void;
  /** 设置设备状态 */
  setDeviceState: (state: DeviceState) => void;
  /** 更新设备信息 */
  updateDeviceInfo: (info: Partial<DeviceInfo>) => void;
  /** 重置状态 */
  reset: () => void;

  // API Actions
  /** 扫描设备 */
  scanDevices: () => Promise<void>;
  /** 连接到设备（扫描后首次连接） */
  connectToDevice: (deviceId: string, deviceName: string) => Promise<void>;
  /** 断开设备连接 */
  disconnectDevice: () => Promise<void>;
  /** 设置通道功率 */
  setPower: (channel: "A" | "B", power: number) => Promise<void>;
  /** 启动设备 */
  startDevice: () => Promise<void>;
  /** 停止设备 */
  stopDevice: () => Promise<void>;
  /** 紧急停止 */
  emergencyStop: () => Promise<void>;
}

const initialState = {
  currentDevice: null,
  scannedDevices: [],
  isScanning: false,
  deviceState: DeviceState.Disconnected,
  powerA: 0,
  powerB: 0,
  isConnected: false,
};

export const useDeviceStore = create<DeviceStore>((set, get) => ({
  ...initialState,

  setCurrentDevice: (device) => set({ 
    currentDevice: device,
    isConnected: device !== null,
  }),

  setScannedDevices: (devices) => set({ scannedDevices: devices }),

  addScannedDevice: (device) =>
    set((state) => {
      // 避免重复添加
      const exists = state.scannedDevices.some((d) => d.id === device.id);
      if (exists) {
        return state;
      }
      return { scannedDevices: [...state.scannedDevices, device] };
    }),

  clearScannedDevices: () => set({ scannedDevices: [] }),

  setScanning: (scanning) => set({ isScanning: scanning }),

  setDeviceState: (state) => set({ deviceState: state }),

  updateDeviceInfo: (info) =>
    set((state) => ({
      currentDevice: state.currentDevice
        ? { ...state.currentDevice, ...info }
        : null,
    })),

  reset: () => set(initialState),

  // API Actions
  scanDevices: async () => {
    set({ isScanning: true, scannedDevices: [] });
    const loadingToast = toast.loading("正在扫描设备...");
    try {
      const devices = await api.scanBleDevices();
      set({ scannedDevices: devices });
      toast.dismiss(loadingToast);
      if (devices.length === 0) {
        toast.info("未发现设备", "请确保设备已开启并在范围内");
      } else {
        toast.success(`发现 ${devices.length} 个设备`);
      }
    } catch (error) {
      toast.dismiss(loadingToast);
      toast.error("扫描失败", error instanceof Error ? error.message : "未知错误");
      console.error("Scan failed:", error);
      throw error;
    } finally {
      set({ isScanning: false });
    }
  },

  connectToDevice: async (deviceId: string, deviceName: string) => {
    const { isConnected } = get();
    if (isConnected) {
      toast.info("设备已连接", "请先断开当前设备");
      return;
    }

    set({ deviceState: DeviceState.Connecting });
    const loadingToast = toast.loading("正在连接设备...");
    try {
      await api.connectBleDevice(deviceId, deviceName);
      const deviceInfo = await api.getDeviceInfo(deviceId);
      set({
        currentDevice: deviceInfo,
        deviceState: DeviceState.Connected,
        isConnected: true,
      });
      toast.dismiss(loadingToast);
      toast.success("设备已连接", `已连接到 ${deviceInfo.name}`);
    } catch (error) {
      set({ deviceState: DeviceState.Disconnected, isConnected: false });
      toast.dismiss(loadingToast);
      toast.error("连接失败", error instanceof Error ? error.message : "未知错误");
      console.error("Connect failed:", error);
      throw error;
    }
  },

  disconnectDevice: async () => {
    const { currentDevice } = get();
    if (!currentDevice) return;

    const loadingToast = toast.loading("正在断开连接...");
    try {
      await api.disconnectDevice(currentDevice.id);
      set({
        currentDevice: null,
        deviceState: DeviceState.Disconnected,
        isConnected: false,
        powerA: 0,
        powerB: 0,
      });
      toast.dismiss(loadingToast);
      toast.success("设备已断开");
    } catch (error) {
      toast.dismiss(loadingToast);
      toast.error("断开失败", error instanceof Error ? error.message : "未知错误");
      console.error("Disconnect failed:", error);
      throw error;
    }
  },

  setPower: async (channel: "A" | "B", power: number) => {
    const { currentDevice } = get();
    if (!currentDevice) {
      toast.error("设置失败", "未连接设备");
      throw new Error("No device connected");
    }

    try {
      await api.setPower(currentDevice.id, channel, power);
      set((state) => ({
        [channel === "A" ? "powerA" : "powerB"]: power,
        currentDevice: state.currentDevice
          ? {
              ...state.currentDevice,
              [channel === "A" ? "power_a" : "power_b"]: power,
            }
          : null,
      }));
      toast.success(`通道 ${channel} 功率已设置为 ${power}`);
    } catch (error) {
      toast.error("设置功率失败", error instanceof Error ? error.message : "未知错误");
      console.error("Set power failed:", error);
      throw error;
    }
  },

  startDevice: async () => {
    const { currentDevice } = get();
    if (!currentDevice) {
      toast.error("启动失败", "未连接设备");
      throw new Error("No device connected");
    }

    try {
      await api.startDevice(currentDevice.id);
      toast.success("设备已启动");
    } catch (error) {
      toast.error("启动设备失败", error instanceof Error ? error.message : "未知错误");
      console.error("Start device failed:", error);
      throw error;
    }
  },

  stopDevice: async () => {
    const { currentDevice } = get();
    if (!currentDevice) {
      toast.error("停止失败", "未连接设备");
      throw new Error("No device connected");
    }

    try {
      await api.stopDevice(currentDevice.id);
      toast.success("设备已停止");
    } catch (error) {
      toast.error("停止设备失败", error instanceof Error ? error.message : "未知错误");
      console.error("Stop device failed:", error);
      throw error;
    }
  },

  emergencyStop: async () => {
    const { currentDevice } = get();
    if (!currentDevice) {
      toast.error("紧急停止失败", "未连接设备");
      throw new Error("No device connected");
    }

    try {
      await api.emergencyStop(currentDevice.id);
      set({ powerA: 0, powerB: 0 });
      toast.success("紧急停止已执行", "所有输出已归零");
    } catch (error) {
      toast.error("紧急停止失败", error instanceof Error ? error.message : "未知错误");
      console.error("Emergency stop failed:", error);
      throw error;
    }
  },
}));
