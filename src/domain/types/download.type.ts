// response by tauri interface
export interface DownloadProgress {
  file_name: string;
  downloaded_bytes: number;
  total_bytes: number | null;
  percentage: number | null;
}
