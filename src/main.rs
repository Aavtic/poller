pub mod poller;
use poller::StdinData;
use std::io::Write;
use std::sync::mpsc::channel;

fn main() {
    let (tx, rx) = channel();

     let txclone = tx.clone();
     let t1 = std::thread::spawn(move || {
         let _ = poller::live_read_stdin("python3".to_string(), vec!["blocking.py".to_string()], txclone);
     });
     let mut stdout = std::io::stdout().lock();
 
     while let Ok(stdin_data) = rx.recv() {
         match stdin_data {
             StdinData::Available(char) => {
                 print!("{}", String::from_utf8(vec![char]).unwrap());
                 stdout.flush().unwrap();
             },
             StdinData::StdinSender(sender) => {
                 std::thread::spawn(move || {
                     std::thread::sleep(std::time::Duration::from_secs(4));
                     sender.send("Aadish\n".to_string()).unwrap();
                     std::thread::sleep(std::time::Duration::from_secs(4));
                     sender.send("18\n".to_string()).unwrap();
                 });
             },
             StdinData::Over => break,
         }
     }
     t1.join().unwrap();
}
