use hound;
use std::path::Path;

mod internals;
pub mod music;

pub struct AudioEngine {}

pub struct Wave {
    pub samples: Vec<[f64; 2]>,
}

impl Wave {
    pub fn save(&self, path: &Path, sample_rate: u32) {
        let spec = hound::WavSpec {
            channels: 2,
            sample_rate: sample_rate,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        };
        let mut writer = hound::WavWriter::create(path, spec).unwrap();
        for sample in &self.samples {
            writer.write_sample(sample[0] as f32).unwrap();
            writer.write_sample(sample[1] as f32).unwrap();
        }
    }
    pub fn add_samples(
        &mut self,
        sample_rate: u32,
        samples: Vec<[f64; 2]>,
        time: f64,
        amp_factor: f64,
    ) {
        let start = (time * sample_rate as f64) as usize;
        for i in start..(start + samples.len()) {
            if i >= self.samples.len() {
                return;
            } else {
                self.samples[i][0] += samples[i - start][0] * amp_factor;
                self.samples[i][1] += samples[i - start][1] * amp_factor;
            }
        }
    }
    pub fn add_echo_0x1(&mut self, sample_rate: u32, first_time: f64, echoes: u32, amp_init: f64) {
        let mut time = first_time;
        let mut amp_factor = amp_init;
        for _i in 0..echoes {
            let samples = self.samples.clone();
            self.add_samples(sample_rate, samples, time, amp_factor);
            time /= 2.0;
            amp_factor /= 2.0;
        }
    }
    pub fn add_echo_0x0(&mut self, sample_rate: u32, first_time: f64, echoes: u32, amp_init: f64) {
        let mut time = first_time;
        let mut amp_factor = amp_init;
        for _i in 0..echoes {
            let samples = self.samples.clone();
            self.add_samples(sample_rate, samples, time, amp_factor);
            time += time / 2.0;
            amp_factor /= 2.0;
        }
    }
    pub fn normalize(&mut self) {
        let mut max = 0.0;
        for i in 0..self.samples.len() {
            if self.samples[i][0] > max {
                max = self.samples[i][0]
            }
            if self.samples[i][1] > max {
                max = self.samples[i][1]
            }
        }
        let max = max.abs();
        if max > 0.0 {
            for i in 0..self.samples.len() {
                self.samples[i][0] /= max;
                self.samples[i][1] /= max;
            }
        }
    }
}
