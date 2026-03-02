use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use midly::Smf;
use midly::Timing;
use midly::TrackEventKind;
use midly::MetaMessage;
use midly::num::u7;
use crate::format::ustx::{UstxFile, TrackData, NoteData, Tempo};

/// Import MIDI file and convert to USTX project
#[tauri::command]
pub fn import_midi(path: String) -> Result<UstxFile, String> {
    let data = std::fs::read(&path)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    
    let smf = Smf::parse(&data)
        .map_err(|e| format!("Failed to parse MIDI: {}", e))?;
    
    let timing = &smf.header.timing;
    
    // Convert timing to ticks per quarter
    let tpq: u64 = match timing {
        Timing::Metrical(tpq) => tpq.as_int() as u64,
        Timing::Timecode(_, _) => 480, // Default fallback
    };
    
    println!("MIDI: tracks={}, tpq={}", smf.tracks.len(), tpq);
    
    let mut all_tracks: Vec<TrackData> = Vec::new();
    
    // Process each track
    for (track_idx, track) in smf.tracks.iter().enumerate() {
        let mut notes: Vec<NoteData> = Vec::new();
        let mut current_tick: u64 = 0;
        let mut active_notes: std::collections::HashMap<u8, u64> = std::collections::HashMap::new();
        
        for event in track {
            let delta = event.delta.as_int() as u64;
            current_tick += delta;
            
            match &event.kind {
                TrackEventKind::Midi { channel: _, message } => {
                    use midly::MidiMessage::*;
                    match message {
                        NoteOn { key, vel } => {
                            if vel.as_int() > 0 {
                                // Note On
                                active_notes.insert(key.as_int(), current_tick);
                            } else {
                                // Note Off (velocity 0 = Note Off)
                                if let Some(start) = active_notes.remove(&key.as_int()) {
                                    let duration = current_tick - start;
                                    if duration > 0 {
                                        notes.push(NoteData {
                                            position: start,
                                            duration,
                                            pitch: key.as_int() as i32,
                                            velocity: vel.as_int() as u32,
                                            vibrato: None,
                                        });
                                    }
                                }
                            }
                        }
                        NoteOff { key, .. } => {
                            if let Some(start) = active_notes.remove(&key.as_int()) {
                                let duration = current_tick - start;
                                if duration > 0 {
                                    notes.push(NoteData {
                                        position: start,
                                        duration,
                                        pitch: key.as_int() as i32,
                                        velocity: 100,
                                        vibrato: None,
                                    });
                                }
                            }
                        }
                        _ => {}
                    }
                }
                TrackEventKind::Meta(MetaMessage::Tempo(tempo)) => {
                    println!("Tempo: {} us/beat", tempo.as_int());
                }
                _ => {}
            }
        }
        
        // Close any remaining notes
        let max_tick = notes.iter().map(|n| n.position + n.duration).max().unwrap_or(0);
        for (pitch, start) in active_notes.iter() {
            let duration = max_tick - start;
            if duration > 0 {
                notes.push(NoteData {
                    position: *start,
                    duration,
                    pitch: *pitch as i32,
                    velocity: 100,
                    vibrato: None,
                });
            }
        }
        
        if !notes.is_empty() {
            notes.sort_by_key(|n| n.position);
            
            let track_data = TrackData {
                name: format!("Track {}", track_idx + 1),
                color: None,
                notes,
            };
            all_tracks.push(track_data);
        }
    }
    
    if all_tracks.is_empty() {
        all_tracks.push(TrackData::default());
    }
    
    let mut project = UstxFile::default();
    project.tracks = all_tracks;
    project.name = Path::new(&path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Imported")
        .to_string();
    project.bpm = 120.0;
    project.tempo = vec![Tempo {
        position: 0,
        bpm: 120.0,
    }];
    
    Ok(project)
}

/// Export USTX project to MIDI
#[tauri::command]
pub fn export_midi(path: String, project: UstxFile) -> Result<(), String> {
    let file = File::create(&path)
        .map_err(|e| format!("Failed to create file: {}", e))?;
    
    let mut writer = BufWriter::new(file);
    
    // MIDI header
    writer.write_all(b"MThd").map_err(|e| e.to_string())?;
    writer.write_all(&6u32.to_be_bytes()).map_err(|e| e.to_string())?;
    writer.write_all(&0u16.to_be_bytes()).map_err(|e| e.to_string())?; // Format 0
    writer.write_all(&(project.tracks.len() as u16).to_be_bytes()).map_err(|e| e.to_string())?;
    writer.write_all(&480u16.to_be_bytes()).map_err(|e| e.to_string())?; // 480 ticks per quarter
    
    // Build track data
    let mut track_data: Vec<u8> = Vec::new();
    
    // Tempo event (meta 0x51)
    track_data.push(0x00);
    track_data.push(0xFF);
    track_data.push(0x51);
    track_data.push(0x03);
    let tempo = (60000000.0 / project.bpm) as u32;
    track_data.extend_from_slice(&tempo.to_be_bytes());
    
    // Collect all notes with their track info
    let mut all_notes: Vec<(&NoteData, usize)> = Vec::new();
    for (track_idx, track) in project.tracks.iter().enumerate() {
        for note in &track.notes {
            all_notes.push((note, track_idx));
        }
    }
    
    // Sort by position
    all_notes.sort_by_key(|(n, _)| n.position);
    
    let mut last_tick: u64 = 0;
    
    for (note, _track_idx) in &all_notes {
        let delta = note.position - last_tick;
        last_tick = note.position;
        
        // Delta time
        write_var_len(&mut track_data, delta as u32);
        // Note On
        track_data.push(0x90);
        track_data.push(note.pitch as u8);
        track_data.push(note.velocity.min(127) as u8);
        
        // Delta time for Note Off
        write_var_len(&mut track_data, note.duration as u32);
        // Note Off
        track_data.push(0x80);
        track_data.push(note.pitch as u8);
        track_data.push(0);
    }
    
    // End of track
    track_data.push(0x00);
    track_data.push(0xFF);
    track_data.push(0x2F);
    track_data.push(0x00);
    
    // Track chunk
    writer.write_all(b"MTrk").map_err(|e| e.to_string())?;
    writer.write_all(&(track_data.len() as u32).to_be_bytes()).map_err(|e| e.to_string())?;
    writer.write_all(&track_data).map_err(|e| e.to_string())?;
    
    writer.flush().map_err(|e| e.to_string())?;
    
    Ok(())
}

fn write_var_len(data: &mut Vec<u8>, value: u32) {
    let mut buffer = Vec::new();
    let mut v = value;
    buffer.push((v & 0x7F) as u8);
    v >>= 7;
    while v > 0 {
        buffer.push(((v & 0x7F) | 0x80) as u8);
        v >>= 7;
    }
    buffer.reverse();
    data.extend_from_slice(&buffer);
}
