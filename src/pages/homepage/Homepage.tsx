


import SearchBox from "./SearchBox";
import ResultList from "./ResultList";

export default function Homepage() {
    return (
        <div className="bg-zinc-500 grow w-full h-full flex flex-col p-4 gap-2">
            <SearchBox />
            <ResultList />
        </div >
    )
}   