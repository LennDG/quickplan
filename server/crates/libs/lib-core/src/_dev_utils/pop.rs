use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;
use tracing::debug;

pub fn pop() {
    // Get a output stream handle to the default physical sound device
    let (_stream, handle) = OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open("sounds/pop.ogg").unwrap());

    sink.append(rodio::Decoder::new(file).unwrap());

    sink.sleep_until_end();
}
