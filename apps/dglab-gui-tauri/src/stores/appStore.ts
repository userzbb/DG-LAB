/**
 * App store - manages global application state
 */

import { create } from "zustand";
import type { SessionInfo } from "../types/session";

interface AppStore {
  // State
  /** 应用是否初始化完成 */
  initialized: boolean;
  /** 当前会话信息 */
  sessionInfo: SessionInfo | null;
  /** 应用主题 (light/dark) */
  theme: "light" | "dark";
  /** 是否显示侧边栏 */
  showSidebar: boolean;
  /** 当前活动页面 */
  activePage: string;
  /** 全局加载状态 */
  loading: boolean;
  /** 全局错误信息 */
  error: string | null;

  // Actions
  /** 设置初始化状态 */
  setInitialized: (initialized: boolean) => void;
  /** 设置会话信息 */
  setSessionInfo: (info: SessionInfo | null) => void;
  /** 切换主题 */
  toggleTheme: () => void;
  /** 设置主题 */
  setTheme: (theme: "light" | "dark") => void;
  /** 切换侧边栏 */
  toggleSidebar: () => void;
  /** 设置活动页面 */
  setActivePage: (page: string) => void;
  /** 设置加载状态 */
  setLoading: (loading: boolean) => void;
  /** 设置错误信息 */
  setError: (error: string | null) => void;
  /** 清除错误 */
  clearError: () => void;
  /** 重置状态 */
  reset: () => void;
}

const initialState = {
  initialized: false,
  sessionInfo: null,
  theme: "light" as const,
  showSidebar: true,
  activePage: "dashboard",
  loading: false,
  error: null,
};

export const useAppStore = create<AppStore>((set) => ({
  ...initialState,

  setInitialized: (initialized) => set({ initialized }),

  setSessionInfo: (info) => set({ sessionInfo: info }),

  toggleTheme: () =>
    set((state) => ({
      theme: state.theme === "light" ? "dark" : "light",
    })),

  setTheme: (theme) => set({ theme }),

  toggleSidebar: () =>
    set((state) => ({
      showSidebar: !state.showSidebar,
    })),

  setActivePage: (page) => set({ activePage: page }),

  setLoading: (loading) => set({ loading }),

  setError: (error) => set({ error }),

  clearError: () => set({ error: null }),

  reset: () => set(initialState),
}));
