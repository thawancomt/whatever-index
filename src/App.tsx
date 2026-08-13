
import { invoke } from "@tauri-apps/api/core";
import Homepage from "./pages/homepage/Homepage";
import { useEffect, useTransition } from "react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { useMetrics } from "./stores/metrics.store";

const queryClient = new QueryClient()

function App() {
  const {actions : {setTotalFiles}} = useMetrics()
  const [isLoading, startTransition] = useTransition();

  const warmUpScan = async () => {
    startTransition(async () => {
      await invoke("re_scan")
    })
  }
  const populateMetrics = async () => {
    const result = await invoke<number>("get_total_files_indexed");
    if (result) {
    setTotalFiles(result)
    }
  }


  useEffect(() => {
    warmUpScan()
    populateMetrics()
  }, [])

  return (
    <div className="w-screen h-screen flex flex-col">
      <QueryClientProvider client={queryClient}>
        {
          isLoading && (
            <p>
              Scanning
            </p>
          )
        }
        <Homepage />
      </QueryClientProvider>
    </div>
  );
}

export default App;
