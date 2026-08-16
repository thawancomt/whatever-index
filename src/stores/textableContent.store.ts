import { create } from "zustand";

interface State {
  path: string | undefined,
  isOpen: boolean;
  open: (path: string) => void;
  close : () => void;
}

export const useTextableContentStore = create<State>(set => {
  return {
    open: (path) => {
      set({path : path, isOpen : true})
    },
    close: () => {
      set({ isOpen : false, path : undefined})
    },
    isOpen : false,
    path : undefined
  }
})
