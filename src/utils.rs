use std::fmt::Display;

pub type Id = u32;
#[derive(Debug)]
pub struct SignalName(pub char, pub u8);
#[macro_export]
macro_rules! sn {
    ($c:literal) => {
        SignalName($c, 0)
    };
    ($c:literal, $n:expr) => {
        SignalName($c, $n)
    };
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
