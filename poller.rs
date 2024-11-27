use std::io;
use std::io::{Write, Read};
use std::sync::mpsc::{channel, Sender};
use std::process::Command;
use std::process::Stdio;

pub enum StdinData {
    StdinSender(Sender<String>),
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

pub fn live_read_stdin(program: String, arguments: Vec<String>, sender: Sender<StdinData>) -> io::Result<()>{
    let (tx, rx) = channel::<String>();

    sender.send(StdinData::StdinSender(tx.clone())).unwrap();

    let mut proc = Command::new(program)
        .args(arguments)
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to execute command");

    let mut stdin = proc.stdin.take().expect("Failed to open stdin");
    let mut stdout = proc.stdout.take().unwrap();

    let sender_clone = sender.clone();
    std::thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(stdin_str) => {
                    if stdin.write_all(stdin_str.as_bytes()).is_err() {
                        sender_clone.send(StdinData::Over).unwrap();
                        return;
                    };
                },
                Err(_e) => {
                }
            }
        }
    });

    let mut buffer = [0;1];
    
    loop {
        match stdout.read(&mut buffer) { 
            Ok(0) => {
                println!("sending over");
                // TODO!
                // Temporary Fix below;
                if sender.send(StdinData::Over).is_err() {
                    return Ok(());
                };
                return Ok(());
            },
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
