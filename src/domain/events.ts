export enum DomainEvent {
    ScanStarted = "scan_started",
    ScanCompleted = "scan_completed",
    NewFilesIndexed = "new_files_indexed",
}

type EventMap = {
    [DomainEvent.ScanStarted]: null;
    [DomainEvent.ScanCompleted]: number;
    [DomainEvent.NewFilesIndexed]: null;
};

type Callback<T> = (data: T) => void;

interface Listener<K extends keyof EventMap = keyof EventMap> {
    event: K;
    callback: Callback<EventMap[K]>;
}

class AppEventBus {
    private listeners: Listener[] = [];

    on<K extends keyof EventMap>(event: K, callback: Callback<EventMap[K]>) {
        this.listeners.push({ event, callback } as Listener);
    }

    off<K extends keyof EventMap>(event: K, callback: Callback<EventMap[K]>) {
        this.listeners = this.listeners.filter(
            (listener) => !(listener.event === event && listener.callback === callback)
        );
    }

    emit<K extends keyof EventMap>(event: K, data: EventMap[K]) {
        this.listeners.forEach((listener) => {
            if (listener.event === event) {
                (listener.callback as Callback<EventMap[K]>)(data);
            }
        });
    }
}

export const AppEvent = new AppEventBus();
