import { Button } from "@/components/ui/button";
import { Dialog, DialogContent, DialogTrigger } from "@/components/ui/dialog";
import { RiSettings2Line } from "@remixicon/react";
import { useMutation, useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";

interface SettingsDialogProps {
  className?: string
}

export default function SettingsDialog({ className }: SettingsDialogProps) {

  const {mutateAsync, status} = useMutation({
    mutationFn: async () => {
      await invoke("reset_index")
    },
  })

  const isLoading = status === "pending"

  return (
    <Dialog>
      <DialogTrigger className={className}>
        <Button variant={"link"}>
          <RiSettings2Line/>
        </Button>
      </DialogTrigger>

      <DialogContent>
        <Button variant={"destructive"} disabled={isLoading} onClick={() => {
          mutateAsync()
        }}>
          {
            isLoading ? "Deletando" : "Reset indexing"
          }
        </Button>
      </DialogContent>
    </Dialog>
  )
}
