use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufRead, Write};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let buf = BufReader::new(stream.try_clone().unwrap());
    for line in buf.lines() {
        if let Ok(line) = line {
            if line.len() == 0 { break }
            let _ = write!(&mut stream, "{}\n", line);
        } else { break; }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            thread::spawn(move|| handle_client(stream));
        }
    }
    drop(listener);
}
