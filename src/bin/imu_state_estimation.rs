// extern crate bench_zeromq_msgpack;
extern crate rs;
extern crate clap;

use rs::nodes::estimations::imu_state_estimation::ImuStateEstimationNode;
use clap::{Arg, App};


fn main() {
    let matches = App::new("imu_state_estimation")
        .version("0.1.0")
        .author("Jérôme Guéry jerome.guery@gmail.com>")
        .about("RS project: state estimation from only imu data")
        .arg(Arg::with_name("pub")
                 .required(true)
                 .takes_value(true)
                 .short("p")
                 .long("publish")
                 .value_name("URL")
                 .help("zeromq url to publish (ex: tcp://*:5555)"))
        .arg(Arg::with_name("sub")
                 .required(true)
                 .takes_value(true)
                 .short("s")
                 .long("subscribe")
                 .value_name("URL")
                 .help("zeromq url to subscribe (ex: tcp://127.0.0.1:5555)"))
        .get_matches();

    let url_pub = matches.value_of("pub").unwrap();
    let url_sub = matches.value_of("sub").unwrap();

    let mut estimation = ImuStateEstimationNode::new(url_pub, url_sub);
    estimation.run();
}
