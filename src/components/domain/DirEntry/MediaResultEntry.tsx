import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Item,
  ItemActions,
  ItemContent,
  ItemHeader,
  ItemMedia,
  ItemTitle,
} from "@/components/ui/item";
import { getFileDomain } from "@/domain/files/file_utils";
import { convertFileSrc, invoke } from "@tauri-apps/api/core";
interface FileResultEntryProps {
  path: string;
  content?: string;
}

export default function MediaResultEntry({
  path,
  content,
}: FileResultEntryProps) {
  const handleOpenFile = async () => {
    await invoke("open_file", { path });
  };

  const src = convertFileSrc(path);

  return (
    <Item variant={"muted"}>
      <ItemContent onClick={handleOpenFile}>
        <img src={src} className="w-full" />
      </ItemContent>
    </Item>
  );
}
