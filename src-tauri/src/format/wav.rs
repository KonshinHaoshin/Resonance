//! WAV Audio File Format Support
//!
//! Provides WAV file reading and writing capabilities for importing and exporting audio.

use std::fs::File;
use std::io::{Read, Write, Seek, SeekFrom};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WavError {
    #[error("Failed to open file: {0}")]
    FileOpen(String),
    #[error("Invalid WAV file: {0}")]
    InvalidFormat(String),
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Invalid sample rate: {0}")]
    InvalidSampleRate(u32),
}

/// WAV file format specifications
#[derive(Debug, Clone)]
pub struct WavSpec {
    pub sample_rate: u32,
    pub channels: u16,
    pub bits_per_sample: u16,
}

impl Default for WavSpec {
    fn default() -> Self {
        Self {
            sample_rate: 44100,
            channels: 2,
            bits_per_sample: 16,
        }
    }
}

/// WAV audio data container
#[derive(Debug, Clone)]
pub struct WavAudio {
    pub spec: WavSpec,
    pub data: Vec<f32>, // Normalized to -1.0 to 1.0
}

impl WavAudio {
    /// Create new WAV audio with spec
    pub fn new(spec: WavSpec) -> Self {
        Self {
            spec,
            data: Vec::new(),
        }
    }

    /// Get the number of samples (per channel)
    pub fn len(&self) -> usize {
        let channel_count = self.spec.channels as usize;
        self.data.len() / channel_count
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get duration in seconds
    pub fn duration(&self) -> f64 {
        self.len() as f64 / self.spec.sample_rate as f64
    }

    /// Create stereo interleaved data
    pub fn to_interleaved(&self) -> Vec<f32> {
        self.data.clone()
    }

    /// Create mono by mixing down stereo
    pub fn to_mono(&self) -> Vec<f32> {
        if self.spec.channels == 1 {
            return self.data.clone();
        }

        let mut mono = Vec::with_capacity(self.data.len() / 2);
        for i in (0..self.data.len()).step_by(2) {
            let left = self.data[i];
            let right = self.data.get(i + 1).copied().unwrap_or(left);
            mono.push((left + right) * 0.5);
        }
        mono
    }
}

/// Read a WAV file from disk
pub fn read_wav(path: &Path) -> Result<WavAudio, WavError> {
    let mut file = File::open(path).map_err(|e| WavError::FileOpen(e.to_string()))?;
    
    // Read RIFF header
    let mut riff_header = [0u8; 12];
    file.read_exact(&mut riff_header)?;
    
    // Verify RIFF header
    if &riff_header[0..4] != b"RIFF" {
        return Err(WavError::InvalidFormat("Missing RIFF header".to_string()));
    }
    if &riff_header[8..12] != b"WAVE" {
        return Err(WavError::InvalidFormat("Not a WAV file".to_string()));
    }

    // Read chunks
    let mut sample_rate = 44100u32;
    let mut channels = 2u16;
    let mut bits_per_sample = 16u16;
    let mut data_size = 0usize;
    let mut data_offset = 0usize;

    loop {
        let mut chunk_header = [0u8; 8];
        if file.read_exact(&mut chunk_header).is_err() {
            break;
        }

        let chunk_id = &chunk_header[0..4];
        let chunk_size = u32::from_le_bytes([chunk_header[4], chunk_header[5], chunk_header[6], chunk_header[7]]) as usize;

        if chunk_id == b"fmt " {
            let mut fmt_data = vec![0u8; chunk_size.min(16)];
            file.read_exact(&mut fmt_data)?;
            
            let audio_format = u16::from_le_bytes([fmt_data[0], fmt_data[1]]);
            channels = u16::from_le_bytes([fmt_data[2], fmt_data[3]]);
            sample_rate = u32::from_le_bytes([fmt_data[4], fmt_data[5], fmt_data[6], fmt_data[7]]);
            bits_per_sample = u16::from_le_bytes([fmt_data[14], fmt_data[15]]);

            if audio_format != 1 && audio_format != 3 && audio_format != 65534 {
                return Err(WavError::UnsupportedFormat(format!("Audio format {} not supported (only PCM)", audio_format)));
            }
        } else if chunk_id == b"data" {
            data_size = chunk_size;
            data_offset = file.stream_position()? as usize;
            // Skip data for now, we'll come back to it
            file.seek(SeekFrom::Current(chunk_size as i64))?;
        } else {
            // Skip unknown chunk
            file.seek(SeekFrom::Current(chunk_size as i64))?;
        }

        // Check if we have all needed data
        if data_offset > 0 && sample_rate > 0 {
            // Try to continue reading to find end of data chunk
            let current_pos = file.stream_position()?;
            if current_pos >= (data_offset + data_size) as u64 {
                break;
            }
        }
    }

    // Validate
    if sample_rate == 0 {
        return Err(WavError::InvalidSampleRate(sample_rate));
    }

    // Read actual audio data
    file.seek(SeekFrom::Start(data_offset as u64))?;
    let bytes_per_sample = (bits_per_sample / 8) as usize;
    let total_samples = data_size / bytes_per_sample;
    let sample_count = total_samples / (channels as usize);

    let mut raw_data = vec![0u8; data_size];
    file.read_exact(&mut raw_data)?;

    // Convert to f32
    let mut audio_data = Vec::with_capacity(sample_count * channels as usize);

    match bits_per_sample {
        8 => {
            for chunk in raw_data.chunks(channels as usize) {
                for &sample in chunk {
                    audio_data.push((sample as f32 - 128.0) / 128.0);
                }
            }
        }
        16 => {
            for chunk in raw_data.chunks(2 * channels as usize) {
                for i in 0..channels as usize {
                    let sample = i16::from_le_bytes([chunk[i * 2], chunk[i * 2 + 1]]);
                    audio_data.push(sample as f32 / 32768.0);
                }
            }
        }
        24 => {
            for chunk in raw_data.chunks(3 * channels as usize) {
                for i in 0..channels as usize {
                    let sample = i32::from_le_bytes([0, chunk[i * 3], chunk[i * 3 + 1], chunk[i * 3 + 2]])
                        >> 8;
                    audio_data.push(sample as f32 / 8388608.0);
                }
            }
        }
        32 => {
            for chunk in raw_data.chunks(4 * channels as usize) {
                for i in 0..channels as usize {
                    let sample = i32::from_le_bytes([
                        chunk[i * 4],
                        chunk[i * 4 + 1],
                        chunk[i * 4 + 2],
                        chunk[i * 4 + 3],
                    ]);
                    audio_data.push(sample as f32 / 2147483648.0);
                }
            }
        }
        _ => {
            return Err(WavError::UnsupportedFormat(format!(
                "{} bits per sample not supported",
                bits_per_sample
            )));
        }
    }

    Ok(WavAudio {
        spec: WavSpec {
            sample_rate,
            channels,
            bits_per_sample,
        },
        data: audio_data,
    })
}

/// Write WAV file to disk
pub fn write_wav(path: &Path, audio: &WavAudio) -> Result<(), WavError> {
    let mut file = File::create(path).map_err(|e| WavError::FileOpen(e.to_string()))?;

    let spec = &audio.spec;
    let bytes_per_sample = (spec.bits_per_sample / 8) as usize;
    let data_size = audio.data.len() * bytes_per_sample;
    let file_size = 36 + data_size;

    // RIFF header
    file.write_all(b"RIFF")?;
    file.write_all(&file_size.to_le_bytes())?;
    file.write_all(b"WAVE")?;

    // fmt chunk
    file.write_all(b"fmt ")?;
    file.write_all(&16u32.to_le_bytes())?; // Chunk size
    file.write_all(&1u16.to_le_bytes())?; // Audio format (1 = PCM)
    file.write_all(&spec.channels.to_le_bytes())?;
    file.write_all(&spec.sample_rate.to_le_bytes())?;
    
    // Byte rate and block align
    let byte_rate = spec.sample_rate as u32 * spec.channels as u32 * bytes_per_sample as u32;
    let block_align = spec.channels * bytes_per_sample as u16;
    file.write_all(&byte_rate.to_le_bytes())?;
    file.write_all(&block_align.to_le_bytes())?;
    file.write_all(&spec.bits_per_sample.to_le_bytes())?;

    // data chunk
    file.write_all(b"data")?;
    file.write_all(&(data_size as u32).to_le_bytes())?;

    // Write samples
    match spec.bits_per_sample {
        8 => {
            for &sample in &audio.data {
                let value = ((sample * 127.0).clamp(-128.0, 127.0) + 128.0) as u8;
                file.write_all(&[value])?;
            }
        }
        16 => {
            for &sample in &audio.data {
                let value = (sample.clamp(-1.0, 1.0) * 32767.0) as i16;
                file.write_all(&value.to_le_bytes())?;
            }
        }
        24 => {
            for &sample in &audio.data {
                let value = (sample.clamp(-1.0, 1.0) * 8388607.0) as i32;
                file.write_all(&value.to_le_bytes()[1..4])?;
            }
        }
        32 => {
            for &sample in &audio.data {
                let value = (sample.clamp(-1.0, 1.0) * 2147483647.0) as i32;
                file.write_all(&value.to_le_bytes())?;
            }
        }
        _ => {
            return Err(WavError::UnsupportedFormat(format!(
                "{} bits per sample not supported",
                spec.bits_per_sample
            )));
        }
    }

    Ok(())
}

/// Write WAV from raw samples (convenience function)
pub fn write_wav_samples(
    path: &Path,
    samples: &[f32],
    sample_rate: u32,
    channels: u16,
    bits_per_sample: u16,
) -> Result<(), WavError> {
    let spec = WavSpec {
        sample_rate,
        channels,
        bits_per_sample,
    };
    let audio = WavAudio {
        spec,
        data: samples.to_vec(),
    };
    write_wav(path, &audio)
}

/// Export AudioBuffer to WAV file
pub fn export_audio_buffer(
    path: &Path,
    samples: &[f32],
    sample_rate: u32,
    channels: u16,
) -> Result<(), WavError> {
    write_wav_samples(path, samples, sample_rate, channels, 16)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Seek;

    #[test]
    fn test_wav_spec_default() {
        let spec = WavSpec::default();
        assert_eq!(spec.sample_rate, 44100);
        assert_eq!(spec.channels, 2);
        assert_eq!(spec.bits_per_sample, 16);
    }

    #[test]
    fn test_wav_audio_basic() {
        let spec = WavSpec::default();
        let audio = WavAudio::new(spec);
        assert!(audio.is_empty());
        assert_eq!(audio.len(), 0);
    }

    #[test]
    fn test_wav_audio_stereo_to_mono() {
        let spec = WavSpec {
            sample_rate: 44100,
            channels: 2,
            bits_per_sample: 16,
        };
        let mut audio = WavAudio::new(spec);
        audio.data = vec![1.0, 1.0, 0.5, 0.5, -1.0, -1.0]; // Stereo
        
        let mono = audio.to_mono();
        assert_eq!(mono.len(), 3);
        assert!((mono[0] - 1.0).abs() < 0.001);
        assert!((mono[1] - 0.5).abs() < 0.001);
        assert!((mono[2] - (-1.0)).abs() < 0.001);
    }

    #[test]
    fn test_roundtrip_16bit_stereo() {
        // Create test audio
        let spec = WavSpec {
            sample_rate: 48000,
            channels: 2,
            bits_per_sample: 16,
        };
        
        // Generate sine wave samples
        let frequency = 440.0;
        let duration = 0.1; // 100ms
        let num_samples = (spec.sample_rate as f64 * duration) as usize * spec.channels as usize;
        let mut samples = Vec::with_capacity(num_samples);
        
        for i in 0..num_samples {
            let t = (i / spec.channels as usize) as f64 / spec.sample_rate as f64;
            let sample = (t * frequency * 2.0 * std::f64::consts::PI).sin() as f32;
            samples.push(sample);
        }

        // Write to temp file
        let temp_path = std::env::temp_dir().join("resonance_test.wav");
        write_wav_samples(&temp_path, &samples, spec.sample_rate, spec.channels, spec.bits_per_sample).unwrap();
        
        // Read back
        let read_audio = read_wav(&temp_path).unwrap();
        
        // Verify
        assert_eq!(read_audio.spec.sample_rate, spec.sample_rate);
        assert_eq!(read_audio.spec.channels, spec.channels);
        assert_eq!(read_audio.spec.bits_per_sample, spec.bits_per_sample);
        assert_eq!(read_audio.len(), num_samples / 2);

        // Check samples (allowing small precision loss)
        for (i, (orig, read)) in samples.iter().zip(read_audio.data.iter()).enumerate() {
            let diff = (orig - read).abs();
            assert!(diff < 0.01, "Sample {} differs: {} vs {}", i, orig, read);
        }

        // Cleanup
        std::fs::remove_file(temp_path).ok();
    }
}
