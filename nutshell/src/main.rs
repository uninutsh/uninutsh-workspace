use cpal::traits::DeviceTrait;
use cpal::traits::HostTrait;
use cpal::traits::StreamTrait;
use cpal::Sample;
use cpal::SampleRate;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;
use std::time::Instant;
use uninutsh::audio::music::generators;
use uninutsh::audio::music::Instrument;
use uninutsh::audio::music::InstrumentComponent;
use uninutsh::audio::music::InstrumentalLine;
use uninutsh::audio::music::NoteWriter;
use uninutsh::audio::music::Song;
use uninutsh::audio::Wave;
use uninutsh::{
    image::Color,
    window::{EventHandler, Window, WindowEvent, WindowOptions},
    Vector2,
};
#[derive(Copy, Clone)]
struct Cell {
    //pub neighbors: Vec<Vector2<u32>>,
    color: u64,
    saturation: u64,
    brightness: u64,
}
struct Nutshell {
    cells: Vec<Vec<Cell>>,
    layers: usize,
    definition: u64,
    size: Vector2<u32>,
    fashion: [Vec<u64>; 3],
}
const VIDEO_SAMPLES_PER_SECOND: usize = 2;
const VIDEO_SAMPLES_LENGHT: usize = VIDEO_SAMPLES_PER_SECOND * SECONDS_PER_FRAME;
const SECONDS_PER_FRAME: usize = 16;
const AUDIO_SAMPLES_LENGHT: usize = SAMPLE_RATE as usize * SECONDS_PER_FRAME;
const VIDEO_SAMPLE_WIDTH: usize = 60;
const VIDEO_SAMPLE_HEIGHT: usize = 60;
const LAYERS: usize = 4;
const SAMPLE_RATE: u32 = 48000;
const DEFINITION: u64 = 12;
const CHANGE: u64 = 1;
impl Nutshell {
    fn new(size: Vector2<u32>, layers: usize, definition: u64) -> Nutshell {
        let mut cells = Vec::with_capacity(layers);
        for _layer in 0..layers {
            let mut layer = Vec::with_capacity(size.x as usize * size.y as usize);
            for _y in 0..size.y {
                for _x in 0..size.x {
                    let cell = Cell {
                        color: 0,
                        saturation: 0,
                        brightness: 0,
                    };
                    layer.push(cell);
                }
            }
            cells.push(layer);
        }
        let mut fashion = [
            Vec::with_capacity(definition as usize),
            Vec::with_capacity(definition as usize),
            Vec::with_capacity(definition as usize),
        ];
        for _i in 0..definition {
            fashion[0].push(0);
            fashion[1].push(0);
            fashion[2].push(0);
        }
        let mut nutshell = Nutshell {
            fashion,
            layers,
            size,
            cells,
            definition,
        };
        nutshell.cells[1][0] = Cell {
            color: 1,
            saturation: 1,
            brightness: 1,
        };

        nutshell
    }
    fn index_at(&self, x: u32, y: u32) -> usize {
        y as usize * self.size.x as usize + x as usize
    }
    pub fn left(&self, i: u32) -> u32 {
        match i {
            0 => self.size.x - 1,
            _ => i - 1,
        }
    }
    pub fn up(&self, i: u32) -> u32 {
        match i {
            0 => self.size.y - 1,
            _ => i - 1,
        }
    }
    pub fn right(&self, i: u32) -> u32 {
        if i == self.size.x - 1 {
            return 0;
        }
        i + 1
    }
    pub fn down(&self, i: u32) -> u32 {
        if i == self.size.y - 1 {
            return 0;
        }
        i + 1
    }
    pub fn rigth_pos(&self, x: u32, y: u32) -> Vector2<u32> {
        Vector2::new(self.right(x), y)
    }
    pub fn left_pos(&self, x: u32, y: u32) -> Vector2<u32> {
        Vector2::new(self.left(x), y)
    }
    pub fn down_pos(&self, x: u32, y: u32) -> Vector2<u32> {
        Vector2::new(x, self.down(y))
    }
    pub fn up_pos(&self, x: u32, y: u32) -> Vector2<u32> {
        Vector2::new(x, self.up(y))
    }
    fn neighborhood(&self, x: u32, y: u32, radius: u32) -> Vec<Vector2<u32>> {
        let side_length = radius * 2 + 1;
        let center = Vector2::new(x, y);
        let mut position = center;
        for _i in 0..radius {
            position = self.left_pos(position.x, position.y);
        }
        for _i in 0..radius {
            position = self.up_pos(position.x, position.y);
        }
        let mut row_pos = position;
        let mut neighborhood = Vec::with_capacity(side_length as usize * side_length as usize);
        for _y in 0..side_length {
            position = row_pos;
            for _x in 0..side_length {
                neighborhood.push(position);
                position = self.rigth_pos(position.x, position.y);
            }
            row_pos = self.down_pos(row_pos.x, row_pos.y);
        }
        neighborhood
    }
    fn iterate(&mut self) {
        // layer 0
        let prev_layer = 1;
        let layer = 0;
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let index = self.index_at(x, y);
                self.cells[layer][index] = self.cells[prev_layer][index];
            }
        }
        // layer 1
        let prev_layer = 0;
        let layer = 1;
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let neighborhood = self.neighborhood(x, y, 1);
                let mut cell = Cell {
                    color: 0,
                    saturation: 0,
                    brightness: 0,
                };
                for neighbor in &neighborhood {
                    let neighbor_index = self.index_at(neighbor.x, neighbor.y);
                    let neighbor_cell = self.cells[prev_layer][neighbor_index];
                    cell.color += neighbor_cell.color;
                    cell.color %= self.definition;
                    cell.brightness += neighbor_cell.brightness;
                    cell.brightness %= self.definition;
                    cell.saturation += neighbor_cell.saturation;
                    cell.saturation %= self.definition;
                }
                let index = self.index_at(x, y);
                self.cells[layer][index] = cell;
            }
        }
        // layer 2
        let prev_layer = 1;
        let layer = 2;
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let index = self.index_at(x, y);
                let prev_cell = self.cells[prev_layer][index];
                let mut cell = self.cells[layer][index];
                if prev_cell.color == 0 {
                    cell.color += CHANGE;
                    cell.color %= self.definition;
                }
                if prev_cell.saturation == 0 {
                    cell.saturation += CHANGE;
                    cell.saturation %= self.definition;
                }
                if prev_cell.brightness == 0 {
                    cell.brightness += CHANGE;
                    cell.brightness %= self.definition;
                }
                self.cells[layer][index] = cell;
            }
        }
        // layer 3
        let prev_layer = 2;
        let layer = 3;
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let neighborhood = self.neighborhood(x, y, 4);
                let mut cell = Cell {
                    color: 0,
                    saturation: 0,
                    brightness: 0,
                };
                for i in 0..self.definition {
                    self.fashion[0][i as usize] = 0;
                    self.fashion[1][i as usize] = 0;
                    self.fashion[2][i as usize] = 0;
                }
                for neighbor in &neighborhood {
                    let neighbor_index = self.index_at(neighbor.x, neighbor.y);
                    let neighbor_cell = self.cells[prev_layer][neighbor_index];
                    self.fashion[0][neighbor_cell.color as usize] += 1;
                    self.fashion[1][neighbor_cell.saturation as usize] += 1;
                    self.fashion[2][neighbor_cell.brightness as usize] += 1;
                }
                for i in 0..self.definition {
                    if self.fashion[0][i as usize] > self.fashion[0][cell.color as usize] {
                        cell.color = i;
                    }
                }
                for i in 0..self.definition {
                    if self.fashion[1][i as usize] > self.fashion[1][cell.saturation as usize] {
                        cell.saturation = i;
                    }
                }
                for i in 0..self.definition {
                    if self.fashion[2][i as usize] > self.fashion[2][cell.brightness as usize] {
                        cell.brightness = i;
                    }
                }
                let index = self.index_at(x, y);
                self.cells[layer][index] = cell;
            }
        }
        /*
        // layer 4
        let prev_layer = 3;
        let layer = 4;
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let neighborhood = self.neighborhood(x, y, 1);
                let mut cell = Cell {
                    color: 0,
                    saturation: 0,
                    brightness: 0,
                };
                for i in 0..self.definition {
                    self.fashion[0][i as usize] = 0;
                    self.fashion[1][i as usize] = 0;
                    self.fashion[2][i as usize] = 0;
                }
                for neighbor in &neighborhood {
                    let neighbor_index = self.index_at(neighbor.x, neighbor.y);
                    let neighbor_cell = self.cells[prev_layer][neighbor_index];
                    self.fashion[0][neighbor_cell.color as usize] += 1;
                    self.fashion[1][neighbor_cell.saturation as usize] += 1;
                    self.fashion[2][neighbor_cell.brightness as usize] += 1;
                }
                for i in 0..self.definition {
                    if self.fashion[0][i as usize] > self.fashion[0][cell.color as usize] {
                        cell.color = i;
                    }
                }
                for i in 0..self.definition {
                    if self.fashion[1][i as usize] > self.fashion[1][cell.saturation as usize] {
                        cell.saturation = i;
                    }
                }
                for i in 0..self.definition {
                    if self.fashion[2][i as usize] > self.fashion[2][cell.brightness as usize] {
                        cell.brightness = i;
                    }
                }
                let index = self.index_at(x, y);
                self.cells[layer][index] = cell;
            }
        }
        // layer 5
        let prev_layer = 4;
        let layer = 5;
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let neighborhood = self.neighborhood(x, y, 1);
                let mut cell = Cell {
                    color: 0,
                    saturation: 0,
                    brightness: 0,
                };
                for i in 0..self.definition {
                    self.fashion[0][i as usize] = 0;
                    self.fashion[1][i as usize] = 0;
                    self.fashion[2][i as usize] = 0;
                }
                for neighbor in &neighborhood {
                    let neighbor_index = self.index_at(neighbor.x, neighbor.y);
                    let neighbor_cell = self.cells[prev_layer][neighbor_index];
                    self.fashion[0][neighbor_cell.color as usize] += 1;
                    self.fashion[1][neighbor_cell.saturation as usize] += 1;
                    self.fashion[2][neighbor_cell.brightness as usize] += 1;
                }
                for i in 0..self.definition {
                    if self.fashion[0][i as usize] > self.fashion[0][cell.color as usize] {
                        cell.color = i;
                    }
                }
                for i in 0..self.definition {
                    if self.fashion[1][i as usize] > self.fashion[1][cell.saturation as usize] {
                        cell.saturation = i;
                    }
                }
                for i in 0..self.definition {
                    if self.fashion[2][i as usize] > self.fashion[2][cell.brightness as usize] {
                        cell.brightness = i;
                    }
                }
                let index = self.index_at(x, y);
                self.cells[layer][index] = cell;
            }
        }
        */
    }
}

struct AudioFrame {
    samples: Vec<f32>,
    //samples: [f32; AUDIO_SAMPLES_LENGHT],
}

#[derive(Clone)]
struct VideoSample {
    pixels: Vec<Color>,
    //pixels: [[Color; VIDEO_SAMPLE_WIDTH]; VIDEO_SAMPLE_HEIGHT],
}

struct VideoFrame {
    samples: Vec<VideoSample>,
    //samples: [VideoSample; VIDEO_SAMPLES_LENGHT],
}
struct Frame {
    audio: Option<AudioFrame>,
    video: Option<VideoFrame>,
}

struct ProcessingThread {
    nutshell: Nutshell,
    primary: Option<Frame>,
    pointer: Vector2<u32>,
    process_receiver: Receiver<Message>,
    process_sender: Sender<Message>,
}

impl ProcessingThread {
    fn video_sample(&mut self) -> VideoSample {
        let mut pixels = Vec::with_capacity(VIDEO_SAMPLE_HEIGHT * VIDEO_SAMPLE_HEIGHT);
        for y in 0..VIDEO_SAMPLE_HEIGHT {
            for x in 0..VIDEO_SAMPLE_WIDTH {
                let index = self.nutshell.index_at(x as u32, y as u32);
                let cell = self.nutshell.cells[self.nutshell.layers - 1][index];
                let hue = cell.color as f64 / self.nutshell.definition as f64;
                let saturation = cell.saturation as f64 / (self.nutshell.definition - 1) as f64;
                let brightness = cell.brightness as f64 / (self.nutshell.definition - 1) as f64;
                let color =
                    Color::from_hsb([hue * 360.0 + 180.0, saturation, 1.0 - brightness], 255);
                pixels.push(color);
            }
        }
        VideoSample { pixels }
    }
    fn color(&mut self) -> u64 {
        let index = self.nutshell.index_at(self.pointer.x, self.pointer.y);
        let mut color = self.nutshell.cells[LAYERS - 1][index].color
            + self.nutshell.cells[LAYERS - 1][index].saturation
            + self.nutshell.cells[LAYERS - 1][index].brightness;
        color /= 3;
        match color % 3 {
            0 => {
                self.pointer = self.nutshell.rigth_pos(self.pointer.x, self.pointer.y);
            }
            1 => {
                self.pointer = self.nutshell.down_pos(self.pointer.x, self.pointer.y);
            }
            2 => {
                self.pointer = self.nutshell.rigth_pos(self.pointer.x, self.pointer.y);
                self.pointer = self.nutshell.down_pos(self.pointer.x, self.pointer.y);
            }
            _ => {}
        }
        color
    }
    fn process(&mut self) -> Frame {
        let mut samples = Vec::with_capacity(AUDIO_SAMPLES_LENGHT * 2);
        for _i in 0..AUDIO_SAMPLES_LENGHT * 2 {
            samples.push(0.0);
        }
        let audio = Some(AudioFrame { samples });
        let video = Some(VideoFrame {
            samples: Vec::with_capacity(VIDEO_SAMPLES_LENGHT),
        });
        let mut frame = Frame { audio, video };

        let mut song = Song::new();
        let comp0x0 = InstrumentComponent::new(1.0, generators::sine, 0.0);
        let comp0x1 = InstrumentComponent::new(1.0, generators::algebraic, 0.0);
        //let comp0x2 = InstrumentComponent::new(1.0, generators::saw, 0.0);
        //let comp0x3 = InstrumentComponent::new(1.0, generators::sigmoid, 0.0);
        //let comp0x4 = InstrumentComponent::new(1.0, generators::square, 0.0);

        let mut instrument0x0 = Instrument::new();
        let base_duration = 1.0 / 6.0;
        instrument0x0.add_component(comp0x0);
        instrument0x0.add_component(comp0x1);
        //instrument0x0.add_component(comp0x2);
        //instrument0x0.add_component(comp0x4);
        let mut line0x0 = InstrumentalLine::new(instrument0x0, 1.0);
        let times = (SECONDS_PER_FRAME as f64 / base_duration) as u64;
        let writers = 7;
        //let notes_count = SECONDS_PER_FRAME as i64;
        //let notes_per_writer = notes_count / writers / VIDEO_SAMPLES_LENGHT as i64;
        let notes_per_writer = 2;
        let contrast = 1.0 / 1.0;
        let in_scale = self.color() % 7;
        let decay = 1.0 / 1.0;

        let adder;
        match in_scale {
            0 => {
                adder = 0;
            }
            1 => {
                adder = 2;
            }
            2 => {
                adder = 2 + 2;
            }
            3 => {
                adder = 2 + 2 + 1;
            }
            4 => {
                adder = 2 + 2 + 1 + 2;
            }
            5 => {
                adder = 2 + 2 + 1 + 2 + 2;
            }
            6 => {
                adder = 2 + 2 + 1 + 2 + 2 + 2;
            }
            _ => {
                panic!("out of scale")
            }
        }
        for i in 0..VIDEO_SAMPLES_LENGHT {
            println!("sample {}", i);
            for w in 0..writers {
                let base_note = (w - 6) * 12 + adder;
                let mut writer = NoteWriter::new(0.0, base_note, base_duration);
                for _i in 0..notes_per_writer {
                    //writer.set_amplitude(1.0 / ((w as f64 + offset) * contrast + 1.0));
                    writer.set_duration(((self.color() % 3) + 1) as f64);
                    writer.set_amplitude(1.0 / ((self.color() % 4) + 1) as f64);

                    //let w_index = writers - w - 1;
                    let w_index = w;

                    writer.decay = decay / base_duration * (w_index as f64 * contrast + 1.0);
                    let mut time_g = 0;

                    //DEF * JS = times
                    let js = (times / DEFINITION + 1) * 2;
                    for _j in 0..js {
                        time_g += self.color();
                        time_g %= times;
                    }
                    let time = base_duration * time_g as f64;
                    if time >= SECONDS_PER_FRAME as f64 {
                        panic!("timeeee")
                    }
                    writer.set_time(time);
                    match self.color() % 8 {
                        0 => {
                            writer.set_note(self.color() as i64 % 7);
                            let note = writer.note_in_scale();
                            //println!("note {}", note.time);
                            line0x0.add_note(note);
                        }
                        _ => {
                            writer.set_note(self.color() as i64 % 3);

                            let note = writer.note_in_minor();
                            //println!("note {}", note.time);
                            line0x0.add_note(note);
                        }
                    }
                }
            }

            frame
                .video
                .as_mut()
                .unwrap()
                .samples
                .push(self.video_sample());

            self.nutshell.iterate();
        }
        song.add_line(line0x0);
        println!("to wave");
        let samples = Vec::with_capacity(AUDIO_SAMPLES_LENGHT);
        let mut wave = Wave { samples };
        song.to_wave_with_samples(SAMPLE_RATE, &mut wave);

        //println!("echo 0");
        //wave.add_echo_0x0(SAMPLE_RATE, 1.0 / 3.0, 4, 1.0 / 3.0);

        //println!("echo 1");
        //wave.add_echo_0x1(SAMPLE_RATE, 1.0 / 3.0, 4, 1.0 / 4.0);

        println!("normalize");
        wave.normalize();
        for i in 0..AUDIO_SAMPLES_LENGHT {
            if i < wave.samples.len() {
                frame.audio.as_mut().unwrap().samples[i * 2] = wave.samples[i][0] as f32;
                frame.audio.as_mut().unwrap().samples[i * 2 + 1] = wave.samples[i][1] as f32;
            }
        }
        println!("frame calculated");
        frame
    }
}

struct WindowThread {
    drawing_frame: Option<VideoSample>,
    frame: Option<VideoFrame>,
    need_frame: bool,
    window_receiver: Receiver<Message>,
    sample_index: usize,
    update_duration: Duration,
    last_update_instant: Instant,
}

impl WindowThread {
    fn try_draw(&mut self, window: &mut Window) {
        if !self.need_frame && Instant::now() >= self.last_update_instant + self.update_duration {
            self.draw(window);
        }
    }
    fn draw(&mut self, window: &mut Window) {
        let frame = self.frame.as_ref().unwrap().samples[self.sample_index].clone();
        self.drawing_frame = Some(frame);
        self.sample_index += 1;
        if self.sample_index == VIDEO_SAMPLES_LENGHT {
            self.need_frame = true;
            self.sample_index = 0;
        }
        window.redraw();
        self.last_update_instant = Instant::now();
    }
}

impl EventHandler for WindowThread {
    fn handle_event(&mut self, event: WindowEvent, window: &mut Window) {
        match event {
            WindowEvent::Exit => {
                window.close();
            }
            WindowEvent::Draw => match &self.drawing_frame {
                Some(sample) => {
                    let mut graphics = window.graphics().unwrap();
                    for y in 0..VIDEO_SAMPLE_HEIGHT {
                        for x in 0..VIDEO_SAMPLE_WIDTH {
                            let color = sample.pixels[y * VIDEO_SAMPLE_WIDTH + x];
                            graphics.set_color(color);
                            graphics.put(x as u32, y as u32);
                        }
                    }
                    graphics.apply();
                    window.return_graphics(Some(graphics));
                }
                None => {}
            },
            WindowEvent::Update(_delta) => {
                if self.need_frame {
                    let message = self.window_receiver.try_recv();
                    match message {
                        Ok(message) => match message {
                            Message::VideoFrameSended(frame) => {
                                self.frame = Some(frame);
                                self.need_frame = false;
                                self.draw(window);
                            }
                            _ => {}
                        },
                        Err(_) => {}
                    }
                } else {
                    self.try_draw(window);
                }
            }
        }
    }
}

enum Message {
    NeedFrame,
    FrameSended(Frame),
    VideoFrameSended(VideoFrame),
}

struct AudioThread {
    audio_receiver: Receiver<Message>,
    audio_sender: Sender<Message>,
    window_sender: Sender<Message>,
    frame: Option<Frame>,
    need_frame: bool,
    sample_index: usize,
    reverb_length: usize,
    reverb: Vec<Option<Vec<f32>>>,
    back_reverb: Vec<Option<Vec<f32>>>,
    reverb_echoes: usize,
    reverb_indexes: Vec<usize>,
}

fn main() {
    println!("Hello, world!");
    let (process_sender, audio_receiver) = mpsc::channel();

    let (audio_sender, process_receiver) = mpsc::channel();
    let (window_sender, window_receiver) = mpsc::channel();
    let reverb_length = SAMPLE_RATE as usize * 4;
    let reverb_echoes = 6;
    let mut reverb = Vec::with_capacity(reverb_echoes);
    let mut back_reverb = Vec::with_capacity(reverb_echoes);
    let mut rev_len = reverb_length;

    for _i in 0..reverb_echoes {
        //reverb.as_mut().unwrap().push(0.0);
        //back_reverb.as_mut().unwrap().push(0.0);
        let mut revs = Vec::with_capacity(rev_len);
        let mut back_revs = Vec::with_capacity(rev_len);
        for _j in 0..rev_len {
            revs.push(0.0);
            back_revs.push(0.0);
        }
        rev_len /= 2;
        reverb.push(Some(revs));
        back_reverb.push(Some(back_revs));
    }
    let mut reverb_indexes = Vec::with_capacity(reverb_echoes);
    for _i in 0..reverb_echoes {
        reverb_indexes.push(0);
    }
    let mut audio_thread = AudioThread {
        need_frame: true,
        sample_index: 0,
        frame: None,
        audio_sender,
        audio_receiver,
        window_sender,
        reverb,
        back_reverb,
        reverb_length,
        reverb_echoes,
        reverb_indexes,
    };

    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no output device available");
    let mut supported_configs_range = device
        .supported_output_configs()
        .expect("error while querying configs");
    let supported_config = supported_configs_range
        .next()
        .expect("no supported config?!")
        .with_sample_rate(SampleRate(SAMPLE_RATE))
        .config();
    let mut echo_amps = Vec::with_capacity(reverb_echoes);
    let first_echo = 1.0 / 4.0;
    let mut echo_amp = first_echo;
    for _i in 0..reverb_echoes {
        echo_amps.push(echo_amp);
        echo_amp *= 1.0 / 2.0;
    }
    let wet = 1.0 / 2.0;
    let stream = device
        .build_output_stream(
            &supported_config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                // react to stream events and read or write stream data here.
                //sb.send(Message::NeedData).unwrap();

                for sample in data.iter_mut() {
                    if audio_thread.sample_index == 0 {
                        audio_thread.audio_sender.send(Message::NeedFrame).unwrap();
                        audio_thread.need_frame = true;
                        while audio_thread.need_frame {
                            let message = audio_thread.audio_receiver.recv().unwrap();
                            println!("Frame consumed");
                            match message {
                                Message::FrameSended(frame) => {
                                    audio_thread.frame = Some(frame);
                                    audio_thread.need_frame = false;
                                    let video_frame =
                                        audio_thread.frame.as_mut().unwrap().video.take().unwrap();
                                    audio_thread
                                        .window_sender
                                        .send(Message::VideoFrameSended(video_frame))
                                        .unwrap();
                                }
                                _ => {}
                            }
                        }
                    }
                    let audio_sample = audio_thread
                        .frame
                        .as_ref()
                        .unwrap()
                        .audio
                        .as_ref()
                        .unwrap()
                        .samples[audio_thread.sample_index];
                    let mut wet_sample = 0.0;
                    for echo_index in 0..audio_thread.reverb_echoes {
                        let reverb_sample = audio_thread.reverb[echo_index].as_ref().unwrap()
                            [audio_thread.reverb_indexes[echo_index]];
                        wet_sample += reverb_sample;
                    }
                    wet_sample *= wet;
                    let final_sample = wet_sample + audio_sample * (1.0 - wet_sample);
                    //audio_sample *= 16.0;
                    *sample = Sample::from(&final_sample);
                    for echo_index in 0..audio_thread.reverb_echoes {
                        let echo_amp = echo_amps[echo_index];
                        audio_thread.back_reverb[echo_index].as_mut().unwrap()
                            [audio_thread.reverb_indexes[echo_index]] = final_sample * echo_amp;
                    }
                    let mut rev_len = audio_thread.reverb_length;
                    for echo_index in 0..audio_thread.reverb_echoes {
                        audio_thread.reverb_indexes[echo_index] += 1;
                        if audio_thread.reverb_indexes[echo_index] >= rev_len {
                            audio_thread.reverb_indexes[echo_index] = 0;
                            let swap = audio_thread.reverb[echo_index].take();
                            audio_thread.reverb[echo_index] =
                                audio_thread.back_reverb[echo_index].take();
                            audio_thread.back_reverb[echo_index] = swap;
                        }
                        rev_len /= 2;
                    }

                    audio_thread.sample_index += 1;
                    if audio_thread.sample_index >= AUDIO_SAMPLES_LENGHT * 2 {
                        audio_thread.sample_index = 0;
                    }
                }
            },
            move |_err| {
                // react to errors here.
            },
        )
        .unwrap();
    stream.play().unwrap();

    let mut processing_thread = ProcessingThread {
        nutshell: Nutshell::new(
            Vector2::new(VIDEO_SAMPLE_WIDTH as u32, VIDEO_SAMPLE_HEIGHT as u32),
            LAYERS,
            DEFINITION,
        ),
        primary: None,
        process_receiver,
        process_sender,
        pointer: Vector2::new(0, 0),
    };
    println!("pre-processing");
    for i in 0..0 {
        println!("pre {}", i);
        processing_thread.nutshell.iterate();
    }
    println!("before processing");
    processing_thread.primary = Some(processing_thread.process());
    println!("after processing primary");

    let window_thread = WindowThread {
        frame: None,
        drawing_frame: None,
        window_receiver,
        need_frame: true,
        sample_index: 0,
        update_duration: Duration::from_secs_f64(
            SECONDS_PER_FRAME as f64 / VIDEO_SAMPLES_LENGHT as f64,
        ),
        last_update_instant: Instant::now(),
    };

    let options = WindowOptions {
        title: String::from("space-time"),
        size: Vector2::new(1280, 720),
        graphics_size: Vector2::new(VIDEO_SAMPLE_WIDTH as u32, VIDEO_SAMPLE_HEIGHT as u32),
        update_delta: Duration::from_millis(16),
    };
    let window = Window::new(options, Box::new(window_thread));
    thread::spawn(move || loop {
        let frame = processing_thread.primary.take().unwrap();
        processing_thread
            .process_sender
            .send(Message::FrameSended(frame))
            .unwrap();
        processing_thread.primary = Some(processing_thread.process());
        let mut must_wait = true;
        while must_wait {
            let message = processing_thread.process_receiver.recv();
            match message {
                Ok(message) => match message {
                    Message::NeedFrame => {
                        must_wait = false;
                    }
                    _ => {}
                },
                Err(_) => {}
            }
        }
    });
    window.event_loop();
}
