use crate::error::Error::SERVER_ColorHexInvalid;
use crate::error::ServerResult;

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        return Self { r, g, b, a };
    }

    pub fn from_hex(hex_color: &str) -> ServerResult<Color> {
        if hex_color.len() != 7 || !hex_color.starts_with("#") {
            return Err(SERVER_ColorHexInvalid);
        }

        let hex_values = &hex_color[1..]; // Skip the '#' character
        let r_str = &hex_values[0..2];
        let g_str = &hex_values[2..4];
        let b_str = &hex_values[4..6];

        if let (Ok(r), Ok(g), Ok(b)) = (
            u8::from_str_radix(r_str, 16),
            u8::from_str_radix(g_str, 16),
            u8::from_str_radix(b_str, 16),
        ) {
            Ok(Color { r, g, b, a: 255 })
        } else {
            Err(SERVER_ColorHexInvalid)
        }
    }

    pub fn from_hex_with_opacity(hex_color: &str) -> ServerResult<Color> {
        if hex_color.len() != 9 || !hex_color.starts_with("#") {
            return Err(SERVER_ColorHexInvalid);
        }

        let hex_values = &hex_color[1..]; // Skip the '#' character
        let r_str = &hex_values[0..2];
        let g_str = &hex_values[2..4];
        let b_str = &hex_values[4..6];
        let a_str = &hex_values[6..8];

        if let (Ok(r), Ok(g), Ok(b), Ok(a)) = (
            u8::from_str_radix(r_str, 16),
            u8::from_str_radix(g_str, 16),
            u8::from_str_radix(b_str, 16),
            u8::from_str_radix(a_str, 16),
        ) {
            Ok(Color { r, g, b, a })
        } else {
            Err(SERVER_ColorHexInvalid)
        }
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    pub fn to_hex_with_opacity(&self) -> String {
        format!("#{:02X}{:02X}{:02X}{:02X}", self.r, self.g, self.b, self.a)
    }
}
