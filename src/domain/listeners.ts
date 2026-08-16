import { listen } from "@tauri-apps/api/event";
import { AppEvent, DomainEvent } from "./events";

export async function setupListeners() {
    await listen<null>(DomainEvent.ScanStarted, (data) => {
        AppEvent.emit(DomainEvent.ScanStarted, data.payload);
    });

    await listen<number>(DomainEvent.ScanCompleted, (data) => {
        AppEvent.emit(DomainEvent.ScanCompleted, data.payload)
    });

    console.log("Events are now being listening")
}
