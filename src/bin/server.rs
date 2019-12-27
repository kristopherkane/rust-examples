use std::thread;
use std::net::{TcpListener, TcpStream};
use std::str::from_utf8;
use std::io::Read;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:5000").unwrap();
    for stream in listener.incoming() {
        match stream {
            //Change this to a thread pool next
            //Reject connections if the pool is full
            Ok(stream) => {
                thread::spawn(move || {
                    connection_request(stream)
                });
            }
            Err(e) => {
                println!("conn failed with: {}", e)
            }
        }
    }
    drop(listener);
}

fn connection_request(mut stream: TcpStream) {
    let mut read_buffer = vec![];
    loop {
        match stream.read_to_end(&mut read_buffer) {
            Ok(_) => {
                println!("Received: {}", from_utf8(&read_buffer).unwrap());
                break
            },
            Err(_) => {
                println!("error");
            }
        }
        {}
    }
}