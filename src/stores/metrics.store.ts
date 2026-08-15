import { create } from "zustand";

interface State {
  total_files: number
  by_extension: Record<string, number>
  actions: {
    readonly setTotalFiles: (v: number) => void;
    readonly setByExtension: (v: Record<string, number>) => void;
  }
}

export const useMetrics = create<State>(set => {
  return {
    total_files: 0,
    by_extension: {},
    actions: {
      setTotalFiles(v) {
        set({ total_files: v })
      },
      setByExtension(v) {
        set({ by_extension: v })
      },
    }
  }
})
