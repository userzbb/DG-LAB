//! DG-LAB 郊狼脉冲主机 V3 BLE 协议
//!
//! 实现了官方 V3 蓝牙协议的数据结构和编解码。
//!
//! # 协议概述
//!
//! V3 协议使用两个主要指令：
//! - **B0 指令**：每 100ms 写入一次，包含通道强度变化和波形数据（固定 20 字节）
//! - **BF 指令**：写入通道强度软上限和平衡参数（固定 7 字节）
//!
//! 设备回应通过 Notify 特征返回：
//! - **B1 消息**：强度变化反馈
//!
//! # 波形频率转换
//!
//! 输入值范围 (10 ~ 1000) 通过 [`compress_frequency`] 压缩为发送值 (10 ~ 240)。
//!
//! # 示例
//!
//! ```
//! use dglab_protocol::v3::{B0Command, StrengthMode, WaveformData, compress_frequency};
//!
//! // 创建一个不修改强度、只输出 A 通道波形的 B0 指令
//! let waveform_a = WaveformData {
//!     frequency: [10, 10, 10, 10],
//!     intensity: [0, 10, 20, 30],
//! };
//! // B 通道静默（至少一个强度值 > 100）
//! let waveform_b = WaveformData::silent();
//!
//! let cmd = B0Command {
//!     sequence: 0,
//!     strength_mode: StrengthMode::both_no_change(),
//!     strength_a: 0,
//!     strength_b: 0,
//!     waveform_a,
//!     waveform_b,
//! };
//!
//! let bytes = cmd.encode();
//! assert_eq!(bytes.len(), 20);
//! assert_eq!(bytes[0], 0xB0);
//!
//! // 频率转换
//! assert_eq!(compress_frequency(50), 50);   // 10-100 直接映射
//! assert_eq!(compress_frequency(200), 120); // 101-600 压缩
//! assert_eq!(compress_frequency(800), 220); // 601-1000 压缩
//! ```

use serde::{Deserialize, Serialize};

/// B0 指令头部
pub const B0_HEAD: u8 = 0xB0;

/// BF 指令头部
pub const BF_HEAD: u8 = 0xBF;

/// B1 回应头部
pub const B1_HEAD: u8 = 0xB1;

/// B0 指令总长度（固定 20 字节）
pub const B0_LENGTH: usize = 20;

/// BF 指令总长度（固定 7 字节）
pub const BF_LENGTH: usize = 7;

/// B1 回应总长度（固定 4 字节）
pub const B1_LENGTH: usize = 4;

/// 通道强度最大值
pub const MAX_STRENGTH: u8 = 200;

/// 波形强度最大值
pub const MAX_WAVE_INTENSITY: u8 = 100;

/// 波形频率最小值
pub const MIN_WAVE_FREQUENCY: u8 = 10;

/// 波形频率最大值（压缩后）
pub const MAX_WAVE_FREQUENCY: u8 = 240;

/// 单通道强度解读方式（2 位）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum ChannelStrengthMode {
    /// 0b00 - 不做改变，强度设定值无效
    NoChange = 0b00,
    /// 0b01 - 相对增加
    Increase = 0b01,
    /// 0b10 - 相对减少
    Decrease = 0b10,
    /// 0b11 - 绝对变化（直接设置）
    Absolute = 0b11,
}

impl From<u8> for ChannelStrengthMode {
    fn from(value: u8) -> Self {
        match value & 0b11 {
            0b00 => Self::NoChange,
            0b01 => Self::Increase,
            0b10 => Self::Decrease,
            0b11 => Self::Absolute,
            _ => unreachable!(),
        }
    }
}

/// 强度值解读方式（4 位，包含 A 和 B 两通道）
///
/// 高 2 位为 A 通道，低 2 位为 B 通道。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct StrengthMode {
    /// A 通道解读方式
    pub channel_a: ChannelStrengthMode,
    /// B 通道解读方式
    pub channel_b: ChannelStrengthMode,
}

impl StrengthMode {
    /// 创建新的强度解读方式
    pub fn new(channel_a: ChannelStrengthMode, channel_b: ChannelStrengthMode) -> Self {
        Self {
            channel_a,
            channel_b,
        }
    }

    /// 两通道都不做改变
    pub fn both_no_change() -> Self {
        Self::new(ChannelStrengthMode::NoChange, ChannelStrengthMode::NoChange)
    }

    /// 编码为 4 位值
    pub fn encode(&self) -> u8 {
        ((self.channel_a as u8) << 2) | (self.channel_b as u8)
    }

    /// 从 4 位值解码
    pub fn decode(value: u8) -> Self {
        Self {
            channel_a: ChannelStrengthMode::from((value >> 2) & 0b11),
            channel_b: ChannelStrengthMode::from(value & 0b11),
        }
    }
}

/// 波形数据（单通道，4 组 25ms 数据 = 100ms）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WaveformData {
    /// 4 组波形频率，每组 1 字节，值范围 10~240
    pub frequency: [u8; 4],
    /// 4 组波形强度，每组 1 字节，值范围 0~100
    pub intensity: [u8; 4],
}

impl WaveformData {
    /// 创建新的波形数据
    pub fn new(frequency: [u8; 4], intensity: [u8; 4]) -> Self {
        Self {
            frequency,
            intensity,
        }
    }

    /// 创建静默波形数据（不输出波形）
    ///
    /// 将至少一个强度值设为 > 100 使设备放弃该通道全部 4 组数据。
    pub fn silent() -> Self {
        Self {
            frequency: [0, 0, 0, 0],
            intensity: [0, 0, 0, 101],
        }
    }

    /// 创建均匀波形数据（4 组相同的频率和强度）
    pub fn uniform(frequency: u8, intensity: u8) -> Self {
        Self {
            frequency: [frequency; 4],
            intensity: [intensity; 4],
        }
    }

    /// 检查波形数据是否有效
    ///
    /// 若某通道的输入值不在有效范围，设备会放弃该通道全部 4 组数据。
    /// 频率有效范围: 10~240, 强度有效范围: 0~100
    pub fn is_valid(&self) -> bool {
        self.frequency
            .iter()
            .all(|&f| (MIN_WAVE_FREQUENCY..=MAX_WAVE_FREQUENCY).contains(&f))
            && self.intensity.iter().all(|&i| i <= MAX_WAVE_INTENSITY)
    }

    /// 编码为 8 字节（频率 4 字节 + 强度 4 字节）
    pub fn encode(&self) -> [u8; 8] {
        let mut buf = [0u8; 8];
        buf[0..4].copy_from_slice(&self.frequency);
        buf[4..8].copy_from_slice(&self.intensity);
        buf
    }

    /// 从 8 字节解码
    pub fn decode(data: &[u8]) -> Option<Self> {
        if data.len() < 8 {
            return None;
        }
        let mut frequency = [0u8; 4];
        let mut intensity = [0u8; 4];
        frequency.copy_from_slice(&data[0..4]);
        intensity.copy_from_slice(&data[4..8]);
        Some(Self {
            frequency,
            intensity,
        })
    }

    /// 编码为 16 字符的 HEX 字符串（用于 WebSocket 协议）
    pub fn to_hex_string(&self) -> String {
        let bytes = self.encode();
        bytes.iter().map(|b| format!("{b:02x}")).collect()
    }

    /// 从 16 字符 HEX 字符串解码
    pub fn from_hex_string(hex: &str) -> Option<Self> {
        if hex.len() != 16 {
            return None;
        }
        let mut bytes = [0u8; 8];
        for (i, byte) in bytes.iter_mut().enumerate() {
            *byte = u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).ok()?;
        }
        Self::decode(&bytes)
    }
}

impl Default for WaveformData {
    fn default() -> Self {
        Self::silent()
    }
}

/// B0 指令 - 写入通道强度变化和波形数据
///
/// 固定 20 字节，每 100ms 写入一次。两通道的数据都在同一条指令中。
///
/// 格式:
/// ```text
/// 0xB0 + [序列号(4bit)|强度解读(4bit)] + A强度(1) + B强度(1) +
/// A频率(4) + A强度(4) + B频率(4) + B强度(4)
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct B0Command {
    /// 序列号 (0~15)
    ///
    /// 如果修改了通道强度且需要设备反馈，设置 > 0。
    /// 设备会通过 B1 消息以相同序列号返回修改后的通道强度。
    pub sequence: u8,
    /// 强度值解读方式
    pub strength_mode: StrengthMode,
    /// A 通道强度设定值 (0~200)
    pub strength_a: u8,
    /// B 通道强度设定值 (0~200)
    pub strength_b: u8,
    /// A 通道波形数据
    pub waveform_a: WaveformData,
    /// B 通道波形数据
    pub waveform_b: WaveformData,
}

impl B0Command {
    /// 创建一个不修改强度的 B0 指令
    pub fn waveform_only(waveform_a: WaveformData, waveform_b: WaveformData) -> Self {
        Self {
            sequence: 0,
            strength_mode: StrengthMode::both_no_change(),
            strength_a: 0,
            strength_b: 0,
            waveform_a,
            waveform_b,
        }
    }

    /// 创建一个仅修改 A 通道强度的 B0 指令（绝对值）
    pub fn set_strength_a(value: u8, sequence: u8) -> Self {
        Self {
            sequence: sequence & 0x0F,
            strength_mode: StrengthMode::new(
                ChannelStrengthMode::Absolute,
                ChannelStrengthMode::NoChange,
            ),
            strength_a: value.min(MAX_STRENGTH),
            strength_b: 0,
            waveform_a: WaveformData::silent(),
            waveform_b: WaveformData::silent(),
        }
    }

    /// 创建一个仅修改 B 通道强度的 B0 指令（绝对值）
    pub fn set_strength_b(value: u8, sequence: u8) -> Self {
        Self {
            sequence: sequence & 0x0F,
            strength_mode: StrengthMode::new(
                ChannelStrengthMode::NoChange,
                ChannelStrengthMode::Absolute,
            ),
            strength_a: 0,
            strength_b: value.min(MAX_STRENGTH),
            waveform_a: WaveformData::silent(),
            waveform_b: WaveformData::silent(),
        }
    }

    /// 编码为 20 字节
    pub fn encode(&self) -> [u8; B0_LENGTH] {
        let mut buf = [0u8; B0_LENGTH];
        buf[0] = B0_HEAD;
        buf[1] = ((self.sequence & 0x0F) << 4) | (self.strength_mode.encode() & 0x0F);
        buf[2] = if self.strength_a <= MAX_STRENGTH {
            self.strength_a
        } else {
            0
        };
        buf[3] = if self.strength_b <= MAX_STRENGTH {
            self.strength_b
        } else {
            0
        };

        let wave_a = self.waveform_a.encode();
        let wave_b = self.waveform_b.encode();
        buf[4..12].copy_from_slice(&wave_a);
        buf[12..20].copy_from_slice(&wave_b);

        buf
    }

    /// 从 20 字节解码
    pub fn decode(data: &[u8]) -> Option<Self> {
        if data.len() < B0_LENGTH || data[0] != B0_HEAD {
            return None;
        }

        let sequence = (data[1] >> 4) & 0x0F;
        let strength_mode = StrengthMode::decode(data[1] & 0x0F);
        let strength_a = data[2];
        let strength_b = data[3];
        let waveform_a = WaveformData::decode(&data[4..12])?;
        let waveform_b = WaveformData::decode(&data[12..20])?;

        Some(Self {
            sequence,
            strength_mode,
            strength_a,
            strength_b,
            waveform_a,
            waveform_b,
        })
    }
}

/// BF 指令 - 设置通道强度软上限和平衡参数
///
/// 固定 7 字节。写入后直接生效，没有返回值。
/// **每次重连设备后必须重新写入 BF 指令设置软上限。**
///
/// 格式:
/// ```text
/// 0xBF + A软上限(1) + B软上限(1) + A频率平衡(1) + B频率平衡(1) +
/// A强度平衡(1) + B强度平衡(1)
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BFCommand {
    /// A 通道强度软上限 (0~200)，断电保存
    pub soft_limit_a: u8,
    /// B 通道强度软上限 (0~200)，断电保存
    pub soft_limit_b: u8,
    /// A 通道波形频率平衡参数 (0~255)，断电保存
    ///
    /// 控制固定通道强度下，不同频率波形的相对体感强度。值越大，低频波形冲击感越强。
    pub freq_balance_a: u8,
    /// B 通道波形频率平衡参数 (0~255)，断电保存
    pub freq_balance_b: u8,
    /// A 通道波形强度平衡参数 (0~255)，断电保存
    ///
    /// 控制固定通道强度下，不同频率波形的相对体感强度。值越大，低频波形刺激越强。
    pub intensity_balance_a: u8,
    /// B 通道波形强度平衡参数 (0~255)，断电保存
    pub intensity_balance_b: u8,
}

impl BFCommand {
    /// 创建默认配置（所有值 0，表示不修改软上限）
    pub fn default_config() -> Self {
        Self {
            soft_limit_a: MAX_STRENGTH,
            soft_limit_b: MAX_STRENGTH,
            freq_balance_a: 0,
            freq_balance_b: 0,
            intensity_balance_a: 0,
            intensity_balance_b: 0,
        }
    }

    /// 编码为 7 字节
    pub fn encode(&self) -> [u8; BF_LENGTH] {
        [
            BF_HEAD,
            self.soft_limit_a,
            self.soft_limit_b,
            self.freq_balance_a,
            self.freq_balance_b,
            self.intensity_balance_a,
            self.intensity_balance_b,
        ]
    }

    /// 从 7 字节解码
    pub fn decode(data: &[u8]) -> Option<Self> {
        if data.len() < BF_LENGTH || data[0] != BF_HEAD {
            return None;
        }

        Some(Self {
            soft_limit_a: data[1],
            soft_limit_b: data[2],
            freq_balance_a: data[3],
            freq_balance_b: data[4],
            intensity_balance_a: data[5],
            intensity_balance_b: data[6],
        })
    }
}

/// B1 回应消息 - 强度变化反馈
///
/// 当脉冲主机强度发生变化时，通过 Notify 特征返回。
///
/// 格式:
/// ```text
/// 0xB1 + 序列号(1) + A通道当前实际强度(1) + B通道当前实际强度(1)
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct B1Response {
    /// 序列号
    ///
    /// 如果是由 B0 指令导致的强度变化，序列号与引起变化的 B0 指令相同，否则为 0。
    pub sequence: u8,
    /// A 通道当前实际强度 (0~200)
    pub strength_a: u8,
    /// B 通道当前实际强度 (0~200)
    pub strength_b: u8,
}

impl B1Response {
    /// 从字节数据解码
    pub fn decode(data: &[u8]) -> Option<Self> {
        if data.len() < B1_LENGTH || data[0] != B1_HEAD {
            return None;
        }

        Some(Self {
            sequence: data[1],
            strength_a: data[2],
            strength_b: data[3],
        })
    }

    /// 编码为 4 字节（主要用于测试）
    pub fn encode(&self) -> [u8; B1_LENGTH] {
        [B1_HEAD, self.sequence, self.strength_a, self.strength_b]
    }
}

/// 从 Notify 特征接收到的消息类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotifyMessage {
    /// B1 强度反馈
    Strength(B1Response),
    /// 未知消息
    Unknown(Vec<u8>),
}

impl NotifyMessage {
    /// 从字节数据解析
    pub fn parse(data: &[u8]) -> Self {
        if data.is_empty() {
            return Self::Unknown(Vec::new());
        }

        match data[0] {
            B1_HEAD => {
                if let Some(resp) = B1Response::decode(data) {
                    Self::Strength(resp)
                } else {
                    Self::Unknown(data.to_vec())
                }
            }
            _ => Self::Unknown(data.to_vec()),
        }
    }
}

/// 将波形频率从用户输入范围 (10~1000) 压缩为发送值 (10~240)
///
/// 转换规则：
/// - 10~100: 直接使用
/// - 101~600: `(input - 100) / 5 + 100`
/// - 601~1000: `(input - 600) / 10 + 200`
/// - 超出范围: 返回 10
///
/// # 示例
///
/// ```
/// use dglab_protocol::v3::compress_frequency;
///
/// assert_eq!(compress_frequency(50), 50);
/// assert_eq!(compress_frequency(100), 100);
/// assert_eq!(compress_frequency(200), 120);
/// assert_eq!(compress_frequency(600), 200);
/// assert_eq!(compress_frequency(1000), 240);
/// ```
pub fn compress_frequency(input: u16) -> u8 {
    match input {
        10..=100 => input as u8,
        101..=600 => ((input - 100) / 5 + 100) as u8,
        601..=1000 => ((input - 600) / 10 + 200) as u8,
        _ => 10,
    }
}

/// 将发送值 (10~240) 解压为用户输入范围 (10~1000)
///
/// 这是 [`compress_frequency`] 的近似逆函数。由于压缩过程中有整数除法，
/// 解压后的值可能与原始值有偏差。
///
/// # 示例
///
/// ```
/// use dglab_protocol::v3::decompress_frequency;
///
/// assert_eq!(decompress_frequency(50), 50);
/// assert_eq!(decompress_frequency(100), 100);
/// assert_eq!(decompress_frequency(120), 200);
/// assert_eq!(decompress_frequency(200), 600);
/// assert_eq!(decompress_frequency(240), 1000);
/// ```
pub fn decompress_frequency(value: u8) -> u16 {
    match value {
        10..=100 => value as u16,
        101..=200 => ((value as u16) - 100) * 5 + 100,
        201..=240 => ((value as u16) - 200) * 10 + 600,
        _ => 10,
    }
}

/// 将脉冲频率 (Hz) 转换为波形频率 (ms)，再压缩为发送值
///
/// 脉冲频率 = 1000 / 波形频率(ms)
///
/// # 示例
///
/// ```
/// use dglab_protocol::v3::pulse_hz_to_value;
///
/// assert_eq!(pulse_hz_to_value(100), 10);  // 100Hz = 10ms
/// assert_eq!(pulse_hz_to_value(10), 100);  // 10Hz = 100ms
/// assert_eq!(pulse_hz_to_value(1), 240);   // 1Hz = 1000ms
/// ```
pub fn pulse_hz_to_value(hz: u16) -> u8 {
    if hz == 0 {
        return 10;
    }
    let ms = 1000u16.saturating_div(hz);
    compress_frequency(ms)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== StrengthMode 测试 ====================

    #[test]
    fn test_strength_mode_encode_decode() {
        let mode = StrengthMode::new(ChannelStrengthMode::Increase, ChannelStrengthMode::Decrease);
        let encoded = mode.encode();
        assert_eq!(encoded, 0b0110);
        let decoded = StrengthMode::decode(encoded);
        assert_eq!(decoded, mode);
    }

    #[test]
    fn test_strength_mode_all_combinations() {
        let modes = [
            ChannelStrengthMode::NoChange,
            ChannelStrengthMode::Increase,
            ChannelStrengthMode::Decrease,
            ChannelStrengthMode::Absolute,
        ];

        for &a_mode in &modes {
            for &b_mode in &modes {
                let mode = StrengthMode::new(a_mode, b_mode);
                let encoded = mode.encode();
                let decoded = StrengthMode::decode(encoded);
                assert_eq!(decoded, mode, "Roundtrip failed for {a_mode:?}/{b_mode:?}");
            }
        }
    }

    #[test]
    fn test_strength_mode_both_no_change() {
        let mode = StrengthMode::both_no_change();
        assert_eq!(mode.encode(), 0b0000);
    }

    // ==================== WaveformData 测试 ====================

    #[test]
    fn test_waveform_data_encode_decode() {
        let wave = WaveformData::new([10, 20, 30, 40], [50, 60, 70, 80]);
        let encoded = wave.encode();
        assert_eq!(encoded, [10, 20, 30, 40, 50, 60, 70, 80]);
        let decoded = WaveformData::decode(&encoded).unwrap();
        assert_eq!(decoded, wave);
    }

    #[test]
    fn test_waveform_data_silent() {
        let wave = WaveformData::silent();
        assert!(!wave.is_valid()); // 静默波形包含 intensity=101，不在有效范围
    }

    #[test]
    fn test_waveform_data_uniform() {
        let wave = WaveformData::uniform(50, 30);
        assert_eq!(wave.frequency, [50, 50, 50, 50]);
        assert_eq!(wave.intensity, [30, 30, 30, 30]);
        assert!(wave.is_valid());
    }

    #[test]
    fn test_waveform_data_valid() {
        assert!(WaveformData::new([10, 100, 240, 50], [0, 50, 100, 25]).is_valid());
        // 频率低于 10
        assert!(!WaveformData::new([9, 10, 10, 10], [0, 0, 0, 0]).is_valid());
        // 频率高于 240
        assert!(!WaveformData::new([10, 10, 10, 241], [0, 0, 0, 0]).is_valid());
        // 强度高于 100
        assert!(!WaveformData::new([10, 10, 10, 10], [0, 0, 0, 101]).is_valid());
    }

    #[test]
    fn test_waveform_data_hex_roundtrip() {
        let wave = WaveformData::new([0x0A, 0x14, 0x1E, 0x28], [0x00, 0x0A, 0x14, 0x1E]);
        let hex = wave.to_hex_string();
        assert_eq!(hex, "0a141e28000a141e");
        assert_eq!(hex.len(), 16);
        let decoded = WaveformData::from_hex_string(&hex).unwrap();
        assert_eq!(decoded, wave);
    }

    #[test]
    fn test_waveform_data_hex_invalid() {
        assert!(WaveformData::from_hex_string("").is_none());
        assert!(WaveformData::from_hex_string("0a141e28000a14").is_none()); // 14 chars
        assert!(WaveformData::from_hex_string("zz141e28000a141e").is_none()); // invalid hex
    }

    // ==================== B0Command 测试 ====================

    #[test]
    fn test_b0_encode_decode_roundtrip() {
        let cmd = B0Command {
            sequence: 5,
            strength_mode: StrengthMode::new(
                ChannelStrengthMode::Increase,
                ChannelStrengthMode::Decrease,
            ),
            strength_a: 10,
            strength_b: 20,
            waveform_a: WaveformData::new([10, 10, 10, 10], [0, 10, 20, 30]),
            waveform_b: WaveformData::new([15, 15, 15, 15], [40, 50, 60, 70]),
        };

        let encoded = cmd.encode();
        assert_eq!(encoded.len(), 20);
        assert_eq!(encoded[0], 0xB0);

        let decoded = B0Command::decode(&encoded).unwrap();
        assert_eq!(decoded, cmd);
    }

    #[test]
    fn test_b0_waveform_only() {
        let cmd = B0Command::waveform_only(WaveformData::uniform(10, 50), WaveformData::silent());
        assert_eq!(cmd.sequence, 0);
        assert_eq!(cmd.strength_mode, StrengthMode::both_no_change());
        assert_eq!(cmd.strength_a, 0);
        assert_eq!(cmd.strength_b, 0);
    }

    #[test]
    fn test_b0_set_strength_a() {
        let cmd = B0Command::set_strength_a(150, 3);
        assert_eq!(cmd.sequence, 3);
        assert_eq!(cmd.strength_a, 150);
        assert_eq!(cmd.strength_mode.channel_a, ChannelStrengthMode::Absolute);
        assert_eq!(cmd.strength_mode.channel_b, ChannelStrengthMode::NoChange);
    }

    #[test]
    fn test_b0_set_strength_b() {
        let cmd = B0Command::set_strength_b(100, 7);
        assert_eq!(cmd.sequence, 7);
        assert_eq!(cmd.strength_b, 100);
        assert_eq!(cmd.strength_mode.channel_a, ChannelStrengthMode::NoChange);
        assert_eq!(cmd.strength_mode.channel_b, ChannelStrengthMode::Absolute);
    }

    #[test]
    fn test_b0_strength_clamped_to_max() {
        let cmd = B0Command::set_strength_a(255, 1);
        assert_eq!(cmd.strength_a, MAX_STRENGTH); // Clamped to 200
    }

    #[test]
    fn test_b0_sequence_masked_to_4bits() {
        let cmd = B0Command::set_strength_a(50, 0xFF);
        assert_eq!(cmd.sequence, 0x0F); // Only lower 4 bits
    }

    #[test]
    fn test_b0_official_example_no1_1() {
        // 官方示例 No.1-1:
        // 不修改通道强度，A 通道连续输出波形
        // HEX: 0xB00000000A0A0A0A000A141E0000000000000065
        let cmd = B0Command {
            sequence: 0,
            strength_mode: StrengthMode::both_no_change(),
            strength_a: 0,
            strength_b: 0,
            waveform_a: WaveformData::new([10, 10, 10, 10], [0, 10, 20, 30]),
            waveform_b: WaveformData::new([0, 0, 0, 0], [0, 0, 0, 101]),
        };
        let encoded = cmd.encode();
        let expected: [u8; 20] = [
            0xB0, 0x00, 0x00, 0x00, 0x0A, 0x0A, 0x0A, 0x0A, 0x00, 0x0A, 0x14, 0x1E, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x65,
        ];
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_b0_official_example_no2_1() {
        // 官方示例 No.2-1:
        // A 通道强度 +5（当前 A=10），A 通道连续输出波形
        // HEX: 0xB00405000A0A0A0A000A141E0000000000000065
        let cmd = B0Command {
            sequence: 0,
            strength_mode: StrengthMode::new(
                ChannelStrengthMode::Increase,
                ChannelStrengthMode::NoChange,
            ),
            strength_a: 5,
            strength_b: 0,
            waveform_a: WaveformData::new([10, 10, 10, 10], [0, 10, 20, 30]),
            waveform_b: WaveformData::new([0, 0, 0, 0], [0, 0, 0, 101]),
        };
        let encoded = cmd.encode();
        let expected: [u8; 20] = [
            0xB0, 0x04, 0x05, 0x00, 0x0A, 0x0A, 0x0A, 0x0A, 0x00, 0x0A, 0x14, 0x1E, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x65,
        ];
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_b0_official_example_no2_4() {
        // 官方示例 No.2-4:
        // seq=1, A通道强度+10, A通道连续输出波形
        // HEX: 0xB0140A00283C5064645A5A5A0000000000000065
        let cmd = B0Command {
            sequence: 1,
            strength_mode: StrengthMode::new(
                ChannelStrengthMode::Increase,
                ChannelStrengthMode::NoChange,
            ),
            strength_a: 10,
            strength_b: 0,
            waveform_a: WaveformData::new([0x28, 0x3C, 0x50, 0x64], [0x64, 0x5A, 0x5A, 0x5A]),
            waveform_b: WaveformData::new([0, 0, 0, 0], [0, 0, 0, 0x65]),
        };
        let encoded = cmd.encode();
        let expected: [u8; 20] = [
            0xB0, 0x14, 0x0A, 0x00, 0x28, 0x3C, 0x50, 0x64, 0x64, 0x5A, 0x5A, 0x5A, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x65,
        ];
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_b0_official_example_no4_1() {
        // 官方示例 No.4-1:
        // AB 两通道均连续输出波形
        // HEX: 0xB00000000A0A0A0A000A141E0A0A0A0A00000000
        let cmd = B0Command {
            sequence: 0,
            strength_mode: StrengthMode::both_no_change(),
            strength_a: 0,
            strength_b: 0,
            waveform_a: WaveformData::new([0x0A, 0x0A, 0x0A, 0x0A], [0x00, 0x0A, 0x14, 0x1E]),
            waveform_b: WaveformData::new([0x0A, 0x0A, 0x0A, 0x0A], [0x00, 0x00, 0x00, 0x00]),
        };
        let encoded = cmd.encode();
        let expected: [u8; 20] = [
            0xB0, 0x00, 0x00, 0x00, 0x0A, 0x0A, 0x0A, 0x0A, 0x00, 0x0A, 0x14, 0x1E, 0x0A, 0x0A,
            0x0A, 0x0A, 0x00, 0x00, 0x00, 0x00,
        ];
        assert_eq!(encoded, expected);
    }

    // ==================== BFCommand 测试 ====================

    #[test]
    fn test_bf_encode_decode_roundtrip() {
        let cmd = BFCommand {
            soft_limit_a: 150,
            soft_limit_b: 100,
            freq_balance_a: 50,
            freq_balance_b: 60,
            intensity_balance_a: 70,
            intensity_balance_b: 80,
        };

        let encoded = cmd.encode();
        assert_eq!(encoded.len(), 7);
        assert_eq!(encoded[0], 0xBF);

        let decoded = BFCommand::decode(&encoded).unwrap();
        assert_eq!(decoded, cmd);
    }

    #[test]
    fn test_bf_default_config() {
        let cmd = BFCommand::default_config();
        assert_eq!(cmd.soft_limit_a, 200);
        assert_eq!(cmd.soft_limit_b, 200);
        assert_eq!(cmd.freq_balance_a, 0);
        assert_eq!(cmd.freq_balance_b, 0);
    }

    // ==================== B1Response 测试 ====================

    #[test]
    fn test_b1_decode() {
        let data = [0xB1, 0x01, 0x0A, 0x14];
        let resp = B1Response::decode(&data).unwrap();
        assert_eq!(resp.sequence, 1);
        assert_eq!(resp.strength_a, 10);
        assert_eq!(resp.strength_b, 20);
    }

    #[test]
    fn test_b1_encode_decode_roundtrip() {
        let resp = B1Response {
            sequence: 5,
            strength_a: 100,
            strength_b: 150,
        };
        let encoded = resp.encode();
        let decoded = B1Response::decode(&encoded).unwrap();
        assert_eq!(decoded, resp);
    }

    #[test]
    fn test_b1_invalid() {
        assert!(B1Response::decode(&[]).is_none());
        assert!(B1Response::decode(&[0xB0, 0, 0, 0]).is_none()); // Wrong head
        assert!(B1Response::decode(&[0xB1, 0, 0]).is_none()); // Too short
    }

    // ==================== NotifyMessage 测试 ====================

    #[test]
    fn test_notify_message_b1() {
        let data = [0xB1, 0x02, 0x0F, 0x1E];
        let msg = NotifyMessage::parse(&data);
        match msg {
            NotifyMessage::Strength(resp) => {
                assert_eq!(resp.sequence, 2);
                assert_eq!(resp.strength_a, 15);
                assert_eq!(resp.strength_b, 30);
            }
            _ => panic!("Expected Strength"),
        }
    }

    #[test]
    fn test_notify_message_unknown() {
        let data = [0xCC, 0x01, 0x02];
        let msg = NotifyMessage::parse(&data);
        assert!(matches!(msg, NotifyMessage::Unknown(_)));
    }

    #[test]
    fn test_notify_message_empty() {
        let msg = NotifyMessage::parse(&[]);
        assert!(matches!(msg, NotifyMessage::Unknown(_)));
    }

    // ==================== 频率转换测试 ====================

    #[test]
    fn test_compress_frequency_direct_range() {
        // 10-100 直接映射
        assert_eq!(compress_frequency(10), 10);
        assert_eq!(compress_frequency(50), 50);
        assert_eq!(compress_frequency(100), 100);
    }

    #[test]
    fn test_compress_frequency_mid_range() {
        // 101-600: (input - 100) / 5 + 100
        assert_eq!(compress_frequency(101), 100); // (101-100)/5 + 100 = 100
        assert_eq!(compress_frequency(200), 120); // (200-100)/5 + 100 = 120
        assert_eq!(compress_frequency(350), 150); // (350-100)/5 + 100 = 150
        assert_eq!(compress_frequency(600), 200); // (600-100)/5 + 100 = 200
    }

    #[test]
    fn test_compress_frequency_high_range() {
        // 601-1000: (input - 600) / 10 + 200
        assert_eq!(compress_frequency(601), 200); // (601-600)/10 + 200 = 200
        assert_eq!(compress_frequency(700), 210); // (700-600)/10 + 200 = 210
        assert_eq!(compress_frequency(800), 220); // (800-600)/10 + 200 = 220
        assert_eq!(compress_frequency(1000), 240); // (1000-600)/10 + 200 = 240
    }

    #[test]
    fn test_compress_frequency_out_of_range() {
        assert_eq!(compress_frequency(0), 10);
        assert_eq!(compress_frequency(5), 10);
        assert_eq!(compress_frequency(1001), 10);
        assert_eq!(compress_frequency(u16::MAX), 10);
    }

    #[test]
    fn test_decompress_frequency() {
        assert_eq!(decompress_frequency(10), 10);
        assert_eq!(decompress_frequency(50), 50);
        assert_eq!(decompress_frequency(100), 100);
        assert_eq!(decompress_frequency(120), 200);
        assert_eq!(decompress_frequency(200), 600);
        assert_eq!(decompress_frequency(240), 1000);
    }

    #[test]
    fn test_compress_decompress_roundtrip_direct() {
        // Direct range should roundtrip exactly
        for i in 10..=100u16 {
            assert_eq!(decompress_frequency(compress_frequency(i)), i);
        }
    }

    #[test]
    fn test_pulse_hz_to_value() {
        assert_eq!(pulse_hz_to_value(100), 10); // 100Hz = 10ms
        assert_eq!(pulse_hz_to_value(50), 20); // 50Hz = 20ms
        assert_eq!(pulse_hz_to_value(10), 100); // 10Hz = 100ms
        assert_eq!(pulse_hz_to_value(1), 240); // 1Hz = 1000ms
        assert_eq!(pulse_hz_to_value(0), 10); // Edge case: 0Hz
    }

    // ==================== 官方文档中的强度解读方式示例 ====================

    #[test]
    fn test_official_strength_mode_examples() {
        // 示例 1: 0b0000 -> 两通道都不变
        let mode = StrengthMode::decode(0b0000);
        assert_eq!(mode.channel_a, ChannelStrengthMode::NoChange);
        assert_eq!(mode.channel_b, ChannelStrengthMode::NoChange);

        // 示例 2: 0b0100 -> A增加, B不变
        let mode = StrengthMode::decode(0b0100);
        assert_eq!(mode.channel_a, ChannelStrengthMode::Increase);
        assert_eq!(mode.channel_b, ChannelStrengthMode::NoChange);

        // 示例 3: 0b0010 -> A不变, B减少
        let mode = StrengthMode::decode(0b0010);
        assert_eq!(mode.channel_a, ChannelStrengthMode::NoChange);
        assert_eq!(mode.channel_b, ChannelStrengthMode::Decrease);

        // 示例 4: 0b0011 -> A不变, B绝对
        let mode = StrengthMode::decode(0b0011);
        assert_eq!(mode.channel_a, ChannelStrengthMode::NoChange);
        assert_eq!(mode.channel_b, ChannelStrengthMode::Absolute);

        // 示例 5: 0b0110 -> A增加, B减少
        let mode = StrengthMode::decode(0b0110);
        assert_eq!(mode.channel_a, ChannelStrengthMode::Increase);
        assert_eq!(mode.channel_b, ChannelStrengthMode::Decrease);

        // 示例 6: 0b1101 -> A绝对, B增加
        let mode = StrengthMode::decode(0b1101);
        assert_eq!(mode.channel_a, ChannelStrengthMode::Absolute);
        assert_eq!(mode.channel_b, ChannelStrengthMode::Increase);
    }
}
