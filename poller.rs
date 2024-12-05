use std::io;
use std::io::{Write, Read};
use std::sync::mpsc::{channel, Sender};
use std::process::Command;
use std::process::Stdio;

pub enum StdinDataStatus {
    Data(String),
    Over(i32),
}

pub enum StdinData {
    StdinSender(Sender<StdinDataStatus>),
    Available(u8),
    Over(i32),
}


pub fn read_line_stdin() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;
    println!("{}", buffer);
    Ok(())
}

pub fn live_read_stdin(program: String, arguments: Vec<String>, sender: Sender<StdinData>) -> io::Result<()>{
    let (tx, rx) = channel::<StdinDataStatus>();

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
                Ok(StdinDataStatus::Data(stdin_str)) => {
                    if stdin.write_all(stdin_str.as_bytes()).is_err() {
                        // TODO!
                        // Temporary Fix below
                        let _ = sender_clone.send(StdinData::Over(-1));
                        return;
                    };
                },
                Ok(StdinDataStatus::Over(_code)) => return,
                Err(_e) => return,
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
                let status = proc.wait().unwrap();
                if let Some(code) = status.code() {
                    if sender.send(StdinData::Over(code)).is_err() {
                        return Ok(());
                    };
                }
                if sender.send(StdinData::Over(-1)).is_err() {
                    return Ok(());
                };
                return Ok(());
            },
            Ok(_) => {
                print!("got data: {:?}", buffer[0]);
                sender.send(StdinData::Available(buffer[0])).unwrap(); 
            },
            Err(_) => {
                let status = proc.wait().unwrap();
                if let Some(code) = status.code() {
                    if sender.send(StdinData::Over(code)).is_err() {
                        return Ok(());
                    };
                }
                if sender.send(StdinData::Over(-1)).is_err() {
                    return Ok(());
                };
                return Ok(());
            }
        }
    }

}
