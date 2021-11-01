pub mod generators;
use super::Wave;

pub struct TimedNote {
    pub index: i64,
    pub time: f64,
    pub duration: f64,
    pub amplitude: f64,
    pub decay: f64,
}

impl TimedNote {
    pub fn new(index: i64, time: f64, duration: f64, amplitude: f64, decay: f64) -> TimedNote {
        TimedNote {
            index,
            time,
            duration,
            amplitude,
            decay,
        }
    }
}
type Generator = fn(f64) -> [f64; 2];
pub struct InstrumentComponent {
    contribution: f64,
    generator: Generator,
    decay: f64,
}

fn index_to_frequency(index: i64) -> f64 {
    440.0 * 2f64.powf(index as f64 / 12.0)
}

impl InstrumentComponent {
    pub fn new(contribution: f64, generator: Generator, decay: f64) -> InstrumentComponent {
        InstrumentComponent {
            contribution,
            generator,
            decay,
        }
    }
    fn write(
        &self,
        note: &TimedNote,
        samples: &mut Vec<[f64; 2]>,
        sample_rate: f64,
        amplitude: f64,
    ) {
        let contribution = self.contribution;
        let generator = self.generator;
        let start = (note.time * sample_rate) as usize;
        let end = start + (note.duration * sample_rate) as usize;
        let frequency = index_to_frequency(note.index);
        let period = 1.0 / frequency;
        let cycle = period * sample_rate;
        let mut decay_factor;
        let mut note_decay;
        for si in start..end {
            let cycle_index = (si - start) as f64 % cycle;
            let wi = cycle_index / cycle;
            while samples.len() <= si {
                samples.push([0.0, 0.0]);
            }
            let seconds = (si - start) as f64 / sample_rate;
            decay_factor = 1.0 / (1.0 + self.decay * seconds);
            note_decay = 1.0 / (1.0 + note.decay * seconds);
            let current = samples[si];
            let to_add = generator(wi);
            let sample = [
                current[0]
                    + to_add[0]
                        * contribution
                        * decay_factor
                        * note_decay
                        * amplitude
                        * note.amplitude,
                current[1]
                    + to_add[1]
                        * contribution
                        * decay_factor
                        * note_decay
                        * amplitude
                        * note.amplitude,
            ];
            samples[si] = sample;
        }
    }
}

pub struct Instrument {
    components: Vec<InstrumentComponent>,
}

impl Instrument {
    pub fn new() -> Instrument {
        let components = Vec::with_capacity(16);
        Instrument { components }
    }
    pub fn add_component(&mut self, component: InstrumentComponent) {
        self.components.push(component);
    }
    fn write(
        &self,
        note: &TimedNote,
        samples: &mut Vec<[f64; 2]>,
        sample_rate: f64,
        amplitude: f64,
    ) {
        for component in &self.components {
            component.write(note, samples, sample_rate, amplitude);
        }
    }
}

pub struct InstrumentalLine {
    amplitude: f64,
    instrument: Instrument,
    notes: Vec<TimedNote>,
}

impl InstrumentalLine {
    pub fn new(instrument: Instrument, amplitude: f64) -> InstrumentalLine {
        let notes = Vec::with_capacity(64);
        InstrumentalLine {
            instrument,
            notes,
            amplitude,
        }
    }
    pub fn add_note(&mut self, note: TimedNote) {
        self.notes.push(note);
    }
    fn write(&self, samples: &mut Vec<[f64; 2]>, sample_rate: f64) {
        for note in &self.notes {
            self.instrument
                .write(note, samples, sample_rate, self.amplitude);
        }
    }
}

pub struct Song {
    lines: Vec<InstrumentalLine>,
}

impl Song {
    pub fn new() -> Song {
        let lines = Vec::with_capacity(16);
        Song { lines }
    }
    pub fn add_line(&mut self, line: InstrumentalLine) {
        self.lines.push(line);
    }
    pub fn to_wave_with_samples(&self, sample_rate: u32, wave: &mut Wave) {
        for line in &self.lines {
            line.write(&mut wave.samples, sample_rate as f64);
        }
    }
    pub fn to_wave(&self, sample_rate: u32) -> Wave {
        let mut samples = Vec::new();
        for line in &self.lines {
            line.write(&mut samples, sample_rate as f64);
        }
        Wave { samples }
    }
}

pub struct NoteWriter {
    pub base_duration: f64,
    pub time: f64,
    pub base_note: i64,
    pub duration: f64,
    pub amplitude: f64,
    pub note: i64,
    pub decay: f64,
}

impl NoteWriter {
    pub fn new(time: f64, base_note: i64, base_duration: f64) -> NoteWriter {
        NoteWriter {
            time,
            base_note,
            base_duration,
            duration: base_duration,
            note: 0,
            amplitude: 1.0,
            decay: 0.0,
        }
    }
    pub fn go_back(&mut self) {
        self.time -= self.duration;
        if self.time < 0.0 {
            panic!("time less than zero")
        }
    }
    pub fn advance_note_scale(&mut self, change: i64) {
        let mut in_scale = change;
        let mut octave = 0;
        while in_scale < 0 {
            in_scale += 7;
            octave -= 1;
        }
        while in_scale >= 7 {
            in_scale -= 7;
            octave += 1;
        }
        let mut index = self.note + 12 * octave;
        index += NoteWriter::scale_change(in_scale);
        self.note = index;
    }
    pub fn advance_for(&mut self, factor: f64) {
        self.time += self.duration * self.base_duration * factor;
    }
    pub fn advance(&mut self) {
        self.time += self.duration * self.base_duration;
    }
    pub fn add_to_base_note(&mut self, change: i64) {
        self.base_note += change;
    }
    pub fn set_base_note(&mut self, note: i64) {
        self.base_note = note;
    }
    pub fn set_time(&mut self, time: f64) {
        self.time = time;
    }

    pub fn set_amplitude(&mut self, amplitude: f64) {
        self.amplitude = amplitude;
    }
    pub fn set_note(&mut self, note: i64) {
        self.note = note;
    }
    pub fn set_duration(&mut self, duration: f64) {
        self.duration = duration;
    }
    pub fn scale_change(in_scale: i64) -> i64 {
        match in_scale {
            0 => {
                return 0;
            }
            1 => {
                return 2;
            }
            2 => {
                return 2 + 2;
            }
            3 => {
                return 2 + 2 + 1;
            }
            4 => {
                return 2 + 2 + 1 + 2;
            }
            5 => {
                return 2 + 2 + 1 + 2 + 2;
            }
            6 => {
                return 2 + 2 + 1 + 2 + 2 + 2;
            }
            _ => {
                panic!("out of scale")
            }
        }
    }
    pub fn note_in_minor(&self) -> TimedNote {
        let mut in_chord = self.note;
        let mut octave = 0;
        while in_chord < 0 {
            in_chord += 3;
            octave -= 1;
        }
        while in_chord >= 3 {
            in_chord -= 3;
            octave += 1;
        }
        let mut index = self.base_note + 12 * octave;
        match in_chord {
            0 => {}
            1 => {
                in_chord += 3;
            }
            2 => {
                in_chord += 3 + 4;
            }
            _ => {
                panic!("error in note")
            }
        }
        index += in_chord;
        TimedNote::new(
            index,
            self.time,
            self.base_duration * self.duration,
            self.amplitude,
            self.decay,
        )
    }
    pub fn note_in_major(&self) -> TimedNote {
        let mut in_major = self.note;
        let mut octave = 0;
        while in_major < 0 {
            in_major += 3;
            octave -= 1;
        }
        while in_major >= 3 {
            in_major -= 3;
            octave += 1;
        }
        let mut index = self.base_note + 12 * octave;
        match in_major {
            0 => {}
            1 => {
                in_major += 5;
            }
            2 => {
                in_major += 5 + 4;
            }
            _ => {
                panic!("error in note")
            }
        }
        index += in_major;
        TimedNote::new(
            index,
            self.time,
            self.base_duration * self.duration,
            self.amplitude,
            self.decay,
        )
    }
    pub fn note_in_scale(&self) -> TimedNote {
        let mut in_scale = self.note;
        let mut octave = 0;
        while in_scale < 0 {
            in_scale += 7;
            octave -= 1;
        }
        while in_scale >= 7 {
            in_scale -= 7;
            octave += 1;
        }
        let mut index = self.base_note + 12 * octave;
        index += NoteWriter::scale_change(in_scale);
        TimedNote::new(
            index,
            self.time,
            self.base_duration * self.duration,
            self.amplitude,
            self.decay,
        )
    }
    pub fn note(&self) -> TimedNote {
        let index = self.base_note + self.note;
        TimedNote::new(
            index,
            self.time,
            self.base_duration * self.duration,
            self.amplitude,
            self.decay,
        )
    }
}
