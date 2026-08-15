import { create } from "zustand"

interface State {
  ocr : boolean
}


export const useFeatures = create<State>(set => {
  return {
    ocr : false
  }
})
