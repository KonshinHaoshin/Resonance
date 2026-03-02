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
use format::ustx::{TrackData, NoteData};
use format::render::{RenderFormat, RenderConfig, AudioRenderer, start_render as start_render_impl, cancel_render as cancel_render_impl, get_render_progress as get_render_progress_impl};
use format::wav::{read_wav, write_wav, export_audio_buffer, WavAudio, WavSpec, WavError};
use plugin::resampler::{Resampler, builtin::WorldlineResampler};
use std::sync::{Mutex, Arc};
use once_cell::sync::Lazy;
use tracing::{info, error, Level};
use tracing_subscriber::FmtSubscriber;

static AUDIO_ENGINE: Lazy<Mutex<AudioEngine>> = Lazy::new(|| {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(true)
        .with_line_number(true)
        .finish();
    
    if tracing::subscriber::set_global_default(subscriber).is_err() {
        eprintln!("Logger already initialized");
    }
    
    info!("Resonance audio engine initializing...");
    Mutex::new(AudioEngine::new())
});

/// Initialize the audio engine
#[tauri::command]
fn create_audio_engine(sample_rate: u32, channels: u16) -> Result<String, String> {
    info!("Creating audio engine: {}Hz, {} channels", sample_rate, channels);
    let mut engine = AUDIO_ENGINE.lock().map_err(|e| {
        error!("Failed to lock audio engine: {}", e);
        e.to_string()
    })?;
    *engine = AudioEngine::with_settings(sample_rate, channels);
    info!("Audio engine created successfully");
    Ok(format!("Audio engine created: {}Hz, {} channels", sample_rate, channels))
}

/// Play audio (generates test tone)
#[tauri::command]
fn play_audio() -> Result<String, String> {
    info!("Play audio command received");
    let mut engine = AUDIO_ENGINE.lock().map_err(|e| e.to_string())?;
    
    // Use Worldline resampler to generate test tone
    let resampler = WorldlineResampler::new(44100);
    let buffer = resampler.resample("a", 60, 100, 960);
    
    let samples: Vec<f32> = buffer.to_vec();
    for chunk in samples.chunks(2) {
        if chunk.len() == 2 {
            engine.add_samples(chunk[0], chunk[1]);
        }
    }
    
    engine.play();
    info!("Audio playback started");
    Ok("Playing".to_string())
}

/// Stop audio
#[tauri::command]
fn stop_audio() -> Result<String, String> {
    info!("Stop audio command received");
    let mut engine = AUDIO_ENGINE.lock().map_err(|e| e.to_string())?;
    engine.stop();
    info!("Audio playback stopped");
    Ok("Stopped".to_string())
}

/// Get audio buffer as samples (for Web Audio API playback)
#[tauri::command]
fn get_audio_samples() -> Result<Vec<f32>, String> {
    let engine = AUDIO_ENGINE.lock().map_err(|e| e.to_string())?;
    Ok(engine.get_samples())
}

/// Get audio buffer length
#[tauri::command]
fn get_audio_buffer_len() -> Result<usize, String> {
    let engine = AUDIO_ENGINE.lock().map_err(|e| e.to_string())?;
    Ok(engine.buffer_len())
}

/// Generate and render project to audio buffer
#[tauri::command]
fn render_project(project: UstxFile) -> Result<usize, String> {
    info!("Rendering project: {}", project.name);
    let engine = AUDIO_ENGINE.lock().map_err(|e| e.to_string())?;
    
    // Clear existing buffer
    engine.clear_buffer();
    
    // Generate audio from project notes
    let sample_rate = engine.sample_rate() as f32;
    let resampler = WorldlineResampler::new(sample_rate as u32);
    
    for track in &project.tracks {
        for note in &track.notes {
            // Convert pitch to note name (simplified)
            let note_names = ["c", "d", "e", "f", "g", "a", "b"];
            let octave = (note.pitch / 12) - 1;
            let note_idx = note.pitch % 12;
            let note_name = format!("{}{}", note_names[(note_idx as usize) % 7], octave);
            
            // Resample note
            let buffer = resampler.resample(&note_name, note.pitch as u8, note.velocity as u8, note.duration);
            
            // Add to audio buffer
            for chunk in buffer.to_vec().chunks(2) {
                if chunk.len() == 2 {
                    engine.add_samples(chunk[0], chunk[1]);
                } else if chunk.len() == 1 {
                    engine.add_sample(chunk[0]);
                }
            }
        }
    }
    
    let len = engine.buffer_len();
    info!("Rendered {} samples", len);
    Ok(len)
}

/// Get audio engine status
#[tauri::command]
fn get_audio_status() -> Result<String, String> {
    let engine = AUDIO_ENGINE.lock().map_err(|e| e.to_string())?;
    Ok(format!(
        "Playing: {}, Paused: {}, Sample Rate: {}Hz, Rate: {:.1}x, Loop: {}",
        engine.is_playing(),
        engine.is_paused(),
        engine.sample_rate(),
        engine.playback_rate(),
        engine.is_loop_enabled()
    ))
}

// ============================================================================
// Advanced Playback Control Commands
// ============================================================================

/// Pause audio playback
#[tauri::command]
fn pause_audio() -> Result<String, String> {
    let mut engine = AUDIO_ENGINE.lock().map_err(|e| e.to_string())?;
    engine.pause();
    Ok("Paused".to_string())
}

/// Resume audio playback
#[tauri::command]
fn resume_audio() -> Result<String, String> {
    let mut engine = AUDIO_ENGINE.lock().map_err(|e| e.to_string())?;
    engine.resume();
    Ok("Resumed".to_string())
}

/// Seek to specific position (in ticks)
#[tauri::command]
fn seek_audio(position: u64) -> Result<String, String> {
    let mut engine = AUDIO_ENGINE.lock().map_err(|e| e.to_string())?;
    engine.seek_to(position);
    Ok(format!("Seeked to position {}", position))
}

/// Set playback rate (0.5 - 2.0)
#[tauri::command]
fn set_playback_rate(rate: f32) -> Result<String, String> {
    let mut engine = AUDIO_ENGINE.lock().map_err(|e| e.to_string())?;
    engine.set_playback_rate(rate);
    Ok(format!("Playback rate set to {:.1}x", rate))
}

/// Get current playback position (in ticks)
#[tauri::command]
fn get_current_position() -> Result<u64, String> {
    let engine = AUDIO_ENGINE.lock().map_err(|e| e.to_string())?;
    Ok(engine.position())
}

/// Set loop mode
#[tauri::command]
fn set_loop_mode(enabled: bool, start: Option<u64>, end: Option<u64>) -> Result<String, String> {
    let mut engine = AUDIO_ENGINE.lock().map_err(|e| e.to_string())?;
    engine.set_loop_enabled(enabled);
    if let (Some(s), Some(e)) = (start, end) {
        engine.set_loop_region(s, e);
    }
    Ok(format!("Loop mode: {}, region: {}-{}", enabled, engine.loop_start(), engine.loop_end()))
}

/// Get playback info
#[tauri::command]
fn get_playback_info() -> Result<String, String> {
    let engine = AUDIO_ENGINE.lock().map_err(|e| e.to_string())?;
    Ok(format!(
        "{{\"playing\": {}, \"paused\": {}, \"position\": {}, \"rate\": {}, \"loop\": {}, \"loopStart\": {}, \"loopEnd\": {}, \"volume\": {}}}",
        engine.is_playing(),
        engine.is_paused(),
        engine.position(),
        engine.playback_rate(),
        engine.is_loop_enabled(),
        engine.loop_start(),
        engine.loop_end(),
        engine.volume()
    ))
}

// ============================================================================
// Volume Control Commands
// ============================================================================

/// Set volume (0.0 - 2.0, default 1.0)
#[tauri::command]
fn set_volume(volume: f32) -> Result<String, String> {
    let mut engine = AUDIO_ENGINE.lock().map_err(|e| e.to_string())?;
    engine.set_volume(volume);
    Ok(format!("Volume set to {:.1}", volume))
}

/// Get current volume
#[tauri::command]
fn get_volume() -> Result<f32, String> {
    let engine = AUDIO_ENGINE.lock().map_err(|e| e.to_string())?;
    Ok(engine.volume())
}

/// Apply fade in to audio buffer
#[tauri::command]
fn apply_fade_in(duration_ms: u32) -> Result<String, String> {
    let sample_rate = {
        let engine = AUDIO_ENGINE.lock().map_err(|e| e.to_string())?;
        engine.sample_rate()
    };
    
    // Convert ms to samples
    let fade_samples = (sample_rate as f32 * duration_ms as f32 / 1000.0) as usize;
    
    let engine = AUDIO_ENGINE.lock().map_err(|e| e.to_string())?;
    if let Ok(mut buf) = engine.buffer().lock() {
        buf.apply_fade(fade_samples, 0);
    }
    
    Ok(format!("Fade in applied: {}ms", duration_ms))
}

/// Apply fade out to audio buffer
#[tauri::command]
fn apply_fade_out(duration_ms: u32) -> Result<String, String> {
    let sample_rate = {
        let engine = AUDIO_ENGINE.lock().map_err(|e| e.to_string())?;
        engine.sample_rate()
    };
    
    // Convert ms to samples
    let fade_samples = (sample_rate as f32 * duration_ms as f32 / 1000.0) as usize;
    
    let engine = AUDIO_ENGINE.lock().map_err(|e| e.to_string())?;
    if let Ok(mut buf) = engine.buffer().lock() {
        buf.apply_fade(0, fade_samples);
    }
    
    Ok(format!("Fade out applied: {}ms", duration_ms))
}

// ============================================================================
// Audio Rendering Commands
// ============================================================================

/// Start rendering project to audio file
#[tauri::command]
fn start_render(
    project: UstxFile,
    output_path: String,
    format: String,
    sample_rate: u32,
    bit_depth: u16,
) -> Result<String, String> {
    let fmt = match format.to_lowercase().as_str() {
        "wav16" => RenderFormat::Wav16,
        "wav24" => RenderFormat::Wav24,
        "wav32" => RenderFormat::Wav32,
        "mp3" => RenderFormat::Mp3,
        "flac" => RenderFormat::Flac,
        _ => return Err(format!("Unsupported format: {}", format)),
    };
    
    let path = std::path::Path::new(&output_path);
    
    start_render_impl(&project, path, fmt, sample_rate, bit_depth)
        .map_err(|e| e.to_string())?;
    
    Ok(format!("Rendered to {}", output_path))
}

/// Cancel ongoing render
#[tauri::command]
fn cancel_render() -> Result<String, String> {
    cancel_render_impl();
    Ok("Render cancelled".to_string())
}

/// Get render progress (0.0 - 100.0)
#[tauri::command]
fn get_render_progress() -> Result<f32, String> {
    Ok(get_render_progress_impl())
}

/// Get supported render formats
#[tauri::command]
fn get_render_formats() -> Result<Vec<String>, String> {
    Ok(vec![
        "wav16".to_string(),
        "wav24".to_string(),
        "wav32".to_string(),
        "mp3".to_string(),
        "flac".to_string(),
    ])
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
    info!("Created note: {} at {} for {} ticks", note.name(), start, duration);
    Ok(format!("Created note: {} at {} for {} ticks", note.name(), start, duration))
}

/// Test resampler
#[tauri::command]
fn test_resampler() -> Result<String, String> {
    info!("Testing resampler");
    let resampler = WorldlineResampler::new(44100);
    let buffer = resampler.resample("a", 60, 100, 480);
    info!("Resampler generated {} samples", buffer.len());
    Ok(format!("Resampler generated {} samples", buffer.len()))
}

// ============================================================================
// WAV Import/Export Commands
// ============================================================================

/// Import WAV audio file
#[tauri::command]
fn import_wav(path: String) -> Result<String, String> {
    info!("Importing WAV file: {}", path);
    let path_ref = std::path::Path::new(&path);
    let audio = read_wav(path_ref).map_err(|e| {
        error!("Failed to import WAV: {}", e);
        e.to_string()
    })?;
    
    info!("Imported WAV: {}Hz, {} channels, {} bits, {:.2}s", 
        audio.spec.sample_rate, 
        audio.spec.channels, 
        audio.spec.bits_per_sample,
        audio.duration());
    
    Ok(format!(
        "{{\"sampleRate\": {}, \"channels\": {}, \"bitsPerSample\": {}, \"duration\": {:.3}, \"samples\": {}}}",
        audio.spec.sample_rate,
        audio.spec.channels,
        audio.spec.bits_per_sample,
        audio.duration(),
        audio.len()
    ))
}

/// Export audio buffer to WAV file
#[tauri::command]
fn export_wav(path: String, sample_rate: u32, channels: u16) -> Result<String, String> {
    info!("Exporting WAV file: {}", path);
    
    let path_ref = std::path::Path::new(&path);
    
    // Get samples from audio engine
    let engine = AUDIO_ENGINE.lock().map_err(|e| e.to_string())?;
    let samples = engine.get_samples();
    
    if samples.is_empty() {
        return Err("No audio samples to export".to_string());
    }
    
    export_audio_buffer(path_ref, &samples, sample_rate, channels).map_err(|e| {
        error!("Failed to export WAV: {}", e);
        e.to_string()
    })?;
    
    info!("Exported {} samples to WAV", samples.len() / channels as usize);
    Ok(format!("Exported to {}", path))
}

/// Export rendered project directly to WAV
#[tauri::command]
fn export_project_wav(
    project: UstxFile,
    output_path: String,
    sample_rate: u32,
    channels: u16,
    bit_depth: u16,
) -> Result<String, String> {
    info!("Exporting project to WAV: {}", output_path);
    
    // First render the project
    let engine = AUDIO_ENGINE.lock().map_err(|e| e.to_string())?;
    engine.clear_buffer();
    
    // Generate audio from project notes
    let resampler = WorldlineResampler::new(sample_rate);
    
    let mut total_samples = 0usize;
    
    for track in &project.tracks {
        for note in &track.notes {
            let note_names = ["c", "d", "e", "f", "g", "a", "b"];
            let octave = (note.pitch / 12) - 1;
            let note_idx = note.pitch % 12;
            let note_name = format!("{}{}", note_names[(note_idx as usize) % 7], octave);
            
            let buffer = resampler.resample(&note_name, note.pitch as u8, note.velocity as u8, note.duration);
            let note_samples = buffer.to_vec();
            total_samples += note_samples.len();
            
            for chunk in note_samples.chunks(2) {
                if chunk.len() == 2 {
                    engine.add_samples(chunk[0], chunk[1]);
                } else if chunk.len() == 1 {
                    engine.add_sample(chunk[0]);
                }
            }
        }
    }
    drop(engine);
    
    // Now export
    let engine = AUDIO_ENGINE.lock().map_err(|e| e.to_string())?;
    let samples = engine.get_samples();
    
    let path_ref = std::path::Path::new(&output_path);
    
    // Use write_wav with specified bit depth
    let spec = WavSpec {
        sample_rate,
        channels,
        bits_per_sample: bit_depth,
    };
    let audio = WavAudio {
        spec,
        data: samples,
    };
    
    write_wav(path_ref, &audio).map_err(|e| {
        error!("Failed to export WAV: {}", e);
        e.to_string()
    })?;
    
    info!("Exported {} samples to WAV: {}", total_samples, output_path);
    Ok(format!("Exported to {}", output_path))
}

/// Get WAV file info without loading full audio
#[tauri::command]
fn get_wav_info(path: String) -> Result<String, String> {
    let path_ref = std::path::Path::new(&path);
    let audio = read_wav(path_ref).map_err(|e| e.to_string())?;
    
    Ok(format!(
        "{{\"sampleRate\": {}, \"channels\": {}, \"bitsPerSample\": {}, \"duration\": {:.3}, \"samples\": {}}}",
        audio.spec.sample_rate,
        audio.spec.channels,
        audio.spec.bits_per_sample,
        audio.duration(),
        audio.len()
    ))
}

/// Get app version
#[tauri::command]
fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(true)
        .with_line_number(true)
        .finish();
    
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");
    
    info!("Resonance v{} starting...", env!("CARGO_PKG_VERSION"));
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            create_audio_engine,
            play_audio,
            stop_audio,
            get_audio_samples,
            get_audio_buffer_len,
            render_project,
            get_audio_status,
            // Advanced playback control
            pause_audio,
            resume_audio,
            seek_audio,
            set_playback_rate,
            get_current_position,
            set_loop_mode,
            get_playback_info,
            // Volume control
            set_volume,
            get_volume,
            apply_fade_in,
            apply_fade_out,
            // Render commands
            start_render,
            cancel_render,
            get_render_progress,
            get_render_formats,
            // Project commands
            get_project_info,
            create_note,
            test_resampler,
            get_version,
            // WAV import/export commands
            import_wav,
            export_wav,
            export_project_wav,
            get_wav_info,
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