extern crate nalgebra;
extern crate zmq;

// use zmq::{Context, Socket};
use nalgebra::Vector3;

use {Message, TimeStamp};

pub struct ImuStateEstimationNode {
    publisher: zmq::Socket,
    subscriber: zmq::Socket,
    estimation: ImuStateEstimation,
    ts: TimeStamp,
}

impl ImuStateEstimationNode {
    pub fn new(url_pub: &str, url_sub: &str) -> ImuStateEstimationNode {
        let context = zmq::Context::new();
        Self::new_with_ctx(url_pub, url_sub, &context)
    }

    pub fn new_with_ctx(
        url_pub: &str,
        url_sub: &str,
        context: &zmq::Context,
    ) -> ImuStateEstimationNode {
        let publisher = context.socket(zmq::PUB).unwrap();
        publisher
            .bind(url_pub)
            .expect(&format!("dummy:publisher: bind error: \"{}\"", url_pub));

        let subscriber = context.socket(zmq::SUB).unwrap();
        subscriber.connect(url_sub).expect(&format!(
            "dummy_dot:subscriber: connect error: \"{}\"",
            url_sub
        ));
        subscriber.set_subscribe(b"").unwrap();

        ImuStateEstimationNode {
            publisher: publisher,
            subscriber: subscriber,
            estimation: ImuStateEstimation::new(),
            ts: TimeStamp::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            let buf = self.subscriber.recv_bytes(0).unwrap();
            match Message::from_msg_pack(&buf) {
                Message::Acc(t, interval, ma) => {
                    for i in 0..ma.len() {
                        self.estimation
                            .update(ma[i], t - (ma.len() - i) as f64 * interval as f64);
                    }
                    let mls = Message::ActualLinearVelocity(
                        self.ts.now(),
                        self.estimation.l_vel().clone(),
                    ).to_msg_pack();
                    self.publisher.send(&mls, 0).unwrap();
                }
                _ => {}
            }
        }
    }
}

pub struct ImuStateEstimation {
    last_acc_ts: Option<f64>,
    l_acc: Vector3<f32>, // keep acc to implement filters
    l_vel: Vector3<f32>,
    l_pos: Vector3<f32>,
}

impl ImuStateEstimation {
    pub fn new() -> ImuStateEstimation {
        ImuStateEstimation {
            last_acc_ts: None,
            l_acc: Vector3::zeros(),
            l_vel: Vector3::zeros(),
            l_pos: Vector3::zeros(),
        }
    }

    pub fn update(&mut self, acc: Vector3<f32>, ts: f64) {
        match self.last_acc_ts {
            Some(last_acc_ts) => {
                if ts >= last_acc_ts {
                    self.l_vel += acc * (ts - last_acc_ts) as f32;
                    self.l_vel[2] = 0.0;
                }
            }
            None => {}
        }
        self.l_acc = acc;
        self.last_acc_ts = Some(ts);
    }

    pub fn l_vel(&self) -> &Vector3<f32> {
        &self.l_vel
    }
}
