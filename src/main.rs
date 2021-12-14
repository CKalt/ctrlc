use std::io::prelude::*;
use std::net::{TcpStream, Shutdown};
use std::sync::Arc;

fn main() {
    let stream = TcpStream::connect("127.0.0.1:5555").unwrap();
    let stream = Arc::new(stream);
    let handle = Arc::clone(&stream);
    ctrlc::set_handler(move || {
        println!("Ctrl-C was received...");
        let _ = handle.shutdown(Shutdown::Read);
    }).unwrap();
    loop {
        let mut buf = [0; 100];
        if let Err(_) = (&*stream).read_exact(&mut buf) {
            eprintln!("got an error, exiting...");
            break;
        }
        eprintln!("got some bytes");
    }
}
