


import ResultList from "./ResultList";
import SearchBox from "./SearchBox";

export default function Homepage() {
    return (
        <div className="bg-zinc-900 grow w-full h-full flex flex-col p-4 gap-2">
            <SearchBox />
            <ResultList />
        </div >
    )
}   