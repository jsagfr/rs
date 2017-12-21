extern crate nalgebra;
extern crate zmq;

// use zmq::{Context, Socket};
use nalgebra::Vector3;
use std::thread;
use std::time::Duration;

use {Message, TimeStamp};

pub struct StubNode {
    publisher: zmq::Socket,
    ts: TimeStamp,
}

const INTERVAL: u64 = 2; // 2ms = 0.002s

impl StubNode {
    pub fn new(url_pub: &str) -> StubNode {
        let context = zmq::Context::new();
        Self::new_with_ctx(url_pub, &context)
    }

    pub fn new_with_ctx(url_pub: &str, context: &zmq::Context) -> StubNode {
        let publisher = context.socket(zmq::PUB).unwrap();
        publisher
            .bind(url_pub)
            .expect(&format!("dummy:publisher: bind error: \"{}\"", url_pub));

        StubNode {
            publisher: publisher,
            ts: TimeStamp::new(),
        }
    }

    pub fn run(&self) {
        let d: Duration = Duration::from_millis(INTERVAL); // 0.002s
        loop {
            let ts1 = self.ts.now() as f32;
            thread::sleep(d);
            let ts2 = self.ts.now() as f32;
            thread::sleep(d);
            let ts3 = self.ts.now() as f32;
            thread::sleep(d);
            let ts4 = self.ts.now() as f32;
            thread::sleep(d);
            let ts = self.ts.now();
            let ts5 = ts as f32;
            let interval = (ts5 - ts1) / 5.;

            let sin_data_lax = vec![
                Vector3::<f32>::new((ts1 * 0.1).sin(), 0., -9.81),
                Vector3::<f32>::new((ts2 * 0.1).sin(), 0., -9.81),
                Vector3::<f32>::new((ts3 * 0.1).sin(), 0., -9.81),
                Vector3::<f32>::new((ts4 * 0.1).sin(), 0., -9.81),
                Vector3::<f32>::new((ts5 * 0.1).sin(), 0., -9.81),
            ];
            let sin_data_aaz = vec![
                Vector3::<f32>::new(0., 0., ts1.sin()),
                Vector3::<f32>::new(0., 0., ts2.sin()),
                Vector3::<f32>::new(0., 0., ts3.sin()),
                Vector3::<f32>::new(0., 0., ts4.sin()),
                Vector3::<f32>::new(0., 0., ts5.sin()),
            ];
            let ran_data_m = vec![
                Vector3::<f32>::new_random(),
                Vector3::<f32>::new_random(),
                Vector3::<f32>::new_random(),
                Vector3::<f32>::new_random(),
                Vector3::<f32>::new_random(),
            ];
            let a = Message::Acc(ts, interval, sin_data_lax).to_msg_pack();
            let g = Message::Gyro(ts, interval, sin_data_aaz).to_msg_pack();
            let m = Message::Magn(ts, interval, ran_data_m).to_msg_pack();
            self.publisher.send(&a, 0).unwrap();
            self.publisher.send(&g, 0).unwrap();
            self.publisher.send(&m, 0).unwrap();
            thread::sleep(d);
        }
    }
}
