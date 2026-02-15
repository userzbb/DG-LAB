/**
 * Common types and utilities
 */

/** API 响应包装 */
export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
}

/** 错误类型 */
export interface AppError {
  code: string;
  message: string;
  details?: unknown;
}

/** 通道类型 */
export type Channel = "A" | "B";

/** 通道索引 (0 = A, 1 = B) */
export type ChannelIndex = 0 | 1;

/** 将通道转换为索引 */
export function channelToIndex(channel: Channel): ChannelIndex {
  return channel === "A" ? 0 : 1;
}

/** 将索引转换为通道 */
export function indexToChannel(index: ChannelIndex): Channel {
  return index === 0 ? "A" : "B";
}
