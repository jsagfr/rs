extern crate nalgebra;
extern crate zmq;
extern crate lsm9ds1;

use nalgebra::Vector3;

use self::lsm9ds1::Lsm9ds1;
use self::lsm9ds1::i2c::I2cInterface;
use self::lsm9ds1::config::{Config, FsXl, DataRate, OutputDataRate, Md};

const G: f32 = 9.81;

use {Message, TimeStamp};

pub struct Lsm9ds1Node {
    lsm9ds1: Lsm9ds1<I2cInterface>,
    publisher: zmq::Socket,
    ts: TimeStamp,
}

const INTERVAL: u64 = 2; // 2ms = 0.002s

impl Lsm9ds1Node {
    pub fn new(url_pub: &str) -> Lsm9ds1Node {
        let context = zmq::Context::new();
        Self::new_with_ctx(url_pub, &context)
    }

    pub fn new_with_ctx(url_pub: &str, context: &zmq::Context) -> Lsm9ds1Node {
        let publisher = context.socket(zmq::PUB).unwrap();
        publisher
            .bind(url_pub)
            .expect(&format!("dummy:publisher: bind error: \"{}\"", url_pub));

        let default = Config::default();
        let interface = I2cInterface::new("/dev/i2c-2");
        let mut lsm9ds1 = Lsm9ds1::from_interface(interface).unwrap();
        lsm9ds1.set_fs_xl(FsXl::Fs8).expect("Unable to set fs_xl");
        lsm9ds1.set_odr_g(DataRate::DR119Hz).expect("Unable to set odr_g");
        lsm9ds1.set_odr_xl(DataRate::DR119Hz).expect("Unable to set odr_xl");
        lsm9ds1.set_output_data_rate(OutputDataRate::Odr10Hz).expect("Unable to set output_data_rate");
        lsm9ds1.set_md(Md::Continuous);
        lsm9ds1.re_read_config().expect("Unable to reread config");
        
        Lsm9ds1Node {
            lsm9ds1: lsm9ds1,
            publisher: publisher,
            ts: TimeStamp::new(),
        }
    }

    pub fn run(&mut self) {
        let interval = 0.0001;
            loop {
            let ts = self.ts.now();

            let data_la = vec![
                Vector3::<f32>::new(G * self.lsm9ds1.lx().unwrap(), G * self.lsm9ds1.ly().unwrap(), G * self.lsm9ds1.lz().unwrap()),
            ];
            let data_aa = vec![
                Vector3::<f32>::new(self.lsm9ds1.gx().unwrap(), self.lsm9ds1.gy().unwrap(), self.lsm9ds1.gz().unwrap()),
            ];
            let data_mv = vec![
                Vector3::<f32>::new(self.lsm9ds1.mx().unwrap(), self.lsm9ds1.my().unwrap(), self.lsm9ds1.mz().unwrap()),
            ];
            let a = Message::Acc(ts, interval, data_la).to_msg_pack();
            let g = Message::Gyro(ts, interval, data_aa).to_msg_pack();
            let m = Message::Magn(ts, interval, data_mv).to_msg_pack();
            self.publisher.send(&a, 0).unwrap();
            self.publisher.send(&g, 0).unwrap();
            self.publisher.send(&m, 0).unwrap();
        }
    }
}
