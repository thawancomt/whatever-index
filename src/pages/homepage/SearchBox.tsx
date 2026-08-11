import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { InputGroup, InputGroupAddon, InputGroupInput } from "@/components/ui/input-group";
import { useSearch } from "@/hooks/useSearch";
import { cn } from "@/lib/utils";
import { useSearchStore } from "@/stores/search.store";
import { RiSearch2Line } from "@remixicon/react";

export default function SearchBox() {
    const { query, setQuery, result } = useSearchStore();

    const hasResult = result && result.length > 0;
    return (
        <Card className={
            cn(
                "transition-all duration-700 grow min-h-[20dvh]",
                hasResult ? "shrink " : " flex flex-col justify-center items-center"
            )
        }>

            <CardContent className="flex flex-col justify-center gap-2 items-center w-full">
                <h1 className=" text-nowrap text-4xl font-black">
                    Whatever Index
                </h1>
                <div className="flex gap-2 items-center w-full  justify-center">
                    <InputGroup className="sm:w-1/2 md:w-2/12 focus-within:w-6/12 focus-within:py-6 transition-all duration-700">
                        <InputGroupAddon>
                            <RiSearch2Line />
                        </InputGroupAddon>
                        <InputGroupInput placeholder="Search for content in your files" value={query} onChange={e => {
                            setQuery(e.target.value)
                        }}
                        />
                    </InputGroup>

                    <Button >
                        Buscar
                    </Button>
                </div>
            </CardContent>
        </Card>
    )
}