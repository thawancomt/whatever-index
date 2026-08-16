import { Badge } from "@/components/ui/badge";
import { Dialog, DialogContent, DialogHeader, DialogTitle } from "@/components/ui/dialog";
import { getFileDomain } from "@/domain/files/file_utils";
import { useTextableContent } from "@/hooks/useTextableContent";
import { useTextableContentStore } from "@/stores/textableContent.store";


export default function TextableContenDialog() {
  const { close, isOpen, path } = useTextableContentStore();
  const { data : content } = useTextableContent(path)

  return (
    <Dialog
      open={isOpen}
      onOpenChange={(e) => {
        if (!e) {
          close();
        }
      }}
    >

      <DialogContent className={"bg-white/5 dark:bg-white/20 text-white backdrop-blur-xs  max-h-[80dvh] overflow-y-auto no-scrollbar w-fit max-w-screen!"}>
        <DialogHeader>
          <DialogTitle>
            {path}
          </DialogTitle>
          <div>
            {getFileDomain(path || "").type.map(f => {
              return <Badge>
              {f}
            </Badge>
              })}
          </div>
        </DialogHeader>
        <pre>
          {
            !!content && content
          }
        </pre>
      </DialogContent>
    </Dialog>
  );
}
