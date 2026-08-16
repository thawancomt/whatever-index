import { useQuery } from "@tanstack/react-query"
import { invoke } from "@tauri-apps/api/core"

export const useTextableContent = (path : string | undefined) => {
  return useQuery({
    enabled : !!path,
    queryKey: ["file_content", path],
    queryFn: async () => {
      return await invoke<string>("get_file_content", {path})
    }
  })
}
