/**
 * Preset types matching Rust backend
 */

import type { Waveform } from "./waveform";

/** 通道配置 */
export interface PresetChannelConfig {
  /** 是否启用 */
  enabled: boolean;
  /** 最小强度 */
  min_power: number;
  /** 最大强度 */
  max_power: number;
  /** 波形 */
  waveform?: Waveform;
}

/** 设备预设 */
export interface Preset {
  /** 预设 ID */
  id: string;
  /** 预设名称 */
  name: string;
  /** 预设描述 */
  description: string;
  /** 创建时间 */
  created_at: string;
  /** 最后修改时间 */
  updated_at: string;
  /** 通道 A 配置 */
  channel_a: PresetChannelConfig;
  /** 通道 B 配置 */
  channel_b: PresetChannelConfig;
  /** 全局设置 */
  settings: Record<string, string>;
}

/** 默认通道配置 */
export const defaultChannelConfig: PresetChannelConfig = {
  enabled: true,
  min_power: 0,
  max_power: 50,
};
