import FileResultEntry from "@/components/domain/DirEntry/FileResultEntry";
import { Badge } from "@/components/ui/badge";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Spinner } from "@/components/ui/spinner";
import { useSearch } from "@/hooks/useSearch";
import { cn } from "@/lib/utils";
import { useSearchStore } from "@/stores/search.store";
import { RiSearch2Line } from "@remixicon/react";
import mapFilesByDomain from "./utils/mapFileByDomain";
import { useMemo, useState } from "react";
import { ExtensionDomains } from "@/domain/files/file_utils";
import { Button } from "@/components/ui/button";

export default function ResultList() {
  const { query } = useSearchStore();
  const { data: searchResult, status } = useSearch(query || "");

  const resultCount = searchResult ? searchResult.length : 0;

  const hasQuery = query && query.trim() !== "";

  const hasResult = searchResult && hasQuery && resultCount > 0;

  const hasInteraction = query && query.trim();

  const mappedByDomain = useMemo(() => {
    return mapFilesByDomain(searchResult || []);
  }, [searchResult]);

  const isLoading = hasQuery && status === "pending";

  const [targetDomain, setTargetDomain] = useState<undefined | ExtensionDomains>(undefined);

  return (
    <Card
      className={cn(
        "grow transition-all duration-700 overflow-y-scroll",
        hasInteraction ? "h-[80dvh]!" : " h-0! ",
      )}
    >
      {hasResult && (
        <CardHeader className="flex flex-col items-start">
          <div className="flex  items-center gap-2">
            <CardTitle className="font-bold">Results</CardTitle>
            <Badge>{resultCount}</Badge>
          </div>

          <div className="w-full flex overflow-x-auto">
            {(
              Object.keys(ExtensionDomains) as (keyof typeof ExtensionDomains)[]
            ).map((domain) => {
              const count = mappedByDomain[domain]?.length || 0;
              if (count === 0) return null;
              return (
                <Button
                  key={domain}
                  variant={"secondary"}
                  className=" inline-flex items-center gap-2"
                  onClick={() => {
                    setTargetDomain(ExtensionDomains[domain]);
                  }}
                >
                  <span>{domain}</span>
                  <Badge className="pb-0">
                    {mappedByDomain[domain]?.length || 0}
                  </Badge>
                </Button>
              );
            })}
          </div>
        </CardHeader>
      )}


      {targetDomain}

      <CardContent
        className={cn(
          hasResult
            ? "flex flex-col gap-2"
            : "flex  grow justify-center items-center",
        )}
      >
        {!isLoading && (
          <>
            {hasResult &&
              (targetDomain && mapFilesByDomain[targetDomain] || searchResult).map((fileEntry) => {
                return <FileResultEntry key={fileEntry} path={fileEntry} />;
              })}

            {hasQuery && !hasResult && <h1>No results found</h1>}

            {!hasResult && !hasQuery && (
              <div className="flex flex-col justify-center items-center gap-2">
                <RiSearch2Line size={36} />
                <h1>Start searching for content in your files</h1>
              </div>
            )}
          </>
        )}
        {isLoading && (
          <div>
            <Spinner className="animate-spin duration-1000" />
          </div>
        )}
      </CardContent>
    </Card>
  );
}
