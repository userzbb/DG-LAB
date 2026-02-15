//! DG-LAB 设备通信协议库
//!
//! 本库提供了与 DG-LAB 设备通信的 BLE 和 WiFi 协议实现。
//!
//! # 模块
//!
//! - [`v3`] - V3 BLE 协议（推荐使用）
//! - [`wifi`] - WebSocket 通信协议
//! - [`ble`] - BLE 设备扫描和连接管理
//! - [`packet`] - 旧版数据包格式（已弃用，请使用 [`v3`]）

#![warn(missing_docs)]

pub mod ble;
pub mod error;
pub mod packet;
pub mod v3;
pub mod wifi;

pub use error::{ProtocolError, Result};
