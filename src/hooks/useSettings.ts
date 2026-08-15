import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";

export interface AppSettings {
    auto_scan: boolean
    index_images: boolean
    index_audio: boolean
}

export function useSettings() {
    return useQuery({
        queryKey: ["settings"],
        queryFn: async () => {
            const result = await invoke<AppSettings>("get_settings");
            return result as AppSettings
        }
    })
}


export function useUpdateSettings() {
    const queryClient = useQueryClient();

    return useMutation({
        mutationFn: async (patch: AppSettings) => {
            await invoke("toggle_settings", { patch });
        },
        onMutate: async (newSettings) => {
            await queryClient.cancelQueries({ queryKey: ["settings"] });
            const previousSettings = queryClient.getQueryData<AppSettings>(["settings"]);
            queryClient.setQueryData(["settings"], newSettings);
            return { previousSettings };
        },
        onError: (_err, _newSettings, context) => {
            if (context?.previousSettings) {
                queryClient.setQueryData(["settings"], context.previousSettings);
            }
        },
        onSettled: () => {
            queryClient.invalidateQueries({ queryKey: ["settings"] });
        },
    });
}
