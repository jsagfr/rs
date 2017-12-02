extern crate clap;
extern crate zmq;
extern crate rs;
extern crate nalgebra;

use clap::{Arg, App};
use rs::Message;

fn main() {
    let matches = App::new("observer")
        .version("0.1.0")
        .author("Jérôme Guéry jerome.guery@gmail.com>")
        .about("Data observer for for RS project")
        .arg(Arg::with_name("URL")
                 .required(true)
                 .takes_value(true)
                 .index(1)
                 .help("zeromq url to observe (ex: tcp://127.0.0.1:5556)"))
        .get_matches();
    let url = matches.value_of("URL").unwrap();

    let context = zmq::Context::new();
    let subscriber = context.socket(zmq::SUB).unwrap();
    subscriber
        .connect(url)
        .expect(&format!("Unable to connect to \"{}\"", url));
    subscriber.set_subscribe("".as_bytes()).unwrap();

    loop {
        let buf = subscriber.recv_bytes(0).unwrap();
        println!("{}: {}", url, Message::from_msg_pack(&buf));
    }
}
