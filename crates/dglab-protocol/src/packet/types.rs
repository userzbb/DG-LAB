//! 数据包类型定义

use serde::{Deserialize, Serialize};

/// 数据包头部
pub const PACKET_HEADER: u8 = 0xAA;

/// 数据包尾部
pub const PACKET_TAIL: u8 = 0x55;

/// 命令类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum CommandType {
    /// 获取设备信息
    GetInfo = 0x01,
    /// 设置通道 A 强度
    SetPowerA = 0x10,
    /// 设置通道 B 强度
    SetPowerB = 0x11,
    /// 设置通道 A 波形
    SetWaveA = 0x12,
    /// 设置通道 B 波形
    SetWaveB = 0x13,
    /// 设置模式
    SetMode = 0x14,
    /// 开始输出
    Start = 0x20,
    /// 停止输出
    Stop = 0x21,
    /// 心跳
    Heartbeat = 0x30,
    /// 设备响应
    Response = 0x80,
    /// 未知命令
    Unknown = 0xFF,
}

impl From<u8> for CommandType {
    fn from(value: u8) -> Self {
        match value {
            0x01 => CommandType::GetInfo,
            0x10 => CommandType::SetPowerA,
            0x11 => CommandType::SetPowerB,
            0x12 => CommandType::SetWaveA,
            0x13 => CommandType::SetWaveB,
            0x14 => CommandType::SetMode,
            0x20 => CommandType::Start,
            0x21 => CommandType::Stop,
            0x30 => CommandType::Heartbeat,
            0x80 => CommandType::Response,
            _ => CommandType::Unknown,
        }
    }
}

impl From<CommandType> for u8 {
    fn from(cmd: CommandType) -> Self {
        cmd as u8
    }
}

/// 波形类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum WaveformType {
    /// 连续波
    Continuous = 0x01,
    /// 脉冲波
    Pulse = 0x02,
    /// 锯齿波
    Sawtooth = 0x03,
    /// 正弦波
    Sine = 0x04,
    /// 方波
    Square = 0x05,
    /// 三角波
    Triangle = 0x06,
    /// 自定义
    Custom = 0xFF,
}

impl From<u8> for WaveformType {
    fn from(value: u8) -> Self {
        match value {
            0x01 => WaveformType::Continuous,
            0x02 => WaveformType::Pulse,
            0x03 => WaveformType::Sawtooth,
            0x04 => WaveformType::Sine,
            0x05 => WaveformType::Square,
            0x06 => WaveformType::Triangle,
            _ => WaveformType::Custom,
        }
    }
}

impl From<WaveformType> for u8 {
    fn from(wave: WaveformType) -> Self {
        wave as u8
    }
}

/// 工作模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum WorkMode {
    /// 自动模式
    Auto = 0x01,
    /// 手动模式
    Manual = 0x02,
    /// 循环模式
    Loop = 0x03,
    /// 随机模式
    Random = 0x04,
}

impl From<u8> for WorkMode {
    fn from(value: u8) -> Self {
        match value {
            0x01 => WorkMode::Auto,
            0x02 => WorkMode::Manual,
            0x03 => WorkMode::Loop,
            0x04 => WorkMode::Random,
            _ => WorkMode::Manual,
        }
    }
}

impl From<WorkMode> for u8 {
    fn from(mode: WorkMode) -> Self {
        mode as u8
    }
}

/// 数据包
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Packet {
    /// 命令类型
    pub command: CommandType,
    /// 数据长度
    pub data_len: u8,
    /// 数据载荷
    pub data: Vec<u8>,
    /// 校验和
    pub checksum: u8,
}

impl Packet {
    /// 创建新的数据包
    pub fn new(command: CommandType, data: Vec<u8>) -> Self {
        let data_len = data.len() as u8;
        let checksum = Self::calculate_checksum(command, data_len, &data);

        Self {
            command,
            data_len,
            data,
            checksum,
        }
    }

    /// 计算校验和
    pub fn calculate_checksum(command: CommandType, data_len: u8, data: &[u8]) -> u8 {
        let mut sum = PACKET_HEADER;
        sum = sum.wrapping_add(command as u8);
        sum = sum.wrapping_add(data_len);
        for &byte in data {
            sum = sum.wrapping_add(byte);
        }
        sum = sum.wrapping_add(PACKET_TAIL);
        sum
    }

    /// 验证校验和
    pub fn verify_checksum(&self) -> bool {
        self.checksum == Self::calculate_checksum(self.command, self.data_len, &self.data)
    }
}

/// 设备信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    /// 设备名称
    pub name: String,
    /// 固件版本
    pub firmware_version: String,
    /// 硬件版本
    pub hardware_version: String,
    /// 电池电量 (0-100)
    pub battery_level: u8,
    /// 通道 A 当前强度
    pub power_a: u8,
    /// 通道 B 当前强度
    pub power_b: u8,
    /// 通道 A 最大强度
    pub max_power_a: u8,
    /// 通道 B 最大强度
    pub max_power_b: u8,
    /// 当前工作模式
    pub work_mode: WorkMode,
}

impl Default for DeviceInfo {
    fn default() -> Self {
        Self {
            name: "Unknown".to_string(),
            firmware_version: "1.0.0".to_string(),
            hardware_version: "1.0".to_string(),
            battery_level: 100,
            power_a: 0,
            power_b: 0,
            max_power_a: 100,
            max_power_b: 100,
            work_mode: WorkMode::Manual,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === CommandType 测试 ===

    #[test]
    fn test_command_type_from_u8_known_values() {
        assert_eq!(CommandType::from(0x01), CommandType::GetInfo);
        assert_eq!(CommandType::from(0x10), CommandType::SetPowerA);
        assert_eq!(CommandType::from(0x11), CommandType::SetPowerB);
        assert_eq!(CommandType::from(0x12), CommandType::SetWaveA);
        assert_eq!(CommandType::from(0x13), CommandType::SetWaveB);
        assert_eq!(CommandType::from(0x14), CommandType::SetMode);
        assert_eq!(CommandType::from(0x20), CommandType::Start);
        assert_eq!(CommandType::from(0x21), CommandType::Stop);
        assert_eq!(CommandType::from(0x30), CommandType::Heartbeat);
        assert_eq!(CommandType::from(0x80), CommandType::Response);
    }

    #[test]
    fn test_command_type_unknown_values() {
        assert_eq!(CommandType::from(0x00), CommandType::Unknown);
        assert_eq!(CommandType::from(0x02), CommandType::Unknown);
        assert_eq!(CommandType::from(0xFE), CommandType::Unknown);
        assert_eq!(CommandType::from(0xFF), CommandType::Unknown);
    }

    #[test]
    fn test_command_type_roundtrip() {
        let commands = [
            CommandType::GetInfo,
            CommandType::SetPowerA,
            CommandType::SetPowerB,
            CommandType::SetWaveA,
            CommandType::SetWaveB,
            CommandType::SetMode,
            CommandType::Start,
            CommandType::Stop,
            CommandType::Heartbeat,
            CommandType::Response,
        ];
        for cmd in commands {
            let byte: u8 = cmd.into();
            let back = CommandType::from(byte);
            assert_eq!(back, cmd, "CommandType roundtrip failed for {:?}", cmd);
        }
    }

    // === WaveformType 测试 ===

    #[test]
    fn test_waveform_type_from_u8_known_values() {
        assert_eq!(WaveformType::from(0x01), WaveformType::Continuous);
        assert_eq!(WaveformType::from(0x02), WaveformType::Pulse);
        assert_eq!(WaveformType::from(0x03), WaveformType::Sawtooth);
        assert_eq!(WaveformType::from(0x04), WaveformType::Sine);
        assert_eq!(WaveformType::from(0x05), WaveformType::Square);
        assert_eq!(WaveformType::from(0x06), WaveformType::Triangle);
    }

    #[test]
    fn test_waveform_type_unknown_maps_to_custom() {
        assert_eq!(WaveformType::from(0x00), WaveformType::Custom);
        assert_eq!(WaveformType::from(0x07), WaveformType::Custom);
        assert_eq!(WaveformType::from(0xFF), WaveformType::Custom);
    }

    #[test]
    fn test_waveform_type_roundtrip() {
        let waveforms = [
            WaveformType::Continuous,
            WaveformType::Pulse,
            WaveformType::Sawtooth,
            WaveformType::Sine,
            WaveformType::Square,
            WaveformType::Triangle,
        ];
        for wf in waveforms {
            let byte: u8 = wf.into();
            let back = WaveformType::from(byte);
            assert_eq!(back, wf, "WaveformType roundtrip failed for {:?}", wf);
        }
    }

    // === WorkMode 测试 ===

    #[test]
    fn test_work_mode_from_u8_known_values() {
        assert_eq!(WorkMode::from(0x01), WorkMode::Auto);
        assert_eq!(WorkMode::from(0x02), WorkMode::Manual);
        assert_eq!(WorkMode::from(0x03), WorkMode::Loop);
        assert_eq!(WorkMode::from(0x04), WorkMode::Random);
    }

    #[test]
    fn test_work_mode_unknown_defaults_to_manual() {
        assert_eq!(WorkMode::from(0x00), WorkMode::Manual);
        assert_eq!(WorkMode::from(0x05), WorkMode::Manual);
        assert_eq!(WorkMode::from(0xFF), WorkMode::Manual);
    }

    #[test]
    fn test_work_mode_roundtrip() {
        let modes = [
            WorkMode::Auto,
            WorkMode::Manual,
            WorkMode::Loop,
            WorkMode::Random,
        ];
        for mode in modes {
            let byte: u8 = mode.into();
            let back = WorkMode::from(byte);
            assert_eq!(back, mode, "WorkMode roundtrip failed for {:?}", mode);
        }
    }

    // === Packet 测试 ===

    #[test]
    fn test_packet_new_sets_data_len_and_checksum() {
        let packet = Packet::new(CommandType::Start, Vec::new());
        assert_eq!(packet.data_len, 0);
        assert!(packet.verify_checksum());

        let packet = Packet::new(CommandType::SetPowerA, vec![50]);
        assert_eq!(packet.data_len, 1);
        assert_eq!(packet.data, vec![50]);
        assert!(packet.verify_checksum());
    }

    #[test]
    fn test_packet_new_with_multi_byte_data() {
        let data = vec![0x01, 0x02, 0x03, 0x04];
        let packet = Packet::new(CommandType::SetWaveA, data.clone());
        assert_eq!(packet.data_len, 4);
        assert_eq!(packet.data, data);
        assert!(packet.verify_checksum());
    }

    #[test]
    fn test_packet_checksum_calculation() {
        // 手动验证: HEADER + command + data_len + data_bytes + TAIL
        // 0xAA + 0x20 (Start) + 0x00 (no data) + 0x55 (tail)
        let expected = 0xAAu8
            .wrapping_add(0x20)
            .wrapping_add(0x00)
            .wrapping_add(0x55);
        let checksum = Packet::calculate_checksum(CommandType::Start, 0, &[]);
        assert_eq!(checksum, expected);
    }

    #[test]
    fn test_packet_checksum_with_data() {
        // 0xAA + 0x10 (SetPowerA) + 0x01 (len) + 0x32 (data=50) + 0x55
        let expected = 0xAAu8
            .wrapping_add(0x10)
            .wrapping_add(0x01)
            .wrapping_add(0x32)
            .wrapping_add(0x55);
        let checksum = Packet::calculate_checksum(CommandType::SetPowerA, 1, &[0x32]);
        assert_eq!(checksum, expected);
    }

    #[test]
    fn test_packet_verify_checksum_invalid() {
        let mut packet = Packet::new(CommandType::Start, Vec::new());
        packet.checksum = packet.checksum.wrapping_add(1); // 破坏校验和
        assert!(!packet.verify_checksum());
    }

    // === DeviceInfo 测试 ===

    #[test]
    fn test_device_info_default() {
        let info = DeviceInfo::default();
        assert_eq!(info.name, "Unknown");
        assert_eq!(info.firmware_version, "1.0.0");
        assert_eq!(info.hardware_version, "1.0");
        assert_eq!(info.battery_level, 100);
        assert_eq!(info.power_a, 0);
        assert_eq!(info.power_b, 0);
        assert_eq!(info.max_power_a, 100);
        assert_eq!(info.max_power_b, 100);
        assert_eq!(info.work_mode, WorkMode::Manual);
    }

    // === 序列化测试 ===

    #[test]
    fn test_packet_serde_roundtrip() {
        let packet = Packet::new(CommandType::SetPowerA, vec![75]);
        let json = serde_json::to_string(&packet).unwrap();
        let deserialized: Packet = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.command, packet.command);
        assert_eq!(deserialized.data_len, packet.data_len);
        assert_eq!(deserialized.data, packet.data);
        assert_eq!(deserialized.checksum, packet.checksum);
    }

    #[test]
    fn test_device_info_serde_roundtrip() {
        let info = DeviceInfo::default();
        let json = serde_json::to_string(&info).unwrap();
        let deserialized: DeviceInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.name, info.name);
        assert_eq!(deserialized.battery_level, info.battery_level);
        assert_eq!(deserialized.work_mode, info.work_mode);
    }

    // === 常量测试 ===

    #[test]
    fn test_packet_constants() {
        assert_eq!(PACKET_HEADER, 0xAA);
        assert_eq!(PACKET_TAIL, 0x55);
    }
}
