/// MIDI Event Types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MidiEventType {
    NoteOn,
    NoteOff,
    Aftertouch,
    ControlChange,
    ProgramChange,
    PitchBend,
    Sysex,
}

/// MIDI Event
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MidiEvent {
    /// Event type
    pub event_type: MidiEventType,
    /// Channel (0-15)
    pub channel: u8,
    /// Data1 (meaning depends on event type)
    pub data1: u8,
    /// Data2 (meaning depends on event type)
    pub data2: u8,
    /// Time position in ticks
    pub tick: u64,
}

impl MidiEvent {
    /// Create a new MIDI event
    pub fn new(event_type: MidiEventType, channel: u8, data1: u8, data2: u8, tick: u64) -> Self {
        Self {
            event_type,
            channel: channel.min(15),
            data1,
            data2,
            tick,
        }
    }

    /// Create a Note On event
    pub fn note_on(channel: u8, note: u8, velocity: u8, tick: u64) -> Self {
        Self::new(MidiEventType::NoteOn, channel, note, velocity, tick)
    }

    /// Create a Note Off event
    pub fn note_off(channel: u8, note: u8, velocity: u8, tick: u64) -> Self {
        Self::new(MidiEventType::NoteOff, channel, note, velocity, tick)
    }

    /// Create a Program Change event
    pub fn program_change(channel: u8, program: u8, tick: u64) -> Self {
        Self::new(MidiEventType::ProgramChange, channel, program, 0, tick)
    }

    /// Create a Control Change event
    pub fn control_change(channel: u8, control: u8, value: u8, tick: u64) -> Self {
        Self::new(MidiEventType::ControlChange, channel, control, value, tick)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_midi_event_creation() {
        let event = MidiEvent::note_on(0, 60, 100, 0);
        assert_eq!(event.event_type, MidiEventType::NoteOn);
        assert_eq!(event.channel, 0);
        assert_eq!(event.data1, 60);
        assert_eq!(event.data2, 100);
    }
}
