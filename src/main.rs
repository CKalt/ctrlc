use std::io::prelude::*;
use std::net::{TcpStream, Shutdown};
use std::sync::{RwLock, Arc};
use std::ops::DerefMut;
use std::{thread, time};

type Connection = Option<TcpStream>;

fn main() {
    let mt_stream: Arc<RwLock<Connection>> = Arc::new(RwLock::new(None));

    let handle = Arc::clone(&mt_stream);
    ctrlc::set_handler(move || {
        println!("Ctrl-C was received...");
        let safe_stream = handle.read().unwrap();
        safe_stream.as_ref().unwrap().shutdown(Shutdown::Read).unwrap();
    }).unwrap();

    let mut session_num = 1;
    loop {
        {
            let mut safe_stream_rwlock = mt_stream.write().unwrap();
            let safe_stream = safe_stream_rwlock.deref_mut();
            if safe_stream.is_none() {
                *safe_stream =
                    Some(TcpStream::connect("127.0.0.1:5555").unwrap());
            } else {
                panic!("expected disconnected stream");
            }
        }

        loop {
            let mut buf = [0; 100];
            {
                let safe_stream = mt_stream.read().unwrap();

                if let Err(_) =
                        safe_stream.as_ref().unwrap().read_exact(&mut buf) {
                    println!("got an error, exiting...");
                    break;
                }
            }
            println!("Seesion Number {}, got some bytes", session_num);
        }

        {
            let mut safe_stream_rwlock = mt_stream.write().unwrap();
            let safe_stream = safe_stream_rwlock.deref_mut();
            *safe_stream = None; // disconnect
        }
        thread::sleep(time::Duration::from_secs(1));

        if session_num >= 5 {
            break;
        }
        session_num += 1;
    }
}
