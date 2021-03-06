#[macro_use]
extern crate golang;

use std::sync::mpsc::{channel, Sender, Receiver};

use std::thread;

use std::time::Duration;

fn ping(tx : Sender<String>) {
    let t = Duration::new(1,0);
    loop {
	print!("ping");
        tx.send(String::from("pong")).unwrap();
        thread::sleep(t);
    }
}

fn pong(rx : Receiver<String>) {
    loop {
        let mesg = rx.recv().unwrap();
        println!("{}", mesg);
    }
}


fn main() {
    let (tx,rx) = channel();

    go!(ping(tx));
    go!(pong(rx));

    thread::sleep(Duration::new(30,0));
    println!("All done!");
}
