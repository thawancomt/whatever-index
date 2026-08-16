import FileResultEntry from "@/components/domain/DirEntry/FileResultEntry";
import MediaResultEntry from "@/components/domain/DirEntry/MediaResultEntry";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Drawer, DrawerContent, DrawerTrigger } from "@/components/ui/drawer";
import { InputGroup, InputGroupAddon, InputGroupInput } from "@/components/ui/input-group";

import {
  ExtensionDomains,
  ExtensionMapper,
} from "@/domain/files/file_utils";
import { useSearch, useSearchFilesByExtension } from "@/hooks/useSearch";
import { useMetrics } from "@/stores/metrics.store";
import { RiSearch2Line } from "@remixicon/react";
import { PropsWithChildren, useMemo, useState } from "react";

interface ExplorerPageProps extends PropsWithChildren { }

export default function ExplorerPage({ children }: ExplorerPageProps) {
  const { by_extension, total_files } = useMetrics();

  const [targetExtension, setTargetExtension] = useState(
    Object.keys(by_extension)[0] || "",
  );

  const isMedia = ExtensionMapper[targetExtension]?.type.includes(
    ExtensionDomains.Media,
  );

  const [query, setQuery] = useState("");

  const { data: filesFromSearch } = useSearch(query)

  const { data: dataFiles } = useSearchFilesByExtension(targetExtension);

  const mergedFiles = useMemo(() => {

    if (query.trim() === "") {
      return dataFiles?.map(f => f.path)
    }

    const filesFromSearchSet = new Set(filesFromSearch);
    const dataFilesSet = new Set(dataFiles?.map(f => f.path))

    return Array.from(filesFromSearchSet.intersection(dataFilesSet))

  }, [dataFiles, filesFromSearch])

  return (
    <Drawer>
      <DrawerTrigger>{children}</DrawerTrigger>
      <DrawerContent className={"h-dvh w-screen flex  p-0!  "}>
        <div className="p-4 space-y-2 shrink-0">
          <h1 className="font-stack text-2xl">
            Explorer view{" "}
            <Badge>
              <span className="text-md">{total_files}</span>
              <span className="text-xs">indexed files</span>
            </Badge>
          </h1>
          <div className="flex gap-2 max-w-full overflow-x-auto">
            {Object.keys(by_extension).map((extension) => {
              return (
                <Button
                  className={"space-x-2"}
                  onClick={() => {
                    setTargetExtension(extension);
                  }}
                >
                  {extension}{" "}
                  <Badge variant={"secondary"}>{by_extension[extension]}</Badge>
                </Button>
              );
            })}
          </div>
        </div>

        <div className="px-4! mb-2">
          <InputGroup className="py-4 focus-within:py-8 transition-all duration-700" >
            <InputGroupAddon>
              <RiSearch2Line />
            </InputGroupAddon>
            <InputGroupInput placeholder="Search into these files content" value={query} onChange={e => setQuery(e.target.value)}/>
          </InputGroup>
        </div>

        <div className="overflow-y-auto px-4!">
          {isMedia && (
            <div className="grid  grid-cols-2 md:grid-cols-3">
              {mergedFiles?.map((f) => {
                return <MediaResultEntry path={f} />;
              })}
            </div>
          )}
          {!isMedia && (
            <div className="flex flex-col gap-2">
              {mergedFiles?.map((f) => {
                return <FileResultEntry path={f} />;
              })}
            </div>
          )}
        </div>
      </DrawerContent>
    </Drawer>
  );
}
