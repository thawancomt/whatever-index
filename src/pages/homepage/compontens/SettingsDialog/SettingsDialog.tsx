import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { RiSettings2Line } from "@remixicon/react";
import { useMutation } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { PropsWithChildren } from "react";
import MetricSection from "./MetricSection";
import OptionsSection from "./OptionsSection";

interface SettingsDialogProps {
  className?: string;
}

interface SettingsSectionProps extends PropsWithChildren {
  title?: string;
}

interface SettingsSectionListItemProps extends PropsWithChildren {
  label: string | React.ReactNode
  className?: string
}

export function SettingsSection({ title, children }: SettingsSectionProps) {
  return (
    <div className="bg-muted/30 w-full p-2 rounded-xl space-y-2">
      <h1 className="text-lg font-semibold font-stack">
        {title}
      </h1>
      <div>
        {children}
      </div>
    </div>
  )
}
export function SettingsSectionList({ children }: Pick<SettingsSectionListItemProps, "children">) {
  return <ul className="*:flex *:items-center  space-y-2">
    {children}
  </ul>
}

export function SettingsSectionListItem({ label, children, className }: SettingsSectionListItemProps) {
  return <li className={className}>
    <span>
      {label}:
    </span>
    {children}
  </li>
}







export default function SettingsDialog({ className }: SettingsDialogProps) {




  const { mutateAsync, status } = useMutation({
    mutationFn: async () => {
      await invoke("reset_index");
    },
  });

  const { mutateAsync : triggerScan } = useMutation({
    mutationFn: async () => {
      await invoke("re_scan");
    },
  });

  const isLoading = status === "pending";

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

        <MetricSection />
        <OptionsSection />

        <div>
          <Button
            disabled={isLoading}
            onClick={() => {
              triggerScan();
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
