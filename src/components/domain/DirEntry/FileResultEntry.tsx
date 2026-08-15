import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Item, ItemActions, ItemContent, ItemHeader, ItemMedia, ItemTitle } from "@/components/ui/item";
import { getFileDomain } from "@/domain/files/file_utils";
import { convertFileSrc, invoke } from "@tauri-apps/api/core";
interface FileResultEntryProps {
    path: string,
    content?: string
}

function isMedia(path: string) {
  const v = path.split(".") || [];
  if (["jpg", "png"].includes(v.pop() || "")) {
    return true
  }

  return false
}

export default function FileResultEntry({ path, content }: FileResultEntryProps) {

    const handleOpenFile = async () => {
        await invoke("open_file", { path })
    }

    const hasSrc = isMedia(path)

    const src = hasSrc ? convertFileSrc(path) : null

    const {
        type,
        icon: Icon
    } = getFileDomain(path)

    return (
        <Item variant={"muted"}>
        <ItemMedia>
          {
            hasSrc && src && (
              <img src={src}  className="w-44"/>
            )
              }
                {<Icon />}
            </ItemMedia>
            <ItemHeader>
                <ItemTitle className="font-semibold flex flex-col items-start">
                    {path}

                    {type && (
                        <div className="inline-flex gap-2">
                            {

                                type.map((type, index) => (
                                    <Badge key={index} variant={"secondary"}>
                                        {type}
                                    </Badge>
                                ))
                            }
                        </div>
                    )}
                </ItemTitle>
                <ItemActions>
                    <Button variant={"secondary"} onClick={handleOpenFile}>
                        Open
                    </Button>
                </ItemActions>
            </ItemHeader>
            {
                content && (
                    <ItemContent>
                        {content}
                    </ItemContent>
                )
            }
        </Item>
    )
}
