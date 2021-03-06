extern crate clap;
extern crate zmq;
extern crate rs;
extern crate nalgebra;
// #[macro_use]
// extern crate chan;

use clap::{Arg, App};
use rs::{Message, TimeStamp};
use nalgebra::Vector3;
use std::{thread, time};

fn main() {
    let matches = App::new("lsm9ds1")
        .version("0.1.0")
        .author("Jérôme Guéry jerome.guery@gmail.com>")
        .about("Lsm9ds1 sensor reader for RS project")
        .arg(Arg::with_name("URL")
                 .required(true)
                 .takes_value(true)
                 .index(1)
                 .help("zeromq url to publish data (ex: tcp://*:5555)"))
        .arg(Arg::with_name("device")
                 .takes_value(true)
                 .short("d")
                 .long("device")
                 .value_name("DEV")
                 .default_value("/dev/i2c-0")
                 .help("i2c bus where lsm9ds1 is connected"))
        .get_matches();
    let url = matches.value_of("URL").unwrap();

    let context = zmq::Context::new();
    let publisher = context.socket(zmq::PUB).unwrap();
    publisher
        .bind(url)
        .expect(&format!("Unable to bind \"{}\"", url));

    // let stub_data = vec![Vector3::new(0., 0., 0.),
    //                      Vector3::new(0., 0., 0.),
    //                      Vector3::new(0., 0., 0.),
    //                      Vector3::new(0., 0., 0.),
    //                      Vector3::new(0., 0., 0.)];
    let mut na: u64 = 0;
    // let mut ng: u64 = 0;
    // let mut nm: u64 = 0;

    let ten_millis = time::Duration::from_millis(10);

    // let tick = chan::tick_ms(10);
    let t = TimeStamp::new();
    loop {
        let start_loop = time::Instant::now();
        // chan_select! {
        //     tick.recv() => {
        let sin_data = vec![Vector3::new((na as f32 * 0.01).sin(), 0., 0.),
                            Vector3::new(((na + 1) as f32 * 0.01).sin(), 0., 0.),
                            Vector3::new(((na + 2) as f32 * 0.01).sin(), 0., 0.),
                            Vector3::new(((na + 3) as f32 * 0.01).sin(), 0., 0.),
                            Vector3::new(((na + 4) as f32 * 0.01).sin(), 0., 0.)];
        let a = Message::Acc(t.now(), 0.002, sin_data.clone()).to_msg_pack();
        let g = Message::Gyro(t.now(), 0.002, sin_data.clone()).to_msg_pack();
        let m = Message::Magn(t.now(), 0.002, sin_data.clone()).to_msg_pack();
        publisher.send(&a, 0).unwrap();
        publisher.send(&g, 0).unwrap();
        publisher.send(&m, 0).unwrap();
        na += 5;
        // ng += 5;
        // nm += 5;
        //     }
        // }
        thread::sleep(ten_millis - start_loop.elapsed());
    }
}
