import { Card, CardContent } from "@/components/ui/card";
import { InputGroup, InputGroupAddon, InputGroupInput } from "@/components/ui/input-group";
import { cn } from "@/lib/utils";
import { useMetrics } from "@/stores/metrics.store";
import { useSearchStore } from "@/stores/search.store";
import { RiSearch2Line } from "@remixicon/react";
import ExplorerPage from "../explorer/Explorer";
import { Button } from "@/components/ui/button";

export default function SearchBox() {
    const { query, setQuery, result } = useSearchStore();
    const { total_files, by_extension } = useMetrics()

    const hasResult = result && result.length > 0;
    return (
        <Card className={
            cn(
                "transition-all duration-700 grow min-h-[20dvh] relative",
                hasResult ? "shrink " : " flex flex-col justify-center items-center"
            )
        }>

            <CardContent className="flex flex-col justify-center gap-2 items-center w-full ">

                <h1 className=" text-nowrap text-4xl font-black font-stack">
                    Whatever Index
                </h1>

                <Button className="text-xs text-muted-foreground" variant={"secondary"}>
                    <ExplorerPage>
                        {total_files} files indexed
                    </ExplorerPage>
                </Button>
                <div className="flex gap-2 items-center w-full  justify-center">
                    <InputGroup className="sm:w-1/2 md:w-4/12 focus-within:w-6/12 focus-within:py-6 transition-all duration-700">
                        <InputGroupAddon>
                            <RiSearch2Line />
                        </InputGroupAddon>
                        <InputGroupInput placeholder="Search" value={query} onChange={e => {
                            setQuery(e.target.value)
                        }}
                        />
                    </InputGroup>

                </div>

            </CardContent>
        </Card>
    )
}
