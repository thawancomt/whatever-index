import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Switch } from "@/components/ui/switch";
import { RiSettings2Line } from "@remixicon/react";
import { QueryClient, useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { PropsWithChildren, useEffect, useState } from "react";

interface SettingsDialogProps {
  className?: string;
}

interface SettingsSectionProps extends PropsWithChildren {
  title?: string;
}

interface SettingsSectionListItemProps extends PropsWithChildren {
  label: string
  className?: string
}

function SettingsSection({ title, children} : SettingsSectionProps) {
  return (
    <div className="bg-muted w-full p-2 rounded-xl space-y-2">
      <h1 className="text-lg font-semibold font-stack">
        {title}
      </h1>
      <div>
        {children}
      </div>
    </div>
  )
}
function SettingsSectionList({children} : Pick<SettingsSectionListItemProps, "children">) {
  return <ul className="*:flex *:items-center  space-y-2">
    {children}
  </ul>
}

function SettingsSectionListItem({label, children, className} : SettingsSectionListItemProps) {
  return <li className={className}>
    <span>
      {label}:
    </span>
    {children}
  </li>
}

interface AppSettings {
  auto_scan : boolean
  index_images : boolean
  index_audio : boolean
}

function useSettings() {
  return useQuery({
    queryKey: ["settings"],
    queryFn: async () => {
      const result = await invoke<AppSettings>("get_settings");

      return result as AppSettings
    }
  })
}



export default function SettingsDialog({ className }: SettingsDialogProps) {
  const [indexedCount, setIndexedCount] = useState(0);
  const { data: settings } = useSettings();

  const { mutateAsync, status } = useMutation({
    mutationFn: async () => {
      await invoke("reset_index");
    },
  });

  const isLoading = status === "pending";

  useEffect(() => {
    const fn = async () => {
      const result = await invoke<number>("get_total_files_indexed");
      setIndexedCount(result);
    };
    fn();
  }, []);

  const queryClient = useQueryClient();

  const {mutateAsync : updateSettingsFn} = useMutation({
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


  const updateSettings = async (key : keyof AppSettings, value : boolean) => {
    if (!settings) return;
    const patch = {
      ...settings,
      [key]: value
    } satisfies AppSettings;
    await updateSettingsFn(patch)
  }

  return (
    <Dialog>
      <DialogTrigger className={className}>
        <Button variant={"link"}>
          <RiSettings2Line />
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle className={"text-2xl font-bold font-stack"}>Whatever Index Settings</DialogTitle>
        </DialogHeader>
        <SettingsSection title="Metrics">
          <SettingsSectionList>
            <SettingsSectionListItem label="Total files indexed">
              <Badge variant={"ghost"} className="font-bold">{indexedCount}</Badge>
            </SettingsSectionListItem>
            <SettingsSectionListItem label="Last scan">
              <Badge variant={"ghost"} className="font-bold">Today</Badge>
            </SettingsSectionListItem>
          </SettingsSectionList>
        </SettingsSection>
        <SettingsSection title="Options">
          <SettingsSectionList>
            <SettingsSectionListItem label="Auto Re-scan" className="w-full  justify-between">
              <Switch checked={settings?.auto_scan} onCheckedChange={(e) => {
                updateSettings("auto_scan", e)
              }}/>
            </SettingsSectionListItem>
            <SettingsSectionListItem label="Scan photos" className="w-full  justify-between">
              <Switch checked={settings?.index_images} onCheckedChange={(e) => {
                updateSettings("index_images", e)
              }}/>
            </SettingsSectionListItem>
            <SettingsSectionListItem label="Scan audio files" className="w-full  justify-between">
              <Switch checked={settings?.index_audio}  onCheckedChange={(e) => {
                updateSettings("index_audio", e)
              }}/>
            </SettingsSectionListItem>
          </SettingsSectionList>
        </SettingsSection>

        <div>
        <Button
          disabled={isLoading}
          onClick={() => {
            mutateAsync();
          }}
        >
          Force re-scan
        </Button>

        <Button
          variant={"secondary"}
          disabled={isLoading}
          onClick={() => {
            mutateAsync();
          }}
        >
          {isLoading ? "Deletando" : "Re-index"}
        </Button>
        </div>
      </DialogContent>
    </Dialog>
  );
}
