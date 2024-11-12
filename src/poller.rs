use std::io;
use std::io::Read;
use std::sync::mpsc::Sender;


pub enum StdinData {
    Available(u8),
    Over,
}


pub fn read_line_stdin() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;
    println!("{}", buffer);
    Ok(())
}

pub fn live_read_stdin(sender: Sender<StdinData>) -> io::Result<()>{
    let stdin = io::stdin();
    let mut buffer = [0;1];
    let mut handle = stdin.lock();
    
    loop {
        match handle.read(&mut buffer) { 
            Ok(0) => {
                sender.send(StdinData::Over).unwrap();
                return Ok(());
            }
            Ok(_) => {
                sender.send(StdinData::Available(buffer[0])).unwrap();
            },
            Err(_) => {
                sender.send(StdinData::Over).unwrap();
                return Ok(());
            }
        }
    }
}

