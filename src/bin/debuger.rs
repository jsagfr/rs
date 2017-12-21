// extern crate bench_zeromq_msgpack;
extern crate rs;
extern crate clap;

use rs::nodes::observer::DebuggerNode;
use clap::{Arg, App};


fn main() {
    let matches = App::new("debugger")
        .version("0.1.0")
        .author("Jérôme Guéry jerome.guery@gmail.com>")
        .about("RS project: debugger")
        .arg(Arg::with_name("sub")
                 .required(true)
                 .takes_value(true)
                 .short("s")
                 .long("subscribe")
                 .value_name("URL")
                 .help("zeromq url to subscribe (ex: tcp://127.0.0.1:5555)"))
        .get_matches();

    let url_sub = matches.value_of("sub").unwrap();

    let debugger = DebuggerNode::new(url_sub);
    debugger.run();
}
