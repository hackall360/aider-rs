use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

use crate::CoreError;

pub fn record() -> Result<String, CoreError> {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .ok_or_else(|| CoreError::Audio("no input device".into()))?;
    let config = device
        .default_input_config()
        .map_err(|e| CoreError::Audio(e.to_string()))?;
    let sample_format = config.sample_format();
    let cfg: cpal::StreamConfig = config.into();

    let collected: Arc<Mutex<Vec<f32>>> = Arc::new(Mutex::new(Vec::new()));
    let data_buf = collected.clone();
    let err_fn = |e| eprintln!("stream error: {e}");

    let stream = match sample_format {
        cpal::SampleFormat::F32 => device.build_input_stream(
            &cfg,
            move |data: &[f32], _| {
                data_buf.lock().unwrap().extend_from_slice(data);
            },
            err_fn,
            None,
        ),
        cpal::SampleFormat::I16 => device.build_input_stream(
            &cfg,
            move |data: &[i16], _| {
                data_buf
                    .lock()
                    .unwrap()
                    .extend(data.iter().map(|s| *s as f32 / i16::MAX as f32));
            },
            err_fn,
            None,
        ),
        cpal::SampleFormat::U16 => device.build_input_stream(
            &cfg,
            move |data: &[u16], _| {
                data_buf.lock().unwrap().extend(
                    data.iter().map(|s| *s as f32 / u16::MAX as f32 - 0.5),
                );
            },
            err_fn,
            None,
        ),
        _ => return Err(CoreError::Audio("unsupported sample format".into())),
    }
    .map_err(|e| CoreError::Audio(e.to_string()))?;

    stream
        .play()
        .map_err(|e| CoreError::Audio(e.to_string()))?;
    thread::sleep(Duration::from_secs(5));
    drop(stream);

    let audio = collected.lock().unwrap().clone();
    let ctx = WhisperContext::new_with_params(
        "resources/ggml-base.en.bin",
        WhisperContextParameters::default(),
    )
    .map_err(|e| CoreError::Audio(e.to_string()))?;
    let mut state = ctx
        .create_state()
        .map_err(|e| CoreError::Audio(e.to_string()))?;
    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
    params.set_n_threads(1);
    state
        .full(params, &audio)
        .map_err(|e| CoreError::Audio(e.to_string()))?;
    let mut out = String::new();
    let n = state
        .full_n_segments()
        .map_err(|e| CoreError::Audio(e.to_string()))?;
    for i in 0..n {
        if let Ok(seg) = state.full_get_segment_text(i) {
            out.push_str(&seg);
        }
    }
    Ok(out)
}
