use std::net::{TcpStream, Shutdown};
use std::sync::{RwLock, Arc};
use std::ops::DerefMut;
use std::io::prelude::*;

type OptStream = Option<TcpStream>;
pub type SharedConnection = Arc<RwLock<OptStream>>;

pub trait Connection {
    fn new() -> Self;
    fn connect(&self, url: &str);
    fn disconnect(&self);
    fn shutdown(&self);
    fn read_exact(&self, buf: &mut[u8]) -> bool;
}

impl Connection for SharedConnection {
    fn new() -> Self {
        Arc::new(RwLock::new(None))
    }
    fn connect(&self, url: &str) {
        let mut safe_stream_rwlock = self.write().unwrap();
        let safe_stream = safe_stream_rwlock.deref_mut();
        if safe_stream.is_none() {
            *safe_stream =
                Some(TcpStream::connect(url).unwrap());
        } else {
            panic!("expected disconnected stream");
        }
    }
    fn disconnect(&self) {
        let mut safe_stream_rwlock = self.write().unwrap();
        let safe_stream = safe_stream_rwlock.deref_mut();
        *safe_stream = None; // disconnect
    }
    fn shutdown(&self) {
        let safe_stream = self.read().unwrap();
        if safe_stream.as_ref().is_some() {
            let stream = safe_stream.as_ref().unwrap();
            stream.shutdown(Shutdown::Read).unwrap();
        }
    }
    fn read_exact(&self, buf: &mut[u8]) -> bool {
        let safe_stream = self.read().unwrap();

        if let Err(_) =
                safe_stream.as_ref().unwrap().read_exact(buf) {
            println!("got an error, exiting...");
            true
        } else {
            false
        }
    }
}
