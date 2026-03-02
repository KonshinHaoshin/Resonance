/// MIDI Event Types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MidiEventType {
    NoteOn,
    NoteOff,
    Aftertouch,
    ControlChange,
    ProgramChange,
    PitchBend,
    ChannelPressure,
    Sysex,
    // Extended event types
    MidiClock,
    MidiStart,
    MidiStop,
    MidiContinue,
    ActiveSensing,
    Reset,
}

/// MIDI Control Change Numbers (MIDI CC)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MidiControlChange {
    // Channel Volume Messages
    BankSelect = 0,
    ModWheel = 1,
    BreathControl = 2,
    FootController = 4,
    PortamentoTime = 5,
    DataEntry = 6,
    ChannelVolume = 7,
    Pan = 10,
    Expression = 11,
    
    // Effect Control
    Effect1 = 12,
    Effect2 = 13,
    Effect3 = 14,
    Effect4 = 15,
    
    // General Purpose
    GeneralPurpose1 = 16,
    GeneralPurpose2 = 17,
    GeneralPurpose3 = 18,
    GeneralPurpose4 = 19,
    
    // LFO
    LfoRate = 76,
    LfoDepth = 77,
    LfoDelay = 78,
    
    // Data
    DataEntryMsb = 0x26,
    DataEntryLsb = 0x27,
    NrpnLsb = 98,
    NrpnMsb = 99,
    RpnLsb = 100,
    RpnMsb = 101,
    
    // Mode Messages
    AllSoundOff = 120,
    ResetAllControllers = 121,
    LocalControl = 122,
    AllNotesOff = 123,
    OmniModeOff = 124,
    OmniModeOn = 125,
    MonoModeOn = 126,
    PolyModeOn = 127,
    
    Unknown = -1,
}

impl MidiControlChange {
    /// Get CC number
    pub fn number(&self) -> u8 {
        *self as i8 as u8
    }

    /// Get CC name
    pub fn name(&self) -> &'static str {
        match self {
            MidiControlChange::BankSelect => "Bank Select",
            MidiControlChange::ModWheel => "Modulation Wheel",
            MidiControlChange::BreathControl => "Breath Control",
            MidiControlChange::FootController => "Foot Controller",
            MidiControlChange::PortamentoTime => "Portamento Time",
            MidiControlChange::DataEntry => "Data Entry",
            MidiControlChange::ChannelVolume => "Channel Volume",
            MidiControlChange::Pan => "Pan",
            MidiControlChange::Expression => "Expression",
            MidiControlChange::Effect1 => "Effect 1",
            MidiControlChange::Effect2 => "Effect 2",
            MidiControlChange::Effect3 => "Effect 3",
            MidiControlChange::Effect4 => "Effect 4",
            MidiControlChange::GeneralPurpose1 => "General Purpose 1",
            MidiControlChange::GeneralPurpose2 => "General Purpose 2",
            MidiControlChange::GeneralPurpose3 => "General Purpose 3",
            MidiControlChange::GeneralPurpose4 => "General Purpose 4",
            MidiControlChange::LfoRate => "LFO Rate",
            MidiControlChange::LfoDepth => "LFO Depth",
            MidiControlChange::LfoDelay => "LFO Delay",
            MidiControlChange::DataEntryMsb => "Data Entry MSB",
            MidiControlChange::DataEntryLsb => "Data Entry LSB",
            MidiControlChange::NrpnLsb => "NRPN LSB",
            MidiControlChange::NrpnMsb => "NRPN MSB",
            MidiControlChange::RpnLsb => "RPN LSB",
            MidiControlChange::RpnMsb => "RPN MSB",
            MidiControlChange::AllSoundOff => "All Sound Off",
            MidiControlChange::ResetAllControllers => "Reset All Controllers",
            MidiControlChange::LocalControl => "Local Control",
            MidiControlChange::AllNotesOff => "All Notes Off",
            MidiControlChange::OmniModeOff => "Omni Mode Off",
            MidiControlChange::OmniModeOn => "Omni Mode On",
            MidiControlChange::MonoModeOn => "Mono Mode On",
            MidiControlChange::PolyModeOn => "Poly Mode On",
            MidiControlChange::Unknown => "Unknown",
        }
    }

    /// Parse from u8
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => MidiControlChange::BankSelect,
            1 => MidiControlChange::ModWheel,
            2 => MidiControlChange::BreathControl,
            4 => MidiControlChange::FootController,
            5 => MidiControlChange::PortamentoTime,
            6 => MidiControlChange::DataEntry,
            7 => MidiControlChange::ChannelVolume,
            10 => MidiControlChange::Pan,
            11 => MidiControlChange::Expression,
            12 => MidiControlChange::Effect1,
            13 => MidiControlChange::Effect2,
            14 => MidiControlChange::Effect3,
            15 => MidiControlChange::Effect4,
            16 => MidiControlChange::GeneralPurpose1,
            17 => MidiControlChange::GeneralPurpose2,
            18 => MidiControlChange::GeneralPurpose3,
            19 => MidiControlChange::GeneralPurpose4,
            38 => MidiControlChange::DataEntryLsb,
            76 => MidiControlChange::LfoRate,
            77 => MidiControlChange::LfoDepth,
            78 => MidiControlChange::LfoDelay,
            98 => MidiControlChange::NrpnLsb,
            99 => MidiControlChange::NrpnMsb,
            100 => MidiControlChange::RpnLsb,
            101 => MidiControlChange::RpnMsb,
            120 => MidiControlChange::AllSoundOff,
            121 => MidiControlChange::ResetAllControllers,
            122 => MidiControlChange::LocalControl,
            123 => MidiControlChange::AllNotesOff,
            124 => MidiControlChange::OmniModeOff,
            125 => MidiControlChange::OmniModeOn,
            126 => MidiControlChange::MonoModeOn,
            127 => MidiControlChange::PolyModeOn,
            _ => MidiControlChange::Unknown,
        }
    }
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

    /// Create a Pitch Bend event
    /// value: 0-16384 (8192 = center/bend none)
    pub fn pitch_bend(channel: u8, value: u16, tick: u64) -> Self {
        let lsb = (value & 0x7F) as u8;
        let msb = ((value >> 7) & 0x7F) as u8;
        Self::new(MidiEventType::PitchBend, channel, lsb, msb, tick)
    }

    /// Create a Pitch Bend event from semitones (-64 to +63)
    pub fn pitch_bend_from_semitones(channel: u8, semitones: i8, tick: u64) -> Self {
        let value = ((semitones as i16 + 64) * 128) as u16;
        Self::pitch_bend(channel, value, tick)
    }

    /// Create an Aftertouch event (per-note pressure)
    pub fn aftertouch(channel: u8, note: u8, pressure: u8, tick: u64) -> Self {
        Self::new(MidiEventType::Aftertouch, channel, note, pressure, tick)
    }

    /// Create a Channel Pressure event (aftertouch)
    pub fn channel_pressure(channel: u8, pressure: u8, tick: u64) -> Self {
        Self::new(MidiEventType::ChannelPressure, channel, pressure, 0, tick)
    }

    /// Create a Modulation event (CC 1)
    pub fn modulation(channel: u8, value: u8, tick: u64) -> Self {
        Self::control_change(channel, 1, value, tick)
    }

    /// Create a Volume event (CC 7)
    pub fn volume(channel: u8, value: u8, tick: u64) -> Self {
        Self::control_change(channel, 7, value, tick)
    }

    /// Create a Pan event (CC 10)
    pub fn pan(channel: u8, value: u8, tick: u64) -> Self {
        Self::control_change(channel, 10, value, tick)
    }

    /// Create an Expression event (CC 11)
    pub fn expression(channel: u8, value: u8, tick: u64) -> Self {
        Self::control_change(channel, 11, value, tick)
    }

    /// Create a Portamento event (CC 65)
    pub fn portamento(channel: u8, enabled: bool, tick: u64) -> Self {
        Self::control_change(channel, 65, if enabled { 127 } else { 0 }, tick)
    }

    /// Create a Sustain event (CC 64)
    pub fn sustain(channel: u8, value: u8, tick: u64) -> Self {
        Self::control_change(channel, 64, value.min(127), tick)
    }

    /// Create a Legato event (CC 68)
    pub fn legato(channel: u8, enabled: bool, tick: u64) -> Self {
        Self::control_change(channel, 68, if enabled { 127 } else { 0 }, tick)
    }

    /// Create a All Notes Off event (CC 123)
    pub fn all_notes_off(channel: u8, tick: u64) -> Self {
        Self::control_change(channel, 123, 0, tick)
    }

    /// Get pitch bend value (0-16384)
    pub fn pitch_bend_value(&self) -> u16 {
        if self.event_type == MidiEventType::PitchBend {
            ((self.data2 as u16) << 7) | (self.data1 as u16)
        } else {
            8192
        }
    }

    /// Get pitch bend in semitones (-64 to +63)
    pub fn pitch_bend_semitones(&self) -> i8 {
        let value = self.pitch_bend_value();
        ((value as i32) - 8192) as i8 / 64
    }

    /// Get control change type
    pub fn control_type(&self) -> MidiControlChange {
        if self.event_type == MidiEventType::ControlChange {
            MidiControlChange::from_u8(self.data1)
        } else {
            MidiControlChange::Unknown
        }
    }

    /// Convert to MIDI bytes (status + data1 + data2)
    pub fn to_midi_bytes(&self) -> [u8; 3] {
        let status = match self.event_type {
            MidiEventType::NoteOff => 0x80,
            MidiEventType::NoteOn => 0x90,
            MidiEventType::Aftertouch => 0xA0,
            MidiEventType::ControlChange => 0xB0,
            MidiEventType::ProgramChange => 0xC0,
            MidiEventType::ChannelPressure => 0xD0,
            MidiEventType::PitchBend => 0xE0,
            _ => 0,
        };
        
        let channel = self.channel & 0x0F;
        
        if matches!(self.event_type, MidiEventType::ProgramChange | MidiEventType::ChannelPressure) {
            [status | channel, self.data1, 0]
        } else {
            [status | channel, self.data1, self.data2]
        }
    }
}

/// Convert note number to frequency
pub fn note_to_frequency(note: u8) -> f64 {
    440.0 * 2.0_f64.powf((note as f64 - 69.0) / 12.0)
}

/// Convert frequency to note number
pub fn frequency_to_note(frequency: f64) -> u8 {
    (69.0 + 12.0 * (frequency / 440.0).log2()).round() as u8
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

    #[test]
    fn test_pitch_bend() {
        let event = MidiEvent::pitch_bend(0, 8192, 0); // center
        assert_eq!(event.pitch_bend_value(), 8192);
        assert_eq!(event.pitch_bend_semitones(), 0);

        let event2 = MidiEvent::pitch_bend(0, 16384, 0); // max bend up
        assert_eq!(event2.pitch_bend_semitones(), 63);

        let event3 = MidiEvent::pitch_bend(0, 0, 0); // max bend down
        assert_eq!(event3.pitch_bend_semitones(), -64);
    }

    #[test]
    fn test_control_change() {
        let event = MidiEvent::control_change(0, 7, 100, 0); // Volume
        assert_eq!(event.control_type(), MidiControlChange::ChannelVolume);
        assert_eq!(event.data1, 7);
        assert_eq!(event.data2, 100);
    }

    #[test]
    fn test_note_to_frequency() {
        assert!((note_to_frequency(69) - 440.0).abs() < 0.01); // A4
        assert!((note_to_frequency(60) - 261.63).abs() < 0.01); // C4
    }

    #[test]
    fn test_frequency_to_note() {
        assert_eq!(frequency_to_note(440.0), 69);
        assert_eq!(frequency_to_note(261.63), 60);
    }

    #[test]
    fn test_midi_event_to_bytes() {
        let event = MidiEvent::note_on(0, 60, 100, 0);
        let bytes = event.to_midi_bytes();
        assert_eq!(bytes, [0x90, 60, 100]);
    }

    #[test]
    fn test_control_change_names() {
        assert_eq!(MidiControlChange::ModWheel.name(), "Modulation Wheel");
        assert_eq!(MidiControlChange::from_u8(1), MidiControlChange::ModWheel);
    }
}
