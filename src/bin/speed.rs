extern crate clap;
extern crate zmq;
extern crate rs;
extern crate nalgebra;

use clap::{Arg, App};
use rs::{TimeStamp, Message};
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

    let context = zmq::Context::new();
    let publisher = context.socket(zmq::PUB).unwrap();
    publisher
        .bind(url)
        .expect(&format!("Unable to bind \"{}\"", url));

    let subscriber = context.socket(zmq::SUB).unwrap();
    subscriber
        .connect(src)
        .expect(&format!("Unable to connect to \"{}\"", url));
    subscriber.set_subscribe(b"").unwrap();


    // let mut na: Option<u64> = None;
    let mut last_ts: Option<f64> = None;
    //    let mut ng: u64 = 0;
    //    let mut nm: u64 = 0;
    let speed_ts = TimeStamp::new();

    let mut l_speed = Vector3::zeros();

    loop {
        let buf = subscriber.recv_bytes(0).unwrap();
        match Message::from_msg_pack(&buf) {
            Message::Acc(t, interval, ma) => {
                match last_ts {
                    // TODO: Easy but not optimum, we drop all data to
                    // have a first valid timestamp:
                    None => {}
                    Some(last_ts) => {
                        let j = 0;
                        for a in &ma {
                            let interval = match j {
                                0 => {
                                    assert!(t > last_ts);
                                    (t - last_ts) as f32 + interval * (ma.len() as f32)
                                }
                                _ => interval,
                            };
                            l_speed += a * interval;
                        }
                        let mls = Message::ActualLinearVelocity(speed_ts.now(), l_speed.clone())
                            .to_msg_pack();
                        publisher.send(&mls, 0).unwrap();
                    }
                };
                last_ts = Some(t);
            }
            _ => {}
        }

    }
}
