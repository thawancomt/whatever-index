import ReactDOM from "react-dom/client";
import { Toaster } from "sonner";
import App from "./App";
import "./global.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <>
        <App />
        <Toaster richColors />
    </>
);
