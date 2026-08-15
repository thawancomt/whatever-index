
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useEffect, useTransition } from "react";
import { toast } from "sonner";
import Homepage from "./pages/homepage/Homepage";
import { useMetrics } from "./stores/metrics.store";
import { AppEvent } from "./domain/events";
import { DownloadProgress } from "./domain/types/download.type";

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



  const initListeners = async () => {
  }

  useEffect(() => {
    const warmUp = async () => {
      const toastId = toast.loading("Warming up the app, please wait...")
      await warmUpScan()
      await populateMetrics()
      toast.success("App is ready!", { id: toastId })
    }
    warmUp();
    initListeners();
  }, [])

  document.documentElement.classList.toggle(
    "dark",
    localStorage.theme === "dark" ||
      (!("theme" in localStorage) && window.matchMedia("(prefers-color-scheme: dark)").matches),
  );
  // Whenever the user explicitly chooses light mode
  localStorage.theme = "light";
  // Whenever the user explicitly chooses dark mode
  localStorage.theme = "dark";
  // Whenever the user explicitly chooses to respect the OS preference
  localStorage.removeItem("theme");

  return (
    <div className="w-screen h-screen flex flex-col">
      <QueryClientProvider client={queryClient}>
        <Homepage />
      </QueryClientProvider>
    </div>
  );
}

export default App;
