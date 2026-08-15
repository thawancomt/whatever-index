import { DownloadProgress } from "./types/download.type";

type APP_EVENT = {
    OCR_MODEL_DOWNLOAD_PROGRESS: DownloadProgress
    OCR_MODEL_DOWNLOAD_CONMPLETED: DownloadProgress
}

type Callback<T> = (data: T) => void;

class AppEventBus {
    private listerners: { event: keyof APP_EVENT, callback: Callback<APP_EVENT[keyof APP_EVENT]> }[] = [];

    on<K extends keyof APP_EVENT>(event: K, callback: Callback<APP_EVENT[K]>) {
        this.listerners.push({ event, callback });
    }
    emit<K extends keyof APP_EVENT>(event: K, data: APP_EVENT[K]) {
        this.listerners.forEach(listener => {
            if (listener.event === event) {
                listener.callback(data);
            }
        });
    }
}

export const AppEvent = new AppEventBus();
