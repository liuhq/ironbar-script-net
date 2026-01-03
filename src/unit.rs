use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SpeedUnit {
    Bps,
    KBps,
    MBps,
    GBps,
}

impl SpeedUnit {
    pub const fn as_str(self) -> &'static str {
        match self {
            SpeedUnit::Bps => "B/s",
            SpeedUnit::KBps => "KB/s",
            SpeedUnit::MBps => "MB/s",
            SpeedUnit::GBps => "GB/s",
        }
    }

    pub const fn factor(self) -> f64 {
        match self {
            SpeedUnit::Bps => 1.0,
            SpeedUnit::KBps => 1024.0,
            SpeedUnit::MBps => 1024_i32.pow(2) as f64,
            SpeedUnit::GBps => 1024_i32.pow(3) as f64,
        }
    }

    pub const ALL: [SpeedUnit; 4] = [
        SpeedUnit::Bps,
        SpeedUnit::KBps,
        SpeedUnit::MBps,
        SpeedUnit::GBps,
    ];
}

impl fmt::Display for SpeedUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Speed {
    pub value: f64,
    pub unit: SpeedUnit,
}

impl Speed {
    pub fn to_human_speed(bytes_per_sec: f64) -> Speed {
        let unit = SpeedUnit::ALL
            .iter()
            .rfind(|u| bytes_per_sec >= u.factor())
            .copied()
            .unwrap_or(SpeedUnit::Bps);

        Speed {
            value: bytes_per_sec / unit.factor(),
            unit,
        }
    }
}

impl fmt::Display for Speed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let digits = f.precision().unwrap_or(1);
        write!(f, "{:.*} {}", digits, self.value, self.unit)
    }
}
