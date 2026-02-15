//! DG-LAB 核心业务逻辑库
//!
//! 提供设备抽象、会话管理、波形生成等核心功能。

#![warn(missing_docs)]

pub mod device;
pub mod error;
pub mod preset;
pub mod script;
pub mod session;
pub mod waveform;

pub use device::{Device, DeviceEvent, DeviceState};
pub use error::{CoreError, Result};
pub use session::SessionManager;
pub use waveform::WaveformGenerator;
