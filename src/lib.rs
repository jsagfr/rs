extern crate nalgebra;
extern crate rmp_serde as rmps;
#[macro_use]
extern crate serde_derive;

pub mod nodes;

use nalgebra::Vector3;
use rmps::{from_slice, to_vec_named};
use std::fmt;
use std::time::Instant;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum Message {
    Acc(f64, f32, Vec<Vector3<f32>>),
    Gyro(f64, f32, Vec<Vector3<f32>>),
    Magn(f64, f32, Vec<Vector3<f32>>),
    ExpectedLinearVelocity(f64, Vector3<f32>),
    ActualLinearVelocity(f64, Vector3<f32>),
    Twist(f64, Vector3<f32>, Vector3<f32>),
    MotorsPower2XLeft(f64, f32),
    MotorsPower2XRight(f64, f32),
}

/// TimeStamp is a relative time from the start of the program, with a
/// nanoseconde of precision
pub struct TimeStamp {
    start: Instant,
}

impl TimeStamp {
    pub fn new() -> TimeStamp {
        TimeStamp {
            start: Instant::now(),
        }
    }

    pub fn now(&self) -> f64 {
        let d = self.start.elapsed();
        d.as_secs() as f64 + d.subsec_nanos() as f64 * 1e-9
    }
}

fn vectort3_to_string(v: &Vector3<f32>) -> String {
    format!(
        "[{}]",
        v.iter()
            .fold(String::new(), |acc, &num| acc + &format!("{:.2}", num)
                + ", ")
    )
}

fn vec_vector3_to_string(d: &[Vector3<f32>]) -> String {
    format!(
        "[{}]",
        d.iter()
            .fold(String::new(), |acc, &v| acc + &vectort3_to_string(&v)
                + ", ")
    )
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Message::Acc(t, i, ref d) => write!(
                f,
                "acc : {:.3}, i={:.3}s: {}",
                t,
                i,
                vec_vector3_to_string(d)
            ),
            &Message::Gyro(t, i, ref d) => write!(
                f,
                "gyro: {:.3}, i={:.3}s: {}",
                t,
                i,
                vec_vector3_to_string(d)
            ),
            &Message::Magn(t, i, ref d) => write!(
                f,
                "magn: {:.3}, i={:.3}s: {}",
                t,
                i,
                vec_vector3_to_string(d)
            ),
            // &Message::LinearVelocity(t, ref v) => {
            //     write!(f, "lvel: {:.3}, {}", t, vectort3_to_string(v))
            // }
            _ => write!(f, "TODO"),
        }
    }
}

// TODO: Manage errors:
impl Message {
    pub fn to_msg_pack(&self) -> Vec<u8> {
        to_vec_named(self).unwrap()
    }

    pub fn from_msg_pack(buf: &[u8]) -> Message {
        from_slice(buf).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use rmps::{from_slice, to_vec_named};
    use nalgebra::Vector3;
    use super::Message;

    #[test]
    fn acc_to_rpm_to_acc() {
        let a1 = Message::Acc(2.0, 0.01, vec![Vector3::new(2.1, 7.5, -4.1)]);
        let buf = a1.to_msg_pack();
        let a2 = Message::from_msg_pack(&buf);

        assert_eq!(a1, a2);
    }

    #[test]
    fn gyro_to_rpm_to_gyro() {
        let g1 = Message::Gyro(2.0, 0.01, vec![Vector3::new(2.1, 7.5, -4.1)]);
        let buf = to_vec_named(&g1).unwrap();
        let g2: Message = from_slice(&buf).unwrap();

        assert_eq!(g1, g2);
    }

}
