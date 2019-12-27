use std::net::TcpStream;
use std::io::Write;

fn main() {
    match TcpStream::connect("localhost:5000") {
        Ok(mut stream) => {
            let msg = b"asdfjkl; 0123456789";
            stream.write(msg).unwrap();
        }
        Err(e) => {
            println!("asdf: {}", e);
        }
    }
}