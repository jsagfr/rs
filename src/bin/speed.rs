extern crate clap;
extern crate zmq;
extern crate rs;
extern crate nalgebra;

use clap::{Arg, App};
use rs::Message;
use nalgebra::Vector3;

fn main() {
    let matches = App::new("lsm9ds1")
        .version("0.1.0")
        .author("Jérôme Guéry jerome.guery@gmail.com>")
        .about("Lsm9ds1 sensor reader for RS project")
        .arg(Arg::with_name("URL")
                 .required(true)
                 .takes_value(true)
                 .index(1)
                 .help("zeromq url to publish data (ex: tcp://*:5556)"))
        .arg(Arg::with_name("src")
                 .takes_value(true)
                 .short("s")
                 .long("src")
                 .value_name("SRC")
                 .default_value("tcp://127.0.0.1:5555")
                 .help("zeromq url to subscribe (ex: tcp://127.0.0.1:5555)"))
        .get_matches();
    let url = matches.value_of("URL").unwrap();
    let src = matches.value_of("src").unwrap();

    println!("url: {}", url);

    let context = zmq::Context::new();
    let publisher = context.socket(zmq::PUB).unwrap();
    publisher
        .bind(url)
        .expect(&format!("Unable to bind \"{}\"", url));

    println!("src: {}", src);
    // let contexts = zmq::Context::new();
    let subscriber = context.socket(zmq::SUB).unwrap();
    subscriber
        .connect(src)
        .expect(&format!("Unable to connect to \"{}\"", url));
    subscriber.set_subscribe("".as_bytes()).unwrap();
    println!("connected: {}", src);


    let mut na: Option<u64> = None;
    //    let mut ng: u64 = 0;
    //    let mut nm: u64 = 0;
    let mut nls: u64 = 0;

    let mut l_speed = Vector3::new(0.0, 0.0, 0.0);

    loop {
        let buf = subscriber.recv_bytes(0).unwrap();
        match Message::from_msg_pack(&buf) {
            Message::Acc(mna, ma) => {
                // TODO: Handle lost acc continuation
                // assert!(mna != na.unwrap(), "lost acc continuation");
                let mut t_na = mna;
                for a in ma {
                    l_speed = l_speed + a * 0.01 / 5.;
                    t_na = t_na + 1;
                }
                na = Some(t_na);
                nls = nls + 1;
                let mls = Message::LinearVelocity(nls, l_speed.clone()).to_msg_pack();
                publisher.send(&mls, 0).unwrap();
            }
            _ => {}
        }

    }
}
