use std::fs::File;
use std::io::BufReader;

use rodio::{Decoder, OutputStream, Source};

use libmpax::add;

fn main() {
    let (_s, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open("some path").unwrap());
    let source = Decoder::new(file).unwrap();
    stream_handle.play_raw(source.convert_samples()).unwrap();
    let x = add(2, 3);
    println!("Hello, world! {x}");
    std::thread::sleep(std::time::Duration::from_secs(100));
}
