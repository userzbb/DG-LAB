/**
 * Hook to setup device event listeners
 */

import { useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { useDeviceStore } from "../stores/deviceStore";
import {
  EVENT_NAMES,
  DeviceStateChangedEvent,
  DevicePowerChangedEvent,
  DeviceInfoUpdatedEvent,
  DeviceBatteryUpdatedEvent,
  DeviceErrorEvent,
} from "../types/events";
import { toast } from "../lib/toast";

/**
 * Setup device event listeners
 */
export function useDeviceEvents() {
  const {
    currentDevice,
    setDeviceState,
    updateDeviceInfo,
    setCurrentDevice,
  } = useDeviceStore();

  useEffect(() => {
    // 设置事件监听器
    const unlistenPromises = [
      // 设备状态变更
      listen<DeviceStateChangedEvent>(
        EVENT_NAMES.DEVICE_STATE_CHANGED,
        (event) => {
          const { device_id, state } = event.payload;
          if (currentDevice && currentDevice.id === device_id) {
            setDeviceState(state);
          }
        }
      ),

      // 设备功率变更
      listen<DevicePowerChangedEvent>(
        EVENT_NAMES.DEVICE_POWER_CHANGED,
        (event) => {
          const { device_id, power_a, power_b } = event.payload;
          if (currentDevice && currentDevice.id === device_id) {
            updateDeviceInfo({ power_a, power_b });
          }
        }
      ),

      // 设备信息更新
      listen<DeviceInfoUpdatedEvent>(
        EVENT_NAMES.DEVICE_INFO_UPDATED,
        (event) => {
          const { device_id, info } = event.payload;
          if (currentDevice && currentDevice.id === device_id) {
            setCurrentDevice(info);
          }
        }
      ),

      // 设备电池电量更新
      listen<DeviceBatteryUpdatedEvent>(
        EVENT_NAMES.DEVICE_BATTERY_UPDATED,
        (event) => {
          const { device_id, battery } = event.payload;
          if (currentDevice && currentDevice.id === device_id) {
            updateDeviceInfo({ battery_level: battery });
          }
        }
      ),

      // 设备错误
      listen<DeviceErrorEvent>(EVENT_NAMES.DEVICE_ERROR, (event) => {
        const { device_id, error } = event.payload;
        if (currentDevice && currentDevice.id === device_id) {
          toast.error("设备错误", error);
        }
      }),
    ];

    // 清理函数
    return () => {
      unlistenPromises.forEach((promise) => {
        promise.then((unlisten) => unlisten());
      });
    };
  }, [currentDevice, setDeviceState, updateDeviceInfo, setCurrentDevice]);
}
