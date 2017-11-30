extern crate clap;
extern crate zmq;
extern crate rs;
extern crate nalgebra;

use clap::{Arg, App};
use rs::Message;
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
    publisher.bind(url).expect(&format!("Unable to bind \"{}\"", url));

    let stub_data = vec![
        Vector3::new(0., 0., 0.),
        Vector3::new(0., 0., 0.),
        Vector3::new(0., 0., 0.),
        Vector3::new(0., 0., 0.)];
    let mut na: u64 = 0;
    let mut ng: u64 = 0;
    let mut nm: u64 = 0;

    let ten_millis = time::Duration::from_millis(10);
    
    loop {
        let a = Message::Acc(na, stub_data.clone()).to_msg_pack();
        let g = Message::Gyro(ng, stub_data.clone()).to_msg_pack();
        let m = Message::Magn(nm, stub_data.clone()).to_msg_pack();
        publisher.send(&a, 0).unwrap();
        publisher.send(&g, 0).unwrap();
        publisher.send(&m, 0).unwrap();
        na += 4;
        ng += 4;
        nm += 4;
        thread::sleep(ten_millis);
    }
}
