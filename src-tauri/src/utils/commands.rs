use std::path::{Path, PathBuf};

#[tauri::command(async)]
pub fn open_file(path: String) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        use std::process::Command;

        let _ = Command::new("xdg-open")
            .arg(path)
            .spawn()
            .map_err(|e| println!("Error while trying to opening: {}", e));
    }

    Ok(())
}
