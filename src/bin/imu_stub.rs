// extern crate bench_zeromq_msgpack;
extern crate rs;
extern crate clap;

use rs::nodes::sensors::imu::stub::StubNode;
use clap::{Arg, App};


fn main() {
    let matches = App::new("imu_stub")
        .version("0.1.0")
        .author("Jérôme Guéry jerome.guery@gmail.com>")
        .about("RS project: imu stub")
        .arg(Arg::with_name("pub")
                 .required(true)
                 .takes_value(true)
                 .short("p")
                 .long("publish")
                 .value_name("URL")
                 .help("zeromq url to publish (ex: tcp://*:5555)"))
        .get_matches();

    let url_pub = matches.value_of("pub").unwrap();

    let stub = StubNode::new(url_pub);
    stub.run();
}
