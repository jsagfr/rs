extern crate rmp_serde as rmps;
#[macro_use]
extern crate serde_derive;
extern crate nalgebra;

use nalgebra::Vector3;
use rmps::{to_vec_named, from_slice};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum Message {
    Acc(u64, Vec<Vector3<f32>>),
    Gyro(u64, Vec<Vector3<f32>>),
    Magn(u64, Vec<Vector3<f32>>),
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

// pub struct Samples<T> {
//     n1: u64,                    // Number of the first sample
//     data: Vec<T>,
// }

#[cfg(test)]
mod tests {
    use rmps::{to_vec_named, from_slice};
    use nalgebra::Vector3;
    use super::Message;

    #[test]
    fn acc_to_rpm_to_acc() {
        let a1 = Message::Acc(2, vec![Vector3::new(2.1, 7.5, -4.1)]);
        let buf = a1.to_msg_pack();
        let a2 = Message::from_msg_pack(&buf);

        assert_eq!(a1, a2);
    }

    #[test]
    fn gyro_to_rpm_to_gyro() {
        let g1 = Message::Gyro(2, vec![Vector3::new(2.1, 7.5, -4.1)]);
        let buf = to_vec_named(&g1).unwrap();
        let g2: Message = from_slice(&buf).unwrap();

        assert_eq!(g1, g2);
    }

}
