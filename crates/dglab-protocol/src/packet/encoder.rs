//! 数据包编码器

use crate::error::{ProtocolError, Result};
use crate::packet::types::{CommandType, Packet, PACKET_HEADER, PACKET_TAIL};

/// 数据包编码器
pub struct PacketEncoder;

impl PacketEncoder {
    /// 编码数据包
    pub fn encode(packet: &Packet) -> Result<Vec<u8>> {
        let mut buf = Vec::with_capacity(4 + packet.data.len());

        buf.push(PACKET_HEADER);
        buf.push(packet.command.into());
        buf.push(packet.data_len);
        buf.extend_from_slice(&packet.data);
        buf.push(packet.checksum);
        buf.push(PACKET_TAIL);

        Ok(buf)
    }

    /// 编码简单命令（无数据）
    pub fn encode_simple(command: CommandType) -> Result<Vec<u8>> {
        let packet = Packet::new(command, Vec::new());
        Self::encode(&packet)
    }

    /// 编码设置功率命令
    pub fn encode_set_power(channel: u8, power: u8) -> Result<Vec<u8>> {
        let command = match channel {
            0 => CommandType::SetPowerA,
            1 => CommandType::SetPowerB,
            _ => return Err(ProtocolError::EncodeError("Invalid channel".to_string())),
        };
        let packet = Packet::new(command, vec![power]);
        Self::encode(&packet)
    }

    /// 编码设置波形命令
    pub fn encode_set_wave(channel: u8, waveform: u8, params: &[u8]) -> Result<Vec<u8>> {
        let command = match channel {
            0 => CommandType::SetWaveA,
            1 => CommandType::SetWaveB,
            _ => return Err(ProtocolError::EncodeError("Invalid channel".to_string())),
        };
        let mut data = vec![waveform];
        data.extend_from_slice(params);
        let packet = Packet::new(command, data);
        Self::encode(&packet)
    }

    /// 编码设置模式命令
    pub fn encode_set_mode(mode: u8) -> Result<Vec<u8>> {
        let packet = Packet::new(CommandType::SetMode, vec![mode]);
        Self::encode(&packet)
    }

    /// 编码心跳命令
    pub fn encode_heartbeat() -> Result<Vec<u8>> {
        Self::encode_simple(CommandType::Heartbeat)
    }

    /// 编码开始命令
    pub fn encode_start() -> Result<Vec<u8>> {
        Self::encode_simple(CommandType::Start)
    }

    /// 编码停止命令
    pub fn encode_stop() -> Result<Vec<u8>> {
        Self::encode_simple(CommandType::Stop)
    }

    /// 编码获取信息命令
    pub fn encode_get_info() -> Result<Vec<u8>> {
        Self::encode_simple(CommandType::GetInfo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::types::{PACKET_HEADER, PACKET_TAIL};

    #[test]
    fn test_encode_basic_structure() {
        // 编码后的格式: [HEADER, command, data_len, ...data, checksum, TAIL]
        let packet = Packet::new(CommandType::Start, Vec::new());
        let bytes = PacketEncoder::encode(&packet).unwrap();

        assert_eq!(bytes[0], PACKET_HEADER);
        assert_eq!(bytes[1], CommandType::Start as u8);
        assert_eq!(bytes[2], 0); // data_len = 0
                                 // bytes[3] = checksum
        assert_eq!(bytes[bytes.len() - 1], PACKET_TAIL);
        assert_eq!(bytes.len(), 5); // header + cmd + len + checksum + tail
    }

    #[test]
    fn test_encode_with_data() {
        let packet = Packet::new(CommandType::SetPowerA, vec![50]);
        let bytes = PacketEncoder::encode(&packet).unwrap();

        assert_eq!(bytes[0], PACKET_HEADER);
        assert_eq!(bytes[1], 0x10); // SetPowerA
        assert_eq!(bytes[2], 1); // data_len = 1
        assert_eq!(bytes[3], 50); // data
                                  // bytes[4] = checksum
        assert_eq!(bytes[bytes.len() - 1], PACKET_TAIL);
        assert_eq!(bytes.len(), 6); // header + cmd + len + data(1) + checksum + tail
    }

    #[test]
    fn test_encode_simple_heartbeat() {
        let bytes = PacketEncoder::encode_heartbeat().unwrap();
        assert_eq!(bytes[0], PACKET_HEADER);
        assert_eq!(bytes[1], CommandType::Heartbeat as u8);
        assert_eq!(bytes[2], 0); // 无数据
        assert_eq!(bytes[bytes.len() - 1], PACKET_TAIL);
    }

    #[test]
    fn test_encode_simple_start() {
        let bytes = PacketEncoder::encode_start().unwrap();
        assert_eq!(bytes[1], CommandType::Start as u8);
        assert_eq!(bytes[2], 0);
    }

    #[test]
    fn test_encode_simple_stop() {
        let bytes = PacketEncoder::encode_stop().unwrap();
        assert_eq!(bytes[1], CommandType::Stop as u8);
        assert_eq!(bytes[2], 0);
    }

    #[test]
    fn test_encode_simple_get_info() {
        let bytes = PacketEncoder::encode_get_info().unwrap();
        assert_eq!(bytes[1], CommandType::GetInfo as u8);
        assert_eq!(bytes[2], 0);
    }

    #[test]
    fn test_encode_set_power_channel_a() {
        let bytes = PacketEncoder::encode_set_power(0, 75).unwrap();
        assert_eq!(bytes[1], CommandType::SetPowerA as u8);
        assert_eq!(bytes[2], 1); // data_len
        assert_eq!(bytes[3], 75); // power value
    }

    #[test]
    fn test_encode_set_power_channel_b() {
        let bytes = PacketEncoder::encode_set_power(1, 100).unwrap();
        assert_eq!(bytes[1], CommandType::SetPowerB as u8);
        assert_eq!(bytes[2], 1);
        assert_eq!(bytes[3], 100);
    }

    #[test]
    fn test_encode_set_power_invalid_channel() {
        let result = PacketEncoder::encode_set_power(2, 50);
        assert!(result.is_err());
    }

    #[test]
    fn test_encode_set_wave_channel_a() {
        let params = [10, 20, 30];
        let bytes = PacketEncoder::encode_set_wave(0, 0x01, &params).unwrap();
        assert_eq!(bytes[1], CommandType::SetWaveA as u8);
        assert_eq!(bytes[2], 4); // data_len = 1 (waveform) + 3 (params)
        assert_eq!(bytes[3], 0x01); // waveform type
        assert_eq!(bytes[4], 10);
        assert_eq!(bytes[5], 20);
        assert_eq!(bytes[6], 30);
    }

    #[test]
    fn test_encode_set_wave_channel_b() {
        let bytes = PacketEncoder::encode_set_wave(1, 0x02, &[5]).unwrap();
        assert_eq!(bytes[1], CommandType::SetWaveB as u8);
        assert_eq!(bytes[3], 0x02); // waveform type
        assert_eq!(bytes[4], 5); // param
    }

    #[test]
    fn test_encode_set_wave_invalid_channel() {
        let result = PacketEncoder::encode_set_wave(3, 0x01, &[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_encode_set_mode() {
        let bytes = PacketEncoder::encode_set_mode(0x02).unwrap();
        assert_eq!(bytes[1], CommandType::SetMode as u8);
        assert_eq!(bytes[2], 1); // data_len
        assert_eq!(bytes[3], 0x02); // mode
    }

    #[test]
    fn test_encode_preserves_checksum() {
        // 确保编码后的校验和是由 Packet::new 正确计算的
        let bytes = PacketEncoder::encode_set_power(0, 50).unwrap();
        let expected_checksum = Packet::calculate_checksum(CommandType::SetPowerA, 1, &[50]);
        // checksum 在 data 之后，tail 之前
        assert_eq!(bytes[bytes.len() - 2], expected_checksum);
    }

    #[test]
    fn test_encode_zero_power() {
        let bytes = PacketEncoder::encode_set_power(0, 0).unwrap();
        assert_eq!(bytes[3], 0);
    }

    #[test]
    fn test_encode_max_power() {
        let bytes = PacketEncoder::encode_set_power(0, 255).unwrap();
        assert_eq!(bytes[3], 255);
    }
}
