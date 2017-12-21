extern crate nalgebra;
extern crate zmq;

// use zmq::{Context, Socket};

use Message;

pub struct DebuggerNode {
    subscriber: zmq::Socket,
}

impl DebuggerNode {
    pub fn new(url_sub: &str) -> DebuggerNode {
        let context = zmq::Context::new();
        Self::new_with_ctx(url_sub, &context)
    }

    pub fn new_with_ctx(url_sub: &str, context: &zmq::Context) -> DebuggerNode {
        let subscriber = context.socket(zmq::SUB).unwrap();
        subscriber.connect(url_sub).expect(&format!(
            "dummy_dot:subscriber: connect error: \"{}\"",
            url_sub
        ));
        subscriber.set_subscribe(b"").unwrap();

        DebuggerNode {
            subscriber: subscriber,
        }
    }

    pub fn run(&self) {
        loop {
            let buf = self.subscriber.recv_bytes(0).unwrap();
            println!("{:?}", Message::from_msg_pack(&buf));
        }
    }
}
