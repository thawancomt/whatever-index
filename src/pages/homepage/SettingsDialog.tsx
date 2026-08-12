import { Button } from "@/components/ui/button";
import { Dialog, DialogContent, DialogTrigger } from "@/components/ui/dialog";
import { RiSettings2Line } from "@remixicon/react";
import { useMutation } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

interface SettingsDialogProps {
  className?: string
}

export default function SettingsDialog({ className }: SettingsDialogProps) {

  const [indexedCount, setIndexedCount] = useState(0);

  const {mutateAsync, status} = useMutation({
    mutationFn: async () => {
      await invoke("reset_index")
    },
  })

  const isLoading = status === "pending"

  useEffect(() => {
    const fn = async () => {
      const result = await invoke<number>("get_total_files_indexed")
      setIndexedCount(result)
    }
    fn()
  }, [])

  return (
    <Dialog>
      <DialogTrigger className={className}>
        <Button variant={"link"}>
          <RiSettings2Line/>
        </Button>
      </DialogTrigger>

      <DialogContent>
        <span>
          Total files indexed:
        {
          indexedCount
        }
        </span>
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
