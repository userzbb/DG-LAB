//! 数据包解码器

use crate::error::{ProtocolError, Result};
use crate::packet::types::{CommandType, DeviceInfo, Packet, WorkMode, PACKET_HEADER, PACKET_TAIL};

/// 数据包解码器
pub struct PacketDecoder {
    /// 缓冲区
    buffer: Vec<u8>,
}

impl PacketDecoder {
    /// 创建新的解码器
    pub fn new() -> Self {
        Self {
            buffer: Vec::with_capacity(256),
        }
    }

    /// 输入数据
    pub fn feed(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
    }

    /// 尝试解码数据包
    pub fn try_decode(&mut self) -> Result<Option<Packet>> {
        // 查找头部
        let Some(header_pos) = self.buffer.iter().position(|&b| b == PACKET_HEADER) else {
            // 没有找到头部，清空缓冲区
            self.buffer.clear();
            return Ok(None);
        };

        // 移除头部之前的数据
        if header_pos > 0 {
            self.buffer.drain(0..header_pos);
        }

        // 检查是否有足够的数据
        if self.buffer.len() < 4 {
            return Ok(None);
        }

        let command = CommandType::from(self.buffer[1]);
        let data_len = self.buffer[2] as usize;

        // 检查是否有完整的数据包
        // 格式: HEADER(1) + command(1) + data_len(1) + data(N) + checksum(1) + TAIL(1) = 5 + N
        let total_len = 5 + data_len;
        if self.buffer.len() < total_len {
            return Ok(None);
        }

        // 验证尾部
        if self.buffer[total_len - 1] != PACKET_TAIL {
            // 尾部不匹配，跳过头部重试
            self.buffer.drain(0..1);
            return self.try_decode();
        }

        // 提取数据
        let data = self.buffer[3..3 + data_len].to_vec();
        let checksum = self.buffer[3 + data_len];

        // 从缓冲区移除已解码的数据
        self.buffer.drain(0..total_len);

        // 创建数据包
        let packet = Packet {
            command,
            data_len: data_len as u8,
            data,
            checksum,
        };

        // 验证校验和
        if !packet.verify_checksum() {
            return Err(ProtocolError::DecodeError("Checksum mismatch".to_string()));
        }

        Ok(Some(packet))
    }

    /// 解码所有可用的数据包
    pub fn decode_all(&mut self) -> Result<Vec<Packet>> {
        let mut packets = Vec::new();
        while let Some(packet) = self.try_decode()? {
            packets.push(packet);
        }
        Ok(packets)
    }

    /// 清空缓冲区
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    /// 解码设备信息响应
    pub fn decode_device_info(packet: &Packet) -> Result<DeviceInfo> {
        if packet.command != CommandType::Response {
            return Err(ProtocolError::DecodeError(
                "Not a response packet".to_string(),
            ));
        }

        let data = &packet.data;
        if data.len() < 16 {
            return Err(ProtocolError::DecodeError(
                "Insufficient data length".to_string(),
            ));
        }

        // 解析设备信息（这是一个示例实现，需要根据实际协议调整）
        let name = String::from_utf8_lossy(&data[0..8])
            .trim_matches('\0')
            .to_string();
        let firmware_version = format!("{}.{}.{}", data[8], data[9], data[10]);
        let hardware_version = format!("{}.{}", data[11], data[12]);
        let battery_level = data[13];
        let power_a = data[14];
        let power_b = data[15];
        let max_power_a = if data.len() > 16 { data[16] } else { 100 };
        let max_power_b = if data.len() > 17 { data[17] } else { 100 };
        let work_mode = if data.len() > 18 {
            WorkMode::from(data[18])
        } else {
            WorkMode::Manual
        };

        Ok(DeviceInfo {
            name: if name.is_empty() {
                "DG-LAB".to_string()
            } else {
                name
            },
            firmware_version,
            hardware_version,
            battery_level,
            power_a,
            power_b,
            max_power_a,
            max_power_b,
            work_mode,
        })
    }
}

impl Default for PacketDecoder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::encoder::PacketEncoder;

    /// 辅助函数：编码一个数据包为字节
    fn encode_packet(command: CommandType, data: Vec<u8>) -> Vec<u8> {
        let packet = Packet::new(command, data);
        PacketEncoder::encode(&packet).unwrap()
    }

    // === try_decode 测试 ===

    #[test]
    fn test_decode_valid_packet_no_data() {
        let mut decoder = PacketDecoder::new();
        let bytes = encode_packet(CommandType::Start, Vec::new());
        decoder.feed(&bytes);

        let packet = decoder.try_decode().unwrap().unwrap();
        assert_eq!(packet.command, CommandType::Start);
        assert_eq!(packet.data_len, 0);
        assert!(packet.data.is_empty());
        assert!(packet.verify_checksum());
    }

    #[test]
    fn test_decode_valid_packet_with_data() {
        let mut decoder = PacketDecoder::new();
        let bytes = encode_packet(CommandType::SetPowerA, vec![75]);
        decoder.feed(&bytes);

        let packet = decoder.try_decode().unwrap().unwrap();
        assert_eq!(packet.command, CommandType::SetPowerA);
        assert_eq!(packet.data_len, 1);
        assert_eq!(packet.data, vec![75]);
        assert!(packet.verify_checksum());
    }

    #[test]
    fn test_decode_valid_packet_with_multi_byte_data() {
        let mut decoder = PacketDecoder::new();
        let data = vec![0x01, 0x10, 0x20, 0x30];
        let bytes = encode_packet(CommandType::SetWaveA, data.clone());
        decoder.feed(&bytes);

        let packet = decoder.try_decode().unwrap().unwrap();
        assert_eq!(packet.command, CommandType::SetWaveA);
        assert_eq!(packet.data, data);
    }

    #[test]
    fn test_decode_empty_buffer_returns_none() {
        let mut decoder = PacketDecoder::new();
        let result = decoder.try_decode().unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_decode_incomplete_packet_returns_none() {
        let mut decoder = PacketDecoder::new();
        // 只发头部和命令字节，不够完整
        decoder.feed(&[PACKET_HEADER, 0x20]);
        let result = decoder.try_decode().unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_decode_incomplete_data_returns_none() {
        let mut decoder = PacketDecoder::new();
        // HEADER + command + data_len=5，但实际只给了 2 字节数据
        decoder.feed(&[PACKET_HEADER, 0x12, 0x05, 0x01, 0x02]);
        let result = decoder.try_decode().unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_decode_no_header_clears_buffer() {
        let mut decoder = PacketDecoder::new();
        decoder.feed(&[0x00, 0x01, 0x02, 0x03]);
        let result = decoder.try_decode().unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_decode_garbage_before_valid_packet() {
        let mut decoder = PacketDecoder::new();
        let valid = encode_packet(CommandType::Heartbeat, Vec::new());
        // 先放垃圾数据，再放有效数据包
        let mut bytes = vec![0x00, 0x01, 0x02];
        bytes.extend_from_slice(&valid);
        decoder.feed(&bytes);

        let packet = decoder.try_decode().unwrap().unwrap();
        assert_eq!(packet.command, CommandType::Heartbeat);
    }

    #[test]
    fn test_decode_invalid_tail_skips_and_retries() {
        let mut decoder = PacketDecoder::new();
        // 构建一个尾部错误的"假包"，然后跟一个真包
        // 假包: HEADER + cmd + len=0 + checksum + 错误tail
        let mut bytes = vec![PACKET_HEADER, 0x20, 0x00, 0x00, 0xBB]; // 错误的tail
        let valid = encode_packet(CommandType::Stop, Vec::new());
        bytes.extend_from_slice(&valid);
        decoder.feed(&bytes);

        // 第一次解码应该跳过假包，找到真包
        let packet = decoder.try_decode().unwrap().unwrap();
        assert_eq!(packet.command, CommandType::Stop);
    }

    #[test]
    fn test_decode_checksum_mismatch_returns_error() {
        let mut decoder = PacketDecoder::new();
        // 手动构建一个校验和错误的数据包
        let real_checksum = Packet::calculate_checksum(CommandType::Start, 0, &[]);
        let bad_checksum = real_checksum.wrapping_add(1);
        decoder.feed(&[PACKET_HEADER, 0x20, 0x00, bad_checksum, PACKET_TAIL]);

        let result = decoder.try_decode();
        assert!(result.is_err());
    }

    // === 增量 feed 测试 ===

    #[test]
    fn test_decode_incremental_feed() {
        let mut decoder = PacketDecoder::new();
        let bytes = encode_packet(CommandType::SetMode, vec![0x02]);

        // 分两次 feed
        let mid = bytes.len() / 2;
        decoder.feed(&bytes[..mid]);
        assert!(decoder.try_decode().unwrap().is_none()); // 还不完整

        decoder.feed(&bytes[mid..]);
        let packet = decoder.try_decode().unwrap().unwrap();
        assert_eq!(packet.command, CommandType::SetMode);
        assert_eq!(packet.data, vec![0x02]);
    }

    // === decode_all 测试 ===

    #[test]
    fn test_decode_all_multiple_packets() {
        let mut decoder = PacketDecoder::new();
        let pkt1 = encode_packet(CommandType::Start, Vec::new());
        let pkt2 = encode_packet(CommandType::SetPowerA, vec![50]);
        let pkt3 = encode_packet(CommandType::Heartbeat, Vec::new());

        let mut bytes = Vec::new();
        bytes.extend_from_slice(&pkt1);
        bytes.extend_from_slice(&pkt2);
        bytes.extend_from_slice(&pkt3);
        decoder.feed(&bytes);

        let packets = decoder.decode_all().unwrap();
        assert_eq!(packets.len(), 3);
        assert_eq!(packets[0].command, CommandType::Start);
        assert_eq!(packets[1].command, CommandType::SetPowerA);
        assert_eq!(packets[1].data, vec![50]);
        assert_eq!(packets[2].command, CommandType::Heartbeat);
    }

    #[test]
    fn test_decode_all_empty_buffer() {
        let mut decoder = PacketDecoder::new();
        let packets = decoder.decode_all().unwrap();
        assert!(packets.is_empty());
    }

    // === clear 测试 ===

    #[test]
    fn test_clear_buffer() {
        let mut decoder = PacketDecoder::new();
        decoder.feed(&[PACKET_HEADER, 0x20, 0x00]);
        decoder.clear();

        // 清空后应该什么都解不出来
        let result = decoder.try_decode().unwrap();
        assert!(result.is_none());
    }

    // === Default trait 测试 ===

    #[test]
    fn test_default_decoder() {
        let mut decoder = PacketDecoder::default();
        let bytes = encode_packet(CommandType::Start, Vec::new());
        decoder.feed(&bytes);
        let packet = decoder.try_decode().unwrap().unwrap();
        assert_eq!(packet.command, CommandType::Start);
    }

    // === encode-decode 往返测试 ===

    #[test]
    fn test_roundtrip_all_simple_commands() {
        let commands = [
            CommandType::GetInfo,
            CommandType::Start,
            CommandType::Stop,
            CommandType::Heartbeat,
        ];

        for cmd in commands {
            let mut decoder = PacketDecoder::new();
            let bytes = encode_packet(cmd, Vec::new());
            decoder.feed(&bytes);
            let packet = decoder.try_decode().unwrap().unwrap();
            assert_eq!(packet.command, cmd, "Roundtrip failed for {:?}", cmd);
            assert!(packet.verify_checksum());
        }
    }

    #[test]
    fn test_roundtrip_with_data() {
        let mut decoder = PacketDecoder::new();
        let data = vec![0xDE, 0xAD, 0xBE, 0xEF];
        let bytes = encode_packet(CommandType::SetWaveB, data.clone());
        decoder.feed(&bytes);
        let packet = decoder.try_decode().unwrap().unwrap();
        assert_eq!(packet.command, CommandType::SetWaveB);
        assert_eq!(packet.data, data);
        assert!(packet.verify_checksum());
    }

    // === decode_device_info 测试 ===

    #[test]
    fn test_decode_device_info_valid_minimal() {
        // 构建一个最小的设备信息响应 (16 字节)
        let mut data = vec![0u8; 16];
        // name: "DG-LAB\0\0" (8 bytes)
        data[0..6].copy_from_slice(b"DG-LAB");
        // firmware: 2.1.3
        data[8] = 2;
        data[9] = 1;
        data[10] = 3;
        // hardware: 1.5
        data[11] = 1;
        data[12] = 5;
        // battery: 85%
        data[13] = 85;
        // power_a: 30
        data[14] = 30;
        // power_b: 45
        data[15] = 45;

        let packet = Packet::new(CommandType::Response, data);
        let info = PacketDecoder::decode_device_info(&packet).unwrap();

        assert_eq!(info.name, "DG-LAB");
        assert_eq!(info.firmware_version, "2.1.3");
        assert_eq!(info.hardware_version, "1.5");
        assert_eq!(info.battery_level, 85);
        assert_eq!(info.power_a, 30);
        assert_eq!(info.power_b, 45);
        // 没有扩展数据，使用默认值
        assert_eq!(info.max_power_a, 100);
        assert_eq!(info.max_power_b, 100);
        assert_eq!(info.work_mode, WorkMode::Manual);
    }

    #[test]
    fn test_decode_device_info_with_extended_data() {
        // 19 字节: 包含 max_power 和 work_mode
        let mut data = vec![0u8; 19];
        data[0..4].copy_from_slice(b"Test");
        data[8] = 1;
        data[9] = 0;
        data[10] = 0;
        data[11] = 2;
        data[12] = 0;
        data[13] = 100;
        data[14] = 0;
        data[15] = 0;
        data[16] = 80; // max_power_a
        data[17] = 90; // max_power_b
        data[18] = 0x03; // WorkMode::Loop

        let packet = Packet::new(CommandType::Response, data);
        let info = PacketDecoder::decode_device_info(&packet).unwrap();

        assert_eq!(info.max_power_a, 80);
        assert_eq!(info.max_power_b, 90);
        assert_eq!(info.work_mode, WorkMode::Loop);
    }

    #[test]
    fn test_decode_device_info_empty_name_defaults() {
        let mut data = vec![0u8; 16];
        // name: 全零 -> 空字符串 -> 默认 "DG-LAB"
        data[8] = 1;
        data[13] = 50;

        let packet = Packet::new(CommandType::Response, data);
        let info = PacketDecoder::decode_device_info(&packet).unwrap();
        assert_eq!(info.name, "DG-LAB");
    }

    #[test]
    fn test_decode_device_info_wrong_command_type() {
        let packet = Packet::new(CommandType::Start, vec![0; 16]);
        let result = PacketDecoder::decode_device_info(&packet);
        assert!(result.is_err());
    }

    #[test]
    fn test_decode_device_info_insufficient_data() {
        let packet = Packet::new(CommandType::Response, vec![0; 10]); // 少于16字节
        let result = PacketDecoder::decode_device_info(&packet);
        assert!(result.is_err());
    }
}
