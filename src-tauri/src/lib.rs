//! Resonance - Open Singing Synthesis Platform
//! 
//! A Rust + React rewrite of OpenUtau

pub mod audio;
pub mod midi;
pub mod format;
pub mod plugin;

use audio::AudioEngine;
use midi::Note;
use format::UstxFile;
use plugin::resampler::{Resampler, builtin::WorldlineResampler};
use std::sync::Mutex;
use once_cell::sync::Lazy;

static AUDIO_ENGINE: Lazy<Mutex<AudioEngine>> = Lazy::new(|| Mutex::new(AudioEngine::new()));

/// Initialize the audio engine
#[tauri::command]
fn create_audio_engine(sample_rate: u32, channels: u16) -> Result<String, String> {
    let mut engine = AUDIO_ENGINE.lock().map_err(|e| e.to_string())?;
    *engine = AudioEngine::with_settings(sample_rate, channels);
    Ok(format!("Audio engine created: {}Hz, {} channels", sample_rate, channels))
}

/// Play audio (generates test tone)
#[tauri::command]
fn play_audio() -> Result<String, String> {
    let mut engine = AUDIO_ENGINE.lock().map_err(|e| e.to_string())?;
    
    let resampler = WorldlineResampler::new(44100);
    let buffer = resampler.resample("a", 60, 100, 960);
    
    let samples: Vec<f32> = buffer.to_vec();
    for chunk in samples.chunks(2) {
        if chunk.len() == 2 {
            engine.add_samples(chunk[0], chunk[1]);
        }
    }
    
    engine.play();
    Ok("Playing".to_string())
}

/// Stop audio
#[tauri::command]
fn stop_audio() -> Result<String, String> {
    let mut engine = AUDIO_ENGINE.lock().map_err(|e| e.to_string())?;
    engine.stop();
    Ok("Stopped".to_string())
}

/// Get audio engine status
#[tauri::command]
fn get_audio_status() -> Result<String, String> {
    let engine = AUDIO_ENGINE.lock().map_err(|e| e.to_string())?;
    Ok(format!("Playing: {}, Sample Rate: {}Hz", engine.is_playing(), engine.sample_rate()))
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
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            create_audio_engine,
            play_audio,
            stop_audio,
            get_audio_status,
            get_project_info,
            create_note,
            test_resampler,
            get_version,
            format::io::load_ustx_file,
            format::io::save_ustx_file,
            format::io::create_new_project,
            format::io::get_default,
            format::midi_io::import_midi,
            format::midi_io::export_midi
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
