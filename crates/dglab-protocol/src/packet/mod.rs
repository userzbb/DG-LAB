//! 数据包编码/解码模块（旧版）
//!
//! **⚠️ 已弃用**: 本模块使用的数据包格式不符合官方 V3 BLE 协议。
//! 请使用 [`crate::v3`] 模块中的 [`crate::v3::B0Command`]、[`crate::v3::BFCommand`]、
//! [`crate::v3::B1Response`] 等类型。
//!
//! 本模块保留仅用于向后兼容。

pub mod decoder;
pub mod encoder;
pub mod types;

pub use decoder::PacketDecoder;
pub use encoder::PacketEncoder;
pub use types::*;
