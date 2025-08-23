use anyhow::{anyhow, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext};

#[derive(Clone)]
pub struct VoiceTranscriber {
    model_path: PathBuf,
    vad_threshold: f32,
}

impl VoiceTranscriber {
    pub fn new(model_path: PathBuf, vad_threshold: f32) -> Self {
        Self {
            model_path,
            vad_threshold,
        }
    }

    pub async fn record_and_transcribe(&self) -> Result<String> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or(anyhow!("no input device available"))?;
        let config = device.default_input_config()?;
        let sample_rate = config.sample_rate().0;
        let channels = config.channels() as usize;
        let data: Arc<Mutex<Vec<f32>>> = Arc::new(Mutex::new(Vec::new()));
        let data_cb = data.clone();
        let err_fn = |err| eprintln!("audio capture error: {err}");
        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_input_stream(
                &config.clone().into(),
                move |samples: &[f32], _| {
                    let mut buf = data_cb.lock().unwrap();
                    for frame in samples.chunks(channels) {
                        buf.push(frame[0]);
                    }
                },
                err_fn,
                None,
            )?,
            cpal::SampleFormat::I16 => device.build_input_stream(
                &config.clone().into(),
                move |samples: &[i16], _| {
                    let mut buf = data_cb.lock().unwrap();
                    for frame in samples.chunks(channels) {
                        buf.push(frame[0] as f32 / i16::MAX as f32);
                    }
                },
                err_fn,
                None,
            )?,
            cpal::SampleFormat::U16 => device.build_input_stream(
                &config.clone().into(),
                move |samples: &[u16], _| {
                    let mut buf = data_cb.lock().unwrap();
                    for frame in samples.chunks(channels) {
                        buf.push(frame[0] as f32 / u16::MAX as f32);
                    }
                },
                err_fn,
                None,
            )?,
            _ => return Err(anyhow!("unsupported sample format")),
        };
        stream.play()?;
        // record for a short duration
        sleep(Duration::from_secs(5)).await;
        drop(stream);
        let mut samples = data.lock().unwrap().clone();
        samples = trim_silence(&samples, self.vad_threshold);
        let audio = resample_to_16k(&samples, sample_rate);
        let ctx = WhisperContext::new_with_params(
            &self.model_path.to_string_lossy(),
            whisper_rs::WhisperContextParameters::default(),
        )?;
        let mut state = ctx.create_state()?;
        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);
        state.full(params, &audio)?;
        let n = state.full_n_segments();
        let mut out = String::new();
        for i in 0..n {
            if let Some(segment) = state.get_segment(i) {
                out.push_str(segment.to_str()?);
            }
        }
        Ok(out.trim().to_string())
    }
}

pub(crate) fn resample_to_16k(input: &[f32], sample_rate: u32) -> Vec<f32> {
    if sample_rate == 16000 {
        return input.to_vec();
    }
    let ratio = 16000.0 / sample_rate as f32;
    let out_len = (input.len() as f32 * ratio) as usize;
    let mut out = Vec::with_capacity(out_len);
    for i in 0..out_len {
        let orig = i as f32 / ratio;
        let idx0 = orig.floor() as usize;
        let idx1 = idx0 + 1;
        let frac = orig - idx0 as f32;
        let s0 = *input.get(idx0).unwrap_or(&0.0);
        let s1 = *input.get(idx1).unwrap_or(&0.0);
        out.push(s0 + (s1 - s0) * frac);
    }
    out
}

pub(crate) fn trim_silence(samples: &[f32], threshold: f32) -> Vec<f32> {
    if samples.is_empty() {
        return Vec::new();
    }
    let start = samples
        .iter()
        .position(|s| s.abs() >= threshold)
        .unwrap_or(0);
    let end = samples
        .iter()
        .rposition(|s| s.abs() >= threshold)
        .map(|i| i + 1)
        .unwrap_or(samples.len());
    samples[start..end].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trims_leading_and_trailing_silence() {
        let samples = [0.0, 0.0, 0.5, 0.0];
        let trimmed = trim_silence(&samples, 0.1);
        assert_eq!(trimmed, vec![0.5]);
    }

    #[test]
    fn resamples_to_expected_length() {
        let input = vec![0.0, 1.0, 0.0, -1.0];
        let output = resample_to_16k(&input, 8000);
        assert_eq!(output.len(), 8);
    }
}
