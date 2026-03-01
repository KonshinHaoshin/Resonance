//! Resonance - Open Singing Synthesis Platform
//! 
//! A Rust + React rewrite of OpenUtau

pub mod audio;
pub mod midi;
pub mod format;
pub mod plugin;

use audio::AudioEngine;
use midi::{Note, Track};
use format::UstxFile;
use plugin::phonemizer::Phonemizer;
use plugin::resampler::Resampler;
use plugin::resampler::builtin::WorldlineResampler;

/// Initialize the audio engine
#[tauri::command]
fn create_audio_engine(sample_rate: u32, channels: u16) -> Result<String, String> {
    let _engine = AudioEngine::with_settings(sample_rate, channels);
    Ok(format!("Audio engine created: {}Hz, {} channels", sample_rate, channels))
}

/// Get project info
#[tauri::command]
fn get_project_info(project: UstxFile) -> Result<String, String> {
    Ok(format!(
        "Project: {} | BPM: {} | Tracks: {}",
        project.name,
        project.bpm,
        project.tracks.len()
    ))
}

/// Create a new note
#[tauri::command]
fn create_note(pitch: u8, velocity: u8, start: u64, duration: u64) -> Result<String, String> {
    let note = Note::new(pitch, velocity, start, duration);
    Ok(format!("Created note: {} at {} for {} ticks", note.name(), start, duration))
}

/// Add a note to track
#[tauri::command]
fn add_note_to_track(track_name: String, pitch: u8, velocity: u8, start: u64, duration: u64) -> Result<String, String> {
    let mut track = Track::new(&track_name);
    let note = Note::new(pitch, velocity, start, duration);
    track.add_note(note);
    Ok(format!("Added {} to track '{}'", note.name(), track_name))
}

/// Test resampler
#[tauri::command]
fn test_resampler() -> Result<String, String> {
    let resampler = WorldlineResampler::new(44100);
    let buffer = resampler.resample("a", 60, 100, 480);
    Ok(format!("Resampler generated {} samples", buffer.len()))
}

/// Get app version
#[tauri::command]
fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            create_audio_engine,
            get_project_info,
            create_note,
            add_note_to_track,
            test_resampler,
            get_version
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
