use std::fs;
use crate::format::UstxFile;

/// Load a USTX file from disk
#[tauri::command]
pub fn load_ustx_file(path: String) -> Result<UstxFile, String> {
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    
    let project: UstxFile = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse USTX: {}", e))?;
    
    Ok(project)
}

/// Save a USTX file to disk
#[tauri::command]
pub fn save_ustx_file(path: String, project: UstxFile) -> Result<(), String> {
    let content = serde_json::to_string_pretty(&project)
        .map_err(|e| format!("Failed to serialize: {}", e))?;
    
    fs::write(&path, content)
        .map_err(|e| format!("Failed to write file: {}", e))?;
    
    Ok(())
}

/// Create a new project
#[tauri::command]
pub fn create_new_project(name: String, bpm: f64) -> UstxFile {
    UstxFile::new(name).with_bpm(bpm)
}

/// Get default project
#[tauri::command]
pub fn get_default() -> UstxFile {
    UstxFile::default()
}

impl UstxFile {
    pub fn with_bpm(mut self, bpm: f64) -> Self {
        self.bpm = bpm;
        self.tempo = vec![crate::format::ustx::Tempo {
            position: 0,
            bpm
        }];
        self
    }
}
