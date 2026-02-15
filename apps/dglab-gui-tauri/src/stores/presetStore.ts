/**
 * Preset store - manages preset state
 */

import { create } from "zustand";
import type { Preset } from "../types/preset";
import { toast } from "../lib/toast";

interface PresetStore {
  // State
  /** 预设列表 */
  presets: Preset[];
  /** 当前选中的预设 */
  currentPreset: Preset | null;

  // Actions
  /** 设置预设列表 */
  setPresets: (presets: Preset[]) => void;
  /** 添加预设 */
  addPreset: (preset: Preset) => void;
  /** 删除预设 */
  deletePreset: (id: string) => void;
  /** 更新预设 */
  updatePreset: (id: string, preset: Preset) => void;
  /** 设置当前预设 */
  setCurrentPreset: (preset: Preset | null) => void;
  /** 按 ID 查找预设 */
  findPresetById: (id: string) => Preset | undefined;
  /** 重置状态 */
  reset: () => void;
}

const initialState = {
  presets: [],
  currentPreset: null,
};

export const usePresetStore = create<PresetStore>((set, get) => ({
  ...initialState,

  setPresets: (presets) => set({ presets }),

  addPreset: (preset) => {
    set((state) => ({
      presets: [...state.presets, preset],
    }));
    toast.success("预设已创建", `预设"${preset.name}"已保存`);
  },

  deletePreset: (id) => {
    const preset = get().presets.find((p) => p.id === id);
    set((state) => ({
      presets: state.presets.filter((p) => p.id !== id),
      currentPreset:
        state.currentPreset?.id === id ? null : state.currentPreset,
    }));
    if (preset) {
      toast.success("预设已删除", `预设"${preset.name}"已删除`);
    }
  },

  updatePreset: (id, preset) =>
    set((state) => ({
      presets: state.presets.map((p) => (p.id === id ? preset : p)),
      currentPreset:
        state.currentPreset?.id === id ? preset : state.currentPreset,
    })),

  setCurrentPreset: (preset) => {
    set({ currentPreset: preset });
    if (preset) {
      toast.success("预设已加载", `正在使用"${preset.name}"`);
    }
  },

  findPresetById: (id) => {
    return get().presets.find((p) => p.id === id);
  },

  reset: () => set(initialState),
}));
