pub mod poller;
use poller::StdinData;

use std::sync::mpsc::channel;
use std::thread::spawn;


fn main() {
    let (sender, receiver) = channel();

    spawn(move || {
        let _ = poller::live_read_stdin(sender.clone());
    });

    loop {
        let char_data = receiver.recv().unwrap();
        match char_data {
            StdinData::Available(c) => {
                let utf8_char = String::from_utf8(vec![c]).unwrap();
                print!("{}", utf8_char);
            },
            StdinData::Over => {
                println!("buffer over");
                break;
            }
        }
    }
}
