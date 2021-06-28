use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color::rgba(r, g, b, 255)
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.len() {
            4 => Self::from_str(&(s.to_owned() + "F")),
            5 => Self::from_str(&{
                let mut r = "#".to_owned();
                &s[1..5].chars().for_each(|c| {
                    r.push(c);
                    r.push(c);
                });
                r
            }),
            7 => Self::from_str(&(s.to_owned() + "FF")),
            9 => Ok(Color {
                r: (u8::from_str_radix(&s[1..3], 16).map_err(|_| ())?),
                g: (u8::from_str_radix(&s[3..5], 16).map_err(|_| ())?),
                b: (u8::from_str_radix(&s[5..7], 16).map_err(|_| ())?),
                a: (u8::from_str_radix(&s[7..9], 16).map_err(|_| ())?),
            }),
            _ => Err(()),
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "#{:02X}{:02X}{:02X}{:02X}",
            self.r, self.g, self.b, self.a
        )
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::colors::Color;

    #[test]
    fn encode_color() {
        assert_eq!(Color::rgb(0, 0, 0).to_string(), "#000000FF");
        assert_eq!(Color::rgba(0x12, 0x0F, 0xF0, 0xAA).to_string(), "#120FF0AA");
    }

    #[test]
    fn color_parsing() {
        // BLACK
        assert_matches!(
            Color::from_str("#000"),
            Ok(Color {
                r: 0,
                g: 0,
                b: 0,
                a: 0xFF
            })
        );
        assert_matches!(
            Color::from_str("#000F"),
            Ok(Color {
                r: 0,
                g: 0,
                b: 0,
                a: 0xFF
            })
        );
        assert_matches!(
            Color::from_str("#000000"),
            Ok(Color {
                r: 0,
                g: 0,
                b: 0,
                a: 0xFF
            })
        );
        assert_matches!(
            Color::from_str("#000000FF"),
            Ok(Color {
                r: 0,
                g: 0,
                b: 0,
                a: 0xFF
            })
        );
        // Transparent
        assert_matches!(
            Color::from_str("#0000"),
            Ok(Color {
                r: 0,
                g: 0,
                b: 0,
                a: 0
            })
        );
        // RED
        assert_matches!(
            Color::from_str("#F00"),
            Ok(Color {
                r: 0xFF,
                g: 0,
                b: 0,
                a: 0xFF
            })
        );
        assert_matches!(
            Color::from_str("#400"),
            Ok(Color {
                r: 0x44,
                g: 0,
                b: 0,
                a: 0xFF
            })
        );
        assert_matches!(
            Color::from_str("#440000"),
            Ok(Color {
                r: 0x44,
                g: 0,
                b: 0,
                a: 0xFF
            })
        );
        // RAND
        assert_matches!(
            Color::from_str("#16813554"),
            Ok(Color {
                r: 0x16,
                g: 0x81,
                b: 0x35,
                a: 0x54
            })
        );
    }
}
