extern crate cpal;

use std::thread;
use cpal::{EventLoop, StreamData, UnknownTypeOutputBuffer};

pub fn try_init_audio() -> bool {
    if let Some(device) = cpal::default_output_device() {
        if let Ok(mut supported_formats_range) = device.supported_output_formats() {
            if let Some(supported_format) = supported_formats_range.next() {
                let format = supported_format.with_max_sample_rate();
                let event_loop = EventLoop::new();
                if let Ok(stream_id) = event_loop.build_output_stream(&device, &format) {
                    event_loop.play_stream(stream_id);
                    thread::spawn(move || {
                        event_loop.run(move |_stream_id, mut stream_data| {
                            match stream_data {
                                StreamData::Output { buffer: UnknownTypeOutputBuffer::U16(mut buffer) } => {
                                    for elem in buffer.iter_mut() {
                                        *elem = 0;
                                    }
                                },
                                StreamData::Output { buffer: UnknownTypeOutputBuffer::I16(mut buffer) } => {
                                    for elem in buffer.iter_mut() {
                                        *elem = 0;
                                    }
                                },
                                StreamData::Output { buffer: UnknownTypeOutputBuffer::F32(mut buffer) } => {
                                    for elem in buffer.iter_mut() {
                                        *elem = 0.0;
                                    }
                                },
                                _ => (),
                            }
                        });
                    });
                    return true;
                }
            }
        }
    }
    false
}