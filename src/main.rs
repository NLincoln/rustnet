use std::io::{ BufReader, BufRead, Read, Write };
use std::net::{ TcpListener };
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4001").unwrap();

    for stream in listener.incoming() {
        thread::spawn(|| {
            let mut reader = BufReader::new(stream.unwrap());
            for line in reader.by_ref().lines() {
                if line.unwrap() == "" {
                    break;
                }
            }
            reader.into_inner().write_all(b"HTTP/1.1 200 OK\n\nHello World").unwrap();
        });
    }
}
