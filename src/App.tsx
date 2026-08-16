
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { useEffect} from "react";
import { toast } from "sonner";
import Homepage from "./pages/homepage/Homepage";
import { useMetrics } from "./stores/metrics.store";
import TextableContenDialog from "./components/domain/DirEntry/TextableContentDialog";
import { setupListeners } from "./domain/listeners";

const queryClient = new QueryClient()

function applyTheme() {
  document.documentElement.classList.toggle(
    "dark",
    localStorage.theme === "dark" ||
      (!("theme" in localStorage) && window.matchMedia("(prefers-color-scheme: dark)").matches),
  );
}

applyTheme();

window.matchMedia("(prefers-color-scheme: dark)").addEventListener("change", () => {
  if (!("theme" in localStorage)) {
    applyTheme();
  }
});

function App() {
  const { actions: { setTotalFiles } } = useMetrics()

  const warmUpScan = async () => {
    await invoke("re_scan")
  }
  const populateMetrics = async () => {
    const result = await invoke<number>("get_total_files_indexed");
    const byExtension = await invoke<Record<string, number>>("get_total_by_extension");
    if (result) {
      setTotalFiles(result);
    }
    if (byExtension) {
      useMetrics.getState().actions.setByExtension(byExtension);
    }
  }

  useEffect(() => {
    const warmUp = async () => {
      const toastId = toast.loading("Warming up the app, please wait...")
      await warmUpScan()
      await populateMetrics()
      toast.success("App is ready!", { id: toastId })
    }
    warmUp();
    setupListeners()
  }, [])

  return (
    <div className="w-screen h-screen flex flex-col">
      <QueryClientProvider client={queryClient}>
        <Homepage />
        <TextableContenDialog key={"teste"}/>
      </QueryClientProvider>
    </div>
  );
}

export default App;
