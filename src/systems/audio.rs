use bevy::prelude::*;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};

#[derive(Resource)]
pub struct AudioState {
    _stream: Arc<OutputStream>,
    _stream_handle: Arc<OutputStreamHandle>,
    sink: Arc<Mutex<Sink>>,
}

impl AudioState {
    pub fn new() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        sink.set_volume(0.5);
        Self {
            _stream: Arc::new(stream),
            _stream_handle: Arc::new(stream_handle),
            sink: Arc::new(Mutex::new(sink)),
        }
    }

    pub fn fade_out(&self, duration: f32) {
        let sink = self.sink.clone();
        let steps = 50;
        let start_volume = sink.lock().unwrap().volume();
        let volume_step = start_volume / steps as f32;
        let sleep_duration = duration / steps as f32;
        
        std::thread::spawn(move || {
            for _ in 0..steps {
                if let Ok(mut sink) = sink.lock() {
                    let new_volume = (sink.volume() - volume_step).max(0.0);
                    sink.set_volume(new_volume);
                }
                std::thread::sleep(std::time::Duration::from_secs_f32(sleep_duration));
            }
            if let Ok(sink) = sink.lock() {
                sink.stop();
            }
        });
    }
}

pub fn setup_audio(mut commands: Commands) {
    let mut audio_state = AudioState::new();
    
    // Load and play the intro music
    let file = BufReader::new(File::open("src/soundtrack_1.mp3").unwrap());
    let source = Decoder::new(file).unwrap();
    if let Ok(mut sink) = audio_state.sink.lock() {
        sink.append(source);
    }
    
    commands.insert_resource(audio_state);
}
