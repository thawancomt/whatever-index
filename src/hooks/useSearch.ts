import { useQuery } from "@tanstack/react-query"
import { invoke } from "@tauri-apps/api/core"
import { useEffect, useState } from "react";

function useDebounce<T>(value: T, delay: number): T {
    const [debouncedValue, setDebouncedValue] = useState(value);

    useEffect(() => {
        const handler = setTimeout(() => setDebouncedValue(value), delay);
        return () => clearTimeout(handler);
    }, [value, delay]);

    return debouncedValue;
}

export function useSearch(query: string) {

    const debouncedQuery = useDebounce(query.trim(), 100);

    return useQuery({
        queryKey: ['search', debouncedQuery],
        queryFn: async () => {
            const result = await invoke<string[]>("search", { search: debouncedQuery })
            return result
        },
        enabled: !!debouncedQuery || !!query,
        staleTime: 1000 * 60 * 60, // 60 minutes
    })
}