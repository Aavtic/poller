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

pub fn live_read_stdin(sender: Sender<StdinData>, reader: &mut impl Read) -> io::Result<()>{
    let mut buffer = [0;1];
    
    loop {
        match reader.read(&mut buffer) { 
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
