use std::io::prelude::*;
use std::net::{TcpStream, Shutdown};
use std::sync::Arc;
//use std::sync::atomic::{AtomicBool, Ordering};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    #[structopt(short, long)]
    use_arc: bool,
}

fn do_arc_trial() {
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

fn main() {
    let opt = Opt::from_args();

    if opt.use_arc {
        do_arc_trial();
    } else {
        println!("atomic bool usage");
    }
}
