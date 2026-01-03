use crate::{read::NetBytes, unit::Speed};

pub struct NetSpeed {
    pub rx: Speed,
    pub tx: Speed,
}

impl NetSpeed {
    pub fn diff_speed(prev: NetBytes, curr: NetBytes) -> Self {
        let dt = (curr.time - prev.time) as f64 / 1000.0;

        NetSpeed {
            rx: Speed::to_human_speed((curr.rx - prev.rx) as f64 / dt),
            tx: Speed::to_human_speed((curr.tx - prev.tx) as f64 / dt),
        }
    }
}
