/**
 * Waveform store - manages waveform state
 */

import { create } from "zustand";
import type { Waveform } from "../types/waveform";
import { defaultWaveform } from "../types/waveform";
import { toast } from "../lib/toast";

interface WaveformStore {
  // State
  /** 通道 A 波形 */
  waveformA: Waveform;
  /** 通道 B 波形 */
  waveformB: Waveform;
  /** 内置波形列表 */
  builtInWaveforms: Waveform[];
  /** 自定义波形列表 */
  customWaveforms: Waveform[];

  // Actions
  /** 设置通道波形 */
  setWaveform: (channel: "A" | "B", waveform: Waveform) => void;
  /** 设置内置波形列表 */
  setBuiltInWaveforms: (waveforms: Waveform[]) => void;
  /** 添加自定义波形 */
  addCustomWaveform: (waveform: Waveform) => void;
  /** 删除自定义波形 */
  deleteCustomWaveform: (name: string) => void;
  /** 更新自定义波形 */
  updateCustomWaveform: (name: string, waveform: Waveform) => void;
  /** 重置状态 */
  reset: () => void;
}

const initialState = {
  waveformA: defaultWaveform,
  waveformB: defaultWaveform,
  builtInWaveforms: [],
  customWaveforms: [],
};

export const useWaveformStore = create<WaveformStore>((set) => ({
  ...initialState,

  setWaveform: (channel, waveform) => {
    set({
      [channel === "A" ? "waveformA" : "waveformB"]: waveform,
    });
    toast.success(`通道 ${channel} 波形已应用`, `波形类型: ${waveform.params.waveform_type}`);
  },

  setBuiltInWaveforms: (waveforms) =>
    set({ builtInWaveforms: waveforms }),

  addCustomWaveform: (waveform) => {
    set((state) => ({
      customWaveforms: [...state.customWaveforms, waveform],
    }));
    toast.success("自定义波形已保存", `波形"${waveform.name}"已添加`);
  },

  deleteCustomWaveform: (name) => {
    set((state) => ({
      customWaveforms: state.customWaveforms.filter((w) => w.name !== name),
    }));
    toast.success("自定义波形已删除", `波形"${name}"已移除`);
  },

  updateCustomWaveform: (name, waveform) =>
    set((state) => ({
      customWaveforms: state.customWaveforms.map((w) =>
        w.name === name ? waveform : w
      ),
    })),

  reset: () => set(initialState),
}));
