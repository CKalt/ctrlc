use std::net::{TcpStream, Shutdown};
use std::sync::{RwLock, Arc};
use std::ops::DerefMut;
use std::io::prelude::*;

type OptStream = Option<TcpStream>;
pub type Connection = Arc<RwLock<OptStream>>;

pub trait ConnectionTrait {
    fn create_new() -> Connection;
    fn connect(&self, url: &str);
    fn shutdown(&self);
    fn read_exact(&self, buf: &mut[u8]) -> bool;
}

impl ConnectionTrait for Connection {
    fn create_new() -> Connection {
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
