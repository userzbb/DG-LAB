/**
 * Waveform types matching Rust backend
 */

/** 波形类型 */
export enum WaveformType {
  /** 连续波 */
  Continuous = "Continuous",
  /** 脉冲波 */
  Pulse = "Pulse",
  /** 锯齿波 */
  Sawtooth = "Sawtooth",
  /** 正弦波 */
  Sine = "Sine",
  /** 方波 */
  Square = "Square",
  /** 三角波 */
  Triangle = "Triangle",
  /** 呼吸波 */
  Breathing = "Breathing",
  /** 渐强渐弱 */
  Fade = "Fade",
  /** 自定义 */
  Custom = "Custom",
}

/** 波形参数 */
export interface WaveformParams {
  /** 波形类型 */
  waveform_type: WaveformType;
  /** 频率 (Hz) */
  frequency: number;
  /** 脉宽 (微秒) */
  pulse_width: number;
  /** 最小强度 */
  min_power: number;
  /** 最大强度 */
  max_power: number;
  /** 周期 (毫秒) */
  period_ms: number;
  /** 占空比 (0-100) */
  duty_cycle: number;
}

/** 波形 */
export interface Waveform {
  /** 波形名称 */
  name: string;
  /** 波形描述 */
  description: string;
  /** 波形参数 */
  params: WaveformParams;
  /** 自定义数据点 */
  custom_points?: Array<[number, number]>;
}

/** 默认波形参数 */
export const defaultWaveformParams: WaveformParams = {
  waveform_type: WaveformType.Continuous,
  frequency: 100,
  pulse_width: 200,
  min_power: 0,
  max_power: 100,
  period_ms: 5000,
  duty_cycle: 50,
};

/** 默认波形 */
export const defaultWaveform: Waveform = {
  name: "Default",
  description: "Default waveform",
  params: defaultWaveformParams,
};
