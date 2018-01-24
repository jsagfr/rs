extern crate rs;
extern crate clap;

use rs::nodes::sensors::imu::lsm9ds1::Lsm9ds1Node;
use clap::{Arg, App};


fn main() {
    let matches = App::new("imu_lm9ds1")
        .version("0.1.0")
        .author("Jérôme Guéry jerome.guery@gmail.com>")
        .about("RS project: imu lsm9ds1")
        .arg(Arg::with_name("pub")
                 .required(true)
                 .takes_value(true)
                 .short("p")
                 .long("publish")
                 .value_name("URL")
                 .help("zeromq url to publish (ex: tcp://*:5555)"))
        .get_matches();

    let url_pub = matches.value_of("pub").unwrap();

    let mut node = Lsm9ds1Node::new(url_pub);
    node.run();
}
