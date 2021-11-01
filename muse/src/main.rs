use std::path::Path;

use uninutsh::audio::music::generators;
use uninutsh::audio::music::Instrument;
use uninutsh::audio::music::InstrumentComponent;
use uninutsh::audio::music::InstrumentalLine;
use uninutsh::audio::music::NoteWriter;
use uninutsh::audio::music::Song;

const SAMPLE_RATE: u32 = 48000;

fn main() {
    println!("Hello, world!");
    let mut song = Song::new();
    let comp0x0 = InstrumentComponent::new(1.0, generators::sine, 0.0);
    let comp0x1 = InstrumentComponent::new(1.0, generators::algebraic, 0.0);
    let comp0x2 = InstrumentComponent::new(1.0, generators::saw, 0.0);

    let mut instrument0x0 = Instrument::new();
    instrument0x0.add_component(comp0x0);
    instrument0x0.add_component(comp0x1);
    instrument0x0.add_component(comp0x2);

    let mut line0x0 = InstrumentalLine::new(instrument0x0, 1.0);
    let mut writer = NoteWriter::new(0.0, -2 * 12, 1.0);
    writer.set_duration(1.0);
    writer.set_note(0);
    line0x0.add_note(writer.note());

    song.add_line(line0x0);
    let mut wave = song.to_wave(SAMPLE_RATE);
    //wave.add_echo_0x1(SAMPLE_RATE, 1.0 / 2.0, 4, 1.0 / 4.0);
    //wave.add_echo_0x0(SAMPLE_RATE, 1.0 / 3.0, 4, 1.0 / 4.0);
    wave.normalize();
    wave.save(Path::new("song.wav"), SAMPLE_RATE);
}
