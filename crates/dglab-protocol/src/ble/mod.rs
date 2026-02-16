//! BLE 通信模块
//!
//! 提供 BLE 设备扫描、连接和通信功能。

pub mod device;
pub mod scanner;

use std::collections::HashMap;
use std::sync::Arc;

use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager, Peripheral};
use tokio::sync::Mutex;
use tracing::{debug, info};

pub use device::{BleDevice, DeviceInfo};
pub use scanner::{BleScanner, ScanResult};

use crate::error::{ProtocolError, Result};

/// DG-LAB 设备相关 UUID（V3 协议）
///
/// 基础 UUID 格式: `0000xxxx-0000-1000-8000-00805f9b34fb`
pub mod uuids {
    use uuid::Uuid;

    /// 主服务 UUID (0x180C)
    pub const SERVICE_UUID: Uuid = Uuid::from_u128(0x0000180c_0000_1000_8000_00805f9b34fb);

    /// 写入特征 UUID (0x150A) - 所有指令都在该特性输入，最长 20 字节
    pub const WRITE_CHAR_UUID: Uuid = Uuid::from_u128(0x0000150a_0000_1000_8000_00805f9b34fb);

    /// 通知特征 UUID (0x150B) - 所有回应消息都在该特性返回，最长 20 字节
    pub const NOTIFY_CHAR_UUID: Uuid = Uuid::from_u128(0x0000150b_0000_1000_8000_00805f9b34fb);

    /// 电池服务 UUID (0x180A)
    pub const BATTERY_SERVICE_UUID: Uuid = Uuid::from_u128(0x0000180a_0000_1000_8000_00805f9b34fb);

    /// 电池电量特征 UUID (0x1500) - 读/通知，1 字节
    pub const BATTERY_CHAR_UUID: Uuid = Uuid::from_u128(0x00001500_0000_1000_8000_00805f9b34fb);
}

/// BLE 管理器
pub struct BleManager {
    /// 蓝牙适配器
    adapter: Adapter,
    /// 已发现的设备
    discovered_devices: Arc<Mutex<HashMap<String, Peripheral>>>,
    /// 已连接的设备
    connected_devices: Arc<Mutex<HashMap<String, BleDevice>>>,
}

impl BleManager {
    /// 创建新的 BLE 管理器
    pub async fn new() -> Result<Self> {
        let manager = Manager::new()
            .await
            .map_err(|e| ProtocolError::BleError(format!("Failed to create manager: {}", e)))?;

        let adapters = manager
            .adapters()
            .await
            .map_err(|e| ProtocolError::BleError(format!("Failed to get adapters: {}", e)))?;

        let adapter = adapters
            .into_iter()
            .next()
            .ok_or_else(|| ProtocolError::BleError("No Bluetooth adapter found".to_string()))?;

        Ok(Self {
            adapter,
            discovered_devices: Arc::new(Mutex::new(HashMap::new())),
            connected_devices: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    /// 开始扫描设备
    pub async fn start_scan(&self) -> Result<()> {
        info!("Starting BLE scan");

        // 使用空过滤器 - 许多设备（包括 DG-LAB）不在广播包中暴露服务 UUID
        // 参考 Web Bluetooth 实现：使用 namePrefix 过滤，服务 UUID 作为 optionalServices
        // 在 get_scan_results() 中通过设备名过滤 DG-LAB 设备
        let filter = ScanFilter::default();

        self.adapter
            .start_scan(filter)
            .await
            .map_err(|e| ProtocolError::BleError(format!("Failed to start scan: {}", e)))?;

        Ok(())
    }

    /// 停止扫描
    pub async fn stop_scan(&self) -> Result<()> {
        info!("Stopping BLE scan");
        self.adapter
            .stop_scan()
            .await
            .map_err(|e| ProtocolError::BleError(format!("Failed to stop scan: {}", e)))?;
        Ok(())
    }

    /// 获取扫描结果
    pub async fn get_scan_results(&self) -> Result<Vec<ScanResult>> {
        let mut results = Vec::new();
        let peripherals =
            self.adapter.peripherals().await.map_err(|e| {
                ProtocolError::BleError(format!("Failed to get peripherals: {}", e))
            })?;

        debug!("Found {} peripherals", peripherals.len());

        for peripheral in peripherals {
            if let Some(properties) = peripheral
                .properties()
                .await
                .map_err(|e| ProtocolError::BleError(format!("Failed to get properties: {}", e)))?
            {
                let local_name = properties
                    .local_name
                    .unwrap_or_else(|| "Unknown".to_string());

                debug!(
                    "Device: {} ({}), RSSI: {:?}, Services: {:?}",
                    local_name,
                    properties.address,
                    properties.rssi,
                    properties.services.len()
                );

                // 检查是否是 DG-LAB 设备
                // 脉冲主机 3.0 蓝牙名称: 47L121000
                // 无线传感器蓝牙名称: 47L120100
                // 2.0 设备名称前缀: D-LAB
                if local_name.starts_with("47L121")
                    || local_name.starts_with("47L120")
                    || local_name.starts_with("47")  // 更宽松的前缀匹配
                    || local_name.starts_with("D-LAB")
                    || local_name.to_lowercase().contains("dglab")
                    || local_name.to_lowercase().contains("coyote")
                    || properties.services.contains(&uuids::SERVICE_UUID)
                {
                    info!(
                        "Found DG-LAB device: {} ({})",
                        local_name, properties.address
                    );
                    results.push(ScanResult {
                        id: peripheral.id().to_string(),
                        name: local_name,
                        address: properties.address.to_string(),
                        rssi: properties.rssi,
                    });

                    let mut discovered = self.discovered_devices.lock().await;
                    discovered.insert(peripheral.id().to_string(), peripheral);
                }
            }
        }

        info!("Found {} DG-LAB devices", results.len());
        Ok(results)
    }

    /// 连接到设备
    pub async fn connect(&self, device_id: &str) -> Result<BleDevice> {
        info!("Connecting to device: {}", device_id);

        let discovered = self.discovered_devices.lock().await;
        let peripheral = discovered
            .get(device_id)
            .ok_or_else(|| ProtocolError::DeviceNotFound(device_id.to_string()))?;

        // 连接设备
        peripheral
            .connect()
            .await
            .map_err(|e| ProtocolError::ConnectionError(format!("Failed to connect: {}", e)))?;

        // 发现服务
        peripheral.discover_services().await.map_err(|e| {
            ProtocolError::ConnectionError(format!("Failed to discover services: {}", e))
        })?;

        // 查找特征
        let characteristics = peripheral.characteristics();
        let write_char = characteristics
            .iter()
            .find(|c| c.uuid == uuids::WRITE_CHAR_UUID)
            .cloned()
            .ok_or_else(|| {
                ProtocolError::ConnectionError("Write characteristic not found".to_string())
            })?;

        let notify_char = characteristics
            .iter()
            .find(|c| c.uuid == uuids::NOTIFY_CHAR_UUID)
            .cloned()
            .ok_or_else(|| {
                ProtocolError::ConnectionError("Notify characteristic not found".to_string())
            })?;

        // 订阅通知
        peripheral
            .subscribe(&notify_char)
            .await
            .map_err(|e| ProtocolError::ConnectionError(format!("Failed to subscribe: {}", e)))?;

        let device = BleDevice::new(
            device_id.to_string(),
            peripheral.clone(),
            write_char,
            notify_char,
        );

        // 保存连接
        let mut connected = self.connected_devices.lock().await;
        connected.insert(device_id.to_string(), device.clone());

        Ok(device)
    }

    /// 断开设备连接
    pub async fn disconnect(&self, device_id: &str) -> Result<()> {
        info!("Disconnecting device: {}", device_id);

        let mut connected = self.connected_devices.lock().await;
        if let Some(device) = connected.remove(device_id) {
            device.disconnect().await?;
        }

        Ok(())
    }
}
