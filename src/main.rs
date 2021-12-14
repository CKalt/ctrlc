use std::io::prelude::*;
use std::net::{TcpStream, Shutdown};
use std::sync::{Mutex, Arc};
use std::{thread, time};

struct Connection {
    opt_stream: Option<TcpStream>,
}

fn main() {
    let mt_stream = Arc::new(Mutex::new(
                    Connection {
                        opt_stream: None
                    }));

    let handle = Arc::clone(&mt_stream);
    ctrlc::set_handler(move || {
        println!("Ctrl-C was received...");
        let safe_stream = handle.lock().unwrap();
        safe_stream.opt_stream.as_ref().unwrap().shutdown(Shutdown::Read).unwrap();
    }).unwrap();

    let mut session_num = 1;
    loop {
        {
            let mut safe_stream = mt_stream.lock().unwrap();
            if let None = safe_stream.opt_stream {
                safe_stream.opt_stream =
                    Some(TcpStream::connect("127.0.0.1:5555").unwrap());
            } else {
                panic!("expected disconnected stream");
            }
        }

        loop {
            let mut buf = [0; 100];
            {
                let safe_stream = mt_stream.lock().unwrap();

                if let Err(_) =
                        safe_stream.opt_stream.as_ref().unwrap().read_exact(&mut buf) {
                    println!("got an error, exiting...");
                    break;
                }
            }
            println!("Seesion Number {}, got some bytes", session_num);
        }

        {
            let mut safe_stream = mt_stream.lock().unwrap();
            safe_stream.opt_stream = None; // disconnect
        }
        thread::sleep(time::Duration::from_secs(1));

        if session_num >= 5 {
            break;
        }
        session_num += 1;
    }
}
