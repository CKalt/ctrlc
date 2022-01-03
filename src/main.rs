mod mtcp;

use std::ops::DerefMut;
use std::{thread, time};
use mtcp::*;

fn main() {
    let mt_stream: SharedConnection = mtcp::Connection::new();
    let handle = mt_stream.clone();

    ctrlc::set_handler(move || {
        handle.shutdown();
    }).unwrap();

    let mut session_num = 1;
    loop {
        mt_stream.connect("127.0.0.1:5555");

        loop {
            let mut buf = [0; 100];
            if mt_stream.read_exact(&mut buf) {
                break;
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
