import SettingsDialog from "./compontens/SettingsDialog/SettingsDialog";
import ResultList from "./ResultList";
import SearchBox from "./SearchBox";

export default function Homepage() {
    return (
      <div className="bg-zinc-900 grow w-full h-full flex flex-col p-4 gap-2 relative">
            <SettingsDialog className="absolute top-4 right-4 z-40" />
            <SearchBox />
            <ResultList />
        </div >
    )
}
