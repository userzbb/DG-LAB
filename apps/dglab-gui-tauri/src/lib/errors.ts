/**
 * Error handling utilities
 */

import type { AppError } from "../types/common";

/** 错误处理类 */
export class DGLabError extends Error {
  code: string;
  details?: unknown;

  constructor(code: string, message: string, details?: unknown) {
    super(message);
    this.name = "DGLabError";
    this.code = code;
    this.details = details;
  }

  toAppError(): AppError {
    return {
      code: this.code,
      message: this.message,
      details: this.details,
    };
  }
}

/** 处理 Tauri 命令错误 */
export function handleTauriError(error: unknown): AppError {
  if (error instanceof DGLabError) {
    return error.toAppError();
  }

  if (typeof error === "string") {
    return {
      code: "UNKNOWN_ERROR",
      message: error,
    };
  }

  if (error instanceof Error) {
    return {
      code: "UNKNOWN_ERROR",
      message: error.message,
      details: error,
    };
  }

  return {
    code: "UNKNOWN_ERROR",
    message: "An unknown error occurred",
    details: error,
  };
}

/** 创建设备错误 */
export function createDeviceError(message: string, details?: unknown): DGLabError {
  return new DGLabError("DEVICE_ERROR", message, details);
}

/** 创建连接错误 */
export function createConnectionError(message: string, details?: unknown): DGLabError {
  return new DGLabError("CONNECTION_ERROR", message, details);
}

/** 创建命令错误 */
export function createCommandError(message: string, details?: unknown): DGLabError {
  return new DGLabError("COMMAND_ERROR", message, details);
}

/** 安全地执行异步操作 */
export async function safeAsync<T>(
  fn: () => Promise<T>
): Promise<{ data: T | null; error: AppError | null }> {
  try {
    const data = await fn();
    return { data, error: null };
  } catch (error) {
    return { data: null, error: handleTauriError(error) };
  }
}
