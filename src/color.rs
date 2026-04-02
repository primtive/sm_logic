pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}
impl Color {
    pub const SINGLE: Self = Self {
        r: 30,
        g: 30,
        b: 30,
    };
    pub const INPUT1: Self = Self { r: 0, g: 0, b: 255 };
    pub const INPUT2: Self = Self { r: 0, g: 0, b: 150 };
    pub const OUTPUT1: Self = Self { r: 255, g: 0, b: 0 };
    pub const OUTPUT2: Self = Self { r: 150, g: 0, b: 0 };
    pub const DISPLAY: Self = Self { r: 0, g: 0, b: 0 };
    // const RED: Self = Self { r: 255, g: 0, b: 0 };
    pub fn to_string(&self) -> String {
        hex::encode_upper(&vec![self.r, self.g, self.b])
    }
}
