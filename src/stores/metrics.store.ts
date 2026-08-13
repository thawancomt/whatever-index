import { create } from "zustand"

interface State {
  total_files: number
  actions: {
    setTotalFiles: (v : number) => void;
  }
}

export const useMetrics = create<State>(set => {
  return {
    total_files: 0,
    actions: {
      setTotalFiles(v) {
        set({total_files : v})
      },
    }
  }
})
