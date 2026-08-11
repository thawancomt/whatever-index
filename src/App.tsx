
import { invoke } from "@tauri-apps/api/core";
import Homepage from "./pages/homepage/Homepage";
import { useEffect } from "react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

const queryClient = new QueryClient()

function App() {


  const warmUpScan = async () => {
    await invoke("re_scan")
  }


  useEffect(() => {
    warmUpScan()
  }, [])

  return (
    <div className="w-screen h-screen flex flex-col">
      <QueryClientProvider client={queryClient}>
        <Homepage />
      </QueryClientProvider>
    </div>
  );
}

export default App;
