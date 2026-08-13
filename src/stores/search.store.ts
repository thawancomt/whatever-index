import { create } from "zustand";

interface SearchState {
    query: string | undefined,
    setQuery: (query: string | undefined) => void,
    result: string[] | undefined
    debounce: number | undefined
}

export const useSearchStore = create<SearchState>(set => {
    return {
        query: undefined,
        setQuery: (query) => {
            set({ query })
        },
        debounce: undefined,
        result: undefined,
    }
})
