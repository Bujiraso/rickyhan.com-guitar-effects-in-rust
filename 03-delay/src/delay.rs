extern crate jack;

use effects::{Effect, CtrlMsg};

pub struct Delay {
    pub bypassing: bool,

    reader_idx: usize,
    writer_idx: usize,

    delay_buffer: Vec<f32>,
    delay_buffer_size: usize,
    delay_time: usize,
    feedback: f32,

    frame_size: u32,
}

impl Effect for Delay {
    fn new() -> Self {
        let (client, _status) = 
            jack::Client::new("rusty_client", jack::ClientOptions::NO_START_SERVER).unwrap();

        // A five second buffer will be used for the delay
        let delay_buffer_size= client.sample_rate() * 5;
        Delay {
            bypassing: false,
            reader_idx: 0,
            writer_idx: 0,

            delay_buffer_size: delay_buffer_size,
            delay_buffer: vec![0.0; delay_buffer_size],
            // A one second delay time
            delay_time: client.sample_rate(),
            // With a third of the volume
            feedback: 0.33,

            frame_size: client.buffer_size(),
        }
    }

    fn name(&self) -> &'static str {
        "delay"
    }

    fn process_samples(&mut self, input: &[f32], output_l: &mut [f32], output_r: &mut [f32]) {
        if self.bypassing { return; }
		for bufidx in 0..self.frame_size as usize {
			if self.writer_idx >= self.delay_buffer_size {
				self.writer_idx = 0;
			}
			self.reader_idx = if self.writer_idx >= self.delay_time {
				self.writer_idx - self.delay_time
			} else {
				self.delay_buffer_size as usize + self.writer_idx - self.delay_time
			};
			let processed = input[bufidx] + (self.delay_buffer[self.reader_idx] * self.feedback);
			self.delay_buffer[self.writer_idx] = processed;
			let out = (processed + 0.5).cos();
			output_r[bufidx] = out;
			output_l[bufidx] = out;
			self.writer_idx += 1;
		}
	}

	fn bypass(&mut self) {
		self.bypassing = !self.bypassing;
	}

	fn ctrl(&mut self, msg: CtrlMsg) {
		use self::CtrlMsg::*;
		match msg {
			Bypass => self.bypass(),
		}
	}
}
