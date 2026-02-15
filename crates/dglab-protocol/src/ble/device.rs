//! BLE 设备实现

use std::sync::Arc;

use btleplug::api::{Characteristic, Peripheral as _, WriteType};
use btleplug::platform::Peripheral;
use futures_util::StreamExt;
use tokio::sync::{mpsc, Mutex};
use tracing::{debug, info};

use crate::error::{ProtocolError, Result};

/// 设备信息
#[derive(Debug, Clone)]
pub struct DeviceInfo {
    /// 设备 ID
    pub id: String,
    /// 设备名称
    pub name: String,
    /// 固件版本
    pub firmware_version: Option<String>,
    /// 硬件版本
    pub hardware_version: Option<String>,
    /// 电池电量 (0-100)
    pub battery_level: Option<u8>,
}

/// BLE 设备
#[derive(Clone)]
pub struct BleDevice {
    /// 设备 ID
    id: String,
    /// 底层外设
    peripheral: Peripheral,
    /// 写入特征
    write_char: Characteristic,
    /// 通知特征
    notify_char: Characteristic,
    /// 数据发送通道
    data_tx: mpsc::Sender<Vec<u8>>,
    /// 数据接收通道
    data_rx: Arc<Mutex<mpsc::Receiver<Vec<u8>>>>,
}

impl BleDevice {
    /// 创建新的 BLE 设备
    pub(crate) fn new(
        id: String,
        peripheral: Peripheral,
        write_char: Characteristic,
        notify_char: Characteristic,
    ) -> Self {
        let (data_tx, data_rx) = mpsc::channel(100);

        let device = Self {
            id,
            peripheral,
            write_char,
            notify_char,
            data_tx,
            data_rx: Arc::new(Mutex::new(data_rx)),
        };

        // 启动通知监听任务
        device.start_notification_listener();

        device
    }

    /// 启动通知监听
    fn start_notification_listener(&self) {
        let peripheral = self.peripheral.clone();
        let notify_char = self.notify_char.clone();
        let data_tx = self.data_tx.clone();

        tokio::spawn(async move {
            if let Ok(mut notifications) = peripheral.notifications().await {
                while let Some(data) = notifications.next().await {
                    if data.uuid == notify_char.uuid {
                        debug!("Received notification: {:02x?}", data.value);
                        let _ = data_tx.send(data.value).await;
                    }
                }
            }
        });
    }

    /// 获取设备 ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// 发送数据到设备
    pub async fn send(&self, data: &[u8]) -> Result<()> {
        debug!("Sending data: {:02x?}", data);

        self.peripheral
            .write(&self.write_char, data, WriteType::WithoutResponse)
            .await
            .map_err(|e| ProtocolError::BleError(format!("Failed to write: {}", e)))?;

        Ok(())
    }

    /// 接收设备数据
    pub async fn receive(&self) -> Result<Vec<u8>> {
        let mut rx = self.data_rx.lock().await;
        rx.recv()
            .await
            .ok_or_else(|| ProtocolError::ConnectionError("Receive channel closed".to_string()))
    }

    /// 带超时的接收
    pub async fn receive_timeout(&self, timeout: std::time::Duration) -> Result<Vec<u8>> {
        tokio::time::timeout(timeout, self.receive())
            .await
            .map_err(|_| ProtocolError::Timeout)?
    }

    /// 发送命令并等待响应
    pub async fn send_command(
        &self,
        command: &[u8],
        timeout: std::time::Duration,
    ) -> Result<Vec<u8>> {
        self.send(command).await?;
        self.receive_timeout(timeout).await
    }

    /// 断开设备连接
    pub async fn disconnect(&self) -> Result<()> {
        info!("Disconnecting device: {}", self.id);

        self.peripheral
            .disconnect()
            .await
            .map_err(|e| ProtocolError::BleError(format!("Failed to disconnect: {}", e)))?;

        Ok(())
    }

    /// 检查是否已连接
    pub async fn is_connected(&self) -> Result<bool> {
        self.peripheral
            .is_connected()
            .await
            .map_err(|e| ProtocolError::BleError(format!("Failed to check connection: {}", e)))
    }
}
