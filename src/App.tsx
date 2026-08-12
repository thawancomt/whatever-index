
import { invoke } from "@tauri-apps/api/core";
import Homepage from "./pages/homepage/Homepage";
import { useEffect, useTransition } from "react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

const queryClient = new QueryClient()

function App() {

  const [isLoading, startTransition] = useTransition();

  const warmUpScan = async () => {
    startTransition(async () => {
      await invoke("re_scan")
    })
  }


  useEffect(() => {
    warmUpScan()
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
