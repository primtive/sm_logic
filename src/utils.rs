use std::fmt::Display;

pub type Id = u32;
#[derive(Debug, Hash, Clone, Copy, PartialOrd)]
pub struct SignalName(pub char, pub u8);
#[macro_export]
macro_rules! sn {
    ($c:literal) => {
        SignalName($c, 0)
    };
    ($c:expr) => {
        SignalName($c, 0)
    };
    ($c:expr, $n:expr) => {
        SignalName($c, $n)
    };
    ($c:literal, $n:expr) => {
        SignalName($c, $n)
    };
}

impl Ord for SignalName {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.0 == other.0 && self.1 == other.1 {
            std::cmp::Ordering::Equal
        } else if self.0 > other.0 || (self.0 == other.0 && self.1 > other.1) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    }
}

impl PartialEq for SignalName {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0) && (self.1 == other.1)
    }
}
impl Eq for SignalName {}
impl Display for SignalName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

pub const fn gen_test_rom_vec() -> [u32; 1024] {
    let mut arr = [0u32; 1024];
    let mut i = 0;
    while i < 1024 {
        arr[i] = (i * 4) as u32;
        i += 1;
    }
    arr
}
